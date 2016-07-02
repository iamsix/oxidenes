use super::*;
use mem_map::*;
use opcodes::*;
// use std::collections::HashSet;

// pub HashMap: ops;


#[derive(Debug)]
pub struct CPU {
    pub cycle: isize,

    pub accumulator: u8, // A

    pub index_x: u8, // X
    pub index_y: u8, // Y

    pub status_reg: StatusReg, // P
    pub program_counter: u16, // PC - should be PCHI/PCLO but easier this way
    pub stack_pointer: u8, // S or SP

    pub bus: Bus,
}

#[derive(Debug, Clone, Copy)]
pub struct StatusReg {
    negative_sign: bool, // N (or sometimes S)
    overflow: bool, // V
    unused: bool, // always 1
    break_flag: bool, // B
    decimal_mode: bool, // D - unimplemented on NES but still sets/clears
    interrupt_disable: bool, // I
    zero: bool, // Z
    carry: bool, // C
}


#[derive(Debug)]
enum RegType {
    A,
    X,
    Y,
}

pub const PPU_MULTIPLIER:isize = 3;

impl CPU {
    pub fn new(bus: Bus, pc: u16) -> CPU {
        CPU {
            cycle: 0,

            accumulator: 0,
            index_x: 0,
            index_y: 0,
            //      nestest.nes uses the wrong start status reg of 0x24 (brk false) instead of 0x34 (brk true)
            //      then improperly checks against that 'dirty' value instead of setting its own
            //        status_reg: 0x34.into(),
            status_reg: 0x24.into(),
            program_counter: pc,
            stack_pointer: 0xfd,
            bus: bus,
        }
    }

    pub fn read_instruction(&mut self) -> (u8, Instruction) {
        let pc = self.program_counter;
        let op = self.cpu_read_u8(pc);
        let mut instr = INSTRUCTIONS[op as usize];

        let operand = if instr.bytes == 3 {
            Some(self.cpu_read_u16(pc + 1))
        } else if instr.bytes == 2 {
            Some(self.cpu_read_u8(pc + 1) as u16)
        } else {
            None
        };

        if operand != None {
            instr.operand = operand.unwrap();
        }

        self.program_counter += instr.bytes as u16;

        let mut page_crossed = false;
        let mut branching = false;
        let dataaddr:Option<u16> = match instr.addr_mode {
            AddressMode::Immediate => Some(pc + 1),
            AddressMode::Absolute => Some(operand.unwrap()),
            AddressMode::AbsoluteX => {
                let addr = operand.unwrap().wrapping_add(self.index_x as u16);
                if instr.page_boundary_cycle && (operand.unwrap() >> 8 != addr >> 8) {
                     page_crossed = true;
                }
                Some(addr)
            }
            AddressMode::AbsoluteY => {
                let addr = operand.unwrap().wrapping_add(self.index_y as u16);
                if instr.page_boundary_cycle && (operand.unwrap() >> 8 != addr >> 8) {
                     page_crossed = true;
                }
                Some(addr)
            }
            AddressMode::XIndirect => {
                let mut tmp = operand.unwrap() as u8;
                tmp = tmp.wrapping_add(self.index_x);
                let lo = self.cpu_read_u8(tmp as u16) as u16;
                let hi = self.cpu_read_u8(tmp.wrapping_add(1) as u16) as u16;
                Some(hi << 8 | lo)
            }
            AddressMode::IndirectY => {
                let tmp = operand.unwrap() as u8;
                let lo = self.cpu_read_u8(tmp as u16) as u16;
                let hi = self.cpu_read_u8(tmp.wrapping_add(1) as u16) as u16;
                let value: u16 = hi << 8 | lo;
                let addr = value.wrapping_add(self.index_y as u16);
                if instr.page_boundary_cycle && (hi != addr >> 8) {
                    page_crossed = true;
                }
                Some(addr)
            }
            AddressMode::Zeropage => Some(operand.unwrap()),
            AddressMode::ZeropageX => {
                let tmp = operand.unwrap() as u8;
                Some(tmp.wrapping_add(self.index_x) as u16)
            }
            AddressMode::ZeropageY => {
                let tmp = operand.unwrap() as u8;
                Some(tmp.wrapping_add(self.index_y) as u16)
            }
            AddressMode::Indirect => Some(pc + 1),
            AddressMode::Relative => {

                // we have to evaluate the whole branch during this phase
                // to accurately count the CPU ticks
                branching = match op {
                    // BVC - relative - 2 bytes
                    0x50 => !self.status_reg.overflow,
                    // BVS - relative - 2 bytes
                    0x70 => self.status_reg.overflow,
                    // BCS - relative - 2 bytes
                    0xB0 => self.status_reg.carry,
                    // BCC - relative - 2 bytes
                    0x90 => !self.status_reg.carry,
                    // BEQ - relative
                    0xF0 => self.status_reg.zero,
                    // BNE - relative
                    0xD0 => !self.status_reg.zero,
                    // BPL - relative
                    0x10 => !self.status_reg.negative_sign,
                    // BMI - rel
                    0x30 => self.status_reg.negative_sign,

                    _ => panic!("instruction {:#X} should not be relative", op)

                };

                // wrapping? not sure how signed deal with those...
                if branching {
                    let tmp = operand.unwrap() as i8 as i16;
                    let addr = (self.program_counter as i16 + tmp) as u16;
                    if addr >> 8 != (self.program_counter) >> 8 {
                        page_crossed = true;
                    };
                    Some(addr)
                } else {
                    Some(self.program_counter)
                }

            }

            _ => None,
        };

        if dataaddr != None {
            instr.dest_addr = dataaddr;
        }
        // increment the cycle counter as-needed.
        // self.cycle += instr.ticks as isize* PPU_MULTIPLIER;
        if page_crossed {
            instr.ticks += 1;
        }

        if branching {
            instr.ticks += 1;
        }



        (op, instr)
    }

    pub fn nmi (&mut self) {
        let hi = (self.program_counter >> 8) as u8;
        self.push_stack(hi);
        let lo = (0x00ff & self.program_counter) as u8;
        self.push_stack(lo);
        let sr: u8 = self.status_reg.into();
        self.push_stack(sr);
 //       println!("NMI");
        let tmp = self.cpu_read_u16(NMI_VECTOR_LOC);
        self.program_counter = tmp;
        // need to add some cycles here..
        self.cycle += 7  * PPU_MULTIPLIER;
    }

    pub fn irq (&mut self) {
        // println!("IRQ test");
        if !self.status_reg.interrupt_disable {
            // println!("IRQ");
            let hi = (self.program_counter >> 8) as u8;
            self.push_stack(hi);
            let lo = (0x00ff & self.program_counter) as u8;
            self.push_stack(lo);
            let sr: u8 = self.status_reg.into();
            self.push_stack(sr);
     //       println!("NMI");
            let tmp = self.cpu_read_u16(IRQ_BRK_VECTOR_LOC);
            self.program_counter = tmp;
            // need to add some cycles here..
            self.cycle += 7  * PPU_MULTIPLIER;
        }
    }


    pub fn execute_op(&mut self, op: &u8, instr: &Instruction) {

        let addr = instr.dest_addr.unwrap_or(0);
        // self.program_counter += 1;
        match *op {

            0x4C => {
                // JMP-absolute
                // let value = instr.operand;
                self.program_counter = instr.operand;
            }

            0x6C => {
                // JMP-indirect
                // let pc = self.program_counter;
                let lotmp = self.cpu_read_u8(addr);
                let hitmp = self.cpu_read_u8(addr + 1);
                // because there is no carry the lo byte of effective addr wraps +1
                // see http://www.6502.org/tutorials/6502opcodes.html under JMP

                let mut tmp = (hitmp as u16) << 8 | lotmp as u16;
                let lo = self.cpu_read_u8(tmp) as u16;
                tmp = (hitmp as u16) << 8 | lotmp.wrapping_add(1) as u16;
                let hi = self.cpu_read_u8(tmp) as u16;
                //            println!("ind: {:#X} JMP ${:X}${:X}", tmp, hi, lo);
                self.program_counter = hi << 8 | lo;
            }

//          BVC  | BVS  | BCS  | BCC  | BEQ  | BNE  | BPL  | BMI
            0x50 | 0x70 | 0xB0 | 0x90 | 0xF0 | 0xD0 | 0x10 | 0x30 =>
                self.program_counter = addr,

            // RTI - implied
            0x40 => {
                let tmp = self.pull_stack();
                self.status_reg = tmp.into();
                let lo = self.pull_stack() as u16;
                let hi = self.pull_stack() as u16;
                let value: u16 = hi << 8 | lo;
                self.program_counter = value;
            }

            // STX
            0x86 | 0x96 | 0x8E => {
                let tmp = self.index_x;
                self.cpu_write_u8(addr, tmp);
            }

            // STA
            0x85 | 0x95 | 0x8D | 0x81 | 0x91 | 0x99 | 0x9D => {
                let tmp = self.accumulator;
                self.cpu_write_u8(addr, tmp);
            }

            // STY - zeropage
            0x84 | 0x94 | 0x8C => {
                let tmp = self.index_y;
                self.cpu_write_u8(addr, tmp);
            }

            // LDA
            0xA9 | 0xA5 | 0xB5 | 0xAD | 0xB9 | 0xBD | 0xA1 | 0xB1 => {
                let value = self.cpu_read_u8(addr);
                self.set_register(value, RegType::A);
            }

            // LDX
            0xA2 | 0xAE | 0xBE | 0xA6 | 0xB6 => {
                let value = self.cpu_read_u8(addr);
                self.set_register(value, RegType::X);
            }

            // LDY-immediate
            0xA0 | 0xA4 | 0xAC | 0xBC | 0xB4 => {
                let value = self.cpu_read_u8(addr);
                self.set_register(value, RegType::Y);
            }

            // LSR
            0x4A | 0x46 | 0x56 | 0x4E | 0x5E => self.shift_right(instr.addr_mode, addr),

            // ASL
            0x0A | 0x06 | 0x16 | 0x0E | 0x1E => self.shift_left(instr.addr_mode, addr),

            // ROR
            0x6A | 0x66 | 0x76 | 0x6E | 0x7E => self.rotate_right(instr.addr_mode, addr),

            // ROL
            0x2A | 0x26 | 0x36 | 0x2E | 0x3E => self.rotate_left(instr.addr_mode, addr),

            // ORA
            0x09 | 0x05 | 0x15 | 0x01 | 0x11 | 0x0D | 0x19 | 0x1D =>
                self.bitwise_op_to_a(|a, m| a | m, addr),

            // EOR
            0x49 | 0x45 | 0x55 | 0x41 | 0x51 | 0x4D | 0x59 | 0x5D =>
                self.bitwise_op_to_a(|a, m| a ^ m, addr),

            // AND
            0x29 | 0x25 | 0x35 | 0x21 | 0x31 | 0x2D | 0x39 | 0x3D =>
                self.bitwise_op_to_a(|a, m| a & m, addr),

            // ADC
            0x69 | 0x65 | 0x75 | 0x61 | 0x71 | 0x6D | 0x79 | 0x7D =>
                self.add_with_carry(addr),

            // SBC
            0xE9 | 0xE5 | 0xF5 | 0xE1 | 0xF1 | 0xED | 0xF9 | 0xFD =>
                self.sub_with_carry(addr),

            // JSR-Absolute
            0x20 => {
                let hi = ((self.program_counter - 1) >> 8) as u8;
                self.push_stack(hi);
                let lo = (0x00ff & (self.program_counter - 1)) as u8;
                self.push_stack(lo);

                self.program_counter = instr.operand;
            }

            // RTS - implied
            0x60 => {
                let lo = self.pull_stack() as u16;
                let hi = self.pull_stack() as u16;
                let value: u16 = hi << 8 | lo;
                self.program_counter = value + 1;
            }

            // NOP
            0xEA => {}


            // TAY - impl
            0xA8 => {
                let tmp = self.accumulator;
                self.set_register(tmp, RegType::Y);
            }

            // TAX - impl
            0xAA => {
                let tmp = self.accumulator;
                self.set_register(tmp, RegType::X);
            }

            // TYA - impl
            0x98 => {
                let tmp = self.index_y;
                self.set_register(tmp, RegType::A)
            }

            // TXA - impl
            0x8A => {
                let tmp = self.index_x;
                self.set_register(tmp, RegType::A)
            }

            // TSX -impl
            0xBA => {
                let tmp = self.stack_pointer;
                self.set_register(tmp, RegType::X)
            }

            // TXS -impl
            0x9A => self.stack_pointer = self.index_x,

            // INY - impl
            0xC8 => {
                let tmp = self.index_y.wrapping_add(1);
                self.set_register(tmp, RegType::Y)
            }

            // INX - impl
            0xE8 => {
                let tmp = self.index_x.wrapping_add(1);
                self.set_register(tmp, RegType::X)
            }

            // DEY - impl
            0x88 => {
                let tmp = self.index_y.wrapping_sub(1);
                self.set_register(tmp, RegType::Y)
            }

            // DEX - impl
            0xCA => {
                let tmp = self.index_x.wrapping_sub(1);
                self.set_register(tmp, RegType::X)
            }

            // INC
            0xE6 | 0xF6 | 0xEE | 0xFE => self.increment_memory(addr),

            // DEC - zeropage
            0xC6 | 0xD6 | 0xCE | 0xDE => self.decrement_memory(addr),

            // BIT
            0x24 | 0x2C => {
                let value = self.cpu_read_u8(addr);
                let result = self.accumulator & value;
                //            println!("BIT {:#x} & {:#x}: {:#x}", self.accumulator, value, result);
                self.status_reg.zero = result == 0;
                self.status_reg.negative_sign = (value & (1 << 7)) != 0;
                self.status_reg.overflow = (value & (1 << 6)) != 0;
            }


            // SEI - impl
            0x78 => self.status_reg.interrupt_disable = true,

            // CLI - impl
            0x58 => self.status_reg.interrupt_disable = false,

            // SED - impl
            0xF8 => self.status_reg.decimal_mode = true,

            // CLD - impl
            0xD8 => self.status_reg.decimal_mode = false,

            // SEC - implied
            0x38 => self.status_reg.carry = true,

            // CLC - implied
            0x18 => self.status_reg.carry = false,

            // CLV - impl
            0xB8 => self.status_reg.overflow = false,

            // PHP - impl
            0x08 => {
                let mut tmp: u8 = self.status_reg.into();
                tmp |= 1 << 4; // set the break flag before pushing
                self.push_stack(tmp);
            }

            // PLP - impl
            0x28 => {
                let mut value = self.pull_stack();
                let brk = if self.status_reg.break_flag {
                    1
                } else {
                    0
                };
                value ^= (brk ^ value) & (1 << 4);
                self.status_reg = value.into();
            }

            // PLA - impl
            0x68 => {
                let value = self.pull_stack();
                self.set_register(value, RegType::A);
            }

            // PHA - impl
            0x48 => {
                let tmp = self.accumulator;
                self.push_stack(tmp);
            }


            // CMP
            0xC9 | 0xC5 | 0xD5 | 0xC1 | 0xD1 | 0xCD | 0xD9 | 0xDD =>
                self.compare(RegType::A, addr),

            // CPY
            0xC0 | 0xC4 | 0xCC => self.compare(RegType::Y, addr),

            // CPX - immediate
            0xE0 | 0xE4 | 0xEC => self.compare(RegType::X, addr),


            // Illegal/undocumented opcodes - these do unusual things..
            //
            //

            // NOP - undocumented opcode
            0x1A | 0x3A | 0x5A | 0x7A | 0xDA | 0xFA => {} // println!("NOP - impl - ndocumented Opcode ${:X}", instr),

            // DOP / NOP / SKB - undocumented Opcode
            0x04 | 0x44 | 0x64 | 0x80 | 0x82 | 0x89 | 0xC2 | 0xE2 | 0x14 | 0x34 |
            0x54 | 0x74 | 0xD4 | 0xF4 => {}

            // TOP / NOP / SKW - undocumented opcode
            0x0C | 0x1C | 0x3C | 0x5C | 0x7C | 0xDC | 0xFC => {}

            // LAX - Ind,x - Undocumented Opcode
            0xA3 | 0xB3 | 0xA7 | 0xB7 | 0xAF | 0xBF => {
                let value = self.cpu_read_u8(addr);
                self.set_register(value, RegType::A);
                self.set_register(value, RegType::X);
            }

            // AAX / SAX / AXS - Undocumented Opcode
            0x83 | 0x87 | 0x97 | 0x8F => self.and_x_a_store(addr),

            // SBC - imm - Undocumented opcode (identical to E9)
            0xEB => self.sub_with_carry(addr),

            // DCP / DCM - Undocumented Opcode
            0xC7 | 0xD7 | 0xCF | 0xDF | 0xDB | 0xC3 | 0xD3 => {
                self.decrement_memory(addr);
                self.compare(RegType::A, addr);
            }

            // ISC / ISB / INS - undocumented
            0xE7 | 0xF7 | 0xEF | 0xFF | 0xFB | 0xE3 | 0xF3 => {
                self.increment_memory(addr);
                self.sub_with_carry(addr);
            }

            // SLO / ASO - undocumented
            0x07 | 0x17 | 0x0F | 0x1F | 0x1B | 0x03 | 0x13 => {
                self.shift_left(instr.addr_mode, addr);
                self.bitwise_op_to_a(|a, m| a | m, addr);
            }

            // RLA - undocumented
            0x27 | 0x37 | 0x2F | 0x3F | 0x3B | 0x23 | 0x33 => {
                self.rotate_left(instr.addr_mode, addr);
                self.bitwise_op_to_a(|a, m| a & m, addr);
            }

            // SRE / LSE - undocumented
            0x47 | 0x57 | 0x4F | 0x5F | 0x5B | 0x43 | 0x53 => {
                self.shift_right(instr.addr_mode, addr);
                self.bitwise_op_to_a(|a, m| a ^ m, addr);
            }

            // RRA - undocumented
            0x67 | 0x77 | 0x6F | 0x7F | 0x7B | 0x63 | 0x73 => {
                self.rotate_right(instr.addr_mode, addr);
                self.add_with_carry(addr);
            },

            // BRK
            0x00 => {
                self.status_reg.break_flag = true;

//                self.program_counter += 1;
                self.status_reg.interrupt_disable = true;
                let hi = (self.program_counter >> 8) as u8;
                self.push_stack(hi);
                let lo = (0x00ff & self.program_counter) as u8;
                self.push_stack(lo);
                let sr: u8 = self.status_reg.into();
                self.push_stack(sr);
                println!("Break");
                let tmp = self.cpu_read_u16(IRQ_BRK_VECTOR_LOC);
                self.program_counter = tmp;
            }

            _ => panic!("The opcode: {:#x} is unrecognized", op),
        }
    }

    // does NOT effect flags
    fn and_x_a_store(&mut self, addr: u16) {
        let value = self.index_x & self.accumulator;
        self.cpu_write_u8(addr, value)
    }

    fn rotate_right(&mut self, addr_mode: AddressMode, addr: u16) {
        let value = if addr_mode == AddressMode::Accumulator {
            self.accumulator
        } else {
            self.cpu_read_u8(addr)
        };

        let c = if self.status_reg.carry {
            1
        } else {
            0
        };
        self.status_reg.carry = (value & (1 << 0)) != 0;
        let value = (value >> 1) | c << 7;
        self.status_reg.zero = value == 0;
        self.status_reg.negative_sign = (value & (1 << 7)) != 0;

        if addr_mode == AddressMode::Accumulator {
            self.set_register(value, RegType::A);
        } else {
            self.cpu_write_u8(addr, value);
        }
    }

    fn rotate_left(&mut self, addr_mode: AddressMode, addr: u16) {
        let value = if addr_mode == AddressMode::Accumulator {
            self.accumulator
        } else {
            self.cpu_read_u8(addr)
        };

        let c = if self.status_reg.carry {
            1
        } else {
            0
        };
        self.status_reg.carry = (value & (1 << 7)) != 0;
        let value = (value << 1) | c << 0;
        self.status_reg.zero = value == 0;
        self.status_reg.negative_sign = (value & (1 << 7)) != 0;

        if addr_mode == AddressMode::Accumulator {
            self.set_register(value, RegType::A);
        } else {
            self.cpu_write_u8(addr, value);
        }
    }

    fn shift_right(&mut self, addr_mode: AddressMode, addr: u16) {
        let value = if addr_mode == AddressMode::Accumulator {
            self.accumulator
        } else {
            self.cpu_read_u8(addr)
        };

        self.status_reg.carry = (value & (1 << 0)) != 0;
        let value = value >> 1;
        self.status_reg.zero = value == 0;
        self.status_reg.negative_sign = (value & (1 << 7)) != 0;

        if addr_mode == AddressMode::Accumulator {
            self.set_register(value, RegType::A);
        } else {
            self.cpu_write_u8(addr, value);
        }
    }

    fn shift_left(&mut self, addr_mode: AddressMode, addr: u16) {
        let value = if addr_mode == AddressMode::Accumulator {
            self.accumulator
        } else {
            self.cpu_read_u8(addr)
        };

        self.status_reg.carry = (value & (1 << 7)) != 0;
        let value = ((value as u16) << 1) as u8;
        self.status_reg.zero = value == 0;
        self.status_reg.negative_sign = (value & (1 << 7)) != 0;

        if addr_mode == AddressMode::Accumulator {
            self.set_register(value, RegType::A);
        } else {
            self.cpu_write_u8(addr, value);
        }
    }

    fn bitwise_op_to_a<F>(&mut self, f: F, addr: u16)
        where F: FnOnce(u8, u8) -> u8
    {

        let m = self.cpu_read_u8(addr);
        let a = self.accumulator;
        let value = f(a, m);
        self.set_register(value, RegType::A);
    }

    fn add_with_carry(&mut self, addr: u16) {
        let value = self.cpu_read_u8(addr) as u16;
        let a = self.accumulator as u16;
        let c = if self.status_reg.carry {
            1
        } else {
            0
        };

        let result = a + value + c;
        //        println!("ADC: A{:#X} + M{:#X} + C{:#X} = {:X}", a, value, c, result);
        self.status_reg.carry = result > 0xff;
        self.status_reg.overflow = ((a ^ result) & (value ^ result) & 0x80) != 0;
        self.set_register(result as u8, RegType::A);
    }

    // implemented as binary add with 1s(ones) compliment of the value being sub from A
    fn sub_with_carry(&mut self, addr: u16) {
        let value = self.cpu_read_u8(addr) as u16;
        let a = self.accumulator as u16;
        let c = if self.status_reg.carry {
            1
        } else {
            0
        };

        let result = a + (0xff - value) + c;
        // println!("SBC: A{:#X} + (0xff-M{:#X}) + C{:#X} = {:X}", a, value, c, result);
        self.status_reg.carry = result > 0xff;
        self.status_reg.overflow = ((a ^ result) & ((0xff - value) ^ result) & 0x80) != 0;
        self.set_register(result as u8, RegType::A);
    }

    fn increment_memory(&mut self, addr: u16) {
        let mut value = self.cpu_read_u8(addr);
        value = value.wrapping_add(1);
        self.status_reg.zero = value == 0;
        self.status_reg.negative_sign = (value & (1 << 7)) != 0;
        self.cpu_write_u8(addr, value);
    }

    fn decrement_memory(&mut self, addr: u16) {
        let mut value = self.cpu_read_u8(addr);
        value = value.wrapping_sub(1);
        self.status_reg.zero = value == 0;
        self.status_reg.negative_sign = (value & (1 << 7)) != 0;
        self.cpu_write_u8(addr, value);
    }

    fn compare(&mut self, reg: RegType, addr: u16) {
        let register = match reg {
            RegType::A => self.accumulator as i16,
            RegType::Y => self.index_y as i16,
            RegType::X => self.index_x as i16,
        };
        let value = self.cpu_read_u8(addr) as i16;
        //        println!("CMY {:#X} - {:#X}", register, value);
        let result = (register - value) as u8;
        self.status_reg.zero = register == value;
        self.status_reg.negative_sign = (result & (1 << 7)) != 0;
        self.status_reg.carry = register >= value;
    }

    // setting the accumulator always sets N and Z appropriately
    fn set_register(&mut self, value: u8, reg: RegType) {
        self.status_reg.zero = value == 0;
        self.status_reg.negative_sign = (value & (1 << 7)) != 0;
        match reg {
            RegType::A => self.accumulator = value,
            RegType::X => self.index_x = value,
            RegType::Y => self.index_y = value,
        }
    }

    fn push_stack(&mut self, value: u8) {
        let addr = 0x100 + self.stack_pointer as u16;
        self.cpu_write_u8(addr, value);
        // println!("stack wrote at 0x01{:x}: {:x}", self.stack_pointer, value);
        self.stack_pointer = self.stack_pointer.wrapping_sub(1);
    }

    fn pull_stack(&mut self) -> u8 {
        self.stack_pointer += 1;
        let tmp = 0x100 + self.stack_pointer as u16;
        self.cpu_read_u8(tmp)
    }

    pub fn cpu_read_u8(&mut self, mut addr: u16) -> u8 {
        // println!("Read {:#X}", addr);
        if addr > 0x2007 && addr < 0x4000 {
            addr = 0x2000 + ((addr - 0x2000) % 8)
        }
        match addr {
            RAM_START...RAM_VIRTUAL_END => {
                let addr = addr % RAM_LEN;

/*              some mario cheats...
                if addr == 0x075A {
                    return 30 // lives
                }
                if addr == 0x079F {
                    return 0xff // star
                }
                */

                self.bus.ram[addr as usize]
            }

            PPUCTRL => self.bus.ppu.lastwrite,
            PPUMASK => self.bus.ppu.lastwrite,
            OAMADDR => self.bus.ppu.lastwrite,
            PPUSCROLL => self.bus.ppu.lastwrite,
            PPUADDR => self.bus.ppu.lastwrite,
            PPUSTATUS => self.bus.ppu.read_ppustatus(),
            PPUDATA => self.bus.ppu.read_ppudata(),
            OAMDATA => self.bus.ppu.read_oamdata(),

            SND_CHN => self.bus.apu.read_status_reg(),
            // TODO: implement joysticks
            JOY1 => self.bus.joy.read_joy1(),
            JOY2 => 0,

            EXPANSION_ROM_START...PRG_ROM_END => self.bus.cart.read_cart_u8(addr),

            _ => panic!("Invalid read location {:#X}", addr),
        }
    }

    // The actual 6502 can't read a u16, this is for convenince only
    fn cpu_read_u16(&self, mut addr: u16) -> u16 {
        if addr > 0x2007 && addr < 0x4000 {
            addr = 0x2000 + ((addr - 0x2000) % 8)
        }
        // println!("Read {:#X}", addr);
        match addr {
            RAM_START...RAM_VIRTUAL_END => {
                let addr = addr % RAM_LEN;
                let lo = self.bus.ram[addr as usize] as u16;
                let hi = self.bus.ram[(addr as usize) + 1] as u16;
                hi << 8 | lo
            }


            EXPANSION_ROM_START...PRG_ROM_END => self.bus.cart.read_cart_u16(addr),

            _ => panic!("Invalid u16 read location {:#X}", addr),
        }
    }

    fn cpu_write_u8(&mut self, mut addr: u16, value: u8) {

        if addr > 0x2007 && addr < 0x4000 {
            addr = 0x2000 + ((addr - 0x2000) % 8)
        }

        match addr {
            RAM_START...RAM_VIRTUAL_END => {
                let addr = addr % RAM_LEN;
                if addr > 0x1FF && addr < 0x300 {
                    // println!("Wrote {:#X} at {:#X} in RAM", value, addr);
                };
                self.bus.ram[addr as usize] = value
            }

//            PPU_REGISTERS_START...PPU_REGISTERS_VIRTUAL_END => panic!("PPU is unimplemented"),


            PPUCTRL => self.bus.ppu.write_ppuctrl(value),
            PPUMASK => self.bus.ppu.write_ppumask(value),
            PPUSTATUS => {},
            OAMADDR => self.bus.ppu.write_oamaddr(value),
            OAMDATA => self.bus.ppu.write_oamdata(value),
            PPUSCROLL => self.bus.ppu.write_ppuscroll(value),
            PPUADDR => self.bus.ppu.write_ppuaddr(value),
            PPUDATA => self.bus.ppu.write_ppudata(value),

            APU_REGISTERS_START...APU_REGISTERS_END | FRAME_TIMER => {
                self.bus.apu.write(addr, value);
            }

            SND_CHN => self.bus.apu.write_status_reg(value, &self.bus.cart),

            OAMDMA => {
                // println!("OAMDMA at {:#X}", value);
                for i in 0..0x100 {
                    // println!("OAMDMA");
                    let ramaddr:usize = (value as usize) << 8 | i;
                    let data = self.bus.ram[ramaddr];
                    // println!("ramaddr: {:#X} data: {:#X}", ramaddr, data);
                    self.bus.ppu.write_oamdata(data);
                }

                self.cycle += 513 * PPU_MULTIPLIER;
                self.cycle %= 341;
                self.bus.ppu.cycles = self.cycle;
                self.bus.ppu.scanline += 5;

                // unlike the ppu the apu actually needs to tick instead of faking it.
                self.bus.apu.tick(513, &self.bus.cart);


            }

            JOY1 => self.bus.joy.strobe_joy(value),

            EXPANSION_ROM_START...PRG_ROM_END => {
                self.bus.cart.write_cart_u8(addr, value, &mut self.bus.ppu.chr);
                // self.bus.ppu.chr.write_mapper(addr, value);
            }

            _ => {panic!("Invalid write location {:#X}", addr);},
        }
    }

    // peek_stack??
}



impl From<u8> for StatusReg {
    fn from(value: u8) -> Self {
        StatusReg {
            negative_sign: (value & (1 << 7)) != 0, // N
            overflow: (value & (1 << 6)) != 0, // V
            unused: true,
            break_flag: (value & (1 << 4)) != 0, // B
            decimal_mode: (value & (1 << 3)) != 0, // D
            interrupt_disable: (value & (1 << 2)) != 0, // I
            zero: (value & (1 << 1)) != 0, // Z
            carry: (value & (1 << 0)) != 0, // C
        }
    }
}

impl Into<u8> for StatusReg {
    fn into(self) -> u8 {
        let mut value: u8 = 0;

        if self.negative_sign {
            value = value | 1 << 7
        }
        if self.overflow {
            value = value | 1 << 6
        }
        if self.unused {
            value = value | 1 << 5
        }
        if self.break_flag {
            value = value | 1 << 4
        }
        if self.decimal_mode {
            value = value | 1 << 3
        }
        if self.interrupt_disable {
            value = value | 1 << 2
        }
        if self.zero {
            value = value | 1 << 1
        }
        if self.carry {
            value = value | 1 << 0
        }

        value
    }
}

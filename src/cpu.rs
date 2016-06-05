use super::*;
use mem_map::*;
// use std::collections::HashSet;

// pub HashMap: ops;
const PPU_MULTIPLIER:usize = 3;

#[derive(Debug)]
pub struct CPU {
    cycle: usize,

    accumulator: u8, // A

    index_x: u8, // X
    index_y: u8, // Y

    status_reg: StatusReg, // P
    program_counter: u16, // PC - should be PCHI/PCLO but easier this way
    stack_pointer: u8, // S or SP

    bus: Bus,
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

#[derive(Debug, Clone, Copy, PartialEq)]
enum AddressMode {
    Accumulator,
    Absolute,
    AbsoluteX,
    AbsoluteY,
    Immediate,
    XIndirect,
    IndirectY,
    Zeropage,
    ZeropageX,
    ZeropageY,
}


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

    pub fn run(&mut self) {

        //            0  1  2  3  4  5  6  7  8  9  A  B  C  D  E  F
        let timing = [7, 6, 0, 8, 3, 3, 5, 5, 3, 2, 2, 2, 4, 4, 6, 6, /* 0 */
                      2, 5, 0, 8, 4, 4, 6, 6, 2, 4, 2, 7, 4, 4, 7, 7, /* 1 */
                      6, 6, 0, 8, 3, 3, 5, 5, 4, 2, 2, 2, 4, 4, 6, 6, /* 2 */
                      2, 5, 0, 8, 4, 4, 6, 6, 2, 4, 2, 7, 4, 4, 7, 7, /* 3 */
                      6, 6, 0, 8, 3, 3, 5, 5, 3, 2, 2, 2, 3, 4, 6, 6, /* 4 */
                      2, 5, 0, 8, 4, 4, 6, 6, 2, 4, 2, 7, 4, 4, 7, 7, /* 5 */
                      6, 6, 0, 8, 3, 3, 5, 5, 4, 2, 2, 2, 5, 4, 6, 6, /* 6 */
                      2, 5, 0, 8, 4, 4, 6, 6, 2, 4, 2, 7, 4, 4, 7, 7, /* 7 */
                      2, 6, 2, 6, 3, 3, 3, 3, 2, 2, 2, 2, 4, 4, 4, 4, /* 8 */
                      2, 6, 0, 6, 4, 4, 4, 4, 2, 5, 2, 5, 5, 5, 5, 5, /* 9 */
                      2, 6, 2, 6, 3, 3, 3, 3, 2, 2, 2, 2, 4, 4, 4, 4, /* A */
                      2, 5, 0, 5, 4, 4, 4, 4, 2, 4, 2, 4, 4, 4, 4, 4, /* B */
                      2, 6, 2, 8, 3, 3, 5, 5, 2, 2, 2, 2, 4, 4, 6, 6, /* C */
                      2, 5, 0, 8, 4, 4, 6, 6, 2, 4, 2, 7, 4, 4, 7, 7, /* D */
                      2, 6, 2, 8, 3, 3, 5, 5, 2, 2, 2, 2, 4, 4, 6, 6, /* E */
                      2, 5, 0, 8, 4, 4, 6, 6, 2, 4, 2, 7, 4, 4, 7, 7];// F
        //            0  1  2  3  4  5  6  7  8  9  A  B  C  D  E  F

        //let mut counter = 0;
        self.status_reg.break_flag = false;
        while !self.status_reg.break_flag {
            let instr = self.cpu_read_u8(self.program_counter);

            // TODO: Move this to a specific debug output
            let tmp: u8 = self.status_reg.into();
            println!("{:#X}  I:{:02X}                  A:{:02X} X:{:02X} Y:{:02X}  P:{:02X}  \
                      SP:{:02X} CYC:{:>3} ",
                     self.program_counter,
                     instr,
                     self.accumulator,
                     self.index_x,
                     self.index_y,
                     tmp,
                     self.stack_pointer,
                     self.cycle); //, self.status_reg);

            // if self.program_counter == 0xf327 {println!("Breaking line 80"); break;}
            self.execute_op(instr as u8);

            self.cycle = self.cycle + (timing[instr as usize] * PPU_MULTIPLIER);
            if self.cycle >= 341 {
                self.cycle -= 341
            }
        }
        // handle breaks here...
    }

    // TODO: Cycle counting for syncing.
    pub fn execute_op(&mut self, instr: u8) {

        self.program_counter += 1;
        match instr {

            0x4C => {
                // JMP-absolute
                let value = self.cpu_read_u16(self.program_counter);
                self.program_counter = value;
            }

            0x6C => {
                // JMP-indirect
                let lotmp = self.cpu_read_u8(self.program_counter);
                let hitmp = self.cpu_read_u8(self.program_counter + 1);
                // because there is no carry the lo byte of effective addr wraps +1
                // see http://www.6502.org/tutorials/6502opcodes.html under JMP

                let mut tmp = (hitmp as u16) << 8 | lotmp as u16;
                let lo = self.cpu_read_u8(tmp) as u16;
                tmp = (hitmp as u16) << 8 | lotmp.wrapping_add(1) as u16;
                let hi = self.cpu_read_u8(tmp) as u16;
                //            println!("ind: {:#X} JMP ${:X}${:X}", tmp, hi, lo);
                self.program_counter = hi << 8 | lo;
            }

            // LSR - A
            0x4A => self.shift_right(AddressMode::Accumulator),

            // LSR - zeropage
            0x46 => self.shift_right(AddressMode::Zeropage),

            // LSR - zeropage,x
            0x56 => self.shift_right(AddressMode::ZeropageX),

            // LSR - abs
            0x4E => self.shift_right(AddressMode::Absolute),

            // LSR - abs,x
            0x5E => self.shift_right(AddressMode::AbsoluteX),

            // ASL - A
            0x0A => self.shift_left(AddressMode::Accumulator),

            // ASL - zpg
            0x06 => self.shift_left(AddressMode::Zeropage),

            // ASL - zpg,x
            0x16 => self.shift_left(AddressMode::ZeropageX),

            // ASL - abs
            0x0E => self.shift_left(AddressMode::Absolute),

            // ASL - abs,x
            0x1E => self.shift_left(AddressMode::AbsoluteX),

            // ROR - A
            0x6A => self.rotate_right(AddressMode::Accumulator),

            // ROR - zpg
            0x66 => self.rotate_right(AddressMode::Zeropage),

            // ROR - zpg,x
            0x76 => self.rotate_right(AddressMode::ZeropageX),

            // ROR - abs
            0x6E => self.rotate_right(AddressMode::Absolute),

            // ROR - abs,x
            0x7E => self.rotate_right(AddressMode::AbsoluteX),

            // ROL - A
            0x2A => self.rotate_left(AddressMode::Accumulator),

            // ROL - zpg
            0x26 => self.rotate_left(AddressMode::Zeropage),

            // ROL - zpg,x
            0x36 => self.rotate_left(AddressMode::ZeropageX),

            // ROL - abs
            0x2E => self.rotate_left(AddressMode::Absolute),

            // ROL - abs,x
            0x3E => self.rotate_left(AddressMode::AbsoluteX),

            // LDA - immediate
            0xA9 => {
                let value = self.load_u8_from_memory(AddressMode::Immediate);
                self.set_register(value, RegType::A);
            }

            // LDA - zeropage
            0xA5 => {
                let value = self.load_u8_from_memory(AddressMode::Zeropage);
                self.set_register(value, RegType::A);
            }

            // LDA - zeropage,x
            0xB5 => {
                let value = self.load_u8_from_memory(AddressMode::ZeropageX);
                self.set_register(value, RegType::A);
            }

            // LDA - absolute
            0xAD => {
                let value = self.load_u8_from_memory(AddressMode::Absolute);
                self.set_register(value, RegType::A);
            }

            // LDA - absolute,Y
            0xB9 => {
                let value = self.load_u8_from_memory(AddressMode::AbsoluteY);
                self.set_register(value, RegType::A);
            }

            // LDA - absolute,X
            0xBD => {
                let value = self.load_u8_from_memory(AddressMode::AbsoluteX);
                self.set_register(value, RegType::A);
            }

            // LDA - indirect,x
            0xA1 => {
                let value = self.load_u8_from_memory(AddressMode::XIndirect);
                self.set_register(value, RegType::A);
            }

            // LDA - indirect,Y
            0xB1 => {
                let value = self.load_u8_from_memory(AddressMode::IndirectY);
                self.set_register(value, RegType::A);
            }


            // LDX-immediate
            0xA2 => {
                let value = self.load_u8_from_memory(AddressMode::Immediate);
                self.set_register(value, RegType::X);
            }

            // LDX-absolute
            0xAE => {
                let value = self.load_u8_from_memory(AddressMode::Absolute);
                self.set_register(value, RegType::X);
            }

            // LDX-absolute,Y
            0xBE => {
                let value = self.load_u8_from_memory(AddressMode::AbsoluteY);
                self.set_register(value, RegType::X);
            }

            // LDX-zeropage
            0xA6 => {
                let value = self.load_u8_from_memory(AddressMode::Zeropage);
                self.set_register(value, RegType::X);
            }

            // LDX-zeropage,y
            0xB6 => {
                let value = self.load_u8_from_memory(AddressMode::ZeropageY);
                self.set_register(value, RegType::X);
            }


            // LDY-immediate
            0xA0 => {
                let value = self.load_u8_from_memory(AddressMode::Immediate);
                self.set_register(value, RegType::Y);
            }

            // LDY - zeropage
            0xA4 => {
                let value = self.load_u8_from_memory(AddressMode::Zeropage);
                self.set_register(value, RegType::Y);
            }

            // LDY - absolute
            0xAC => {
                let value = self.load_u8_from_memory(AddressMode::Absolute);
                self.set_register(value, RegType::Y);
            }

            // LDY - absolute,x
            0xBC => {
                let value = self.load_u8_from_memory(AddressMode::AbsoluteX);
                self.set_register(value, RegType::Y);
            }

            // LDY - zeropage,X
            0xB4 => {
                let value = self.load_u8_from_memory(AddressMode::ZeropageX);
                self.set_register(value, RegType::Y);
            }

            // RTI - implied
            0x40 => {
                let tmp = self.pull_stack();
                self.status_reg = tmp.into();
                let lo = self.pull_stack() as u16;
                let hi = self.pull_stack() as u16;
                let value: u16 = hi << 8 | lo;
                self.program_counter = value;
            }

            // ORA - imm
            0x09 => self.bitwise_op_to_a(|a, m| a | m, AddressMode::Immediate),

            // ORA - zpg
            0x05 => self.bitwise_op_to_a(|a, m| a | m, AddressMode::Zeropage),

            // ORA - zpg,x
            0x15 => self.bitwise_op_to_a(|a, m| a | m, AddressMode::ZeropageX),

            // ORA - ind,x
            0x01 => self.bitwise_op_to_a(|a, m| a | m, AddressMode::XIndirect),

            // ORA - ind,y
            0x11 => self.bitwise_op_to_a(|a, m| a | m, AddressMode::IndirectY),

            // ORA - abs
            0x0D => self.bitwise_op_to_a(|a, m| a | m, AddressMode::Absolute),

            // ORA - abs,y
            0x19 => self.bitwise_op_to_a(|a, m| a | m, AddressMode::AbsoluteY),

            // ORA - abs,x
            0x1D => self.bitwise_op_to_a(|a, m| a | m, AddressMode::AbsoluteX),

            // EOR - imm
            0x49 => self.bitwise_op_to_a(|a, m| a ^ m, AddressMode::Immediate),

            // EOR - zpg
            0x45 => self.bitwise_op_to_a(|a, m| a ^ m, AddressMode::Zeropage),

            // EOR - zpg,x
            0x55 => self.bitwise_op_to_a(|a, m| a ^ m, AddressMode::ZeropageX),

            // EOR - ind,x
            0x41 => self.bitwise_op_to_a(|a, m| a ^ m, AddressMode::XIndirect),

            // EOR - ind,y
            0x51 => self.bitwise_op_to_a(|a, m| a ^ m, AddressMode::IndirectY),

            // EOR - abs
            0x4D => self.bitwise_op_to_a(|a, m| a ^ m, AddressMode::Absolute),

            // EOR - abs,y
            0x59 => self.bitwise_op_to_a(|a, m| a ^ m, AddressMode::AbsoluteY),

            // EOR - abs,x
            0x5D => self.bitwise_op_to_a(|a, m| a ^ m, AddressMode::AbsoluteX),

            // ADC - imm
            0x69 => self.add_with_carry(AddressMode::Immediate),

            // ADC - zpg
            0x65 => self.add_with_carry(AddressMode::Zeropage),

            // ADC - zpg,x
            0x75 => self.add_with_carry(AddressMode::ZeropageX),

            // ADC - ind,x
            0x61 => self.add_with_carry(AddressMode::XIndirect),

            // ADC - ind,y
            0x71 => self.add_with_carry(AddressMode::IndirectY),

            // ADC - abs
            0x6D => self.add_with_carry(AddressMode::Absolute),

            // ADC - abs,y
            0x79 => self.add_with_carry(AddressMode::AbsoluteY),

            // ADC - abs,x
            0x7D => self.add_with_carry(AddressMode::AbsoluteX),

            // SBC - imm
            0xE9 => self.sub_with_carry(AddressMode::Immediate),

            // SBC - zpg
            0xE5 => self.sub_with_carry(AddressMode::Zeropage),

            // SBC - zpg,x
            0xF5 => self.sub_with_carry(AddressMode::ZeropageX),

            // SBC - ind,x
            0xE1 => self.sub_with_carry(AddressMode::XIndirect),

            // SBC - ind,y
            0xF1 => self.sub_with_carry(AddressMode::IndirectY),

            // SBC - abs
            0xED => self.sub_with_carry(AddressMode::Absolute),

            // SBC - abs,y
            0xF9 => self.sub_with_carry(AddressMode::AbsoluteY),

            // SBC - abs,x
            0xFD => self.sub_with_carry(AddressMode::AbsoluteX),

            // STX-zeropage
            0x86 => {
                let tmp = self.index_x;
                self.store_u8_in_memory(tmp, AddressMode::Zeropage);
            }

            // STX-zeropage,y
            0x96 => {
                let tmp = self.index_x;
                self.store_u8_in_memory(tmp, AddressMode::ZeropageY);
            }

            // STX-absolute
            0x8E => {
                let tmp = self.index_x;
                self.store_u8_in_memory(tmp, AddressMode::Absolute);
            }

            // STA-zeropage
            0x85 => {
                let tmp = self.accumulator;
                self.store_u8_in_memory(tmp, AddressMode::Zeropage);
            }

            // STA-zeropage,X
            0x95 => {
                let tmp = self.accumulator;
                self.store_u8_in_memory(tmp, AddressMode::ZeropageX);
            }

            // STA-absolue
            0x8D => {
                let tmp = self.accumulator;
                self.store_u8_in_memory(tmp, AddressMode::Absolute);
            }

            // STA-ind,x
            0x81 => {
                let tmp = self.accumulator;
                self.store_u8_in_memory(tmp, AddressMode::XIndirect);
            }

            // STA-ind,y
            0x91 => {
                let tmp = self.accumulator;
                self.store_u8_in_memory(tmp, AddressMode::IndirectY);
            }

            // STA-abs,y
            0x99 => {
                let tmp = self.accumulator;
                self.store_u8_in_memory(tmp, AddressMode::AbsoluteY);
            }

            // STA-abs,x
            0x9D => {
                let tmp = self.accumulator;
                self.store_u8_in_memory(tmp, AddressMode::AbsoluteX);
            }

            // STY - zeropage
            0x84 => {
                let tmp = self.index_y;
                self.store_u8_in_memory(tmp, AddressMode::Zeropage);
            }

            // STY - zeropage,x
            0x94 => {
                let tmp = self.index_y;
                self.store_u8_in_memory(tmp, AddressMode::ZeropageX);
            }

            // STY - absolute
            0x8C => {
                let tmp = self.index_y;
                self.store_u8_in_memory(tmp, AddressMode::Absolute);
            }

            // JSR-Absolute
            0x20 => {
                let value = self.cpu_read_u16(self.program_counter);
                self.program_counter += 1;

                let hi = (self.program_counter >> 8) as u8;
                self.push_stack(hi);
                let lo = (0x00ff & self.program_counter) as u8;
                self.push_stack(lo);

                self.program_counter = value;
            }

            // NOP // TODO: 2 cycles
            0xEA => {} // println!("NOP"),

            0xA8 => {
                // TAY - impl
                let tmp = self.accumulator;
                self.set_register(tmp, RegType::Y);
            }

            0xAA => {
                // TAX - impl
                let tmp = self.accumulator;
                self.set_register(tmp, RegType::X);
            }

            0x98 => {
                // TYA - impl
                let tmp = self.index_y;
                self.set_register(tmp, RegType::A)
            }

            0x8A => {
                // TXA - impl
                let tmp = self.index_x;
                self.set_register(tmp, RegType::A)
            }

            0xC8 => {
                // INY - impl
                let tmp = self.index_y.wrapping_add(1);
                self.set_register(tmp, RegType::Y)
            }

            0x88 => {
                // DEY - impl
                let tmp = self.index_y.wrapping_sub(1);
                self.set_register(tmp, RegType::Y)
            }

            0xE8 => {
                // INX - impl
                let tmp = self.index_x.wrapping_add(1);
                self.set_register(tmp, RegType::X)
            }

            // INC - zeropage
            0xE6 => self.increment_memory(AddressMode::Zeropage),

            // INC - zeropage,x
            0xF6 => self.increment_memory(AddressMode::ZeropageX),

            // INC - absolute
            0xEE => self.increment_memory(AddressMode::Absolute),

            // INC - absolute,x
            0xFE => self.increment_memory(AddressMode::AbsoluteX),

            // DEC - zeropage
            0xC6 => self.decrement_memory(AddressMode::Zeropage),

            // DEC - zeropage,x
            0xD6 => self.decrement_memory(AddressMode::ZeropageX),

            // DEC - absolute
            0xCE => self.decrement_memory(AddressMode::Absolute),

            // DEC - absolute,x
            0xDE => self.decrement_memory(AddressMode::AbsoluteX),

            // DEX - impl
            0xCA => {
                let tmp = self.index_x.wrapping_sub(1);
                self.set_register(tmp, RegType::X)
            }

            // TSX -impl
            0xBA => {
                let tmp = self.stack_pointer;
                self.set_register(tmp, RegType::X)
            }

            // TXS -impl
            0x9A => self.stack_pointer = self.index_x,

            // BVC - relative - 2 bytes
            0x50 => {
                let tmp = !self.status_reg.overflow;
                self.branch(tmp)
            }

            0x70 => {
                // BVS - relative - 2 bytes
                let tmp = self.status_reg.overflow;
                self.branch(tmp)
            }

            0xb0 => {
                // BCS - relative - 2 bytes
                let tmp = self.status_reg.carry;
                self.branch(tmp)
            }

            0x90 => {
                // BCC - relative - 2 bytes
                let tmp = !self.status_reg.carry;
                self.branch(tmp)
            }

            0xf0 => {
                // BEQ - relative
                let tmp = self.status_reg.zero;
                self.branch(tmp)
            }

            0xd0 => {
                // BNE - relative
                let tmp = !self.status_reg.zero;
                self.branch(tmp)
            }

            0x10 => {
                // BPL - relative
                let tmp = !self.status_reg.negative_sign;
                self.branch(tmp)
            }

            0x30 => {
                // BMI - rel
                let tmp = self.status_reg.negative_sign;
                self.branch(tmp)
            }

            0x24 => {
                // BIT - zeropage
                let value = self.load_u8_from_memory(AddressMode::Zeropage);
                let result = self.accumulator & value;
                //            println!("BIT {:#x} & {:#x}: {:#x}", self.accumulator, value, result);
                self.status_reg.zero = result == 0;
                self.status_reg.negative_sign = (value & (1 << 7)) != 0;
                self.status_reg.overflow = (value & (1 << 6)) != 0;
            }

            0x2C => {
                // BIT - absolute
                let value = self.load_u8_from_memory(AddressMode::Absolute);
                let result = self.accumulator & value;
                //           println!("BIT {:#x} & {:#x}: {:#x}", self.accumulator, value, result);
                self.status_reg.zero = result == 0;
                self.status_reg.negative_sign = (value & (1 << 7)) != 0;
                self.status_reg.overflow = (value & (1 << 6)) != 0;
            }

            0x60 => {
                // RTS - implied
                let lo = self.pull_stack() as u16;
                let hi = self.pull_stack() as u16;
                let value: u16 = hi << 8 | lo;
                self.program_counter = value + 1;
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

            // AND - immediate
            0x29 => self.bitwise_op_to_a(|a, m| a & m, AddressMode::Immediate),

            // AND - zeropage
            0x25 => self.bitwise_op_to_a(|a, m| a & m, AddressMode::Zeropage),

            // AND - zeropage,X
            0x35 => self.bitwise_op_to_a(|a, m| a & m, AddressMode::ZeropageX),

            // AND - xindirect
            0x21 => self.bitwise_op_to_a(|a, m| a & m, AddressMode::XIndirect),

            // AND - ind,y
            0x31 => self.bitwise_op_to_a(|a, m| a & m, AddressMode::IndirectY),

            // AND absolute
            0x2D => self.bitwise_op_to_a(|a, m| a & m, AddressMode::Absolute),

            // AND absolute,y
            0x39 => self.bitwise_op_to_a(|a, m| a & m, AddressMode::AbsoluteY),

            // AND absolute,x
            0x3D => self.bitwise_op_to_a(|a, m| a & m, AddressMode::AbsoluteX),

            // CMP - immediate
            0xC9 => self.compare(RegType::A, AddressMode::Immediate),

            // CMP - zpg
            0xC5 => self.compare(RegType::A, AddressMode::Zeropage),

            // CMP - zpg,x
            0xD5 => self.compare(RegType::A, AddressMode::ZeropageX),

            // CMP - x,ind
            0xC1 => self.compare(RegType::A, AddressMode::XIndirect),

            // CMP - ind,y
            0xD1 => self.compare(RegType::A, AddressMode::IndirectY),

            // CMP - abs
            0xCD => self.compare(RegType::A, AddressMode::Absolute),

            // CMP - abs,y
            0xD9 => self.compare(RegType::A, AddressMode::AbsoluteY),

            // CMP - abs,x
            0xDD => self.compare(RegType::A, AddressMode::AbsoluteX),

            // CPY - immediate
            0xC0 => self.compare(RegType::Y, AddressMode::Immediate),

            // CPY - zpg
            0xC4 => self.compare(RegType::Y, AddressMode::Zeropage),

            // CPY - abs
            0xCC => self.compare(RegType::Y, AddressMode::Absolute),

            // CPX - immediate
            0xE0 => self.compare(RegType::X, AddressMode::Immediate),

            // CPX - zpg
            0xE4 => self.compare(RegType::X, AddressMode::Zeropage),

            // CPX - abs
            0xEC => self.compare(RegType::X, AddressMode::Absolute),

            // Illegal/undocumented opcodes - these do unusual things..
            //
            //
            // NOP - undocumented opcode
            0x1A | 0x3A | 0x5A | 0x7A | 0xDA | 0xFA => {} // println!("NOP - impl - ndocumented Opcode ${:X}", instr),

            // DOP - Zeropage - undocumented Opcode
            0x04 | 0x44 | 0x64 => {
                //           println!("DOP zpg - undocumented Opcode ${:X}", instr);
                self.program_counter += 1;
            }

            // DOP - immediate - undocumented Opcode
            0x80 | 0x82 | 0x89 | 0xC2 | 0xE2 => {
                //            println!("DOP imm - undocumented Opcode ${:X}", instr);
                self.program_counter += 1;
            }

            // DOP / NOP / SKB - zeropage,X - Undocumented Opcode
            0x14 | 0x34 | 0x54 | 0x74 | 0xD4 | 0xF4 => {
                //           println!("DOP zpg,x - undocumented Opcode ${:X}", instr);
                self.program_counter += 1;
            }

            // TOP - abs - Undocumented Opcode.
            0x0C => {
                //            println!("TOP abs - undocumented Opcode ${:X}", instr);
                self.program_counter += 2;
            }

            // TOP / NOP / SKW - Abs,X - undocumented opcode
            0x1C | 0x3C | 0x5C | 0x7C | 0xDC | 0xFC => {
                // println!("TOP abs,x - undocumented Opcode ${:X}", instr);
                // read the value and throw it away for cycle counting
                let tmp = self.load_u8_from_memory(AddressMode::AbsoluteX);
            }

            // LAX - Ind,x - Undocumented Opcode
            0xA3 => {
                let value = self.load_u8_from_memory(AddressMode::XIndirect);
                self.set_register(value, RegType::A);
                self.set_register(value, RegType::X);
            }

            // LAX - Ind,y - Undocumented Opcode
            0xB3 => {
                let value = self.load_u8_from_memory(AddressMode::IndirectY);
                self.set_register(value, RegType::A);
                self.set_register(value, RegType::X);
            }

            // LAX - zpg - Undocumented Opcode
            0xA7 => {
                let value = self.load_u8_from_memory(AddressMode::Zeropage);
                self.set_register(value, RegType::A);
                self.set_register(value, RegType::X);
            }

            // LAX - zpg,y - Undocumented Opcode
            0xB7 => {
                let value = self.load_u8_from_memory(AddressMode::ZeropageY);
                self.set_register(value, RegType::A);
                self.set_register(value, RegType::X);
            }

            // LAX - abs - Undocumented Opcode
            0xAF => {
                let value = self.load_u8_from_memory(AddressMode::Absolute);
                self.set_register(value, RegType::A);
                self.set_register(value, RegType::X);
            }

            // LAX - abs,y - Undocumented Opcode
            0xBF => {
                let value = self.load_u8_from_memory(AddressMode::AbsoluteY);
                self.set_register(value, RegType::A);
                self.set_register(value, RegType::X);
            }

            // AAX / SAX / AXS - ind,x - Undocumented Opcode
            0x83 => self.and_x_a_store(AddressMode::XIndirect),

            // AAX / SAX / AXS - zpg - Undocumented Opcode
            0x87 => self.and_x_a_store(AddressMode::Zeropage),

            // AAX / SAX / AXS - zpg,y - Undocumented Opcode
            0x97 => self.and_x_a_store(AddressMode::ZeropageY),

            // AAX / SAX / AXS - abs - Undocumented Opcode
            0x8F => self.and_x_a_store(AddressMode::Absolute),

            // SBC - imm - Undocumented opcode (identical to E9)
            0xEB => self.sub_with_carry(AddressMode::Immediate),

            // DCP / DCM - zpg - Undocumented Opcode
            0xC7 => self.dec_then_cmp(AddressMode::Zeropage),

            // DCP / DCM - zpg,x - Undocumented Opcode
            0xD7 => self.dec_then_cmp(AddressMode::ZeropageX),

            // DCP / DCM - abs - Undocumented Opcode
            0xCF => self.dec_then_cmp(AddressMode::Absolute),

            // DCP / DCM - abs,x - Undocumented Opcode
            0xDF => self.dec_then_cmp(AddressMode::AbsoluteX),

            // DCP / DCM - abs,y - Undocumented Opcode
            0xDB => self.dec_then_cmp(AddressMode::AbsoluteY),

            // DCP / DCM - ind,x - Undocumented Opcode
            0xC3 => self.dec_then_cmp(AddressMode::XIndirect),

            // DCP / DCM - ind,y - Undocumented Opcode
            0xD3 => self.dec_then_cmp(AddressMode::IndirectY),

            // ISC / ISB / INS - zpg - undocumented
            0xE7 => self.inc_then_sbc(AddressMode::Zeropage),

            // ISC / ISB / INS - zpg,x - undocumented
            0xF7 => self.inc_then_sbc(AddressMode::ZeropageX),

            // ISC / ISB / INS - abs - undocumented
            0xEF => self.inc_then_sbc(AddressMode::Absolute),

            // ISC / ISB / INS - abs,x - undocumented
            0xFF => self.inc_then_sbc(AddressMode::AbsoluteX),

            // ISC / ISB / INS - abs,y - undocumented
            0xFB => self.inc_then_sbc(AddressMode::AbsoluteY),

            // ISC / ISB / INS - ind,x - undocumented
            0xE3 => self.inc_then_sbc(AddressMode::XIndirect),

            // ISC / ISB / INS - ind,y - undocumented
            0xF3 => self.inc_then_sbc(AddressMode::IndirectY),

            // SLO / ASO - zpg - undocumented
            0x07 => self.asl_then_ora(AddressMode::Zeropage),

            // SLO / ASO - zpg,x - undocumented
            0x17 => self.asl_then_ora(AddressMode::ZeropageX),

            // SLO / ASO - abs - undocumented
            0x0F => self.asl_then_ora(AddressMode::Absolute),

            // SLO / ASO - abs,x - undocumented
            0x1F => self.asl_then_ora(AddressMode::AbsoluteX),

            // SLO / ASO - abs,y - undocumented
            0x1B => self.asl_then_ora(AddressMode::AbsoluteY),

            // SLO / ASO - ind,x - undocumented
            0x03 => self.asl_then_ora(AddressMode::XIndirect),

            // SLO / ASO - ind,y - undocumented
            0x13 => self.asl_then_ora(AddressMode::IndirectY),

            // RLA - zpg - undocumented
            0x27 => self.rol_then_and(AddressMode::Zeropage),

            // RLA - zpg,x - undocumented
            0x37 => self.rol_then_and(AddressMode::ZeropageX),

            // RLA - abs - undocumented
            0x2F => self.rol_then_and(AddressMode::Absolute),

            // RLA - abs,x - undocumented
            0x3F => self.rol_then_and(AddressMode::AbsoluteX),

            // RLA - abs,y - undocumented
            0x3B => self.rol_then_and(AddressMode::AbsoluteY),

            // RLA - ind,x - undocumented
            0x23 => self.rol_then_and(AddressMode::XIndirect),

            // RLA - ind,y - undocumented
            0x33 => self.rol_then_and(AddressMode::IndirectY),

            // SRE / LSE - zpg - undocumented
            0x47 => self.lsr_then_eor(AddressMode::Zeropage),

            // SRE / LSE - zpg,x - undocumented
            0x57 => self.lsr_then_eor(AddressMode::ZeropageX),

            // SRE / LSE - abs - undocumented
            0x4F => self.lsr_then_eor(AddressMode::Absolute),

            // SRE / LSE - abs,x - undocumented
            0x5F => self.lsr_then_eor(AddressMode::AbsoluteX),

            // SRE / LSE - abs,y - undocumented
            0x5B => self.lsr_then_eor(AddressMode::AbsoluteY),

            // SRE / LSE - ind,x - undocumented
            0x43 => self.lsr_then_eor(AddressMode::XIndirect),

            // SRE / LSE - ind,y - undocumented
            0x53 => self.lsr_then_eor(AddressMode::IndirectY),

            // RRA - zpg - undocumented
            0x67 => self.ror_then_adc(AddressMode::Zeropage),

            // RRA - zpg,x - undocumented
            0x77 => self.ror_then_adc(AddressMode::ZeropageX),

            // RRA - abs - undocumented
            0x6F => self.ror_then_adc(AddressMode::Absolute),

            // RRA - abs,x - undocumented
            0x7F => self.ror_then_adc(AddressMode::AbsoluteX),

            // RRA - abs,y - undocumented
            0x7B => self.ror_then_adc(AddressMode::AbsoluteY),

            // RRA - ind,x - undocumented
            0x63 => self.ror_then_adc(AddressMode::XIndirect),

            // RRA - ind,y - undocumented
            0x73 => self.ror_then_adc(AddressMode::IndirectY),

            // BRK
            0x00 => {
                self.status_reg.break_flag = true;

                self.program_counter += 1;
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

            _ => panic!("The opcode: {:#x} is unrecognized", instr),
        }
    }

    // All of the XXX then YYY instructions are undocumented opcodes
    // They usually have a hack for static cycle counts by resetting the cycle
    // count at the end of both insturctions
    // normally abs,x/y and ind,y have an extra cycle for page boundary
    // but not on the undocumented ones for some reason


    fn ror_then_adc(&mut self, addr_mode: AddressMode) {
        let tmp = self.cycle;
        self.rotate_right(addr_mode);
        self.reset_pc_for_double_op(addr_mode);
        self.add_with_carry(addr_mode);
        self.cycle = tmp;
    }

    fn lsr_then_eor(&mut self, addr_mode: AddressMode) {
        let tmp = self.cycle;
        self.shift_right(addr_mode);
        self.reset_pc_for_double_op(addr_mode);
        self.bitwise_op_to_a(|a, m| a ^ m, addr_mode);
        self.cycle = tmp;
    }

    fn rol_then_and(&mut self, addr_mode: AddressMode) {
        let tmp = self.cycle;
        self.rotate_left(addr_mode);
        self.reset_pc_for_double_op(addr_mode);
        self.bitwise_op_to_a(|a, m| a & m, addr_mode);
        self.cycle = tmp;
    }

    fn asl_then_ora(&mut self, addr_mode: AddressMode) {
        let tmp = self.cycle;
        self.shift_left(addr_mode);
        self.reset_pc_for_double_op(addr_mode);
        self.bitwise_op_to_a(|a, m| a | m, addr_mode);
        self.cycle = tmp;
    }

    fn dec_then_cmp(&mut self, addr_mode: AddressMode) {
        let tmp = self.cycle;
        self.decrement_memory(addr_mode);
        self.reset_pc_for_double_op(addr_mode);
        self.compare(RegType::A, addr_mode);
        self.cycle = tmp;
    }

    // ISB
    fn inc_then_sbc(&mut self, addr_mode: AddressMode) {
        let tmp = self.cycle;
        self.increment_memory(addr_mode);
        self.reset_pc_for_double_op(addr_mode);
        self.sub_with_carry(addr_mode);
        self.cycle = tmp;
    }

    fn reset_pc_for_double_op(&mut self, addr_mode: AddressMode) {
        if addr_mode == AddressMode::Absolute || addr_mode == AddressMode::AbsoluteX ||
           addr_mode == AddressMode::AbsoluteY {
            self.program_counter -= 2;
        } else {
            self.program_counter -= 1;
        }
    }

    // does NOT effect flags
    fn and_x_a_store(&mut self, addr_mode: AddressMode) {
        let value = self.index_x & self.accumulator;
        self.store_u8_in_memory(value, addr_mode)
    }

    fn rotate_right(&mut self, addr_mode: AddressMode) {
        let value: u8;
        let addr: u16;
        if addr_mode == AddressMode::Accumulator {
            value = self.accumulator;
            addr = 0;
        } else {
            addr = self.memory_lookup(addr_mode, false) as u16;
            value = self.cpu_read_u8(addr);
        }

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

    fn rotate_left(&mut self, addr_mode: AddressMode) {
        let value: u8;
        let addr: u16;
        if addr_mode == AddressMode::Accumulator {
            value = self.accumulator;
            addr = 0;
        } else {
            addr = self.memory_lookup(addr_mode, false) as u16;
            value = self.cpu_read_u8(addr);
        }

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

    fn shift_right(&mut self, addr_mode: AddressMode) {
        let value: u8;
        let addr: u16;
        if addr_mode == AddressMode::Accumulator {
            value = self.accumulator;
            addr = 0;
        } else {
            addr = self.memory_lookup(addr_mode, false) as u16;
            value = self.cpu_read_u8(addr);
        }

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

    fn shift_left(&mut self, addr_mode: AddressMode) {
        let value: u8;
        let addr: u16;
        if addr_mode == AddressMode::Accumulator {
            value = self.accumulator;
            addr = 0;
        } else {
            addr = self.memory_lookup(addr_mode, false) as u16;
            value = self.cpu_read_u8(addr);
        }

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

    fn bitwise_op_to_a<F>(&mut self, f: F, addr_mode: AddressMode)
        where F: FnOnce(u8, u8) -> u8
    {

        let m = self.load_u8_from_memory(addr_mode);
        let a = self.accumulator;
        let value = f(a, m);
        self.set_register(value, RegType::A);
    }

    fn add_with_carry(&mut self, addr_mode: AddressMode) {
        let value = self.load_u8_from_memory(addr_mode) as u16;
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

    // impl as binary add with 1s compliment of the value being sub from A
    fn sub_with_carry(&mut self, addr_mode: AddressMode) {
        let value = self.load_u8_from_memory(addr_mode) as u16;
        let a = self.accumulator as u16;
        let c = if self.status_reg.carry {
            1
        } else {
            0
        };

        let result = a + (0xff - value) + c;
        // TODO: debug
        // println!("SBC: A{:#X} + (0xff-M{:#X}) + C{:#X} = {:X}", a, value, c, result);
        self.status_reg.carry = result > 0xff;
        self.status_reg.overflow = ((a ^ result) & ((0xff - value) ^ result) & 0x80) != 0;
        self.set_register(result as u8, RegType::A);
    }

    fn increment_memory(&mut self, addr_mode: AddressMode) {
        let addr = self.memory_lookup(addr_mode, false) as u16;
        let mut value = self.cpu_read_u8(addr);
        value = value.wrapping_add(1);
        self.status_reg.zero = value == 0;
        self.status_reg.negative_sign = (value & (1 << 7)) != 0;
        self.cpu_write_u8(addr, value);
    }

    fn decrement_memory(&mut self, addr_mode: AddressMode) {
        let addr = self.memory_lookup(addr_mode, false) as u16;
        let mut value = self.cpu_read_u8(addr);
        value = value.wrapping_sub(1);
        self.status_reg.zero = value == 0;
        self.status_reg.negative_sign = (value & (1 << 7)) != 0;
        self.cpu_write_u8(addr, value);
    }

    fn compare(&mut self, reg: RegType, addr_mode: AddressMode) {
        let register = match reg {
            RegType::A => self.accumulator as i16,
            RegType::Y => self.index_y as i16,
            RegType::X => self.index_x as i16,
        };
        let value = self.load_u8_from_memory(addr_mode) as i16;
        //        println!("CMY {:#X} - {:#X}", register, value);
        let result = (register - value) as u8;
        self.status_reg.zero = register == value;
        self.status_reg.negative_sign = (result & (1 << 7)) != 0;
        self.status_reg.carry = register >= value;
    }

    // All branch functions seem to be identical
    // I have no idea what happens if it under/overflows
    // maybe should be wrapping
    fn branch(&mut self, condition: bool) {
        if condition {
            self.cycle += 1 * PPU_MULTIPLIER;
            let value = self.load_u8_from_memory(AddressMode::Immediate) as i8;
            //            println!("Branch: PC:{:#X} + {:}", self.program_counter, value);
            //let tmp = (self.program_counter as i16 + value as i16) as u16;
            let tmp = (self.program_counter as i16 + value as i16) as u16;
            if tmp >> 8 != (self.program_counter + 1) >> 8 {
                self.cycle += 1 * PPU_MULTIPLIER;
            };
            self.program_counter = tmp as u16;
        } else {
            self.program_counter += 1;
        }
    }

    fn memory_lookup(&mut self, addr_mode: AddressMode, page_check: bool) -> u16 {
        match addr_mode {
            AddressMode::Immediate => {
                self.program_counter += 1;
                self.program_counter - 1
            }
            AddressMode::Zeropage => {
                let tmp = self.cpu_read_u8(self.program_counter);
                self.program_counter += 1;
                tmp as u16
            }

            AddressMode::ZeropageX => self.zeropage_xy(RegType::X),
            AddressMode::ZeropageY => self.zeropage_xy(RegType::Y),
            AddressMode::Absolute => {
                let tmp = self.cpu_read_u16(self.program_counter);
                self.program_counter += 2;
                tmp
            }
            AddressMode::AbsoluteX => self.absolute_xy(RegType::X, page_check),
            AddressMode::AbsoluteY => self.absolute_xy(RegType::Y, page_check),
            AddressMode::XIndirect => self.x_indirect(),
            AddressMode::IndirectY => self.indirect_y(page_check),

            _ => panic!("memory_lookup doesn't work on {:?}", addr_mode),
        }
    }

    fn load_u8_from_memory(&mut self, addr_mode: AddressMode) -> u8 {
        let mut page_check = false;
        if addr_mode == AddressMode::AbsoluteX ||
           addr_mode == AddressMode::AbsoluteY ||
           addr_mode == AddressMode::IndirectY
        {
            page_check = true;
        }
        if addr_mode == AddressMode::Accumulator {
            return self.accumulator
        } else {
            let addr = self.memory_lookup(addr_mode, page_check);
            self.cpu_read_u8(addr)
        }
    }

    fn store_u8_in_memory(&mut self, value: u8, addr_mode: AddressMode) {
        let addr = self.memory_lookup(addr_mode, false);
        self.cpu_write_u8(addr, value);
    }

    fn x_indirect(&mut self) -> u16 {
        let mut tmp = self.cpu_read_u8(self.program_counter);
        self.program_counter += 1;
        tmp = tmp.wrapping_add(self.index_x);
        let lo = self.cpu_read_u8(tmp as u16) as u16;
        let hi = self.cpu_read_u8(tmp.wrapping_add(1) as u16) as u16;
        hi << 8 | lo
    }

    fn zeropage_xy(&mut self, reg: RegType) -> u16 {
        let tmp = self.cpu_read_u8(self.program_counter);
        self.program_counter += 1;
        match reg {
            RegType::X => tmp.wrapping_add(self.index_x) as u16,
            RegType::Y => tmp.wrapping_add(self.index_y) as u16,
            _ => panic!("can not zpg,A"),
        }
    }

    // afaik I can ignore 'with carry' in the description because I'm using a u16
    // I'm assuming the addr would wrap here.. no idea
    fn absolute_xy(&mut self, reg: RegType, page_check: bool) -> u16 {
        let tmp = self.cpu_read_u16(self.program_counter);
        self.program_counter += 2;
        let result = match reg {
            RegType::X => tmp.wrapping_add(self.index_x as u16),
            RegType::Y => tmp.wrapping_add(self.index_y as u16),
            _ => panic!("can not abs,A"),
        };

         if page_check && (tmp >> 8 != result >> 8) {
                self.cycle += 1 * PPU_MULTIPLIER;
         }
        // TODO: Debugger
        // println!("Load abs,{:?} is {:#X} + X:{:#X} or Y:{:#X} = {:#X}",
        //          reg, tmp,self.index_x, self.index_y, result);
        result
    }

    fn indirect_y(&mut self, page_check: bool) -> u16 {
        let tmp = self.cpu_read_u8(self.program_counter); // zpg addr
        self.program_counter += 1;
        let lo = self.cpu_read_u8(tmp as u16) as u16;
        let hi = self.cpu_read_u8(tmp.wrapping_add(1) as u16) as u16;
        let value: u16 = hi << 8 | lo;
        let result = value.wrapping_add(self.index_y as u16);
        if page_check && (hi != result >> 8) {
            self.cycle += 1 * PPU_MULTIPLIER;
        }
        result
        // TODO:: debugger
        // println!("Load ind,Y is {:#X} + {:#X} = {:#X}", value, self.index_y, result);
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
        self.stack_pointer -= 1;
    }

    fn pull_stack(&mut self) -> u8 {
        self.stack_pointer += 1;
        self.cpu_read_u8(0x100 + self.stack_pointer as u16)
    }

    fn cpu_read_u8(&self, addr: u16) -> u8 {
        match addr {
            RAM_START...RAM_VIRTUAL_END => {
                let addr = addr % RAM_LEN;
                // println!("Read {:#X} at {:#X} in RAM", self.bus.ram[addr as usize], addr);
                self.bus.ram[addr as usize]
            }

            PPU_REGISTERS_START...PPU_REGISTERS_VIRTUAL_END => panic!("PPU is unimplemented"),

            APU_REGISTERS_START...APU_REGISTERS_END => panic!("APU is unimplemented"),

            EXPANSION_ROM_START...EXPANSION_ROM_END => {
                // Used by some mappers, can usually be ignored
                panic!("Expansion rom is unimplemented")
            }

            SRAM_START...SRAM_END => panic!("SRAM is unimplemented"),

            PRG_ROM_START...PRG_ROM_END => self.bus.cart.read_cart_u8(addr),

            _ => panic!("Invalid read location {:#X}", addr),
        }
    }

    // The actual 6502 can't read a u16, this is for convenince only
    fn cpu_read_u16(&self, addr: u16) -> u16 {
        match addr {
            RAM_START...RAM_VIRTUAL_END => {
                let addr = addr % RAM_LEN;
                let lo = self.bus.ram[addr as usize] as u16;
                let hi = self.bus.ram[(addr as usize) + 1] as u16;
                hi << 8 | lo
            }

            PPU_REGISTERS_START...PPU_REGISTERS_VIRTUAL_END => panic!("PPU is unimplemented"),

            APU_REGISTERS_START...APU_REGISTERS_END => panic!("APU is unimplemented"),

            EXPANSION_ROM_START...EXPANSION_ROM_END => {
                // Used by some mappers, can usually be ignored
                panic!("Expansion rom is unimplemented")
            }

            SRAM_START...SRAM_END => panic!("SRAM is unimplemented"),

            PRG_ROM_START...PRG_ROM_END => self.bus.cart.read_cart_u16(addr),

            _ => panic!("Invalid read location {:#X}", addr),
        }
    }

    fn cpu_write_u8(&mut self, addr: u16, value: u8) {
        match addr {
            RAM_START...RAM_VIRTUAL_END => {
                let addr = addr % RAM_LEN;
                //                println!("Wrote {:#X} at {:#X} in RAM", value, addr);
                self.bus.ram[addr as usize] = value
            }

            PPU_REGISTERS_START...PPU_REGISTERS_VIRTUAL_END => panic!("PPU is unimplemented"),

            APU_REGISTERS_START...APU_REGISTERS_END => {
                self.bus.apu.write(addr, value);
            }

            EXPANSION_ROM_START...EXPANSION_ROM_END => {
                // Used by some mappers, can usually be ignored
                panic!("Expansion rom is unimplemented")
            }

            SRAM_START...SRAM_END => panic!("SRAM is unimplemented"),

            PRG_ROM_START...PRG_ROM_END => panic!("Can't write to PRG rom location"),

            _ => panic!("Invalid write location {:#X}", addr),
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

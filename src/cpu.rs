use super::*;
use std::collections::HashSet;

//pub HashMap: ops;

#[derive(Debug)]
pub struct CPU {
    // A
    accumulator: u8,

    index_x: u8,
    index_y: u8,
    // P
    status_reg: StatusReg,
    program_counter: u16,
    // PCL: u8 is pc low   --
    // PCH: u8 is pc high  -- here just represented as a single u16
    // S or SP
    stack_pointer: u8,
    bus: Bus
}

#[derive(Debug, Clone, Copy)]
pub struct StatusReg {
    // N (or sometimes S)
    negative_sign: bool,
    // V
    overflow: bool,
    unused: bool, // always 1
    // B
    break_flag: bool,
    // D
    decimal_mode: bool,  // unimplemented on NES
    // I
    interrupt_disable: bool,
    // Z
    zero: bool,
    // C
    carry: bool,
}

enum RegType {
    A,
    X,
    Y,
}

enum AddressMode {
    absolute,
    absolute_x,
    absolute_y,
    immediate,
    x_indirect,
    indirect_y,
    zeropage,
    zeropage_x,
    zeropage_y
}
//const ops-imm:[u8; 2] = [0x32, 0x24];
// also do ops-abs, obs-impl, etc - to determine byte length
// maybe do enum ora = 0x01 etc.
// might make the match more efficient?
// maybe just do separate match functions...
//println!("{:#?}", ops-imm.iter().any(|&y| y == 0x32u8));

// box slice of 256 string literals??
//const ops:[string, 256] = ["ORA", "", "", "", "ORA",
// etc - then index with ops[0x01] for debug

impl CPU {
    pub fn new(bus: Bus, pc: u16) -> CPU {
      CPU {
        accumulator: 0,
        index_x: 0,
        index_y: 0,
//      nestest.nes uses the wrong start status reg of 0x24 (brk false) instead of 0x34 (brk true)
//      then improperly checks against that 'dirty' value instead of setting its own
//        status_reg: 0x34.into(),
        status_reg: 0x24.into(),
        program_counter: pc,
        stack_pointer: 0xfd,
        bus: bus
      }
    }

    pub fn run(&mut self) {
     loop  {
//        break;
        let instr = self.bus.cart.read_cart_u8(self.program_counter);
        // TODO: Move this to a specific debug output
        let tmp:u8 = self.status_reg.into();
        println!("{:#X}  I:{:02X}                  A:{:02X} X:{:02X} Y:{:02X}  P:{:02X}  SP:{:02X}",
                   self.program_counter, instr, self.accumulator, self.index_x, self.index_y,
                   tmp, self.stack_pointer); //, self.status_reg);
//        if self.program_counter == 0xcf52 {break;}
        self.execute_op(instr as u8);
      }
    }

    // TODO: Increment PC automatically on cart read?
    // alternatively increment on specific instruction lengths

    pub fn execute_op(&mut self, instr: u8) {
      // TODO: Break these in to actual instructions by name
      // then match the addressing mode (imm/abs/zeropage/etc)
      self.program_counter += 1;
      match instr {

        0x4C => { // JMP-absolute
            let value = self.bus.cart.read_cart_u16(self.program_counter);
            self.program_counter = value;
        }

        0x4A => { // LSR - A
            self.status_reg.carry = (self.accumulator & (1 << 0)) != 0;
            let value = self.accumulator >> 1;
            self.set_register(value, RegType::A);
        }

        0x0A => { // ASL - A
            self.status_reg.carry = (self.accumulator & (1 << 7)) != 0;
            let value = ((self.accumulator as u16) << 1) as u8;
            self.set_register(value, RegType::A);
        }

        0x6A => { // ROR - A
            let c = if self.status_reg.carry { 1 } else { 0 };
            self.status_reg.carry = (self.accumulator & (1 << 0)) != 0;
            let value = (self.accumulator >> 1) | c << 7;
            self.set_register(value, RegType::A);
        }

        0x2A => { // ROL - A
            let c = if self.status_reg.carry { 1 } else { 0 };
            self.status_reg.carry = (self.accumulator & (1 << 7)) != 0;
            let value = (self.accumulator << 1) | c << 0;
            self.set_register(value, RegType::A);
        }

        0xA9 => { // LDA - immediate
            let value = self.load_u8_from_memory(AddressMode::immediate);
            self.set_register(value, RegType::A);
        }

        0xA5 => { // LDA - zeropage
            let value = self.load_u8_from_memory(AddressMode::zeropage);
            self.set_register(value, RegType::A);
        }

        0xAD => { // LDA - absolute
            let value = self.load_u8_from_memory(AddressMode::absolute);
            self.set_register(value, RegType::A);
        }

        0xA2 => { // LDX-immediate
            let value = self.load_u8_from_memory(AddressMode::immediate);
            self.set_register(value, RegType::X);
        }

       0xAE => { // LDX-absolute
            let value = self.load_u8_from_memory(AddressMode::absolute);
            self.set_register(value, RegType::X);
        }

        0xa0 => { // LDY-immediate
            let value = self.load_u8_from_memory(AddressMode::immediate);
            self.set_register(value, RegType::Y);
        }

        0x40 => { // RTI - implied
            let tmp = self.pull_stack();
            self.status_reg = tmp.into();
            let lo = self.pull_stack() as u16;
            let hi = self.pull_stack() as u16;
            let value:u16 = hi << 8 | lo;
            self.program_counter = value;
        }

        0x09 => { // ORA - imm
            let value = self.load_u8_from_memory(AddressMode::immediate);
            let result = self.accumulator | value;
            self.set_register(result, RegType::A);
        }

        0x49 => { // EOR - imm
            let value = self.load_u8_from_memory(AddressMode::immediate);
            let result = self.accumulator ^ value;
            self.set_register(result, RegType::A);
        }

        0x69 => { // ADC - imm
            let value = self.load_u8_from_memory(AddressMode::immediate) as u16;
            let a = self.accumulator as u16;
            let c = if self.status_reg.carry { 1 } else { 0 };

            let result = a + value + c;
            println!("ADC: A{:#X} + M{:#X} + C{:#X} = {:X}", a, value, c, result);
            self.status_reg.carry = result > 0xff;
            self.status_reg.overflow = !(value & (1 << 7) != 0 ||
                                         self.status_reg.negative_sign) &&
                                         result >= 0x80;
            self.set_register(result as u8, RegType::A);
        }

        0xE9 => { // SBC - imm
            let value = self.load_u8_from_memory(AddressMode::immediate) as i16;
            let a = self.accumulator as i16;
            let c = if self.status_reg.carry { 0 } else { 1 };

            let result = a - value - c;
            println!("SBC: A{:#X} - M{:#X} - C{:#X} = {:#?}", a, value, c, result);
            self.status_reg.carry = result >= 0;
            self.status_reg.overflow = (value & (1 << 7) != 0 ||
                                         self.status_reg.negative_sign) &&
                                         (result as u8) < 0x80;
            self.set_register(result as u8, RegType::A);
        }

        0x86 => { // STX-zeropage
            let value = self.load_u8_from_memory(AddressMode::immediate);
            self.bus.ram[value as usize] = self.index_x;
        }

        0x8E => { // STX-absolute
            let value = self.bus.cart.read_cart_u16(self.program_counter);
            self.program_counter +=2;
            self.bus.ram[value as usize] = self.index_x;
        }

        0x85 => { // STA-zeropage
            let value = self.load_u8_from_memory(AddressMode::immediate);
            // TODO: Make write_u8 function to better map the CPU memory
            self.bus.ram[value as usize] = self.accumulator;
        }

        0x20 => { // JSR-Absolute
            let value = self.bus.cart.read_cart_u16(self.program_counter);
            self.program_counter += 1;
            let hi = (self.program_counter >> 8) as u8;
            self.push_stack(hi);
            let lo = (0x00ff & self.program_counter) as u8;
            self.push_stack(lo);
            self.program_counter = value;
        }

                  // NOP // TODO: 2 cycles
        0xEA => {println!("NOP");}

        0xA8 => { //TAY - impl
            let tmp = self.accumulator;
            self.set_register(tmp, RegType::Y);
        }

        0xAA => { //TAX - impl
            let tmp = self.accumulator;
            self.set_register(tmp, RegType::X);
        }

        0x98 => { //TYA - impl
            let tmp = self.index_y;
            self.set_register(tmp, RegType::A)
        }

        0x8A => { //TXA - impl
            let tmp = self.index_x;
            self.set_register(tmp, RegType::A)
        }

        0xC8 => { //INY - impl
            let tmp = self.index_y.wrapping_add(1);
            self.set_register(tmp, RegType::Y)
        }

        0x88 => { //DEY - impl
            let tmp = self.index_y.wrapping_sub(1);
            self.set_register(tmp, RegType::Y)
        }

        0xE8 => { //INX - impl
            let tmp = self.index_x.wrapping_add(1);
            self.set_register(tmp, RegType::X)
        }

        0xBA => { // TSX -impl
            let tmp = self.stack_pointer;
            self.set_register(tmp, RegType::X)
        }

                  // TXS -impl
        0x9A => self.stack_pointer = self.index_x,

        0xCA => { //DEX - impl
            let tmp = self.index_x.wrapping_sub(1);
            self.set_register(tmp, RegType::X)
        }

        0x50 => { // BVC - relative - 2 bytes
            let tmp = !self.status_reg.overflow;
            self.branch(tmp)
        }

        0x70 => { // BVS - relative - 2 bytes
            let tmp = self.status_reg.overflow;
            self.branch(tmp)
        }

        0xb0 => { // BCS - relative - 2 bytes
            let tmp = self.status_reg.carry;
            self.branch(tmp)
        }

        0x90 => { // BCS - relative - 2 bytes
            let tmp = !self.status_reg.carry;
            self.branch(tmp)
        },

        0xf0 => { // BEQ - relative
            let tmp = self.status_reg.zero;
            self.branch(tmp)
        }

        0xd0 => { // BNE - relative
            let tmp = !self.status_reg.zero;
            self.branch(tmp)
        }

        0x10 => { // BPL - relative
            let tmp = !self.status_reg.negative_sign;
            self.branch(tmp)
        }

        0x30 => {  // BMI - rel
            let tmp = self.status_reg.negative_sign;
            self.branch(tmp)
        }

        0x24 => { // BIT - zeropage
            let value = self.load_u8_from_memory(AddressMode::zeropage);
            let result = self.accumulator & value;
            println!("BIT {:#x} & {:#x}: {:#x}", self.accumulator, value, result);
            self.status_reg.zero = result == 0;
            self.status_reg.negative_sign = (value & (1 << 7)) != 0;
            self.status_reg.overflow = (value & (1 << 6)) != 0;
        }

        0x60 => { // RTS - implied
            let lo = self.pull_stack() as u16;
            let hi = self.pull_stack() as u16;
            let value:u16 = hi << 8 | lo;
            self.program_counter = value + 1;
        }

                  // SEI - impl
        0x78 => self.status_reg.interrupt_disable = true,

                  // CLI - impl
        0x58 => self.status_reg.interrupt_disable = false,

                  // SED - impl
        0xF8 => self.status_reg.decimal_mode = true,

                  // CLD - impl
        0xD8 =>  self.status_reg.decimal_mode = false,

                  // SEC - implied
        0x38 => self.status_reg.carry = true,

                  // CLC - implied
        0x18 => self.status_reg.carry = false,

                  // CLV - impl
        0xB8 => self.status_reg.overflow = false,

        0x08 => { // PHP - impl
            let mut tmp:u8 = self.status_reg.into();
            tmp |= 1 << 4; // set the break flag before pushing
            self.push_stack(tmp);
        }

        0x28 => { // PLP - impl
            let mut value = self.pull_stack();
            let brk = if self.status_reg.break_flag { 1 } else { 0 };
            value ^= (brk ^ value) & (1 << 4);
            self.status_reg = value.into();
        }

        0x68 => { // PLA - impl
            let value = self.pull_stack();
            self.set_register(value, RegType::A);
        }

        0x48 => { // PHA - impl
            let tmp = self.accumulator;
            self.push_stack(tmp);
        }

        0x29 => {  // AND - immediate
            let value = self.load_u8_from_memory(AddressMode::immediate);
            let result = self.accumulator & value;
            self.set_register(result, RegType::A);
        }

        0xC9 => { // CMP - immediate
            let a = self.accumulator as i16;
            self.compare(a);
        }

        0xC0 => { // CPY - immediate
            let y = self.index_y as i16;
            self.compare(y);
        }

        0xE0 => { // CPY - immediate
            let x = self.index_x as i16;
            self.compare(x);
        }

        _ => panic!("The opcode: {:#x} is unrecognized", instr)
      }
    }

    fn load_u8_from_memory(&mut self, addr_mode: AddressMode) -> u8 {
        match addr_mode {
            AddressMode::immediate => {
                self.program_counter += 1;
                self.bus.cart.read_cart_u8(self.program_counter - 1)
            }

            AddressMode::zeropage => {
                let tmp = self.bus.cart.read_cart_u8(self.program_counter);
                self.program_counter += 1;
                self.bus.ram[tmp as usize]
            }

            AddressMode::zeropage_x => {
                let mut tmp = self.bus.cart.read_cart_u8(self.program_counter);
                self.program_counter += 1;
                tmp = tmp.wrapping_add(self.index_x);
                self.bus.ram[tmp as usize]
            }

            AddressMode::zeropage_y => {
                let mut tmp = self.bus.cart.read_cart_u8(self.program_counter);
                self.program_counter += 1;
                tmp = tmp.wrapping_add(self.index_y);
                self.bus.ram[tmp as usize]
            }

            AddressMode::absolute => {
                let tmp = self.bus.cart.read_cart_u16(self.program_counter);
                self.program_counter += 2;
                self.bus.ram[tmp as usize]
            }

            AddressMode::absolute_x => {
                let mut tmp = self.bus.cart.read_cart_u16(self.program_counter);
                self.program_counter += 2;
                println!("Load abs,X is {:#X} + {:#X}", tmp, self.index_x);
                panic!("carry not yet implemented here, it might fail if I continue");
                // TODO: implement carry
                tmp += self.index_x as u16;
                self.bus.ram[tmp as usize]
            }

            AddressMode::absolute_y => {
                let mut tmp = self.bus.cart.read_cart_u16(self.program_counter);
                self.program_counter += 2;
                println!("Load abs,Y is {:#X} + {:#X}", tmp, self.index_y);
                panic!("carry not yet implemented here, it might fail if I continue");
                // TODO: implement carry
                tmp += self.index_y as u16;
                self.bus.ram[tmp as usize]
            }

            AddressMode::x_indirect => {
                let mut tmp = self.bus.cart.read_cart_u8(self.program_counter);
                self.program_counter += 1;
                tmp = tmp.wrapping_add(self.index_x);
                self.bus.ram[tmp as usize]
            }

            AddressMode::indirect_y => {
                let mut tmp = self.bus.cart.read_cart_u8(self.program_counter);
                self.program_counter += 1;
                println!("Load ind,Y is {:#X} + {:#X}", tmp, self.index_y);
                panic!("carry not yet implemented here, it might fail if I continue");
                tmp += self.index_y;
                self.bus.ram[tmp as usize]
            }
        }
    }

    fn compare(&mut self, register:i16) {
        let value = self.load_u8_from_memory(AddressMode::immediate) as i16;
        println!("CMY {:#X} - {:#X}", register, value);
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
            let value = self.load_u8_from_memory(AddressMode::immediate) as i8;
            println!("{:}", value);
            if value >= 0 {
                self.program_counter += value as u16;
            } else {
                self.program_counter -= value as u16;
            }
        } else {
            self.program_counter += 1;
        }
    }

    // setting the accumulator always sets N and Z appropriately
    fn set_register (&mut self, value: u8, reg: RegType) {
        self.status_reg.zero = value == 0;
        self.status_reg.negative_sign = (value & (1 << 7)) != 0;
        match reg {
            RegType::A => self.accumulator = value,
            RegType::X => self.index_x = value,
            RegType::Y => self.index_y = value,
        }
    }

    fn push_stack (&mut self, value: u8) {
        self.bus.ram[0x100 + self.stack_pointer as usize] = value;
        println!("stack wrote at 0x01{:x}: {:x}", self.stack_pointer, value);
        self.stack_pointer -= 1;
    }

    fn pull_stack (&mut self) -> u8 {
        self.stack_pointer += 1;
        self.bus.ram[0x100 + self.stack_pointer as usize]
    }

    // peek_stack??
}



impl From<u8> for StatusReg {
    fn from(value: u8) -> Self {
      StatusReg {
        negative_sign: (value & (1 << 7)) != 0,     //N
        overflow: (value & (1 << 6)) != 0,          //V
        unused: true,
        break_flag: (value & (1 << 4)) != 0,        //B
        decimal_mode: (value & (1 << 3)) != 0,      //D
        interrupt_disable: (value & (1 << 2)) != 0, //I
        zero: (value & (1 << 1)) != 0,              //Z
        carry: (value & (1 << 0)) != 0              //C
      }
    }
}

impl Into<u8> for StatusReg {
   fn into(self) -> u8 {
     let mut value:u8 = 0;

     if self.negative_sign {value = value | 1 << 7}
     if self.overflow {value = value | 1 << 6}
     if self.unused {value = value | 1 << 5}
     if self.break_flag {value = value | 1 << 4}
     if self.decimal_mode {value = value | 1 << 3}
     if self.interrupt_disable {value = value | 1 << 2}
     if self.zero {value = value | 1 << 1}
     if self.carry {value = value | 1 << 0}

     value
   }

}



use super::*;
//use std::collections::HashSet;

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

#[derive(Debug)]
enum RegType {
    A,
    X,
    Y,
}

#[derive(Debug)]
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
 //       if self.program_counter == 0xd942 {break;}
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

        // LSR - A
        0x4A => self.shift_right_to_A(AddressMode::Accumulator),

        // LSR - zeropage
        0x46 => self.shift_right_to_A(AddressMode::Zeropage),

        // LSR - abs
        0x4E => self.shift_right_to_A(AddressMode::Absolute),

        // ASL - A
        0x0A => self.shift_left_to_A(AddressMode::Accumulator),

        // ASL - zpg
        0x06 => self.shift_left_to_A(AddressMode::Zeropage),

        // ASL - abs
        0x0E => self.shift_left_to_A(AddressMode::Absolute),

        // ROR - A
        0x6A => self.rotate_right_to_A(AddressMode::Accumulator),

        // ROR - zpg
        0x66 => self.rotate_right_to_A(AddressMode::Zeropage),

        // ROR - abs
        0x6E => self.rotate_right_to_A(AddressMode::Absolute),

        // ROL - A
        0x2A => self.rotate_left_to_A(AddressMode::Accumulator),

        // ROL - zpg
        0x26 => self.rotate_left_to_A(AddressMode::Zeropage),

        // ROL - abs
        0x2E => self.rotate_left_to_A(AddressMode::Absolute),

        0xA9 => { // LDA - immediate
            let value = self.load_u8_from_memory(AddressMode::Immediate);
            self.set_register(value, RegType::A);
        }

        0xA5 => { // LDA - zeropage
            let value = self.load_u8_from_memory(AddressMode::Zeropage);
            self.set_register(value, RegType::A);
        }

        0xAD => { // LDA - absolute
            let value = self.load_u8_from_memory(AddressMode::Absolute);
            self.set_register(value, RegType::A);
        }

        0xA1 => { // LDA - indirect,x
            let value = self.load_u8_from_memory(AddressMode::XIndirect);
            self.set_register(value, RegType::A);
        }

        0xB1 => { // LDA - indirect,Y
            let value = self.load_u8_from_memory(AddressMode::IndirectY);
            self.set_register(value, RegType::A);
        }

        0xA2 => { // LDX-immediate
            let value = self.load_u8_from_memory(AddressMode::Immediate);
            self.set_register(value, RegType::X);
        }

        0xAE => { // LDX-absolute
            let value = self.load_u8_from_memory(AddressMode::Absolute);
            self.set_register(value, RegType::X);
        }

        0xA6 => { // LDX-zeropage
            let value = self.load_u8_from_memory(AddressMode::Zeropage);
            self.set_register(value, RegType::X);
        }

        0xA0 => { // LDY-immediate
            let value = self.load_u8_from_memory(AddressMode::Immediate);
            self.set_register(value, RegType::Y);
        }

        0xA4 => { // LDY - zeropage
            let value = self.load_u8_from_memory(AddressMode::Zeropage);
            self.set_register(value, RegType::Y);
        }

        0xAC => { // LDY - absolute
            let value = self.load_u8_from_memory(AddressMode::Absolute);
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

        // ORA - imm
        0x09 => self.bitwise_op_to_A(|a, m| a | m, AddressMode::Immediate),

        // ORA - zpg
        0x05 => self.bitwise_op_to_A(|a, m| a | m, AddressMode::Zeropage),

        // ORA - ind,x
        0x01 => self.bitwise_op_to_A(|a, m| a | m, AddressMode::XIndirect),

        // ORA - abs
        0x0D => self.bitwise_op_to_A(|a, m| a | m, AddressMode::Absolute),

        // EOR - imm
        0x49 => self.bitwise_op_to_A(|a, m| a ^ m, AddressMode::Immediate),

        // EOR - zpg
        0x45 => self.bitwise_op_to_A(|a, m| a ^ m, AddressMode::Zeropage),

        // EOR - ind,x
        0x41 => self.bitwise_op_to_A(|a, m| a ^ m, AddressMode::XIndirect),

        // EOR - abs
        0x4D => self.bitwise_op_to_A(|a, m| a ^ m, AddressMode::Absolute),

        // ADC - imm
        0x69 => self.add_with_carry(AddressMode::Immediate),

        // ADC - zpg
        0x65 => self.add_with_carry(AddressMode::Zeropage),

        // ADC - ind,x
        0x61 => self.add_with_carry(AddressMode::XIndirect),

        // ADC - abs
        0x6D => self.add_with_carry(AddressMode::Absolute),

        // SBC - imm
        0xE9 => self.sub_with_carry(AddressMode::Immediate),

        // SBC - zpg
        0xE5 => self.sub_with_carry(AddressMode::Zeropage),

        // SBC - ind,x
        0xE1 => self.sub_with_carry(AddressMode::XIndirect),

        // SBC - ind,x
        0xED => self.sub_with_carry(AddressMode::Absolute),

        0x86 => { // STX-zeropage
            let tmp = self.index_x;
            self.store_u8_in_memory(tmp, AddressMode::Zeropage);
        }

        0x8E => { // STX-absolute
            let tmp = self.index_x;
            self.store_u8_in_memory(tmp, AddressMode::Absolute);
        }

        0x85 => { // STA-zeropage
            let tmp = self.accumulator;
            self.store_u8_in_memory(tmp, AddressMode::Zeropage);
        }

        0x8D => { // STA-absolue
            let tmp = self.accumulator;
            self.store_u8_in_memory(tmp, AddressMode::Absolute);
        }

        0x81 => { // STA-ind,x
            let tmp = self.accumulator;
            self.store_u8_in_memory(tmp, AddressMode::XIndirect);
        }

        0x84 => { //STY - zeropage
            let tmp = self.index_y;
            self.store_u8_in_memory(tmp, AddressMode::Zeropage);
        }

        //STY - absolute
        0x8C => {
            let tmp = self.index_y;
            self.store_u8_in_memory(tmp, AddressMode::Absolute);
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

        // INC - zeropage
        0xE6 => self.increment_memory(AddressMode::Zeropage),

        // INC - absolute
        0xEE => self.increment_memory(AddressMode::Absolute),

        // DEC - zeropage
        0xC6 => self.decrement_memory(AddressMode::Zeropage),

        // DEC - absolute
        0xCE => self.decrement_memory(AddressMode::Absolute),

        0xCA => { //DEX - impl
            let tmp = self.index_x.wrapping_sub(1);
            self.set_register(tmp, RegType::X)
        }

        0xBA => { // TSX -impl
            let tmp = self.stack_pointer;
            self.set_register(tmp, RegType::X)
        }

        // TXS -impl
        0x9A => self.stack_pointer = self.index_x,


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
            let value = self.load_u8_from_memory(AddressMode::Zeropage);
            let result = self.accumulator & value;
            println!("BIT {:#x} & {:#x}: {:#x}", self.accumulator, value, result);
            self.status_reg.zero = result == 0;
            self.status_reg.negative_sign = (value & (1 << 7)) != 0;
            self.status_reg.overflow = (value & (1 << 6)) != 0;
        }

        0x2C => { // BIT - absolute
            let value = self.load_u8_from_memory(AddressMode::Absolute);
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

        // PHA - impl
        0x48 => {
            let tmp = self.accumulator;
            self.push_stack(tmp);
        }

        // AND - immediate
        0x29 => self.bitwise_op_to_A(|a, m| a & m, AddressMode::Immediate),

        // AND - zeropage
        0x25 => self.bitwise_op_to_A(|a, m| a & m, AddressMode::Zeropage),

        // AND - xindirect
        0x21 => self.bitwise_op_to_A(|a, m| a & m, AddressMode::XIndirect),

        // AND absolute
        0x2D => self.bitwise_op_to_A(|a, m| a & m, AddressMode::Absolute),

        // CMP - immediate
        0xC9 => self.compare(RegType::A, AddressMode::Immediate),

        // CMP - immediate
        0xC5 => self.compare(RegType::A, AddressMode::Zeropage),

        // CMP - x,ind
        0xC1 => self.compare(RegType::A, AddressMode::XIndirect),

        // CMP - abs
        0xCD => self.compare(RegType::A, AddressMode::Absolute),

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

        _ => panic!("The opcode: {:#x} is unrecognized", instr)
      }
    }

    fn rotate_right_to_A(&mut self, addr_mode: AddressMode) {
        let value = self.load_u8_from_memory(addr_mode);
        let c = if self.status_reg.carry { 1 } else { 0 };
        self.status_reg.carry = (value & (1 << 0)) != 0;
        let value = (value >> 1) | c << 7;
        self.set_register(value, RegType::A);
    }

    fn rotate_left_to_A(&mut self, addr_mode: AddressMode) {
        let value = self.load_u8_from_memory(addr_mode);
        let c = if self.status_reg.carry { 1 } else { 0 };
        self.status_reg.carry = (value & (1 << 7)) != 0;
        let value = (value << 1) | c << 0;
        self.set_register(value, RegType::A);
    }

    fn shift_right_to_A(&mut self, addr_mode: AddressMode) {
        let value = self.load_u8_from_memory(addr_mode);
        self.status_reg.carry = (value & (1 << 0)) != 0;
        let value = value >> 1;
        self.set_register(value, RegType::A);
    }

    fn shift_left_to_A(&mut self, addr_mode: AddressMode) { // ASL - zeropage
        let value = self.load_u8_from_memory(addr_mode);
        self.status_reg.carry = (value & (1 << 7)) != 0;
        let value = ((value as u16) << 1) as u8;
        self.set_register(value, RegType::A);
    }

    fn bitwise_op_to_A<F>(&mut self, f: F, addr_mode: AddressMode)
                          where F: FnOnce(u8, u8) -> u8 {

        let m = self.load_u8_from_memory(addr_mode);
        let a = self.accumulator;
        let value = f(a, m);
        self.set_register(value, RegType::A);
    }

    fn add_with_carry(&mut self, addr_mode: AddressMode) {
        let value = self.load_u8_from_memory(addr_mode) as u16;
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

    fn sub_with_carry(&mut self, addr_mode: AddressMode) {
        let value = self.load_u8_from_memory(addr_mode) as i16;
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

    fn increment_memory(&mut self, addr_mode: AddressMode) {
        let addr = self.memory_lookup(addr_mode);
        let mut value = self.bus.ram[addr];
        value = value.wrapping_add(1);
        self.status_reg.zero = value == 0;
        self.status_reg.negative_sign = (value & (1 << 7)) != 0;
        self.bus.ram[addr] = value;
    }

    fn decrement_memory(&mut self, addr_mode: AddressMode) {
        let addr = self.memory_lookup(addr_mode);
        let mut value = self.bus.ram[addr];
        value = value.wrapping_sub(1);
        self.status_reg.zero = value == 0;
        self.status_reg.negative_sign = (value & (1 << 7)) != 0;
        self.bus.ram[addr] = value;
    }

    fn memory_lookup(&mut self, addr_mode: AddressMode) -> usize {
        match addr_mode {
            AddressMode::Zeropage => {
                let tmp = self.bus.cart.read_cart_u8(self.program_counter);
                self.program_counter += 1;
                tmp as usize
            }

            AddressMode::ZeropageX => self.zeropage_xy(RegType::X),
            AddressMode::Absolute => {
                let tmp = self.bus.cart.read_cart_u16(self.program_counter);
                self.program_counter += 2;
                tmp as usize
            }
            AddressMode::AbsoluteX => self.absolute_xy(RegType::X),

            _ => panic!("address_lookup doesn't work on {:?}", addr_mode)
        }
    }

    fn load_u8_from_memory(&mut self, addr_mode: AddressMode) -> u8 {
        match addr_mode {
            AddressMode::Accumulator => self.accumulator,
            AddressMode::Immediate => {
                self.program_counter += 1;
                self.bus.cart.read_cart_u8(self.program_counter - 1)
            }

            AddressMode::Zeropage => {
                let tmp = self.bus.cart.read_cart_u8(self.program_counter);
                self.program_counter += 1;
                self.bus.ram[tmp as usize]
            }

            AddressMode::ZeropageX => self.bus.ram[self.zeropage_xy(RegType::X)],

            AddressMode::ZeropageY => self.bus.ram[self.zeropage_xy(RegType::Y)],

            AddressMode::Absolute => {
                let tmp = self.bus.cart.read_cart_u16(self.program_counter);
                self.program_counter += 2;
                self.bus.ram[tmp as usize]
            }

            AddressMode::AbsoluteX => self.bus.ram[self.absolute_xy(RegType::X)],

            AddressMode::AbsoluteY => self.bus.ram[self.absolute_xy(RegType::Y)],

            AddressMode::XIndirect => self.bus.ram[self.x_indirect()],

            AddressMode::IndirectY => self.bus.ram[self.indirect_y()],

        }
    }

    fn store_u8_in_memory(&mut self, value: u8, addr_mode: AddressMode) {
        match addr_mode {
            AddressMode::Zeropage => {
                let tmp = self.bus.cart.read_cart_u8(self.program_counter) as usize;
                self.program_counter += 1;
                self.bus.ram[tmp] = value;
            }

            AddressMode::ZeropageX => self.bus.ram[self.zeropage_xy(RegType::X)] = value,

            AddressMode::ZeropageY => self.bus.ram[self.zeropage_xy(RegType::Y)] = value,

            AddressMode::Absolute => {
                let tmp = self.bus.cart.read_cart_u16(self.program_counter);
                self.program_counter += 2;
                self.bus.ram[tmp as usize] = value;
            }

            AddressMode::AbsoluteX => self.bus.ram[self.absolute_xy(RegType::X)] = value,

            AddressMode::AbsoluteY => self.bus.ram[self.absolute_xy(RegType::Y)] = value,

            AddressMode::XIndirect => self.bus.ram[self.x_indirect()] = value,

            AddressMode::IndirectY => self.bus.ram[self.indirect_y()] = value,

            _ => panic!("It's not possible to write with {:?}", addr_mode),

        }
    }

    fn x_indirect(&mut self) -> usize {
        let mut tmp = self.bus.cart.read_cart_u8(self.program_counter);
        self.program_counter += 1;
        tmp = tmp.wrapping_add(self.index_x);
        let lo = self.bus.ram[tmp as usize] as usize;
        let hi = self.bus.ram[(tmp.wrapping_add(1)) as usize] as usize;
        hi << 8 | lo
    }

    fn zeropage_xy(&mut self, reg: RegType) -> usize {
        let tmp = self.bus.cart.read_cart_u8(self.program_counter);
        self.program_counter += 1;
        match reg {
            RegType::X => tmp.wrapping_add(self.index_x) as usize,
            RegType::Y => tmp.wrapping_add(self.index_y) as usize,
            _ => panic!("can not zpg,A")
        }
    }

    // wrapping add?
    fn absolute_xy(&mut self, reg: RegType) -> usize {
        let tmp = self.bus.cart.read_cart_u16(self.program_counter);
        self.program_counter += 2;
        let result = match reg {
            RegType::X => (tmp + self.index_x as u16) as usize,
            RegType::Y => (tmp + self.index_y as u16) as usize,
            _ => panic!("can not abs,A")
        };
        println!("Load abs,{:?} is {:#X} + X:{:#X} or Y:{:#X}", reg, tmp,self.index_x, self.index_y);
        panic!("carry not yet implemented here, it might fail if I continue");
        // TODO: implement carry
        result
    }

    fn indirect_y(&mut self) -> usize {
        let tmp = self.bus.cart.read_cart_u8(self.program_counter);
        self.program_counter += 1;
        let lo = self.bus.ram[tmp as usize] as u32;
        let hi = self.bus.ram[tmp.wrapping_add(1) as usize] as u32;
        let value:u32 = hi << 8 | lo;
        let result = value + self.index_y as u32;
        self.status_reg.carry = result > 0xFFFF;
        println!("Load ind,Y is {:#X} + {:#X} = {:#X}", value, self.index_y, result);
        result as u16 as usize
    }

    fn compare(&mut self, reg:RegType, addr_mode:AddressMode) {
        let register = match reg {
            RegType::A => self.accumulator as i16,
            RegType::Y => self.index_y as i16,
            RegType::X => self.index_x as i16,
        };
        let value = self.load_u8_from_memory(addr_mode) as i16;
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
            let value = self.load_u8_from_memory(AddressMode::Immediate) as i8;
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



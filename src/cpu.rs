use super::*;

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
	let instr = self.bus.cart.read_cart_u8(self.program_counter);
        // TODO: Move this to a specific debug output
        let tmp:u8 = self.status_reg.into();
        println!("{:#X}  I:{:02X}     A:{:02X} X:{:02X} Y:{:02X} P:{:02X} SP:{:02X}",
                   self.program_counter, instr, self.accumulator, self.index_x, self.index_y,
                   tmp, self.stack_pointer); //, self.status_reg);

        self.execute_op(instr as u8);
      }
    }

    // TODO: Increment PC automatically on cart read?
    // alternatively increment on specific instruction lengths

    pub fn execute_op(&mut self, instr: u8) {
      // TODO: Break these in to actual instructions by name
      // then match the specific instruction type (imm/abs/zeropage/etc)
      self.program_counter += 1;
      match instr {
        0x4c => { // JMP-absolute
                  let value = self.bus.cart.read_cart_u16(self.program_counter);
                  self.program_counter = value;
                }

        0xa2 => { // LDX-immediate
                  let value = self.bus.cart.read_cart_u8(self.program_counter);
                  self.program_counter += 1;
                  self.set_accumulator(value);
                }

        0xa9 => { // LDA - immediate
                  let value = self.bus.cart.read_cart_u8(self.program_counter);
                  self.program_counter += 1;
                  self.set_accumulator(value);
                }

        0x86 => { // STX-zeropage
                  let value = self.bus.cart.read_cart_u8(self.program_counter);
                  self.program_counter += 1;
                  // TODO: Make write_byte function to better map the CPU memory
                  self.bus.ram[value as usize] = self.index_x;
                }

        0x85 => { // STA-zeropage
                  let value = self.bus.cart.read_cart_u8(self.program_counter);
                  self.program_counter += 1;
                  // TODO: Make write_byte function to better map the CPU memory
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

        0xea => { }//println!("NOP");}// NOP // TODO: 2 cycles


        0x50 => { // BVc - relative - 2 bytes
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

        0x24 => { // BIT - zeropage
                    let value = self.bus.cart.read_cart_u8(self.program_counter);
                    self.program_counter += 1;
                    let tmp = self.bus.ram[value as usize] & self.accumulator;
                    println!("BIT result: {:#x}", tmp);
                    if tmp == 0 {self.status_reg.zero = true;}
                    self.status_reg.negative_sign = (tmp & (1 << 7)) != 0;
                    self.status_reg.overflow = (tmp & (1 << 6)) != 0;
                }

        0x60 => { // RTS - implied
                    //TODO ADD stack pull
                    let lo = self.pull_stack() as u16;
                    let hi = self.pull_stack() as u16;
                    let value:u16 = hi << 8 | lo;
                    self.program_counter = value + 1;
                }

                  // SEI - impl
        0x78 => self.status_reg.interrupt_disable = true,

                  // SED - impl
        0xF8 => self.status_reg.decimal_mode = true,

                  // CLD - impl
        0xD8 =>  self.status_reg.decimal_mode = false,

                // SEC - implied
        0x38 => self.status_reg.carry = true,

                // CLC - implied
        0x18 => self.status_reg.carry = false,

        0x08 => { // PHP - impl
                    let mut tmp:u8 = self.status_reg.into();
                    tmp |= 1<<4; // clear the break flag before pushing
                    self.push_stack(tmp);
                }

        0x28 => { // PLP - impl
                    let value = self.pull_stack();
                    self.status_reg = value.into();
                }

        0x68 => { // PLA - impl
                    let value = self.pull_stack();
                    self.set_accumulator(value);
                }

        0x48 => { // PHA - impl
                    let tmp = self.accumulator;
                    self.push_stack(tmp);
                }

        0x29 => {  // AND - immediate
                    let value = self.bus.cart.read_cart_u8(self.program_counter);
                    self.program_counter += 1;
                    let result = self.accumulator & value;
                    self.set_accumulator(result);
                }
/*
        0xC9 => { // CMP - immediate
                    let value = self.bus.cart.read_cart_u8(self.program_counter);
                    self.program_counter += 1;

                    if value == 0x64 { panic!("this needs to implement carry flag");}
                    let result = self.accumulator - value;
                    self.status_reg.zero = result == 0;
                    self.status_reg.negative_sign = (result & (1 << 7)) != 0;
                    // TODO: somehow implement carry flag?
                    // see http://www.6502.org/tutorials/compare_beyond.html
                    // http://www.righto.com/2012/12/the-6502-overflow-flag-explained.html
                }
*/
        _ => panic!("The opcode: {:#x} is unrecognized", instr)
      }
    }

    // All branch functions seem to be identical
    fn branch(&mut self, condition: bool) {
        if condition {
            let value = self.bus.cart.read_cart_u8(self.program_counter);
            self.program_counter += 1;
            self.program_counter += value as u16;
        } else {
            self.program_counter += 1;
        }
    }

    // setting the accumulator always sets N and Z
    fn set_accumulator (&mut self, value: u8) {
        self.status_reg.zero = value == 0;
        self.status_reg.negative_sign = (value & (1 << 7)) != 0;
        self.accumulator = value;
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
        unused: (value & (1 << 5)) != 0,
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



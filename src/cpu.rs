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

        status_reg: 0x34.into(),

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
        println!("{:#X}  I:{:02X}     A:{:02X} X:{:02X} Y:{:02X} P:{:02X} SP:{:02X} \n{:?}", 
                   self.program_counter, instr, self.accumulator, self.index_x, self.index_y,
                   tmp, self.stack_pointer, self.status_reg);

        self.execute_op(instr as u8);
      }
    }
 
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
                  self.status_reg.zero = value == 0;
                  self.status_reg.negative_sign = (value & (1 << 7)) != 0;
                  self.index_x = value;
                }

        0xa9 => { // LDA - immediate
                  let value = self.bus.cart.read_cart_u8(self.program_counter);
                  self.program_counter += 1;
                  self.status_reg.zero = value == 0;
                  self.status_reg.negative_sign = (value & (1 << 7)) != 0;
                  self.accumulator = value;
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
                  // little-endianness oddness
                  // TODO: write_stack | pop_stack | etc
                  self.bus.ram[0x100 + self.stack_pointer as usize] = (self.program_counter >> 8) as u8;
                  self.stack_pointer -= 1;
                  self.bus.ram[0x100 + self.stack_pointer as usize] = (0x00ff & self.program_counter) as u8;
                  self.stack_pointer -= 1;
                  println!("stack wrote: {:x} {:x}", self.bus.ram[0x01fc], self.bus.ram[0x01fd]); 
                  self.program_counter = value;
                }

        0xea => { println!("NOP");}// NOP // TODO: 2 cycles

        0x38 => { // SEC - implied
                  self.status_reg.carry = true;
                }

        0x50 => { // BVc - relative - 2 bytes
                  if !self.status_reg.overflow {
                    let value = self.bus.cart.read_cart_u8(self.program_counter);
                    self.program_counter += 1;
                    self.program_counter += value as u16;
                  } else {
                    self.program_counter += 1; }
                }

        0x70 => { // BVS - relative - 2 bytes
                  if self.status_reg.overflow {
                    let value = self.bus.cart.read_cart_u8(self.program_counter);
                    self.program_counter += 1;
                    self.program_counter += value as u16;
                  } else {
                    self.program_counter += 1; }
                }

        0xb0 => { // BCS - relative - 2 bytes
                  if self.status_reg.carry {
                    let value = self.bus.cart.read_cart_u8(self.program_counter);
                    self.program_counter += 1;
                    self.program_counter += value as u16;
                  } else {
                    self.program_counter += 1; } 
                }

        0x18 => { // CLC - implied
                  self.status_reg.carry = false; 
                }

        0x90 => { // BCS - relative - 2 bytes
                  if !self.status_reg.carry {
                    let value = self.bus.cart.read_cart_u8(self.program_counter);
                    self.program_counter += 1;
                    self.program_counter += value as u16;
                  } else {
                    self.program_counter += 1; }
                }


        0xf0 => { // BEQ - relative
                  if self.status_reg.zero {
                    let value = self.bus.cart.read_cart_u8(self.program_counter);
                    self.program_counter += 1;
                    self.program_counter += value as u16;
                  } else {
                    self.program_counter += 1; }
                }
                
        0xd0 => { // BNE - relative
                  if !self.status_reg.zero {
                    let value = self.bus.cart.read_cart_u8(self.program_counter);
                    self.program_counter += 1;
                    self.program_counter += value as u16;
                  } else {
                    self.program_counter += 1; }
                }
        
        0x10 => { // BPL - relative
                  if !self.status_reg.negative_sign {
                    let value = self.bus.cart.read_cart_u8(self.program_counter);
                    self.program_counter += 1;
                    self.program_counter += value as u16;
                  } else {
                    self.program_counter += 1; }
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
                    self.stack_pointer += 1;
                    let lo = self.bus.ram[0x100 + self.stack_pointer as usize] as u16;
                    self.stack_pointer += 1;
                    let hi = self.bus.ram[0x100 + self.stack_pointer as usize] as u16;
                    let value:u16 = hi << 8 | lo;
                    self.program_counter = value + 1; 
                }

        0x78 => { // SEI - impl
                    self.status_reg.interrupt_disable = true;
                }

        0xF8 => { // SED - impl
                    self.status_reg.decimal_mode = true;
                }

        0x08 => { // PHP - impl
                    //TODO: push to stack.
                    self.bus.ram[0x100 + self.stack_pointer as usize] = self.status_reg.into(); 
                    self.stack_pointer -= 1;
                }

        0x68 => { // PLA - impl
                    //TODO: pull from stack
                    self.stack_pointer += 1;
                    let value = self.bus.ram[0x100 + self.stack_pointer as usize];
                    self.accumulator = value;
                }

        _ => panic!("The opcode: {:#x} is unrecognized", instr)
      }
    }
}
// TODO: Generic branch function - check for sign when branching.




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



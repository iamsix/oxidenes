// use std::env;
use std::fmt;

mod cart;
mod mem_map;
mod cpu;

use mem_map::*;

pub struct Bus {
//    cpu: cpu::CPU,
    ram: Box<[u8]>,
    cart: cart::Cart,
//    mem_map: mem_map::*
}

fn main() {

    let cart = cart::Cart::new();
    println!("{:#?}", cart);
    
//  for now we're going to start at 0xc000 instead of the reset vector
//  nestest.nes starts execution here for automation but has a reset
//  vector of 0xc004 for actual execution
    let _pc = cart.read_cart_u16(RESET_VECTOR_LOC);
    let pc = 0xc000;
    let nmi = cart.read_cart_u16(NMI_VECTOR_LOC);
    let brk = cart.read_cart_u16(IRQ_BRK_VECTOR_LOC);

    let bus = Bus { 
         ram: vec![0; RAM_LEN as usize].into_boxed_slice(),
         cart: cart 
    };

    let mut cpu = cpu::CPU::new(bus, pc as u16);
    println!("{:#?}", cpu);
    cpu.run(); 

}

impl fmt::Debug for Bus {
 fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    writeln!(f, "")
 }
}


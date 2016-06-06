// use std::env;
use std::fmt;

mod cart;
mod mem_map;
mod cpu;
mod apu;
mod ppu;

use mem_map::*;
use cpu::RunCondition;

pub struct Bus {
    ram: Box<[u8]>,
    cart: cart::Cart,
    apu: apu::APU,
    ppu: ppu::PPU,
}

fn main() {

    let cart = cart::Cart::new();
    println!("{:#?}", cart);
    let apu = apu::APU::new();
    let ppu = ppu::PPU::new();
    let mut bus = Bus {
        ram: vec![0; RAM_LEN as usize].into_boxed_slice(),
        cart: cart,
        apu: apu,
        ppu: ppu,
    };

    let pc = bus.cart.read_cart_u16(RESET_VECTOR_LOC);
    let mut cpu = cpu::CPU::new(bus, pc as u16);
    println!("{:#?}", cpu);

    {
        cpu.run(RunCondition::NextScanline);
        cpu.bus.ppu.render_scanline();
    }

 //   bus.ppu.render_scanline();

}

impl fmt::Debug for Bus {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f, "")
    }
}

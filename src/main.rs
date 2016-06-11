// use std::env;
use std::fmt;

mod cart;
mod mem_map;
mod cpu;
mod apu;
mod ppu;

use mem_map::*;
// use cpu::RunCondition;

pub struct Bus {
    ram: Box<[u8]>,
    cart: cart::Cart,
    apu: apu::APU,
    ppu: ppu::PPU,
}

fn main() {

    let cart = cart::Cart::new();
    println!("{:#?}", cart);
    let chr_rom = cart::ChrRom::new();
    let apu = apu::APU::new();

    let ppu = ppu::PPU::new(chr_rom);
    let mut cpubus = Bus {
        ram: vec![0; RAM_LEN as usize].into_boxed_slice(),
        cart: cart,
        apu: apu,
        ppu: ppu,
    };

    let pc = cpubus.cart.read_cart_u16(RESET_VECTOR_LOC);
    let mut cpu = cpu::CPU::new(cpubus, pc as u16);
    println!("{:#?}", cpu);

    let mut nmi = false;
    loop {

        let mut pc = cpu.program_counter;
        let mut instr = cpu.cpu_read_u8(pc);
        // TODO: Move this to a specific debug output
        let tmp: u8 = cpu.status_reg.into();
        println!("{:#X}  I:{:02X}                  A:{:02X} X:{:02X} Y:{:02X}  P:{:02X}  \
                  SP:{:02X} CYC:{:>3} SL:{:}",
                 cpu.program_counter,
                 instr,
                 cpu.accumulator,
                 cpu.index_x,
                 cpu.index_y,
                 tmp,
                 cpu.stack_pointer,
                 cpu.cycle % 341,
                 cpu.bus.ppu.scanline,
                 ); //, self.status_reg);


        cpu.cycle = cpu.cycle + (cpu::timing[instr as usize] * cpu::PPU_MULTIPLIER);

        if cpu.cycle >= 341 {
            // println!("Cycles {:}", cpu.cycle);
            cpu.cycle %= 341;
            nmi = cpu.bus.ppu.render_scanline();
        }

    // loop here until the ticks would be 341?
        cpu.execute_op(instr);
        if nmi && cpu.cycle > 2 {
            cpu.nmi();
            nmi = false;
        }

        //if cpu.cycle >= 341 {
        //    cpu.cycle %= 341;
        //}

  //if cpu.program_counter == 0x8057 {break;}


    }

}

impl fmt::Debug for Bus {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f, "")
    }
}

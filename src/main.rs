extern crate sdl2;
extern crate time;

use sdl2::pixels::PixelFormatEnum;
use sdl2::keyboard::Keycode;
use sdl2::event::{Event,WindowEventId};

// use time;

use std::env;
use std::fmt;

mod cart;
mod mem_map;
mod cpu;
mod apu;
mod ppu;
mod opcodes;

use opcodes::AddressMode;

use mem_map::*;
// use cpu::RunCondition;

const PPU_MULTIPLIER:isize = 3;

pub struct Bus {
    ram: Box<[u8]>,
    cart: cart::Cart,
    apu: apu::APU,
    ppu: ppu::PPU,
}

fn main() {
    let rompath = env::args().nth(1).unwrap_or(String::from("smb.nes"));

    let sdl = sdl2::init().unwrap();
    let video = sdl.video().unwrap();
    let window = video.window("OxideNES", 256, 240)
        .position_centered()
        .opengl()
        .build()
        .unwrap();

    let mut renderer = window.renderer().build().unwrap();
    let mut texture = renderer.create_texture_streaming(PixelFormatEnum::RGB24,
                                                        256,
                                                        240).unwrap();
    let mut events = sdl.event_pump().unwrap();


    let cart = cart::Cart::new(&rompath);
    println!("{:#?}", cart);
    let chr_rom = cart::ChrRom::new(&rompath);
    let apu = apu::APU::new();

    let ppu = ppu::PPU::new(chr_rom);
    let cpubus = Bus {
        ram: vec![0; RAM_LEN as usize].into_boxed_slice(),
        cart: cart,
        apu: apu,
        ppu: ppu,
    };

    let pc = cpubus.cart.read_cart_u16(RESET_VECTOR_LOC);
    let mut cpu = cpu::CPU::new(cpubus, pc as u16);
    println!("{:#?}", cpu);

    // let mut ticks = 0;
    // TODO: re-add specific run conditions for debugging
    println!("start time is {}", time::precise_time_ns());

    let mut nmi = false;
    'main: loop {

        let (op, instr) = cpu.read_instruction();


        // TODO: Move this to a specific debug output
        let debug = false;
        if debug {
            cpu_debug(&op, &instr, &cpu);
        }

        cpu.cycle += instr.ticks as isize * PPU_MULTIPLIER;

        if cpu.cycle >= 341 {
            cpu.cycle %= 341;
            nmi = cpu.bus.ppu.render_scanline();
        }
        if !cpu.bus.ppu.sprite0_hit &&
            cpu.bus.ppu.sprite0_dot != 0xFF &&
            cpu.cycle > cpu.bus.ppu.sprite0_dot as isize
        {
            cpu.bus.ppu.sprite0_hit = true;
        }

        cpu.execute_op(op, instr);
        // If the cycle count isn't > 1 yet
        // then the vblank flag wouldn't have been set at this point
        // since vblank is set on dot 1 of line 341
        if nmi && cpu.cycle > 2 {
            cpu.nmi();
            nmi = false;
        }



        if cpu.bus.ppu.scanline == 240 {
            // println!("screen 10,10 properly: {:#X}", cpu.bus.ppu.screen[10][10]);
            render_frame(&cpu.bus.ppu.screen, &mut renderer, &mut texture);
            for event in events.poll_iter() {
                match event {
                    Event::Quit {..} | Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                        break 'main
                    }
                    _ => ()
                }
            }
        }

        if cpu.bus.ppu.scanline == -1 {


            let keys: Vec<Keycode> = events.keyboard_state().pressed_scancodes().
                            filter_map(Keycode::from_scancode).collect();

            for key in keys {
                match key {
                    Keycode::LCtrl => {
                        // panic!("Works..");
                        cpu.joy1 |= 1 << 0;
                    }
                    Keycode::LShift => {
                        cpu.joy1 |= 1 << 1;
                    }
                    Keycode::Space => {
                        cpu.joy1 |= 1 << 2;
                    }
                    Keycode::Return => {
                        cpu.joy1 |= 1 << 3;
                    }
                    Keycode::Up => {
                        cpu.joy1 |= 1 << 4;
                    }
                    Keycode::Down => {
                        cpu.joy1 |= 1 << 5;
                    }
                    Keycode::Left => {
                        cpu.joy1 |= 1 << 6;
                    }
                    Keycode::Right => {
                        cpu.joy1 |= 1 << 7;
                    }
                    _ => ()// panic!("Unkown key {:?}", key),

                }

            }
        }

    }

    println!("start time is {}", time::precise_time_ns());
}


fn cpu_debug (op: &u8, instr: &opcodes::Instruction, cpu: &cpu::CPU) {
    let pc = cpu.program_counter - instr.bytes as u16;
    let operand = if instr.bytes != 1 {
        // opr = instr.operand.unwrap();
        if instr.operand > 0xFF {
            let opr1 = instr.operand & 0xFF;
            let opr2 = instr.operand >> 8;
            format!("{:02X} {:02X}", opr1 as u8, opr2 as u8)
        } else {
            format!("{:02X}   ", instr.operand as u8)
        }
    } else {
        format!("     ")
    };

    let addrs = if instr.dest_addr != None {
        let addr = instr.dest_addr.unwrap();
        let value = if addr < 0x800 {
            format!(" = {:02X}", cpu.bus.ram[addr as usize])
        } else {
            format!("")
        };
        match instr.addr_mode {
            AddressMode::Immediate => format!("#${:02X}", instr.operand as u8),
            AddressMode::Absolute => format!("${:04X}{}", addr, value),
            AddressMode::AbsoluteX => format!("${:04X},X @ {:04X}{}", instr.operand,
                                                                      addr,
                                                                      value),
            AddressMode::AbsoluteY => format!("${:04X},Y @ {:04X}{}", instr.operand,
                                                                      addr,
                                                                      value),
            AddressMode::XIndirect => {
                format!("(${:02X},X) @ {:02X} = {:04X}{}", instr.operand,
                                                            instr.operand.wrapping_add(cpu.index_x as u16),
                                                            addr,
                                                            value)
            }
            AddressMode::IndirectY => {
                format!("(${:02X}),Y = {:04X} @ {:04X}{}", instr.operand,
                                                            addr.wrapping_sub(cpu.index_y as u16),
                                                            addr,
                                                            value)
            }
            AddressMode::Zeropage => format!("${:02X}{}", addr as u8, value),
            AddressMode::ZeropageX => format!("${:02X},X @ {:02X}{}", instr.operand,
                                                                        addr,
                                                                        value),
            AddressMode::ZeropageY => format!("${:02X},Y @ {:02X}{}", instr.operand,
                                                                        addr,
                                                                        value),
            AddressMode::Indirect => format!("(${:04X}) = {:04X}", instr.operand, addr),
            AddressMode::Relative => format!("${:04X}", (cpu.program_counter as i16 +
                                                        instr.operand as i8 as i16) as u16),
            _ => format!(""),
        }
    } else {
        format!("")
    };
    let tmp: u8 = cpu.status_reg.into();
    print!("{:04X}  {:02X} {} {:>4} {:<27} A:{:02X} X:{:02X} Y:{:02X} P:{:02X} \
              SP:{:02X} CYC:{:>3} SL:{:} \r\n",
             pc,
             op,
             operand,
             instr.name,
             addrs,
             cpu.accumulator,
             cpu.index_x,
             cpu.index_y,
             tmp,
             cpu.stack_pointer,
             cpu.cycle % 341,
             cpu.bus.ppu.scanline,
             ); //, self.status_reg);

}


fn render_frame(screen: &[[u32; 256]; 240],
                renderer: &mut sdl2::render::Renderer,
                texture: &mut sdl2::render::Texture,
                // events: &mut sdl2::EventPump,
                )
{
    //println!("Screen 10,10 {:#X}", screen[10][10]);
    texture.with_lock(None, |buffer: &mut [u8], pitch: usize| {
        // println!("pitch is: {:}", pitch);
        for row in 0..240 {
            let offset1 = row * pitch;
            for col in 0..256 {
                let offset2 = col * 3;
                let pixel = screen[row][col];
                let r = (pixel >> 16) as u8;
                let g = ((pixel >> 8) & 0xff) as u8;
                let b = (pixel & 0xff) as u8;

                buffer[offset1 + 0 + offset2] = r;
                buffer[offset1 + 1 + offset2] = g;
                buffer[offset1 + 2 + offset2] = b;

            }
        }
    }).unwrap();

    renderer.clear();
    renderer.copy(&texture, None, None);
    renderer.present();

}








impl fmt::Debug for Bus {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f, "")
    }
}

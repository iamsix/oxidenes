use std::fmt;
use std::fs::File;
use std::io::Read;

use mem_map::*;

pub struct Cart {
    rom: Box<[u8]>,

    prg_rom_banks: u8,
    chr_rom_banks: u8,
    prg_ram_chunks: u8,

    horizontal_mirroring: bool,
    vertical_mirroring: bool,
    four_screen_vram: bool,
    //   prg_ram_present: bool,
    //   trainer: bool,
    mapper: u8,
}

impl Cart {
    pub fn new() -> Cart {
        let romfile = read_rom_file();
        Cart {
            prg_rom_banks: romfile[4],
            chr_rom_banks: romfile[5],
            prg_ram_chunks: romfile[8],

            horizontal_mirroring: romfile[6] & (1 << 0) == 0,
            vertical_mirroring: romfile[6] & (1 << 0) != 0,
            four_screen_vram: romfile[6] & (2 << 3) != 0,
            //           prg_ram_present: false,
            //           trainer: false,
            mapper: (romfile[6] & 0b11110000) >> 4 | romfile[7] & 0b11110000,

            rom: romfile,
        }
    }

    pub fn read_cart_u8(&self, addr: u16) -> u8 {
        let read_pos = self.map_rom(addr);
        // println!("Read position {:#x}", read_pos)
        let value = self.rom[read_pos];
        // println!("Read byte: {:#x}", value);
        value
    }

    pub fn read_cart_u16(&self, addr: u16) -> u16 {
        let read_pos = self.map_rom(addr);
        let value = ((self.rom[read_pos + 1] as u16) << 8 | (self.rom[read_pos] as u16)) as u16;
        // println!("Read 2 bytes: {:#x}", value);
        value
    }

    fn map_rom(&self, addr: u16) -> usize {
        const INES_OFFSET: u16 = 0x10;
        //        println!("Read Address: {:#x}", addr);

        let read_pos: usize;

        if addr >= PRG_ROM_LOWER_START && addr < PRG_ROM_LOWER_START + PRG_ROM_LOWER_LEN - 1 {
            // println!("shouldn't be here yet");
            read_pos = ((addr - PRG_ROM_LOWER_START) + INES_OFFSET) as usize;
        } else if addr >= PRG_ROM_UPPER_START && addr <= PRG_ROM_UPPER_START + (PRG_ROM_UPPER_LEN - 1) {
            // UPPER BLOCK
            let mut block = 0;
            if self.prg_rom_banks > 1 {
                // TODO: MAPPERS!!
                block = 1024 * 16;
            }
            read_pos = ((addr - PRG_ROM_UPPER_START) + block + INES_OFFSET) as usize;
        } else {
            panic!("virtual memory address {:#X} is not in the PRG rom space",
                   addr)
        }

        read_pos
    }
}

// TODO: Read rom file path from args
fn read_rom_file() -> Box<[u8]> {
    let mut rom_file = File::open("nestest.nes").unwrap();
    let mut rom_buffer = Vec::new();
    rom_file.read_to_end(&mut rom_buffer).unwrap();
    rom_buffer.into_boxed_slice()
}


// impl this myself because I don't want to print the actual rom every time
impl fmt::Debug for Cart {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f,
                 "Cart: (
        prg_rom_banks: {:#02}
        chr_rom_banks: {:#02}
        \
                  prg_ram_chunks: {:#02}
        horizontal_mirroring: {:#?}
        \
                  vertical_mirroring: {:#?}
        four_screen_VRAM: {:#?}
        mapper: \
                  {:#02}
     )",
                 self.prg_rom_banks,
                 self.chr_rom_banks,
                 self.prg_ram_chunks,
                 self.horizontal_mirroring,
                 self.vertical_mirroring,
                 self.four_screen_vram,
                 self.mapper)

    }
}

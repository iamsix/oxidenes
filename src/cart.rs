use std::fmt;
use std::fs::File;
use std::io::Read;

use mem_map::*;
const INES_OFFSET: u16 = 0x10;

#[derive(Debug)]
pub struct ChrRom {
    rom: Box<[u8]>,

    prg_rom_banks: u8,
    chr_rom_banks: u8,
    pub horizontal_mirroring: bool,
    pub vertical_mirroring: bool,
    pub four_screen_vram: bool,

    _mapper: u8,
}

// TODO: separate rom_file reads to read only the relevant parts
impl ChrRom {
    pub fn new(rompath: &String) -> ChrRom {

        let romfile = read_rom_file(rompath);

        let mut chr = ChrRom {
            prg_rom_banks: romfile[4],
            chr_rom_banks: romfile[5],

            horizontal_mirroring: romfile[6] & (1 << 0) == 0,
            vertical_mirroring: romfile[6] & (1 << 0) != 0,
            four_screen_vram: romfile[6] & (2 << 3) != 0,
            //           prg_ram_present: false,
            //           trainer: false,
            _mapper: (romfile[6] & 0b11110000) >> 4 | romfile[7] & 0b11110000,

            rom: vec![0; 0x2000].into_boxed_slice(),
        };

        if chr.chr_rom_banks != 0 {
            let offset = INES_OFFSET as usize + (1024 * 16 * chr.prg_rom_banks as usize);
            let mut rom = Vec::new();
            rom.extend_from_slice(&romfile[offset..offset + 0x2000]);
            chr.rom = rom.into_boxed_slice();
        }

        chr
    }

    pub fn read_u8(&self, addr: u16) -> u8 {
        // TODO: MAPPERS!
        self.rom[addr as usize]
    }

    pub fn write_u8(&mut self, addr:u16, data: u8) {

        self.rom[addr as usize] = data;
    }
}



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
    pub fn new(rompath: &String) -> Cart {
        let romfile = read_rom_file(rompath);
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
        // println!("Read byte: {:#x} from {:#x}", value, read_pos);
        value
    }

    pub fn read_cart_u16(&self, addr: u16) -> u16 {
        let read_pos = self.map_rom(addr);
        let value = ((self.rom[read_pos + 1] as u16) << 8 | (self.rom[read_pos] as u16)) as u16;
        // println!("Read 2 bytes: {:#x}", value);
        value
    }

    fn map_rom(&self, addr: u16) -> usize {

        //        println!("Read Address: {:#x}", addr);

        let read_pos: usize;

        if addr >= PRG_ROM_LOWER_START && addr < PRG_ROM_LOWER_START + PRG_ROM_LOWER_LEN {
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
fn read_rom_file(rompath: &String) -> Box<[u8]> {
    let mut rom_file = File::open(rompath).unwrap();
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

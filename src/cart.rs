use std::fmt;
use std::fs::File;
use std::io::Read;

use mem_map::*;
const INES_OFFSET: usize = 0x10;

#[derive(Debug)]
pub struct ChrRom {
    rom: Box<[u8]>,

    prg_rom_banks: u8,
    chr_rom_banks: u8,
    pub horizontal_mirroring: bool,
    pub vertical_mirroring: bool,
    pub four_screen_vram: bool,

    mapper: u8,

// offset addresses, defined during the write stage
/*
    chr_bank_0000: usize,
    chr_bank_0400: usize,
    chr_bank_0800: usize,
    chr_bank_0C00: usize,
    chr_bank_1000: usize,
    chr_bank_1400: usize,
    chr_bank_1800: usize,
    chr_bank_1C00: usize,
    */
    bank: u8,
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
            mapper: (romfile[6] & 0b11110000) >> 4 | romfile[7] & 0b11110000,

            rom: vec![0; 0x2000].into_boxed_slice(),
            bank: 0,
        };

        if chr.chr_rom_banks != 0 {
            let offset = INES_OFFSET as usize + (1024 * 16 * chr.prg_rom_banks as usize);
            let mut rom = Vec::new();
            rom.extend_from_slice(&romfile[offset..]);
            chr.rom = rom.into_boxed_slice();
        }

        chr
    }

    pub fn read_u8(&self, addr: u16) -> u8 {
        // TODO: MAPPERS!
        self.rom[addr as usize + (self.bank as usize * 0x2000)]
    }

    pub fn write_u8(&mut self, addr:u16, data: u8) {

        self.rom[addr as usize + (self.bank as usize * 0x2000)] = data;
    }

}


pub struct Cart {
    rom: Box<[u8]>,
    prg_ram: Box<[u8]>,

    prg_rom_banks: u8,
    chr_rom_banks: u8,
    prg_ram_chunks: u8,

    horizontal_mirroring: bool,
    vertical_mirroring: bool,
    four_screen_vram: bool,
    //   prg_ram_present: bool,
    //   trainer: bool,
    pub mapper: u8,

    pub low_prg_bank: u8,

    prg_bank_8000: usize,
    prg_bank_A000: usize,
    prg_bank_C000: usize,
    prg_bank_E000: usize,

}

impl Cart {
    pub fn new(rompath: &String) -> Cart {
        let romfile = read_rom_file(rompath);
        let mapper = (romfile[6] & 0b11110000) >> 4 | romfile[7] & 0b11110000;
        let rom_banks = romfile[4];
        let prg_bank_8000: usize;
        let prg_bank_A000: usize;
        let prg_bank_C000: usize;
        let prg_bank_E000: usize;
        match mapper {
            0 | 3 => {
                prg_bank_8000 = 0x0000;
                prg_bank_A000 = 0x2000;
                if rom_banks == 1 {
                    prg_bank_C000 = 0x0000;
                    prg_bank_E000 = 0x2000;
                } else {
                    prg_bank_C000 = 0x4000;
                    prg_bank_E000 = 0x6000;
                }
            }
            2 => {
                prg_bank_8000 = 0x0000;
                prg_bank_A000 = 0x2000;
                prg_bank_C000 = (1024 * 16) * (rom_banks as usize - 1);
                prg_bank_E000 = ((1024 * 16) * (rom_banks as usize - 1)) + 0x2000;
            }
            _ => {panic!("Mapper {} not supported", mapper)}

        }

        Cart {
            prg_rom_banks: rom_banks,
            chr_rom_banks: romfile[5],
            prg_ram_chunks: romfile[8],

            horizontal_mirroring: romfile[6] & (1 << 0) == 0,
            vertical_mirroring: romfile[6] & (1 << 0) != 0,
            four_screen_vram: romfile[6] & (2 << 3) != 0,
            //           prg_ram_present: false,
            //           trainer: false,
            mapper: mapper,
            low_prg_bank: 0,
            prg_ram: vec![0; 0x2000].into_boxed_slice(),
            rom: romfile,

            prg_bank_8000: prg_bank_8000,
            prg_bank_A000: prg_bank_A000,
            prg_bank_C000: prg_bank_C000,
            prg_bank_E000: prg_bank_E000,

        }
    }

    pub fn write_cart_u8(&mut self, addr: u16, value: u8, chr: &mut ChrRom) {

        match addr {
            SRAM_START...SRAM_END => {
                // TODO: some mappers have more than 8kb
                let real_addr = (addr - SRAM_START) as usize;
                self.prg_ram[real_addr] = value;
            }
            PRG_ROM_START...PRG_ROM_END => {
                match self.mapper {
                    0 => {}
                    1 => panic!("MMC1 unimplemented"),
                    2 => {
                        let bank = (value & 0xF) as usize;
                        self.prg_bank_8000 = bank * (1024 * 16);
                        self.prg_bank_A000 = (bank * (1024 * 16)) + 0x2000;
                    },
                    3 => chr.bank = value & 0xF,
                    _ => panic!("Mapper {} is unimplemented", self.mapper),
                }
            }
            _ => {println!("tried to write to {:#X}", addr)}

        }

    }

    pub fn read_cart_u8(&self, addr: u16) -> u8 {
        //let read_pos = self.map_rom(addr as usize);
        let value = self.rom[self.map_rom(addr as usize)];
//        println!("Read byte: {:#x} from {:#x}", value, read_pos);
        value
    }

    pub fn read_cart_u16(&self, addr: u16) -> u16 {
        let read_pos = self.map_rom(addr as usize);
        let value = ((self.rom[read_pos + 1] as u16) << 8 | (self.rom[read_pos] as u16)) as u16;
//        println!("Read u16: {:#x} from {:#x}", value, read_pos);
        value
    }

    fn map_rom(&self, addr: usize) -> usize {
        //        println!("Read Address: {:#x}", addr);
        match addr {
            0x8000...0x9FFF => {(addr - 0x8000) + self.prg_bank_8000 + INES_OFFSET}
            0xA000...0xBFFF => {(addr - 0xA000) + self.prg_bank_A000 + INES_OFFSET}
            0xC000...0xDFFF => {(addr - 0xC000) + self.prg_bank_C000 + INES_OFFSET}
            0xE000...0xFFFF => {(addr - 0xE000) + self.prg_bank_E000 + INES_OFFSET}
            _ => {println!("not in rom space {:#X}", addr);
                0
            }
        }
    }
}

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

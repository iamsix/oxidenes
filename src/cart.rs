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

    chr_bank_0000: usize,
    chr_bank_0400: usize,
    chr_bank_0800: usize,
    chr_bank_0C00: usize,
    chr_bank_1000: usize,
    chr_bank_1400: usize,
    chr_bank_1800: usize,
    chr_bank_1C00: usize,


    pub irq: bool,
    irq_latch: u8,
    irq_counter: u8,
    irq_enabled: bool,
    irq_reload_flag: bool,
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


            chr_bank_0000: 0,
            chr_bank_0400: 0x400,
            chr_bank_0800: 0x800,
            chr_bank_0C00: 0xC00,
            chr_bank_1000: 0x1000,
            chr_bank_1400: 0x1400,
            chr_bank_1800: 0x1800,
            chr_bank_1C00: 0x1C00,

            irq: false,
            irq_latch: 0,
            irq_counter: 0,
            irq_enabled: false,
            irq_reload_flag: false,
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
        self.rom[self.map_chr_rom(addr)]
    }

    pub fn write_u8(&mut self, addr:u16, data: u8) {
        // this will probably fail badly due to chr ram vs rom...
        self.rom[self.map_chr_rom(addr)] = data;
    }

    fn map_chr_rom(&self, addr: u16) -> usize {
        match addr {
            0x0000...0x03FF => self.chr_bank_0000 + addr as usize,
            0x0400...0x07FF => self.chr_bank_0400 + (addr as usize - 0x0400),
            0x0800...0x0BFF => self.chr_bank_0800 + (addr as usize - 0x0800),
            0x0C00...0x0FFF => self.chr_bank_0C00 + (addr as usize - 0x0C00),
            0x1000...0x13FF => self.chr_bank_1000 + (addr as usize - 0x1000),
            0x1400...0x17FF => self.chr_bank_1400 + (addr as usize - 0x1400),
            0x1800...0x1BFF => self.chr_bank_1800 + (addr as usize - 0x1800),
            0x1C00...0x1FFF => self.chr_bank_1C00 + (addr as usize - 0x1C00),
            _ => panic!("Address not in CHR rom space"),
        }
    }

    pub fn switch_8kb_bank (&mut self, bank: u8) {
        self.chr_bank_0000 = (bank as usize * 0x2000) + 0x0000;
        self.chr_bank_0400 = (bank as usize * 0x2000) + 0x0400;
        self.chr_bank_0800 = (bank as usize * 0x2000) + 0x0800;
        self.chr_bank_0C00 = (bank as usize * 0x2000) + 0x0C00;
        self.chr_bank_1000 = (bank as usize * 0x2000) + 0x1000;
        self.chr_bank_1400 = (bank as usize * 0x2000) + 0x1400;
        self.chr_bank_1800 = (bank as usize * 0x2000) + 0x1800;
        self.chr_bank_1C00 = (bank as usize * 0x2000) + 0x1C00;
    }

    pub fn switch_4kb_bank (&mut self, bank: u8, lower_window: bool) {
        if lower_window {
            self.chr_bank_0000 = (bank as usize * 0x1000) + 0x0000;
            self.chr_bank_0400 = (bank as usize * 0x1000) + 0x0400;
            self.chr_bank_0800 = (bank as usize * 0x1000) + 0x0800;
            self.chr_bank_0C00 = (bank as usize * 0x1000) + 0x0C00;
        } else {
            self.chr_bank_1000 = (bank as usize * 0x1000) + 0x0000;
            self.chr_bank_1400 = (bank as usize * 0x1000) + 0x0400;
            self.chr_bank_1800 = (bank as usize * 0x1000) + 0x0800;
            self.chr_bank_1C00 = (bank as usize * 0x1000) + 0x0C00;
        }
    }

    pub fn switch_2kb_bank (&mut self, bank: u8, window: u8) {
        match window {
            0 => {
                self.chr_bank_0000 = bank as usize * 0x800;
                self.chr_bank_0400 = (bank as usize * 0x800) + 0x400;
            }
            1 => {
                self.chr_bank_0800 = bank as usize * 0x800;
                self.chr_bank_0C00 = (bank as usize * 0x800) + 0x400;
            }
            2 => {
                self.chr_bank_1000 = bank as usize * 0x800;
                self.chr_bank_1400 = (bank as usize * 0x800) + 0x400;
            }
            3 => {
                self.chr_bank_1800 = bank as usize * 0x800;
                self.chr_bank_1C00 = (bank as usize * 0x800) + 0x400;
            }
            _ => panic!("There aren't that many 2kb windows in chr"),
        }
    }

    pub fn switch_1kb_bank (&mut self, bank: u8, window: u8) {
        match window {
            0 => self.chr_bank_0000 = bank as usize * 0x400,
            1 => self.chr_bank_0400 = bank as usize * 0x400,
            2 => self.chr_bank_0800 = bank as usize * 0x400,
            3 => self.chr_bank_0C00 = bank as usize * 0x400,
            4 => self.chr_bank_1000 = bank as usize * 0x400,
            5 => self.chr_bank_1400 = bank as usize * 0x400,
            6 => self.chr_bank_1800 = bank as usize * 0x400,
            7 => self.chr_bank_1C00 = bank as usize * 0x400,
            _ => panic!("There aren't that many 1kb windows in chr"),
        }
    }

    // ppu clocks mapper 4 for me.
    pub fn irq_clock (&mut self) -> bool {
        if self.mapper == 4 {
            if self.irq_counter == 0 && self.irq_enabled {
                self.irq_counter = self.irq_latch - 1;
                self.irq = true;
//                return true
            } else if self.irq_reload_flag || self.irq_counter == 0 {
                self.irq_reload_flag = false;
                self.irq_counter = self.irq_latch - 1;
            }

            self.irq_counter -= 1;
        }
        return self.irq;
    }

}

// not sure how I'm going to do more complex mppers here...

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

    prg_bank_8000: usize,
    prg_bank_A000: usize,
    prg_bank_C000: usize,
    prg_bank_E000: usize,

    // used by mappers - not as nice as names but works as long as I keep them straight
    // I'll have to see if there's a better way to do this
    generic_registers: [u8; 32],
    last_write_addr: u16,

}

// TODO: Change horizontal/vertical/single/4screen to an enum
// and decide who owns that enum (probably chr since ppu uses it)

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
            1 | 2 | 4 => {
                // technically first bank is undefined on mmc3 but it doesn't matter
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
            prg_ram: vec![0; 0x2000].into_boxed_slice(),
            rom: romfile,

            prg_bank_8000: prg_bank_8000,
            prg_bank_A000: prg_bank_A000,
            prg_bank_C000: prg_bank_C000,
            prg_bank_E000: prg_bank_E000,

            generic_registers: [0; 32],
            last_write_addr: 0x0,
        }
    }

    pub fn write_cart_u8(&mut self, addr: u16, value: u8, chr: &mut ChrRom) {
        self.last_write_addr = addr;

        match addr {
            SRAM_START...SRAM_END => {
                // TODO: some mappers have more than 8kb
                let real_addr = (addr - SRAM_START) as usize;
                self.prg_ram[real_addr] = value;
            }
            PRG_ROM_START...PRG_ROM_END => {
                match self.mapper {
                    0 => {}
                    1 => self.mmc1_write(addr, value, chr),
                    2 => {
                        let bank = (value & 0xF) as usize;
                        self.set_16kb_prg_bank(bank, true);
                    },
                    3 => chr.switch_8kb_bank(value & 0xF),
                    4 => self.mmc3_write(addr, value, chr),
                    _ => panic!("Mapper {} is unimplemented", self.mapper),
                }
            }
            _ => {println!("tried to write to {:#X} PRG", addr)}

        }

    }

    // fn irq_check?? - not sure if all irqs work the same or if I should generic_registers it

    fn set_8kb_prg_bank (&mut self, bank: usize, window: u8) {
        match window {
            0 => self.prg_bank_8000 = (1024 * 8) * bank,
            1 => self.prg_bank_A000 = (1024 * 8) * bank,
            2 => self.prg_bank_C000 = (1024 * 8) * bank,
            3 => self.prg_bank_E000 = (1024 * 8) * bank,
            _ => panic!("there's no that many 8kb prg windows")
        }
    }

    fn set_16kb_prg_bank (&mut self, bank: usize, lower_window: bool) {
        if lower_window {
            self.prg_bank_8000 = (1024 * 16) * bank;
            self.prg_bank_A000 = ((1024 * 16) * bank) + 0x2000;
        } else {
            self.prg_bank_C000 = (1024 * 16) * bank;
            self.prg_bank_E000 = ((1024 * 16) * bank) + 0x2000;
        }
    }

    fn set_32kb_prg_bank (&mut self, bank: usize) {
        self.prg_bank_8000 = bank * (1024 * 32);
        self.prg_bank_A000 = bank * (1024 * 32) + 0x2000;
        self.prg_bank_C000 = bank * (1024 * 32) + 0x4000;
        self.prg_bank_E000 = bank * (1024 * 32) + 0x6000;
    }

    fn mmc3_write (&mut self, addr: u16, value: u8, chr: &mut ChrRom) {
        // 0 - R0
        // 1 - R1
        // ...
        // 7 - R7
        // 31 = Bank select  (control)
        match addr & 0xE000 {
            0x8000 => {
                // the meat of the mapper...
                if addr % 2 == 0 {
                    self.generic_registers[31] = value;
                } else {
                    let bankmode = self.generic_registers[31] & 0x7;
                    self.generic_registers[bankmode as usize] = value;
                }
                let reg = self.generic_registers;

                let prgmode = self.generic_registers[31] & 0x40;
                if prgmode == 0 {
                    self.set_8kb_prg_bank(reg[6] as usize & 0x3F, 0);
                    self.set_8kb_prg_bank(reg[7] as usize & 0x3F, 1);
                    let lastbank = self.prg_rom_banks as usize - 1;
                    self.set_16kb_prg_bank(lastbank, false);
                } else {
                    // 8kb banks instead of 16kb banks specified by rom header
                    let lastbank = ((self.prg_rom_banks * 2) - 1) as usize;
                    self.set_8kb_prg_bank((lastbank - 1), 0);
                    self.set_8kb_prg_bank(reg[7] as usize & 0x3F, 1);
                    self.set_8kb_prg_bank(reg[6] as usize & 0x3F, 2);
                    self.set_8kb_prg_bank(lastbank, 3);
                }


                let chrmode = self.generic_registers[31] & 0x80;
                if chrmode == 0 {
                    chr.switch_1kb_bank(reg[0] & 0xFE, 0);
                    chr.switch_1kb_bank(reg[0] | 1, 1);
                    chr.switch_1kb_bank(reg[1] & 0xFE, 2);
                    chr.switch_1kb_bank(reg[1] | 1, 3);
                    chr.switch_1kb_bank(reg[2], 4);
                    chr.switch_1kb_bank(reg[3], 5);
                    chr.switch_1kb_bank(reg[4], 6);
                    chr.switch_1kb_bank(reg[5], 7);
                } else {
                    chr.switch_1kb_bank(reg[2], 0);
                    chr.switch_1kb_bank(reg[3], 1);
                    chr.switch_1kb_bank(reg[4], 2);
                    chr.switch_1kb_bank(reg[5], 3);
                    chr.switch_1kb_bank(reg[0] & 0xFE, 4);
                    chr.switch_1kb_bank(reg[0] | 1, 5);
                    chr.switch_1kb_bank(reg[1] & 0xFE, 6);
                    chr.switch_1kb_bank(reg[1] | 1, 7);
                }
            }
            0xA000 => {
                if addr % 2 == 0 {
                    self.horizontal_mirroring = (value & 1) == 1;
                    chr.horizontal_mirroring = self.horizontal_mirroring;
                    self.vertical_mirroring = (value & 1) == 0;
                    chr.vertical_mirroring = self.vertical_mirroring;
                } else {
                    // prg ram stuff... can be ignored pretty safely
                }
            }
            0xC000 => {
                if addr % 2 == 0 {
                    chr.irq_latch = value;
                } else {
                    chr.irq_reload_flag = true;
                }
            }
            0xE000 => {
                if addr % 2 == 0 {
                    chr.irq_enabled = false;
                    chr.irq = false;
                } else {
                    chr.irq_enabled = true;
                }
            }
            _ => panic!("Invalid address MMC3 write"),
        }
    }

    fn mmc1_write (&mut self, addr: u16, value: u8, chr: &mut ChrRom) {
        // 0 = load register
        // 1 = control register
        // 8 = load reg write counter
        if value & 0x80 != 0 {
            self.generic_registers[0] = 0;
            self.generic_registers[1] |= 0x0C;
            self.generic_registers[8] = 0;
        } else {
            self.generic_registers[0] >>= 1;
            self.generic_registers[0] |= (value & 1) << 4;
            self.generic_registers[8] += 1;
        }
        if self.generic_registers[8] == 5 {
            // println!("control load {:#X} on {:#X}", self.generic_registers[0], self.last_write_addr);
            match addr {
                // control
                0x8000...0x9FFF => {
                    self.generic_registers[1] = self.generic_registers[0];
                    self.vertical_mirroring = self.generic_registers[1] & 3 == 2;
                    chr.vertical_mirroring = self.vertical_mirroring;
                    self.horizontal_mirroring = self.generic_registers[1] & 3 == 3;
                    chr.horizontal_mirroring = self.horizontal_mirroring;
                    if (self.generic_registers[1] & 3) < 2 {
                        panic!("single screen mirroring");
                    }
                }

                // chr bank 0 or 8kb bank
                0xA000...0xBFFF => {
                    if self.generic_registers[1] & 0x10 != 0 {
                        let bank = self.generic_registers[0];
                        chr.switch_4kb_bank(bank, true);
                    } else {
                        let bank = self.generic_registers[0] & 0xE;
                        chr.switch_8kb_bank(bank);
                    }
                }

                // chr bank 1
                0xC000...0xDFFF => {
                    if self.generic_registers[1] & 0x10 != 0 {
                        let bank = self.generic_registers[0];
                        chr.switch_4kb_bank(bank, false);
                    }
                }

                // prg bank
                0xE000...0xFFFF => {
                    let switchmode = (self.generic_registers[1] >> 2) & 3;
                    let bank = (self.generic_registers[0] & 0xF) as usize;
                    if switchmode == 0 || switchmode == 1 {
                        // 32kb switch mode
                        let bank = (self.generic_registers[0] & 0xE) as usize;
                        self.set_32kb_prg_bank(bank);
                    }
                    if switchmode == 2 {
                        self.set_16kb_prg_bank(0, true);
                        self.set_16kb_prg_bank(bank, false);
                    }
                    if switchmode == 3 {
                        self.set_16kb_prg_bank(bank, true);
                        let lastbank = self.prg_rom_banks as usize - 1;
                        self.set_16kb_prg_bank(lastbank, false);
                    }
                }

                _ => panic!("unreachable mmc1"),
            }
            self.generic_registers[0] = 0;
            self.generic_registers[8] = 0;
        }
    }

    pub fn read_cart_u8(&self, addr: u16) -> u8 {
        match addr {
            SRAM_START...SRAM_END => {
                // TODO: some mappers have more than 8kb
                let real_addr = (addr - SRAM_START) as usize;
                self.prg_ram[real_addr]
            }
            PRG_ROM_START...PRG_ROM_END => self.rom[self.map_rom(addr as usize)],
            _ => {0}
        }
    }

    // should only be used by pc so sram isn't entirely needed
    pub fn read_cart_u16(&self, addr: u16) -> u16 {
        match addr {
            SRAM_START...SRAM_END => {
                // TODO: some mappers have more than 8kb
                let real_addr = (addr - SRAM_START) as usize;
                (self.prg_ram[real_addr + 1] as u16) << 8 | self.prg_ram[real_addr] as u16
            }
            PRG_ROM_START...PRG_ROM_END => {
                let read_pos = self.map_rom(addr as usize);
                ((self.rom[read_pos + 1] as u16) << 8 | (self.rom[read_pos] as u16)) as u16
            }
            _ => panic!("not in prg rom space u16")
        }
    }

    fn map_rom(&self, addr: usize) -> usize {
        //        println!("Read Address: {:#x}", addr);
        match addr {
            0x8000...0x9FFF => {(addr - 0x8000) + self.prg_bank_8000 + INES_OFFSET}
            0xA000...0xBFFF => {(addr - 0xA000) + self.prg_bank_A000 + INES_OFFSET}
            0xC000...0xDFFF => {(addr - 0xC000) + self.prg_bank_C000 + INES_OFFSET}
            0xE000...0xFFFF => {
                let actual = (addr - 0xE000) + self.prg_bank_E000 + INES_OFFSET;
                if addr == RESET_VECTOR_LOC as usize {
                    println!("Actual reset vector is at {:#X}", actual)
                }
                actual
            }
            _ => {println!("not in PRG rom space {:#X}", addr);
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

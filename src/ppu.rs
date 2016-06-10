

use cart;

pub struct PPU {
    // PPUCTRL $2000
    base_nametable: u16,
    vram_increment: bool,
    sprite_table_high: bool,
    bg_table_high: bool,
    sprite_8x16: bool,
    ppu_master: bool,
    nmi_enable: bool,

    // PPUMASK #2001
    grayscale: bool,
    bg_left_8px: bool,
    sprite_left_8px: bool,
    show_bg: bool,
    show_sprites: bool,
    emphasize_red: bool,
    emphasize_green: bool,
    emphasize_blue: bool,

    // PPUSTATUS $2002
    sprite_overflow: bool,
    sprite0_hit: bool,
    vblank: bool,

    ppu_addr: u16,
    oam_addr: u8,

    oam: Box<[u8]>,

    scroll_x: u8,
    scroll_y: u8,
    vram_addr: u16,
    t_vram_addr: u16,
    w_toggle: bool,

    pub scanline: i16,

    palette: Box<[u8]>,
    vram: Box<[u8]>,
    chr: cart::ChrRom,

    lastwrite: u8,
    initial_reset: bool,
}

impl PPU {
    pub fn new(chr: cart::ChrRom) -> PPU {
        PPU {
            // PPUCTRL $2000
            base_nametable: 0x2000,
            vram_increment: false,
            sprite_table_high: false,
            bg_table_high: false,
            sprite_8x16: false,
            ppu_master: false,
            nmi_enable: false,

            // PPUMASK $2001
            grayscale: false,
            bg_left_8px: false,
            sprite_left_8px: false,
            show_bg: false,
            show_sprites: false,
            emphasize_red: false,
            emphasize_green: false,
            emphasize_blue: false,

            // PPUSTATUS $2002
            sprite_overflow: false,
            sprite0_hit: false,
            vblank: false,

            ppu_addr: 0,
            oam_addr: 0,

            oam: vec![0; 256].into_boxed_slice(),

            scroll_x: 0,
            scroll_y: 0,
            vram_addr: 0,
            t_vram_addr: 0,
            w_toggle: false,

            scanline: 241,

            palette: vec![0; 32].into_boxed_slice(),
            // TODO: mappers!
            // research if we can just give it 4kb all the time
            // then logic out the extra RAM
            vram: vec![0; 1024 * 4].into_boxed_slice(),
            chr: chr,

            lastwrite: 0,
            initial_reset: true,
        }
    }

    pub fn write_ppuctrl(&mut self, data: u8){
        self.lastwrite = data;
        println!("Write PPUCTRL {:#b}", data);
        self.base_nametable = match data & 3 {
            0 => 0x2000,
            1 => 0x2400,
            2 => 0x2800,
            3 => 0x2C00,
            _ => 0
        };

        self.vram_increment = (data & (1 << 2)) != 0;
        self.sprite_table_high = (data & (1 << 3)) != 0;
        self.bg_table_high = (data & (1 << 4)) != 0;
        self.sprite_8x16 = (data & (1 << 5)) != 0;
        self.ppu_master = (data & (1 << 6)) != 0;
        self.nmi_enable = (data & (1 << 7)) != 0;
    }

    pub fn write_ppumask(&mut self, data: u8){
        self.lastwrite = data;
        println!("Write PPUMASK {:#b}", data);
        self.grayscale = (data & (1 << 0)) != 0;
        self.bg_left_8px = (data & (1 << 1)) != 0;
        self.sprite_left_8px = (data & (1 << 2)) != 0;
        self.show_bg = (data & (1 << 3)) != 0;
        self.show_sprites = (data & (1 << 4)) != 0;
        self.emphasize_red = (data & (1 << 5)) != 0;
        self.emphasize_green = (data & (1 << 6)) != 0;
        self.emphasize_blue = (data & (1 << 7)) != 0;
    }

    pub fn read_ppustatus(&mut self) -> u8{
        let mut value:u8 = 0;
        if self.sprite_overflow{
            value |= 1 << 5
        }
        if self.sprite0_hit{
            value |= 1 << 6
        }
        if self.vblank{
            value |= 1 << 7;
            self.vblank = false;
        }
        self.w_toggle = false;
        value | (self.lastwrite & 0b11111)
    }

    pub fn write_oamaddr(&mut self, data: u8) {
        self.lastwrite = data;
        self.oam_addr = data;
        println!("OAMADDR set: {:#X}", data);
    }

    pub fn write_oamdata(&mut self, data: u8) {
        self.lastwrite = data;
        self.oam[self.oam_addr as usize] = data;
        self.oam_addr = self.oam_addr.wrapping_add(1);
    }

    pub fn write_ppuscroll(&mut self, data: u8) {
        self.lastwrite = data;
        if !self.w_toggle {
            self.scroll_x = data;
        } else {
            self.scroll_y = data;
        }
        self.w_toggle = !self.w_toggle;
        println!("PPUSCROLL set X: {:#x} Y {:#X}", self.scroll_x, self.scroll_y);
    }

    pub fn write_ppuaddr(&mut self, data: u8) {
        self.lastwrite = data;
        if !self.w_toggle {
            self.ppu_addr = (data as u16) << 8;

        } else {
            self.ppu_addr |= data as u16;
        }
        println!("PPUADDR set: {:#X}", self.ppu_addr);
        self.w_toggle = !self.w_toggle;
    }

    // TODO - mappers - CHRROM etc.
    pub fn write_ppudata(&mut self, data:u8) {
        self.lastwrite = data;
        println!("PPUDATA virtual addr {:#X}", self.ppu_addr);
        match self.ppu_addr {
            0x0000...0x1FFF => panic!("CHR RAM not done yet"),
            0x2000...0x2FFF => {
                let mut offset:u16 = 0;
                if self.chr.vertical_mirroring && self.ppu_addr >= 0x2800 {
                    offset = 0x800;
                } else if self.chr.horizontal_mirroring &&
                    (self.ppu_addr >= 0x2400 && self.ppu_addr < 0x2800) ||
                    (self.ppu_addr >= 0x2C00 && self.ppu_addr < 0x3000)
                {
                    offset = 0x400;
                }

                let realaddr = (self.ppu_addr - 0x2000) - offset;
                self.vram[realaddr as usize] = data;
                println!("Writing RAM {:#X} at {:#X}", data, realaddr);
            }
            0x3000...0x3EFF => panic!("Need mirrors of 0x2000-0x2EFF"),
            0x3F00...0x3FFF => {
                let realaddr = (self.ppu_addr - 0x3F00) % 0x20;
                println!("Writing palette data {:#x} at {:#x}", data, realaddr);
                self.palette[realaddr as usize] = data;
            }
            _ => panic!("need mirrors of all vram")
        }
        if !self.vram_increment {
            self.ppu_addr += 1;
        } else {
            // not sure of this...
            self.ppu_addr += 32;
        }
    }

    pub fn read_ppudata(&mut self) -> u8 {
        let data = match self.ppu_addr {
            0x0000...0x1FFF => self.chr.read_u8(self.ppu_addr),
            0x2000...0x2FFF => {
                let mut offset:u16 = 0;
                if self.chr.vertical_mirroring && self.ppu_addr >= 0x2800 {
                    offset = 0x800;
                } else if self.chr.horizontal_mirroring &&
                    (self.ppu_addr >= 0x2400 && self.ppu_addr < 0x2800) ||
                    (self.ppu_addr >= 0x2C00 && self.ppu_addr < 0x3000)
                {
                    offset = 0x400;
                }

                let realaddr = (self.ppu_addr - 0x2000) - offset;
                self.vram[realaddr as usize]
            }
            0x3000...0x3EFF => panic!("Need mirrors of 0x2000-0x2EFF"),
            0x3F00...0x3FFF => {
                let realaddr = (self.ppu_addr - 0x3F00) % 0x20;
                self.palette[realaddr as usize]
            }
            _ => panic!("need mirrors of all vram")

        };

        println!("Read PPUDATA {:#X} from {:#X}", data, self.ppu_addr);

        if !self.vram_increment {
            self.ppu_addr += 1;
        } else {
            // not sure of this...
            self.ppu_addr += 32;
        }

        data
    }

//    $0000-$0FFF 	$1000 	Pattern table 0
//    $1000-$1FFF 	$1000 	Pattern Table 1
//    $2000-$23FF 	$0400 	Nametable 0
//    $2400-$27FF 	$0400 	Nametable 1
//    $2800-$2BFF 	$0400 	Nametable 2
//    $2C00-$2FFF 	$0400 	Nametable 3
//    $3F00-$3F1F 	$0020 	Palette RAM indexes
//    $3F30-$3FFFF    Mirrors Pallete RAM - MUST BE EMULATED

// nametables contain offsets referring to patterns..

    pub fn render_scanline(&mut self) -> bool {
        if self.scanline == -1 {
            self.vblank  = false;
            self.sprite0_hit = false;
        }
        if self.scanline >= 0 && self.scanline < 240 {
            // Do rendering here.
        }

        // if self.scanline == 240 {};

        self.scanline += 1;

        if self.scanline == 241 && !self.initial_reset {
            self.vblank = true;
        }

        if self.scanline > 260 {
            self.scanline = -1;
            if self.initial_reset {self.initial_reset = false};
        }

        if self.vblank && self.nmi_enable {
            return true;
        }

        false
        //println!("Read from CHR read byte {:#x} ", self.chr.read_u8(0));
    }

}

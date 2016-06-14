use cart;
use std::io::prelude::*;
use std::fs::File;
use std::io;

const PALETTE: [u32; 64] = [
    0x656565, 0x002D69, 0x131F7F, 0x3C137C, 0x600B62, 0x730A37, 0x710F07, 0x5A1A00,
    0x342800, 0x0B3400, 0x003C00, 0x003D10, 0x003840, 0x000000, 0x000000, 0x000000,

    0xAEAEAE, 0x0F63B3, 0x4051D0, 0x7841CC, 0xA736A9, 0xC03470, 0xBD3C30, 0x9F4A00,
    0x6D5C00, 0x366D00, 0x077704, 0x00793D, 0x00727D, 0x000000, 0x000000, 0x000000,

    0xFEFEFF, 0x5DB3FF, 0x8FA1FF, 0xC890FF, 0xF785FA, 0xFF83C0, 0xFF8B7F, 0xEF9A49,
    0xBDAC2C, 0x85BC2F, 0x55C753, 0x3CC98C, 0x3EC2CD, 0x4E4E4E, 0x000000, 0x000000,

    0xFEFEFF, 0xBCDFFF, 0xD1D8FF, 0xE8D1FF, 0xFBCDFD, 0xFFCCE5, 0xFFCFCA, 0xF8D5B4,
    0xE4DCA8, 0xCCE3A9, 0xB9E8B8, 0xAEE8D0, 0xAFE5EA, 0xB6B6B6, 0x000000, 0x000000,
];

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
    // vram_addr: u16,
    // t_vram_addr: u16,
    w_toggle: bool,

    pub scanline: i16,

    palette: Box<[u8]>,
    vram: Box<[u8]>,
    chr: cart::ChrRom,

    lastwrite: u8,
    ppudata_buffer: u8,
    initial_reset: bool,

    pub screen: [[u32; 256]; 240],

    framecount: usize,
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
            // vram_addr: 0,
            // t_vram_addr: 0,
            w_toggle: false,

            scanline: 241,

            palette: vec![0; 32].into_boxed_slice(),
            // TODO: mappers!
            // research if we can just give it 4kb all the time
            // then logic out the extra RAM
            vram: vec![0; 1024 * 4].into_boxed_slice(),
            chr: chr,

            lastwrite: 0,
            ppudata_buffer: 0,
            initial_reset: true,

            screen: [[0; 256]; 240],

            framecount: 0,
        }
    }

    pub fn write_ppuctrl(&mut self, data: u8){
        self.lastwrite = data;
        // println!("Write PPUCTRL {:#b}", data);
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
        // println!("Write PPUMASK {:#b}", data);
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
        // println!("OAMADDR set: {:#X}", data);
    }

    pub fn write_oamdata(&mut self, data: u8) {
        self.lastwrite = data;
        self.oam[self.oam_addr as usize] = data;
        // println!("OAMDATA set: {:#X} at {:#X}", data, self.oam_addr);
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
        // println!("PPUSCROLL set X: {:#x} Y {:#X}", self.scroll_x, self.scroll_y);
        if self.scroll_x !=0 || self.scroll_y != 0 {
            panic!("Scroll not implemented yet");
        }
    }

    pub fn write_ppuaddr(&mut self, data: u8) {
        self.lastwrite = data;
        if !self.w_toggle {
            self.ppu_addr = (data as u16) << 8;

        } else {
            self.ppu_addr |= data as u16;
        }
        // println!("PPUADDR set: {:#X}", self.ppu_addr);
        self.w_toggle = !self.w_toggle;
    }

    // TODO - mappers - CHRRAM etc.
    pub fn write_ppudata(&mut self, data:u8) {
        self.lastwrite = data;
        // println!("PPUDATA virtual addr {:#X}", self.ppu_addr);
        match self.ppu_addr {
            0x0000...0x1FFF => self.chr.write_u8(self.ppu_addr, data),
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
                // println!("Writing PPURAM {:#X} at {:#X}", data, realaddr);
            }
            0x3000...0x3EFF => panic!("Need mirrors of 0x2000-0x2EFF"),
            0x3F00...0x3FFF => {
                let mut realaddr = (self.ppu_addr - 0x3F00) % 0x20;
                if realaddr == 0x10 || realaddr == 0x14 || realaddr == 0x18 || realaddr == 0x1C {
                    realaddr -= 0x10;
                }
                println!("Writing palette data {:#x} at {:#x} ({:#X})", data, realaddr, self.ppu_addr);
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

    fn read_data(&mut self, addr: u16) -> u8 {
//        println!("read from {:#X}", addr);
        match addr {
            0x0000...0x1FFF => self.chr.read_u8(addr),
            0x2000...0x2FFF => {
                let mut offset:u16 = 0;
                if self.chr.vertical_mirroring && addr >= 0x2800 {
                    offset = 0x800;
                } else if self.chr.horizontal_mirroring &&
                    (addr >= 0x2400 && addr < 0x2800) ||
                    (addr >= 0x2C00 && addr < 0x3000)
                {
                    offset = 0x400;
                }

                let realaddr = (addr - 0x2000) - offset;
                self.vram[realaddr as usize]
            }
            0x3000...0x3EFF => panic!("Need mirrors of 0x2000-0x2EFF"),
            0x3F00...0x3FFF => {
                let mut realaddr = (addr - 0x3F00) % 0x20;
                if realaddr == 0x10 || realaddr == 0x14 || realaddr == 0x18 || realaddr == 0x1C {
                    realaddr -= 0x10;
                }
                return self.palette[realaddr as usize];
            }
            _ => panic!("need mirrors of all vram")

        }
    }

    pub fn read_ppudata(&mut self) -> u8 {
        let tmp = self.ppu_addr;
        let data = self.read_data(tmp);

       // println!("Read PPUDATA {:#X} from {:#X}", data, self.ppu_addr);

        if !self.vram_increment {
            self.ppu_addr += 1;
        } else {
            // not sure of this...
            self.ppu_addr += 32;
        }

        let ret = self.ppudata_buffer;
        self.ppudata_buffer = data;
        ret
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
    fn render_bg(&mut self) {
        // TODO: scrolling - which needs messing with nametable stuff
        // TODO: use left 8px setting
        /*
        read the nametable at the base nametable address to lookup the tile
        read the attribute table after that to lookup the palette
        read the looked-up tile data which is stored as a bit-pair of bytes
        calculate the actual pixel colours from the bit-pair
        then use the palette data to assign each pixel a colour based on the calculation
        */
        // using the PPU's actual functions for this should be correct..
        let sl = self.scanline;
        let bgcolor = PALETTE[self.palette[0] as usize];
        self.ppu_addr = self.base_nametable + 32 * (sl as u16 / 8);
        let incrtmp = self.vram_increment;
        self.vram_increment = false;
        let _ = self.read_ppudata(); // junk read just like real nes..
        for col in 0..32 {
            let sloffset = (sl as u16 / 32) * 8;
            let att_tbl_addr = self.base_nametable + 0x3C0 + (col / 4) + sloffset;
            let attr_table = self.read_data(att_tbl_addr);
            // println!("Attr table is {:#X} read from {:#X}", attr_table, att_tbl_addr);
            let attr:usize;
            if (sl % 32) / 16 == 0 {
                if (col % 4) / 2  == 0 {
                    attr = ((attr_table & 0b0000_0011) >> 0) as usize;
                } else {
                    attr = ((attr_table & 0b0000_1100) >> 2) as usize;
                }
            } else {
                if (col % 4) / 2 == 0 {
                    attr = ((attr_table & 0b0011_0000) >> 4) as usize;
                } else {
                    attr = ((attr_table & 0b1100_0000) >> 6) as usize;
                }
            }
            // println!("Attr is {:}", attr);

            let tile_palette:[u32; 4] = [
                bgcolor,
                PALETTE[self.palette[1 + (attr * 4)] as usize],
                PALETTE[self.palette[2 + (attr * 4)] as usize],
                PALETTE[self.palette[3 + (attr * 4)] as usize],
            ];
            // println!("whole palette is {:#?}", tile_palette);

            let mut tile_addr = (self.read_ppudata() as u16) * 16;
            if self.bg_table_high {
                tile_addr += 0x1000
            }

            let tile_data1 = self.read_data(tile_addr + (sl % 8) as u16);
            let tile_data2 = self.read_data(tile_addr + 8 + (sl % 8) as u16);

            for px in 0..8 {
                let pv = ((tile_data2 & (1 << 7 - px)) >> 7 - px) << 1 | (tile_data1 & (1 << 7 - px)) >> 7 - px;
                let pixel = tile_palette[pv as usize];

                self.screen[sl as usize][((col * 8) + px) as usize] = pixel;
            }
        }
        // println!("universal BG is {:#X}", bgcolor);
        self.vram_increment = incrtmp;
    }

    fn render_sprite(&mut self, bg: bool) {
        // TODO: deal with more than 8 sprites on a scanline
        // TODO: 8x16 sprites
        let sl = self.scanline;
        for mut sprite in 0..64 {
            sprite = 63 - sprite;
            // let offset = if bg
            let condition = if bg {
                (self.oam[(sprite * 4) + 2] & 0x20) != 0
            } else {
                (self.oam[(sprite * 4) + 2] & 0x20) == 0
            };

            let y = self.oam[sprite * 4] as i16;
            // println!("sprite {:} is at {:}", sprite, y);
            if (y <= sl && (y >= (sl - 8))) && condition {
                let mut index = self.oam[(sprite * 4) + 1] as u16 * 16;
                if self.sprite_table_high {
                    index += 0x1000;
                }
                let pal = 0x11 + (self.oam[(sprite * 4) + 2] & 0b11) * 4;

                let flip_h = (self.oam[(sprite * 4) + 2] & (1 << 6)) != 0;
                let flip_v = (self.oam[(sprite * 4) + 2] & (1 << 7)) != 0;
                let x = self.oam[(sprite * 4) + 3];
                // println!("Sprite {:} is {:?} on {:},{:}", sprite, bg, y, x);

                let flipv_offset = if flip_v {
                    7 - (sl % 8)
                } else {
                    sl % 8
                };

                let sprite_data1 = self.read_data(index + flipv_offset as u16);
                let sprite_data2 = self.read_data(index + 8 + flipv_offset as u16);

                for px in 0..8 {
                    let pv:u8;
                    if flip_h {
                        pv = ((sprite_data2 & (1 << px)) >> px) << 1 | (sprite_data1 & (1 << px)) >> px;
                    } else {
                        pv = ((sprite_data2 & (1 << 7 - px)) >> 7 - px) << 1 | (sprite_data1 & (1 << 7 - px)) >> 7 - px;
                    }
                    if pv > 0 {
                        let plt = self.palette[pal as usize + (pv - 1) as usize] as usize;
                        let pixel = PALETTE[plt];
                        // println!("pv: {:} pixel {:#X}", pv, pixel);
                        self.screen[sl as usize][x as usize + px as usize] = pixel;
                        if sprite == 0 {
                            self.sprite0_hit = true;
                        }
                    }
                }
            }
        }
    }

    pub fn render_scanline(&mut self) -> bool {

        // println!("Before: Scanline {:} Vblank: {:?} NMI: {:?}", self.scanline, self.vblank, self.nmi_enable);
        if self.scanline == -1 {
            self.vblank  = false;
            self.sprite0_hit = false;
        }
        if self.scanline >= 0 && self.scanline < 240 {
            // let each function run the whole scanline and paint 'over' eachother
            if self.show_sprites {
                // render Behind-BG sprites
                self.render_sprite(true);
            }
            if self.show_bg {
                self.render_bg();
            }
            if self.show_sprites {
                // render FG sprites
                self.render_sprite(false);
            }

        }

        self.scanline += 1;

        if self.scanline == 241 && !self.initial_reset {
            self.vblank = true;

            if self.show_bg {
                // println!("Frame: {:} - Sprite0 hit {:?}", self.framecount, self.sprite0_hit);
                self.framecount += 1;
                // panic!("stop here for now");
            }
        }

        if self.scanline > 260 {
            self.scanline = -1;
            if self.initial_reset {self.initial_reset = false};
        }

        // println!("After: Scanline {:} Vblank: {:?} NMI: {:?}", self.scanline, self.vblank, self.nmi_enable);
        if self.vblank && self.nmi_enable {
            return true;
        }

        false
        //println!("Read from CHR read byte {:#x} ", self.chr.read_u8(0));
    }

}


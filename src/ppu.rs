use cart;

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


// #[derive(Debug)]
pub struct PPU {
    // PPUCTRL $2000
    // base_nametable: u16,
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
    pub sprite0_hit: bool,
    vblank: bool,


    pub sprite0_dot: u8,
    // ppu_addr: u16,
    oam_addr: u8,

    oam: Box<[u8]>,

    // scroll_x: u8,
    // scroll_y: u8,
    vram_addr: u16,
    t_vram_addr: u16,
    fine_x: u8,
    w_toggle: bool,

    pub scanline: i16,

    palette: Box<[u8]>,
    vram: Box<[u8]>,
    chr: cart::ChrRom,

    pub lastwrite: u8,
    ppudata_buffer: u8,
    initial_reset: bool,
    nmi_generated: bool,

    sprite0_prerender: [u8; 8],
    sprite0_bg_prerender: [u8; 256],

    pub screen: [[u32; 256]; 240],

    pub framecount: usize,
}

impl PPU {
    pub fn new(chr: cart::ChrRom) -> PPU {
        PPU {
            // PPUCTRL $2000
            // base_nametable: 0x2000,
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

            sprite0_dot: 0xFF,
            // ppu_addr: 0,
            oam_addr: 0,

            oam: vec![0; 256].into_boxed_slice(),

            // scroll_x: 0,
            // scroll_y: 0,
            vram_addr: 0,
            t_vram_addr: 0,
            fine_x: 0,
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
            nmi_generated: false,

            screen: [[0; 256]; 240],
            sprite0_prerender: [0; 8],
            sprite0_bg_prerender: [0; 256],

            framecount: 0,
        }
    }

    // $2000
    pub fn write_ppuctrl(&mut self, data: u8){
        self.lastwrite = data;

        self.t_vram_addr &= 0x73FF;
        self.t_vram_addr |= (data as u16 & 3) << 10;
      //  println!("Write PPUCTRL {:#b} scanlien is {}", data, self.scanline);
/*        self.base_nametable = match data & 3 {
            0 => 0x2000,
            1 => 0x2400,
            2 => 0x2800,
            3 => 0x2C00,
            _ => 0
        };
*/
        self.vram_increment = (data & (1 << 2)) != 0;
        self.sprite_table_high = (data & (1 << 3)) != 0;
        self.bg_table_high = (data & (1 << 4)) != 0;
        self.sprite_8x16 = (data & (1 << 5)) != 0;
        self.ppu_master = (data & (1 << 6)) != 0;
        self.nmi_enable = (data & (1 << 7)) != 0;
    }

    // $2001
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

    // $2002
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
            self.nmi_generated = false;
        }
        self.w_toggle = false;
        value | (self.lastwrite & 0b11111)
    }

    pub fn write_oamaddr(&mut self, data: u8) {
        self.lastwrite = data;
        self.oam_addr = data;
        // println!("OAMADDR set: {:#X}", data);
    }

    // $2004
    pub fn write_oamdata(&mut self, data: u8) {
        self.lastwrite = data;
        self.oam[self.oam_addr as usize] = data;
        // println!("OAMDATA set: {:#X} at {:#X}", data, self.oam_addr);
        self.oam_addr = self.oam_addr.wrapping_add(1);
    }

    // $2004
    pub fn read_oamdata(&mut self) -> u8 {
        self.oam[self.oam_addr as usize]
    }

    // $2005
    pub fn write_ppuscroll(&mut self, data: u8) {
        self.lastwrite = data;
        if !self.w_toggle {
            self.fine_x = data & 0x7;
            // println!("Finex set to {} after SL {}", self.fine_x, self.scanline);
            self.t_vram_addr &= 0xFFE0;
            self.t_vram_addr |= ((data & 0xF8) >> 3) as u16;
            // self.scroll_x = data;
        } else {
            self.t_vram_addr &= 0x0C1F;
            self.t_vram_addr |= (data as u16 & 0x7) << 12;
            self.t_vram_addr |= (data as u16 & 0xF8) << 2;
            // self.scroll_y = data;
        }
        self.w_toggle = !self.w_toggle;
        // println!("PPUSCROLL set: {:#x} t_vram {:#X}", data, self.t_vram_addr);
    }

    // $2006
    pub fn write_ppuaddr(&mut self, data: u8) {
        self.lastwrite = data;
        if !self.w_toggle {
            self.t_vram_addr &= 0x00FF;
            self.t_vram_addr |= (data as u16 & 0x7F) << 8;
            // self.ppu_addr = (data as u16) << 8;
        } else {
            self.t_vram_addr &= 0xFF00;
            self.t_vram_addr |= data as u16;

            self.vram_addr = self.t_vram_addr;
            // self.ppu_addr |= data as u16;
//            println!("PPUADDR set: {:#X}", self.vram_addr);
        }
        self.w_toggle = !self.w_toggle;
    }

    // TODO - mappers
    pub fn write_ppudata(&mut self, data:u8) {
        self.lastwrite = data;
        let v_addr = self.vram_addr;
        // println!("write PPUDATA {:#x} at virtual addr {:#X}", data, self.vram_addr);
        match v_addr {
            0x0000...0x1FFF => self.chr.write_u8(v_addr, data),
            0x2000...0x2FFF => {
                // if v_addr > 0x3000 {
                //    v_addr -= 0x1000;
                // }
                let mut offset:u16 = 0;
                if self.chr.vertical_mirroring && v_addr >= 0x2800 {
                    offset = 0x800;
                } else if self.chr.horizontal_mirroring &&
                    (v_addr >= 0x2400 && v_addr < 0x2800) ||
                    (v_addr >= 0x2C00 && v_addr < 0x3000)
                {
                    offset = 0x400;
                }

                let realaddr = (v_addr - 0x2000) - offset;
                self.vram[realaddr as usize] = data;
                // if realaddr =
         //       println!("Writing PPURAM {:#X} at {:#X} = {:#X}", data, v_addr, realaddr);
            }
            0x3000...0x3EFF => panic!("Need mirrors of 0x2000-0x2EFF"),
            0x3F00...0x3FFF => {
                let mut realaddr = (self.vram_addr - 0x3F00) % 0x20;
                if realaddr == 0x10 || realaddr == 0x14 || realaddr == 0x18 || realaddr == 0x1C {
                    realaddr -= 0x10;
                }
                // println!("Writing palette data {:#x} at {:#x} ({:#X})", data, realaddr, self.ppu_addr);
                self.palette[realaddr as usize] = data;
            }
            _ => panic!("need mirrors of all vram")
        }
        if !self.vram_increment {
            self.vram_addr += 1;
        } else {
            // not sure of this...
            self.vram_addr += 32;
        }
    }

    fn read_data(&self, addr: u16) -> u8 {
//        println!("read from {:#X}", addr);
        match addr {
            0x0000...0x1FFF => self.chr.read_u8(addr),
            0x2000...0x2FFF => {
                //if addr > 0x3000 {
                //    addr -= 0x1000;
                //  }
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
                self.palette[realaddr as usize]
            }
            _ => panic!("need mirrors of all vram")

        }
    }

    pub fn read_ppudata(&mut self) -> u8 {
        let tmp = self.vram_addr;
        let data = self.read_data(tmp);

        // println!("Read PPUDATA {:#X} from {:#X}", data, self.vram_addr);

        if !self.vram_increment {
            self.vram_addr += 1;
        } else {
            // not sure of this...
            self.vram_addr += 32;
        }

        if tmp >= 0x3F00 && tmp <= 0x3FFF {
            // println!("palette read {:#X} from {:#X}", data, tmp);
            self.ppudata_buffer = self.read_data(tmp - 0x1000);
            data
        } else {
            let ret = self.ppudata_buffer;
            self.ppudata_buffer = data;
            ret
        }
    }

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
        let endcol = if self.fine_x != 0 {
            33
        } else {
            32
        };

        for col in 0..endcol {
            let coarsex = self.vram_addr & 0x1F;
            let coarsey = (self.vram_addr >> 5) & 0x1F;

            let att_tbl_addr = 0x23C0 |
                                (self.vram_addr & 0x0C00) |
                                ((self.vram_addr >> 4) & 0x38) |
                                ((self.vram_addr >> 2) & 0x07);

            // TODO need to offset attr table with fine_x
            let attr_table = &self.read_data(att_tbl_addr);
            // println!("Attr table is {:#X} read from {:#X}", attr_table, att_tbl_addr);
            let attr:usize;
            if (coarsey % 4) < 2 {
                if (coarsex % 4) < 2 {
                    attr = ((attr_table & 0b0000_0011) >> 0) as usize;
                } else {
                    attr = ((attr_table & 0b0000_1100) >> 2) as usize;
                }
            } else {
                if (coarsex % 4) < 2 {
                    attr = ((attr_table & 0b0011_0000) >> 4) as usize;
                } else {
                    attr = ((attr_table & 0b1100_0000) >> 6) as usize;
                }
            }

            let nametable_addr = 0x2000 | (self.vram_addr & 0x0FFF);
            let mut tile_addr = self.read_data(nametable_addr) as u16 * 16;
            if self.bg_table_high {
                tile_addr += 0x1000
            }

            let offset = self.vram_addr >> 12;
            let tile_data1 = &self.read_data(tile_addr + offset);
            let tile_data2 = &self.read_data(tile_addr + 8 + offset);

            for mut px in 0..8 {
                let pixel_x = (col as isize * 8) - self.fine_x as isize + px as isize;
                if pixel_x <= 255 && pixel_x >= 0 {
                    px = 7 - px;
                    let pv = ((tile_data2 & (1 << px)) >> px) << 1 | (tile_data1 & (1 << px)) >> px;
                    if pv > 0 {
                        let pixel = PALETTE[self.palette[pv as usize + (attr as usize * 4)] as usize];
                        self.screen[sl as usize][pixel_x as usize] = pixel;

                        if self.sprite0_dot == 0xFF && sl >= self.oam[0] as i16 + 1 && sl <= self.oam[0] as i16 + 8 {
                            self.sprite0_bg_prerender[pixel_x as usize] = pv;
                        }
                    }
                }
            }

            self.increment_x();

        }
        // println!("universal BG is {:#X}", bgcolor);
}

    fn render_sprites(&mut self) {
        // TODO: deal with more than 8 sprites on a scanline
        // TODO: 8x16 sprites
        let sl = self.scanline;

        for mut sprite in 0..64 {
            sprite = 63 - sprite;

            let y = self.oam[sprite * 4] as i16 + 1;
            // println!("sprite {:} is at {:}", sprite, y);
            if sl >= y && sl <= y + 7 {
                let mut index = self.oam[(sprite * 4) + 1] as u16 * 16;
                if self.sprite_table_high {
                    index += 0x1000;
                }
                let pal = 0x11 + (self.oam[(sprite * 4) + 2] & 0b11) * 4;

                let background = (self.oam[(sprite * 4) + 2] & 0x20) != 0;

                let flip_h = (self.oam[(sprite * 4) + 2] & (1 << 6)) != 0;
                let flip_v = (self.oam[(sprite * 4) + 2] & (1 << 7)) != 0;
                let x = self.oam[(sprite * 4) + 3];
                // println!("Sprite {:} is {:?} on {:},{:}", sprite, bg, y, x);

                let offset = if flip_v {
                    7 - (sl - y)
                } else {
                    sl - y
                };
                //println!("offset {}", offset);

                let sprite_data1 = self.read_data(index + offset as u16);
                let sprite_data2 = self.read_data(index + 8 + offset as u16);

                let bgcolor = self.palette[0] as usize;
                for px in 0..8 {
                    let pv:u8;
                    if flip_h {
                        pv = ((sprite_data2 & (1 << px)) >> px) << 1 | (sprite_data1 & (1 << px)) >> px;
                    } else {
                        pv = ((sprite_data2 & (1 << 7 - px)) >> 7 - px) << 1 | (sprite_data1 & (1 << 7 - px)) >> 7 - px;
                    }
                    if (x as usize + px as usize <= 255) && pv > 0 {
                        let bgpixel = self.screen[sl as usize][x as usize + px as usize];
                        let pixel = if background && bgpixel != PALETTE[bgcolor] {
                            bgpixel
                        } else {
                            let plt = self.palette[pal as usize + (pv - 1) as usize] as usize;
                            PALETTE[plt]
                        };
                        self.screen[sl as usize][x as usize + px as usize] = pixel;
                    }
                    if sprite == 0 && !self.sprite0_hit {
                        self.sprite0_prerender[px] = pv;
                    }
                }
            }
        }
    }

    pub fn render_scanline(&mut self) -> bool {

        // println!("Before: Scanline {:} Vblank: {:?} NMI: {:?}", self.scanline, self.vblank, self.nmi_enable);

        self.scanline += 1;
        if self.scanline > 260 {
            self.scanline = -1;
            self.nmi_generated = false;
            // self.sprite0_hit = false;
            if self.initial_reset {self.initial_reset = false};

            self.sprite0_hit = false;
            self.sprite0_dot = 0xFF;
            self.vblank  = false;

            // self.increment_y();
            // copy Vertical bits from t to v
            if self.show_bg || self.show_sprites {
                // println!("Frame# {}", self.framecount);
                let bgcolor = PALETTE[self.palette[0] as usize];
                self.screen = [[bgcolor; 256]; 240];
                self.vram_addr &= 0x041F;
                self.vram_addr |= self.t_vram_addr & !0x041F;
            }

        }


        if self.scanline >= 0 && self.scanline < 240 {
            if !self.sprite0_hit && self.show_bg && self.show_sprites {
                self.sprite0_prerender = [0; 8];
                self.sprite0_bg_prerender = [0; 256];
            }

            // let each function run the whole scanline and paint 'over' eachother
            if self.show_bg {
                self.render_bg();
            }
            if self.show_sprites {
                self.render_sprites();
            }


            if self.show_bg || self.show_sprites {
                self.increment_y();

                // copy horizontal bits from t to v
                self.vram_addr &= 0x7BE0;
                self.vram_addr |= self.t_vram_addr & !0x7BE0;

            }

            if !self.sprite0_hit && self.show_bg && self.show_sprites {
                //evaluate 1 line ahead?

                let px0 = self.oam[3] as usize;
                for x in 0..8 {
                    let offset = px0 + x;
                    let ls = if offset > 254 {
                        0
                    } else {
                        self.sprite0_bg_prerender[offset]
                    };

                    if  ls != 0 && self.sprite0_prerender[x] != 0 {
                        // self.sprite0_hit = true;
                        self.sprite0_dot = offset as u8;

                    }
                }
            }

        }


        if self.scanline == 241 && !self.initial_reset {
            self.vblank = true;
            // println!("sprite0 {:?}", self.sprite0_hit);
            if self.show_bg {
                self.framecount += 1;
            }
        }


        if self.vblank && self.nmi_enable && !self.nmi_generated{
            self.nmi_generated = true;
            return true;
        }
        false
    }

    fn increment_y(&mut self) {
        // y increment V...
        // println!("pre-INCR y {:#X}", self.vram_addr);
        if (self.vram_addr & 0x7000) != 0x7000 {  // if fine Y < 7
            self.vram_addr += 0x1000;  // Incr fine y
            // println!("Incr fine y");
        } else {
            // println!("Incr coarse y");
            self.vram_addr &= !0x7000;
            let mut y = (self.vram_addr & 0x03E0) >> 5;
            if y == 29 {
                y = 0;
                self.vram_addr ^= 0x0800;
            } else if y == 31 {
                y = 0;
            } else {
                y+= 1;
            }
            self.vram_addr = (self.vram_addr & !0x03E0) | (y << 5);
        }
    }

    fn increment_x(&mut self) {
        if (self.vram_addr & 0x001F) == 31 {
            self.vram_addr &= !0x001F;
            self.vram_addr ^= 0x0400;
        } else {
            self.vram_addr += 1
        }
    }

}


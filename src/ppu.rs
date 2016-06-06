

//use cart;

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

    oam_addr: u8,

    oam: Box<[u8]>,

    scroll_x: u8,
    scroll_y: u8,
    vram_addr: u16,
    t_vram_addr: u16,
    scroll_w: bool,


    vram: Box<[u8]>,
    chr_ram: Box<[u8]>,
}

impl PPU {
    pub fn new() -> PPU {
        PPU {
            base_nametable: 0x2000,
            vram_increment: false,
            sprite_table_high: false,
            bg_table_high: false,
            sprite_8x16: false,
            ppu_master: false,
            nmi_enable: false,

            // PPUMASK #2001
            grayscale: false,
            bg_left_8px: false,
            sprite_left_8px: false,
            show_bg: false,
            show_sprites: false,
            emphasize_red: false,
            emphasize_green: false,
            emphasize_blue: false,

            // PPUSTATUS $2002
            sprite_overflow: true,
            sprite0_hit: false,
            vblank: true,

            oam_addr: 0,

            oam: vec![0; 256].into_boxed_slice(),

            scroll_x: 0,
            scroll_y: 0,
            vram_addr: 0,
            t_vram_addr: 0,
            scroll_w: false,

            vram: vec![0; 1024 * 2].into_boxed_slice(),
            chr_ram: vec![0; 0].into_boxed_slice(),
        }
    }

    pub fn write_ppuctrl(&mut self, data: u8){
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
        self.scroll_w = false;
        value
    }

    pub fn write_ppuscroll(&mut self, data: u8) {
        if !self.scroll_w {
            self.scroll_x = data;
        } else {
            self.scroll_y = data;
        }
        self.scroll_w = !self.scroll_w;
    }

    pub fn render_scanline(&mut self) {
        //super::cpu_read_u8(0xc000)
    }

}

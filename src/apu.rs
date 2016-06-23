const APU_STATUS_REG: u16 = 0x4015;

#[derive(Debug)]
pub struct APU {
    dmc_en: bool,
    noise_lc_en: bool,
    triangle_lc_en: bool,
    pulse2_lc_en: bool,
    pulse1_lc_en: bool,
}

impl APU {
    pub fn new() -> APU {
        APU {
            dmc_en: false,
            noise_lc_en: false,
            triangle_lc_en: false,
            pulse2_lc_en: false,
            pulse1_lc_en: false,
        }

    }

    pub fn write(&mut self, addr: u16, value: u8) {
        if addr == APU_STATUS_REG {
            self.write_status_reg(value);
        } else {
            // println!("APU write {:#b} at {:#X} - unimplemented", value, addr);
        }
    }
/*
    pub fn read_status_reg(&self) -> u8 {
        println!("APU status read, returning 0");
        0
    }
*/
    fn write_status_reg(&mut self, value: u8) {
        self.dmc_en = (value & (1 << 4)) != 0;        //D
        self.noise_lc_en = (value & (1 << 3)) != 0;   //N
        self.triangle_lc_en = (value & (1 << 2)) != 0; //T
        self.pulse2_lc_en = (value & (1 << 1)) != 0;  //2
        self.pulse1_lc_en = (value & (1 << 0)) != 0;  //1
    }
}

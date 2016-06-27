use std::sync::{Arc, Mutex};
use sdl2::audio::AudioDevice;
use super::ApuOut;

const APU_STATUS_REG: u16 = 0x4015;

const LEN_TABLE: [u8;32] = [
    10, 254, 20,  2, 40,  4, 80,  6, 160,  8, 60, 10, 14, 12, 26, 14,
    12,  16, 24, 18, 48, 20, 96, 22, 192, 24, 72, 26, 16, 28, 32, 30
];


pub struct APU {
    triangle: Triangle,
    dmc_en: bool,
    noise_lc_en: bool,
    pulse2_lc_en: bool,
    pulse1_lc_en: bool,

    frame_clock: usize,
    interrupt_disable: bool,
    interrupt: bool,
    four_step: bool,

    pulse_mix_table: [f32; 31],
    tri_noise_dmc_mix_table: [f32; 203],

    pub output: Arc<Mutex<Vec<f32>>>,
    buff_ctr: usize,
    out_ctr: usize,
}

const STEP1: usize = 7457;
const STEP2: usize = 14913;
const STEP3: usize = 22371;
const STEP4: usize = 29829;
const IRQSTEP: usize = 29828;
const STEP5: usize = 37281;

impl APU {
    pub fn new() -> APU {
        let triangle = Triangle::new();
        let mut pmt: [f32; 31] = [0.0; 31];
        pmt[0] = 0.0;
        for n in 1..31 {
            pmt[n as usize] =  95.88/(8128.0/n as f32 + 100.0);
        }
        let mut tndmt: [f32; 203] = [0.0; 203];
        tndmt[0] = 0.0;
        for n in 1..203 {
            tndmt[n as usize] = 163.67/(24329.0/n as f32 + 100.0);
        }

        APU {
            triangle: triangle,
            dmc_en: false,
            noise_lc_en: false,
            pulse2_lc_en: false,
            pulse1_lc_en: false,

            frame_clock: 0,
            interrupt_disable: false,
            interrupt: true,
            four_step: true,

            pulse_mix_table: pmt,
            tri_noise_dmc_mix_table: tndmt,

            output: Arc::new(Mutex::new(Vec::new())),
            buff_ctr: 0,
            out_ctr: 0,
        }
    }

    pub fn tick(&mut self, ticks: isize) {
        for _ in 0..ticks {
            self.frame_clock = (self.frame_clock + 1) % 29830;
            // 5-step frame clock goes up to 37282..
            
            self.frame_counter();

            self.triangle.period_counter -= 1;
            if self.triangle.period_counter == 0 {
                self.triangle.period_counter = self.triangle.period + 1;
                self.triangle.generate_triangle();

            }

            if self.frame_clock % 41 == 0 {
                let output = self.pulse_mix_table[
                                0 +
                                0
                            ] + self.tri_noise_dmc_mix_table[
                                3 * self.triangle.output as usize +
                                0 +
                                0
                            ];
                self.output.lock().unwrap().insert(0, output);
            }

        }
        // TODO: return interrupts as necessary
    }

    //fn quarter frame

    //fn half frame

    fn frame_counter(&mut self) {
        if self.four_step {
            match self.frame_clock {
                0 => {
                    if !self.interrupt_disable {
                        self.interrupt = true;
                    }
                }
                // 3728.5
                STEP1 => {
                    self.triangle.linear_counter_tick();
                }
                // 7456.5
                STEP2 => {
                    self.triangle.linear_counter_tick();
                    self.triangle.len_tick();
                }
                // 11185.5
                STEP3 => {
                    self.triangle.linear_counter_tick();
                }
                // 14914
                IRQSTEP => {
                    if !self.interrupt_disable {
                        self.interrupt = true;
                    }
                }
                // 14914.5
                STEP4 => {
                    self.triangle.linear_counter_tick();
                    self.triangle.len_tick();
                    if !self.interrupt_disable {
                        self.interrupt = true;
                    }
                }
                _ => {}
            }
        }
    }

    pub fn write(&mut self, addr: u16, value: u8) {
        let reg = addr & 0x1F;
        match reg {
            0x08 => self.triangle.write_4008(value),
            0x0A => self.triangle.write_400A(value),
            0x0B => self.triangle.write_400B(value),
            0x15 => self.write_status_reg(value),
            0x17 => {
                self.four_step = (value & (1 << 7)) != 0;
                self.interrupt_disable = (value & (1 << 6)) != 0;
            }
            _ => {}
        }
    }

    pub fn read_status_reg(&mut self) -> u8 {
        let mut value: u8 = 0;
        // TODO: all of this.
        if self.interrupt {
            value |= 1 << 6;
        }
        if  self.triangle.length_counter > 0 {
            value |= 1 << 2;
        }
        self.interrupt = false;
        println!("APU status read - possibly wrong value returned");
        value
    }

    fn write_status_reg(&mut self, value: u8) {
        self.dmc_en = (value & (1 << 4)) != 0;        //D
        self.noise_lc_en = (value & (1 << 3)) != 0;   //N
        self.triangle.enabled = (value & (1 << 2)) != 0; //T
        self.pulse2_lc_en = (value & (1 << 1)) != 0;  //2
        self.pulse1_lc_en = (value & (1 << 0)) != 0;  //1
    }
}

const PULSE_DUTY: [[u8; 8]; 4] = [
    [0, 1, 0, 0, 0, 0, 0, 0],
    [0, 1, 1, 0, 0, 0, 0, 0],
    [0, 1, 1, 1, 1, 0, 0, 0],
    [1, 0, 0, 1, 1, 1, 1, 1],
];

struct Pulse {
    duty: u8,
    envelope_loop_length_halt: bool,
    constant_vol: bool,
    envelope: u8,
    sweep: bool,
    sweep_negate: bool,
    sweep_period: u8,
    sweep_shift: u8,

    timer: usize,
    length: u8,

}

const TRI_WAVEFORM: [u8;32] = [
    15, 14, 13, 12, 11, 10, 9,  8,  7,  6,  5,  4,  3,  2,  1,  0,
    0,  1,  2,  3,  4,  5,  6,  7,  8,  9,  10, 11, 12, 13, 14, 15];

#[derive(Debug)]
struct Triangle {
    enabled: bool,
    counter_halt: bool,
    counter_reload: bool,
    linear_counter: u8,
    linear_counter_reload: u8,
    period: usize,
    period_counter: usize,
    length_counter: u8,

    wave_pos: usize,

    output: u8,
    pub changed: bool,
}

impl Triangle {
    pub fn new() -> Triangle {
        Triangle {
            enabled: false,
            counter_halt: true,
            counter_reload: false,
            linear_counter: 0,
            linear_counter_reload: 0,
            period: 0,
            period_counter: 1,
            length_counter: 0,

            wave_pos: 0,

            output: 0,
            changed: false,
        }
    }

    pub fn write_4008 (&mut self, value: u8) {
        self.counter_halt = (value >> 7) != 0;
        self.linear_counter_reload = value & 0x7F;
    }

    pub fn write_400A (&mut self, value: u8) {
        self.period = (self.period & 0xFF00) | value as usize;
    }

    pub fn write_400B (&mut self, value: u8) {
        self.counter_reload = true;
        if self.enabled {
            self.length_counter = LEN_TABLE[value as usize >> 3];
        }
        self.period = (self.period & 0xFF) | ((value as usize & 7) << 8);
    }

    pub fn generate_triangle(&mut self) {
        if self.length_counter > 0 && self.linear_counter > 0 &&
            self.period > 2
        {
            self.wave_pos = (self.wave_pos + 1) % 32;
            self.output = TRI_WAVEFORM[self.wave_pos];
            // self.changed = true;
//            println!("triangle out {}", self.output);
        }
    }

    pub fn linear_counter_tick(&mut self) {
        if self.counter_reload {
            self.counter_reload = self.counter_halt;
            self.linear_counter = self.linear_counter_reload;
        } else if self.linear_counter > 0 {
            self.linear_counter -= 1;
        }
    }

    pub fn len_tick(&mut self)  {
        if  !self.counter_halt && self.length_counter > 0 {
            self.length_counter -= 1;
        }
    }
}

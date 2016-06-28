use std::sync::{Arc, Mutex};
// use std::sync::mpsc::Sender;

const APU_STATUS_REG: u16 = 0x4015;

const LEN_TABLE: [u8;32] = [
    10, 254, 20,  2, 40,  4, 80,  6, 160,  8, 60, 10, 14, 12, 26, 14,
    12,  16, 24, 18, 48, 20, 96, 22, 192, 24, 72, 26, 16, 28, 32, 30
];


pub struct APU {
    triangle: Triangle,
    pulse1: Pulse,
    pulse2: Pulse,
    noise: Noise,

    dmc_en: bool,

    frame_clock: usize,
    interrupt_disable: bool,
    interrupt: bool,
    four_step: bool,
    even_clock: bool,

    pulse_mix_table: [f32; 31],
    tri_noise_dmc_mix_table: [f32; 203],

    pub output: Arc<Mutex<Vec<f32>>>,
//    output2: Sender<f32>,
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
    // pub fn new(tx: Sender<f32>) -> APU {
    pub fn new() -> APU {
        let triangle = Triangle::new();
        let pulse1 = Pulse::new(true);
        let pulse2 = Pulse::new(false);
        let noise = Noise::new();

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
            pulse1: pulse1,
            pulse2: pulse2,
            noise: noise,
            dmc_en: false,

            frame_clock: 0,
            interrupt_disable: false,
            interrupt: true,
            four_step: true,
            even_clock: false,

            pulse_mix_table: pmt,
            tri_noise_dmc_mix_table: tndmt,

            output: Arc::new(Mutex::new(Vec::new())),
            // output2: tx,
            buff_ctr: 0,
            out_ctr: 0,
        }
    }

    pub fn tick(&mut self, ticks: isize) {
        for _ in 0..ticks {
            self.frame_clock = (self.frame_clock + 1) % 29830;
            // 5-step frame clock goes up to 37282..

            self.frame_counter();

            self.even_clock = !self.even_clock;
            if self.even_clock {
                self.pulse1.period_counter -= 1;
                if self.pulse1.period_counter == 0 {
                    self.pulse1.period_counter = self.pulse1.period + 1;
                    self.pulse1.generate_pulse();
                }

                self.pulse2.period_counter -= 1;
                if self.pulse2.period_counter == 0 {
                    self.pulse2.period_counter = self.pulse2.period + 1;
                    self.pulse2.generate_pulse();
                }
            }

            self.noise.period_counter -= 1;
            if self.noise.period_counter == 0 {
                self.noise.period_counter = self.noise.period + 1;
                self.noise.generate_noise();
            }

            self.triangle.period_counter -= 1;
            if self.triangle.period_counter == 0 {
                self.triangle.period_counter = self.triangle.period + 1;
                self.triangle.generate_triangle();
            }


            if self.frame_clock % 41 == 0 {
                let output = self.pulse_mix_table[
                                self.pulse1.output as usize +
                                self.pulse2.output as usize

                            ] + self.tri_noise_dmc_mix_table[
                                3 * self.triangle.output as usize +
                                2 * self.noise.output as usize +
                                0 // DMC is not multiplied
                            ];
                self.output.lock().unwrap().insert(0, output);
                // self.output2.send(output);
            }

        }
        // TODO: return interrupts as necessary
    }

    fn quarter_frame (&mut self) {
        self.triangle.linear_counter_tick();
        self.pulse1.do_envelope();
        self.pulse2.do_envelope();
        self.noise.do_envelope();
    }

    fn half_frame (&mut self) {
        self.quarter_frame();

        self.triangle.len_tick();
        self.pulse1.len_tick();
        self.pulse2.len_tick();
        self.noise.len_tick();
    }

    fn irq_check (&mut self) {
        if !self.interrupt_disable {
            self.interrupt = true;
        }
    }

    fn frame_counter(&mut self) {
        if self.four_step {
            match self.frame_clock {
                0 => self.irq_check(),
                // 3728.5 - QTR
                STEP1 => self.quarter_frame(),
                // 7456.5 - HALF
                STEP2 => self.half_frame(),
                // 11185.5 - QTR
                STEP3 => self.quarter_frame(),
                // 14914
                IRQSTEP => self.irq_check(),
                // 14914.5 - HALF
                STEP4 => {
                    self.half_frame();
                    self.irq_check();
                }
                _ => {}
            }
        }
    }

    pub fn write(&mut self, addr: u16, value: u8) {
        let reg = addr & 0x1F;
        match reg {
            0x00 => self.pulse1.write_4000_4004(value),
            0x01 => self.pulse1.write_4001_4005(value),
            0x02 => self.pulse1.write_4002_4006(value),
            0x03 => self.pulse1.write_4003_4007(value),

            0x04 => self.pulse2.write_4000_4004(value),
            0x05 => self.pulse2.write_4001_4005(value),
            0x06 => self.pulse2.write_4002_4006(value),
            0x07 => self.pulse2.write_4003_4007(value),

            0x08 => self.triangle.write_4008(value),
            0x0A => self.triangle.write_400A(value),
            0x0B => self.triangle.write_400B(value),

            0x0C => self.noise.write_400c(value),
            0x0E => self.noise.write_400e(value),
            0x0F => self.noise.write_400f(value),

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


        // DMC Interrupt

        if self.interrupt {
            value |= 1 << 6;
        }

        // DMC Active

        if self.noise.length > 0 {
            value |= 1 << 3;
        }
        if  self.triangle.length_counter > 0 {
            value |= 1 << 2;
        }
        if self.pulse2.length > 0 {
            value |= 1 << 1;
        }
        if self.pulse1.length > 0 {
            value |= 1 << 0;
        }
        self.interrupt = false;
        println!("APU status read - possibly wrong value returned");
        value
    }

    fn write_status_reg(&mut self, value: u8) {
        self.dmc_en = (value & (1 << 4)) != 0;        //D


        self.noise.enabled = (value & (1 << 3)) != 0;   //N
        if !self.noise.enabled {self.noise.length = 0};
        self.noise.output_noise();

        self.triangle.enabled = (value & (1 << 2)) != 0; //T
        if !self.triangle.enabled {self.triangle.length_counter = 0};

        self.pulse2.enabled = (value & (1 << 1)) != 0;  //2
        if !self.pulse2.enabled {self.pulse2.length = 0};
        self.pulse2.output_pulse();

        self.pulse1.enabled = (value & (1 << 0)) != 0;  //1
        if !self.pulse1.enabled {self.pulse1.length = 0};
        self.pulse1.output_pulse();
    }
}



const PULSE_DUTY: [[u8; 8]; 4] = [
    [0, 1, 0, 0, 0, 0, 0, 0],
    [0, 1, 1, 0, 0, 0, 0, 0],
    [0, 1, 1, 1, 1, 0, 0, 0],
    [1, 0, 0, 1, 1, 1, 1, 1],
];

struct Pulse {
    enabled: bool,
    pulse1: bool,

    duty: usize,
    wave_pos: usize,

    constant_vol: bool,
    volume: u8,
    envelope_volume: u8,
    envelope_divider: u8,
    envelope_start: bool,

    sweep: bool,
    sweep_negate: bool,
    sweep_period: u8,
    sweep_period_counter: u8,
    sweep_shift: usize,
    sweep_reload: bool,
    sweep_target: isize,

    period: usize,
    period_counter: usize,

    length_halt: bool,
    length: u8,

    output: u8,
}

impl Pulse {
    pub fn new(p1: bool) -> Pulse {
        Pulse {
            enabled: false,
            pulse1: p1,

            duty: 0,
            wave_pos: 0,

            constant_vol: false,

            volume: 15,
            envelope_volume: 15,
            envelope_divider: 0,
            envelope_start: true,

            sweep: false,
            sweep_negate: false,
            sweep_period: 0,
            sweep_period_counter: 0,
            sweep_shift: 0,
            sweep_reload: false,
            sweep_target: 0,

            period: 0,
            period_counter: 1,

            length_halt: false,
            length: 0,

            output: 0,
        }
    }

    pub fn write_4000_4004 (&mut self, value: u8) {
        self.duty = (value >> 6) as usize;
        self.length_halt = (value & (1 << 5)) != 0;
        self.constant_vol = (value & (1 << 4)) != 0;
        self.volume = value & 0xF;
        self.envelope_divider = self.volume;

        self.output_pulse();
    }

    pub fn write_4001_4005 (&mut self, value: u8) {
        self.sweep = (value & (1 << 7)) != 0;
        self.sweep_period = (value & 0x70) >> 4;
        self.sweep_negate = (value & (1 << 3)) != 0;
        self.sweep_shift = (value & 7) as usize;

        self.sweep_reload = true;

        self.target_sweep();
        self.output_pulse();
    }

    pub fn write_4002_4006 (&mut self, value: u8) {
        self.period = (self.period & 0xFF00) | value as usize;

        self.target_sweep();
        self.output_pulse();
    }

    pub fn write_4003_4007 (&mut self, value: u8) {
        if self.enabled {
            self.length = LEN_TABLE[value as usize >> 3];
        } else {
            self.length = 0;
        }
        self.period = (self.period & 0xFF) | ((value as usize & 7) << 8);

        self.wave_pos = 0;
        self.envelope_start = true;

        self.target_sweep();
        self.output_pulse();
    }

    fn target_sweep (&mut self) {
        let mut value = (self.period >> self.sweep_shift) as isize;
        if self.sweep_negate {
            value = if self.pulse1 {
                !value
            } else {
                -value
            };
        }

        self.sweep_target = self.period as isize + value
    }

    pub fn do_envelope (&mut self) {
        if self.envelope_start {
            self.envelope_start = false;
            self.envelope_volume = 15;
            self.envelope_divider = self.volume;

        } else {

            if self.envelope_divider == 0 {
                self.envelope_divider = self.volume;

                if self.envelope_volume > 0 {
                    self.envelope_volume -= 1;
                } else if self.length_halt {
                    self.envelope_volume = 15;
                } else {
                    self.envelope_volume = 0;
                }
            }

            if self.envelope_divider > 0 {self.envelope_divider -= 1};
        }

        self.output_pulse();
    }

    fn output_pulse (&mut self) {
        self.output = if self.length == 0 ||
                         self.period < 8 ||
                         self.sweep_target > 0x7FF ||
                         PULSE_DUTY[self.duty][self.wave_pos] == 0
        {
            0
        } else {
            if self.constant_vol {
                // if self.pulse1 {println!("C {}", self.envelope_volume)};
                self.volume
            } else {
                // if self.pulse1 {println!("E {}", self.envelope_volume)};
                self.envelope_volume
            }
        };
    }

    pub fn generate_pulse (&mut self) {
        self.wave_pos = (self.wave_pos + 1) % 8;

        self.output_pulse();
    }

    pub fn len_tick (&mut self) {
        if (!self.length_halt) && self.length > 0 {
            self.length -= 1;
            self.output_pulse();
        }

        if self.sweep_period_counter == 0 &&
           self.sweep &&
           self.period >= 8 &&
           self.sweep_shift != 0 &&
           self.sweep_target >= 0 &&
           self.sweep_target <= 0x7FF
        {
            self.period = self.sweep_target as usize;
            self.target_sweep();
            self.output_pulse();
        }

        if self.sweep_reload || self.sweep_period_counter == 0 {
            self.sweep_reload = false;
            self.sweep_period_counter = self.sweep_period;
        } else {
            self.sweep_period_counter -= 1;
        }
    }

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
            self.output = TRI_WAVEFORM[self.wave_pos];;
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



// NTSC
const NOISE_PERIOD: [usize; 16] = [4, 8, 16, 32, 64, 96, 128, 160, 202,
                                   254, 380, 508, 762, 1016, 2034, 4068];

struct Noise {
    enabled: bool,

    length_halt: bool,
    constant_vol: bool,

    volume: u8,
    envelope_volume: u8,
    envelope_divider: u8,
    envelope_start: bool,

    mode: bool,
    period: usize,
    period_counter: usize,
    shift: usize,

    length: u8,

    output: u8,
}

impl Noise {

    pub fn new() -> Noise {
        Noise {
            enabled: false,

            length_halt: false,
            constant_vol: false,

            volume: 0,
            envelope_volume: 0,
            envelope_divider: 0,
            envelope_start: false,

            mode: false,
            period: NOISE_PERIOD[0],
            period_counter: 1,
            shift: 1,

            length: 0,

            output: 0,
        }
    }

    pub fn write_400c (&mut self, value: u8) {
        self.length_halt = (value & (1 << 5)) != 0;
        self.constant_vol = (value & (1 << 4)) != 0;

        self.volume = value & 0xF;
        self.envelope_divider = self.volume;

        self.output_noise();
    }

    pub fn write_400e (&mut self, value: u8) {
        self.mode = (value & (1 << 7)) != 0;
        self.period = NOISE_PERIOD[(value & 0xF) as usize];
    }

    pub fn write_400f (&mut self, value: u8) {
        if self.enabled {
            self.length = value >> 3;
            self.output_noise();
        }
        self.envelope_start = true;
    }

    fn output_noise (&mut self) {
        self.output = if self.length == 0 || self.shift & 1 == 1 {
            0
        } else {
            if self.constant_vol {
                self.volume
            } else {
                self.envelope_volume
            }
        }
    }

    pub fn generate_noise (&mut  self) {
        let feedback = (if self.mode {
            (self.shift >> 6) ^ self.shift
        } else {
            (self.shift >> 1) ^ self.shift
        }) & 1;
        self.shift = feedback << 14 | self.shift >> 1;

        self.output_noise();
    }

    pub fn do_envelope (&mut self) {
        if self.envelope_start {
            self.envelope_start = false;
            self.envelope_volume = 15;
            self.envelope_divider = self.volume;

        } else {

            if self.envelope_divider == 0 {
                self.envelope_divider = self.volume;

                if self.envelope_volume > 0 {
                    self.envelope_volume -= 1;
                } else if self.length_halt {
                    self.envelope_volume = 15;
                } else {
                    self.envelope_volume = 0;
                }
            }

            if self.envelope_divider > 0 {self.envelope_divider -= 1};
        }

        self.output_noise();
    }

    pub fn len_tick (&mut self) {
        if !self.length_halt && self.length > 0 {
            self.length -= 1;
            self.output_noise();
        }
    }
}

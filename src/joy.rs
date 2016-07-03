use sdl2::keyboard::Keycode;

pub struct Joy {
    joy1: u8,
    _joy2: u8,

    joy1_read: u8,
    joy2_read: u8,
}

impl Joy {
    pub fn new() -> Joy {
        Joy {
            joy1: 0,
            _joy2: 0,

            joy1_read: 0,
            joy2_read: 0,
        }
    }

    pub fn set_keys(&mut self, keys: Vec<Keycode>) {
        self.joy1 = 0;
        for key in keys {
            match key {
                Keycode::LCtrl => {
                    self.joy1 |= 1 << 0;
                }
                Keycode::LShift => {
                    self.joy1 |= 1 << 1;
                }
                Keycode::Space => {
                    self.joy1 |= 1 << 2;
                }
                Keycode::Return => {
                    self.joy1 |= 1 << 3;
                }
                Keycode::Up => {
                    self.joy1 |= 1 << 4;
                }
                Keycode::Down => {
                    self.joy1 |= 1 << 5;
                }
                Keycode::Left => {
                    self.joy1 |= 1 << 6;
                }
                Keycode::Right => {
                    self.joy1 |= 1 << 7;
                }
                _ => ()// panic!("Unkown key {:?}", key),
            }
        }
    }

    pub fn strobe_joy(&mut self, value: u8) {
        if value == 0 {
            self.joy1_read = 0;
            // self.joy1 = 0;
            self.joy2_read = 0;
        } else {
            // self.joy1 = 1;
        }
    }

    pub fn read_joy1(&mut self) -> u8 {
        let ret = (self.joy1 & (1 << self.joy1_read)) >> self.joy1_read;
        // println!("joy1 {:#b} ret {} at {}", self.joy1, ret, self.joy1_read);
        // self.joy1 &= !(1 << self.joy1_read);
        self.joy1_read = (self.joy1_read + 1) % 8;
        // if self.joy1_read > 8
        ret
    }
}

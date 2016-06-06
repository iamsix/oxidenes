

pub struct Nes {
    ram: Box<[u8]>,
    cart: cart::Cart,
    apu: apu::APU,
    ppu: ppu::PPU,
    cpu: cpu::CPU
}

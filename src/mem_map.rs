// TODO: check zero page and stack specifically
// they are all ram however
const _ZERO_PAGE: u16 = 0x0000;
const _ZERO_PAGE_LEN: u8 = 0xff;
const _STACK: u16 = 0x0100;
const _STACK_LEN: u8 = 0xff;

pub const RAM_START: u16  = 0x0000;
pub const RAM_LEN: u16 = 0x800;
pub const RAM_VIRTUAL_END: u16 = 0x1999;
// do a  mod 0x800 to mirror 0x800 through 0x1999

pub const PPU_REGISTERS_START: u16  = 0x2000;
const _PPU_REGISTERS_LEN: usize  = 0x8;
pub const PPU_REGISTERS_VIRTUAL_END: u16 = 0x3fff;
// http://wiki.nesdev.com/w/index.php/PPU_registers
// do a mod 8 to get real addr

pub const APU_REGISTERS_START: u16 = 0x4000;
pub const APU_REGISTERS_END: u16 = 0x401F;
// http://wiki.nesdev.com/w/index.php/2A03

pub const EXPANSION_ROM_START: u16 = 0x4020;
pub const EXPANSION_ROM_END: u16 = 0x5FFF;

pub const SRAM_START: u16 = 0x6000;
pub const SRAM_END: u16 = 0x7FFF;

pub const PRG_ROM_LOWER_START: u16 = 0x8000;
pub const PRG_ROM_LOWER_LEN: u16 = 0x4000;
pub const PRG_ROM_UPPER_START: u16 = 0xc000;
pub const PRG_ROM_UPPER_LEN: u16 = 0x4000;
pub const PRG_ROM_START: u16 = 0x8000;
pub const PRG_ROM_END: u16 = 0xFFFF;

pub const NMI_VECTOR_LOC: u16 = 0xFFFA;
pub const RESET_VECTOR_LOC: u16 = 0xFFFC;
pub const IRQ_BRK_VECTOR_LOC: u16 = 0xFFFE;

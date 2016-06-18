#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Instruction {
    pub name: &'static str,
    pub bytes: u8,
    pub operand: u16,
    pub ticks: u8,
    pub addr_mode: AddressMode,
    pub dest_addr: Option<u16>,
    pub page_boundary_cycle: bool,
}


pub const INSTRUCTIONS: [Instruction; 256] = [// 00
                                              Instruction {
                                                  name: "BRK",
                                                  bytes: 1,
                                                  operand: 0,
                                                  ticks: 7,
                                                  addr_mode: AddressMode::Implied,
                                                  dest_addr: None,
                                                  page_boundary_cycle: false,
                                              },

                                              // 01
                                              Instruction {
                                                  name: "ORA",
                                                  bytes: 2,
                                                  operand: 0,
                                                  ticks: 6,
                                                  addr_mode: AddressMode::XIndirect,
                                                  dest_addr: None,
                                                  page_boundary_cycle: false,
                                              },

                                              // 02 - not implemented
                                              Instruction {
                                                  name: "*KIL",
                                                  bytes: 1,
                                                  operand: 0,
                                                  ticks: 0,
                                                  addr_mode: AddressMode::Implied,
                                                  dest_addr: None,
                                                  page_boundary_cycle: false,
                                              },

                                              // 03
                                              Instruction {
                                                  name: "*SLO",
                                                  bytes: 2,
                                                  operand: 0,
                                                  ticks: 8,
                                                  addr_mode: AddressMode::XIndirect,
                                                  dest_addr: Some(0),
                                                  page_boundary_cycle: false,
                                              },

                                              // 04
                                              Instruction {
                                                  name: "*DOP",
                                                  bytes: 2,
                                                  operand: 0,
                                                  ticks: 3,
                                                  addr_mode: AddressMode::Zeropage,
                                                  dest_addr: None,
                                                  page_boundary_cycle: false,
                                              },

                                              // 05
                                              Instruction {
                                                  name: "ORA",
                                                  bytes: 2,
                                                  operand: 0,
                                                  ticks: 3,
                                                  addr_mode: AddressMode::Zeropage,
                                                  dest_addr: None,
                                                  page_boundary_cycle: false,
                                              },

                                              // 06
                                              Instruction {
                                                  name: "ASL",
                                                  bytes: 2,
                                                  operand: 0,
                                                  ticks: 5,
                                                  addr_mode: AddressMode::Zeropage,
                                                  dest_addr: Some(0),
                                                  page_boundary_cycle: false,
                                              },

                                              // 07
                                              Instruction {
                                                  name: "*SLO",
                                                  bytes: 2,
                                                  operand: 0,
                                                  ticks: 5,
                                                  addr_mode: AddressMode::Zeropage,
                                                  dest_addr: Some(0),
                                                  page_boundary_cycle: false,
                                              },

                                              // 08
                                              Instruction {
                                                  name: "PHP",
                                                  bytes: 1,
                                                  operand: 0,
                                                  ticks: 3,
                                                  addr_mode: AddressMode::Implied,
                                                  dest_addr: None,
                                                  page_boundary_cycle: false,
                                              },

                                              // 09
                                              Instruction {
                                                  name: "ORA",
                                                  bytes: 2,
                                                  operand: 0,
                                                  ticks: 2,
                                                  addr_mode: AddressMode::Immediate,
                                                  dest_addr: None,
                                                  page_boundary_cycle: false,
                                              },

                                              // 0A
                                              Instruction {
                                                  name: "ASL",
                                                  bytes: 1,
                                                  operand: 0,
                                                  ticks: 2,
                                                  addr_mode: AddressMode::Accumulator,
                                                  dest_addr: None,
                                                  page_boundary_cycle: false,
                                              },

                                              // 0B - not implemented
                                              Instruction {
                                                  name: "*AAC",
                                                  bytes: 2,
                                                  operand: 0,
                                                  ticks: 2,
                                                  addr_mode: AddressMode::Immediate,
                                                  dest_addr: None,
                                                  page_boundary_cycle: false,
                                              },

                                              // 0C
                                              Instruction {
                                                  name: "*TOP",
                                                  bytes: 3,
                                                  operand: 0,
                                                  ticks: 4,
                                                  addr_mode: AddressMode::Absolute,
                                                  dest_addr: None,
                                                  page_boundary_cycle: false,
                                              },

                                              // 0D
                                              Instruction {
                                                  name: "ORA",
                                                  bytes: 3,
                                                  operand: 0,
                                                  ticks: 4,
                                                  addr_mode: AddressMode::Absolute,
                                                  dest_addr: None,
                                                  page_boundary_cycle: false,
                                              },

                                              // 0E
                                              Instruction {
                                                  name: "ASL",
                                                  bytes: 3,
                                                  operand: 0,
                                                  ticks: 6,
                                                  addr_mode: AddressMode::Absolute,
                                                  dest_addr: Some(0),
                                                  page_boundary_cycle: false,
                                              },

                                              // 0F
                                              Instruction {
                                                  name: "*SLO",
                                                  bytes: 3,
                                                  operand: 0,
                                                  ticks: 6,
                                                  addr_mode: AddressMode::Absolute,
                                                  dest_addr: Some(0),
                                                  page_boundary_cycle: false,
                                              },

                                              // 10
                                              Instruction {
                                                  name: "BPL",
                                                  bytes: 2,
                                                  operand: 0,
                                                  ticks: 2,
                                                  addr_mode: AddressMode::Relative,
                                                  dest_addr: None,
                                                  page_boundary_cycle: false,
                                              },

                                              // 11
                                              Instruction {
                                                  name: "ORA",
                                                  bytes: 2,
                                                  operand: 0,
                                                  ticks: 5,
                                                  addr_mode: AddressMode::IndirectY,
                                                  dest_addr: None,
                                                  page_boundary_cycle: true,
                                              },

                                              // 12
                                              Instruction {
                                                  name: "*KIL",
                                                  bytes: 1,
                                                  operand: 0,
                                                  ticks: 0,
                                                  addr_mode: AddressMode::Implied,
                                                  dest_addr: None,
                                                  page_boundary_cycle: false,
                                              },

                                              // 13
                                              Instruction {
                                                  name: "*SLO",
                                                  bytes: 2,
                                                  operand: 0,
                                                  ticks: 8,
                                                  addr_mode: AddressMode::IndirectY,
                                                  dest_addr: Some(0),
                                                  page_boundary_cycle: false,
                                              },

                                              // 14
                                              Instruction {
                                                  name: "*DOP",
                                                  bytes: 2,
                                                  operand: 0,
                                                  ticks: 4,
                                                  addr_mode: AddressMode::ZeropageX,
                                                  dest_addr: None,
                                                  page_boundary_cycle: false,
                                              },

                                              // 15
                                              Instruction {
                                                  name: "ORA",
                                                  bytes: 2,
                                                  operand: 0,
                                                  ticks: 4,
                                                  addr_mode: AddressMode::ZeropageX,
                                                  dest_addr: None,
                                                  page_boundary_cycle: false,
                                              },

                                              // 16
                                              Instruction {
                                                  name: "ASL",
                                                  bytes: 2,
                                                  operand: 0,
                                                  ticks: 6,
                                                  addr_mode: AddressMode::ZeropageX,
                                                  dest_addr: Some(0),
                                                  page_boundary_cycle: false,
                                              },

                                              // 17
                                              Instruction {
                                                  name: "*SLO",
                                                  bytes: 2,
                                                  operand: 0,
                                                  ticks: 6,
                                                  addr_mode: AddressMode::ZeropageX,
                                                  dest_addr: Some(0),
                                                  page_boundary_cycle: false,
                                              },

                                              // 18
                                              Instruction {
                                                  name: "CLC",
                                                  bytes: 1,
                                                  operand: 0,
                                                  ticks: 2,
                                                  addr_mode: AddressMode::Implied,
                                                  dest_addr: None,
                                                  page_boundary_cycle: false,
                                              },

                                              // 19
                                              Instruction {
                                                  name: "ORA",
                                                  bytes: 3,
                                                  operand: 0,
                                                  ticks: 4,
                                                  addr_mode: AddressMode::AbsoluteY,
                                                  dest_addr: None,
                                                  page_boundary_cycle: true,
                                              },

                                              // 1A
                                              Instruction {
                                                  name: "*NOP",
                                                  bytes: 1,
                                                  operand: 0,
                                                  ticks: 2,
                                                  addr_mode: AddressMode::Implied,
                                                  dest_addr: None,
                                                  page_boundary_cycle: false,
                                              },

                                              // 1B
                                              Instruction {
                                                  name: "*SLO",
                                                  bytes: 3,
                                                  operand: 0,
                                                  ticks: 7,
                                                  addr_mode: AddressMode::AbsoluteY,
                                                  dest_addr: Some(0),
                                                  page_boundary_cycle: false,
                                              },

                                              // 1C
                                              Instruction {
                                                  name: "*TOP",
                                                  bytes: 3,
                                                  operand: 0,
                                                  ticks: 4,
                                                  addr_mode: AddressMode::AbsoluteX,
                                                  dest_addr: None,
                                                  page_boundary_cycle: true,
                                              },

                                              // 1D
                                              Instruction {
                                                  name: "ORA",
                                                  bytes: 3,
                                                  operand: 0,
                                                  ticks: 4,
                                                  addr_mode: AddressMode::AbsoluteX,
                                                  dest_addr: None,
                                                  page_boundary_cycle: true,
                                              },

                                              // 1E
                                              Instruction {
                                                  name: "ASL",
                                                  bytes: 3,
                                                  operand: 0,
                                                  ticks: 7,
                                                  addr_mode: AddressMode::AbsoluteX,
                                                  dest_addr: Some(0),
                                                  page_boundary_cycle: false,
                                              },

                                              // 1F
                                              Instruction {
                                                  name: "*SLO",
                                                  bytes: 3,
                                                  operand: 0,
                                                  ticks: 7,
                                                  addr_mode: AddressMode::AbsoluteX,
                                                  dest_addr: Some(0),
                                                  page_boundary_cycle: false,
                                              },

                                              // 20
                                              Instruction {
                                                  name: "JSR",
                                                  bytes: 3,
                                                  operand: 0,
                                                  ticks: 6,
                                                  addr_mode: AddressMode::Absolute,
                                                  dest_addr: None,
                                                  page_boundary_cycle: false,
                                              },

                                              // 21
                                              Instruction {
                                                  name: "AND",
                                                  bytes: 2,
                                                  operand: 0,
                                                  ticks: 6,
                                                  addr_mode: AddressMode::XIndirect,
                                                  dest_addr: None,
                                                  page_boundary_cycle: false,
                                              },

                                              // 22
                                              Instruction {
                                                  name: "*KIL",
                                                  bytes: 1,
                                                  operand: 0,
                                                  ticks: 0,
                                                  addr_mode: AddressMode::Implied,
                                                  dest_addr: None,
                                                  page_boundary_cycle: false,
                                              },

                                              // 23
                                              Instruction {
                                                  name: "*RLA",
                                                  bytes: 2,
                                                  operand: 0,
                                                  ticks: 8,
                                                  addr_mode: AddressMode::XIndirect,
                                                  dest_addr: Some(0),
                                                  page_boundary_cycle: false,
                                              },

                                              // 24
                                              Instruction {
                                                  name: "BIT",
                                                  bytes: 2,
                                                  operand: 0,
                                                  ticks: 3,
                                                  addr_mode: AddressMode::Zeropage,
                                                  dest_addr: None,
                                                  page_boundary_cycle: false,
                                              },

                                              // 25
                                              Instruction {
                                                  name: "AND",
                                                  bytes: 2,
                                                  operand: 0,
                                                  ticks: 3,
                                                  addr_mode: AddressMode::Zeropage,
                                                  dest_addr: None,
                                                  page_boundary_cycle: false,
                                              },

                                              // 26
                                              Instruction {
                                                  name: "ROL",
                                                  bytes: 2,
                                                  operand: 0,
                                                  ticks: 5,
                                                  addr_mode: AddressMode::Zeropage,
                                                  dest_addr: Some(0),
                                                  page_boundary_cycle: false,
                                              },

                                              // 27
                                              Instruction {
                                                  name: "*RLA",
                                                  bytes: 2,
                                                  operand: 0,
                                                  ticks: 5,
                                                  addr_mode: AddressMode::Zeropage,
                                                  dest_addr: Some(0),
                                                  page_boundary_cycle: false,
                                              },

                                              // 28
                                              Instruction {
                                                  name: "PLP",
                                                  bytes: 1,
                                                  operand: 0,
                                                  ticks: 4,
                                                  addr_mode: AddressMode::Implied,
                                                  dest_addr: None,
                                                  page_boundary_cycle: false,
                                              },

                                              // 29
                                              Instruction {
                                                  name: "AND",
                                                  bytes: 2,
                                                  operand: 0,
                                                  ticks: 2,
                                                  addr_mode: AddressMode::Immediate,
                                                  dest_addr: None,
                                                  page_boundary_cycle: false,
                                              },

                                              // 2A
                                              Instruction {
                                                  name: "ROL",
                                                  bytes: 1,
                                                  operand: 0,
                                                  ticks: 2,
                                                  addr_mode: AddressMode::Accumulator,
                                                  dest_addr: None,
                                                  page_boundary_cycle: false,
                                              },

                                              // 2B
                                              Instruction {
                                                  name: "*AAC",
                                                  bytes: 2,
                                                  operand: 0,
                                                  ticks: 2,
                                                  addr_mode: AddressMode::Immediate,
                                                  dest_addr: None,
                                                  page_boundary_cycle: false,
                                              },

                                              // 2C
                                              Instruction {
                                                  name: "BIT",
                                                  bytes: 3,
                                                  operand: 0,
                                                  ticks: 4,
                                                  addr_mode: AddressMode::Absolute,
                                                  dest_addr: None,
                                                  page_boundary_cycle: false,
                                              },

                                              // 2D
                                              Instruction {
                                                  name: "AND",
                                                  bytes: 3,
                                                  operand: 0,
                                                  ticks: 4,
                                                  addr_mode: AddressMode::Absolute,
                                                  dest_addr: None,
                                                  page_boundary_cycle: false,
                                              },

                                              // 2E
                                              Instruction {
                                                  name: "ROL",
                                                  bytes: 3,
                                                  operand: 0,
                                                  ticks: 6,
                                                  addr_mode: AddressMode::Absolute,
                                                  dest_addr: None,
                                                  page_boundary_cycle: false,
                                              },

                                              // 2F
                                              Instruction {
                                                  name: "*RLA",
                                                  bytes: 3,
                                                  operand: 0,
                                                  ticks: 6,
                                                  addr_mode: AddressMode::Absolute,
                                                  dest_addr: Some(0),
                                                  page_boundary_cycle: false,
                                              },

                                              // 30
                                              Instruction {
                                                  name: "BMI",
                                                  bytes: 2,
                                                  operand: 0,
                                                  ticks: 2,
                                                  addr_mode: AddressMode::Relative,
                                                  dest_addr: None,
                                                  page_boundary_cycle: false,
                                              },

                                              // 31
                                              Instruction {
                                                  name: "AND",
                                                  bytes: 2,
                                                  operand: 0,
                                                  ticks: 5,
                                                  addr_mode: AddressMode::IndirectY,
                                                  dest_addr: None,
                                                  page_boundary_cycle: true,
                                              },

                                              // 32
                                              Instruction {
                                                  name: "*KIL",
                                                  bytes: 1,
                                                  operand: 0,
                                                  ticks: 0,
                                                  addr_mode: AddressMode::Implied,
                                                  dest_addr: None,
                                                  page_boundary_cycle: false,
                                              },

                                              // 33
                                              Instruction {
                                                  name: "*RLA",
                                                  bytes: 2,
                                                  operand: 0,
                                                  ticks: 8,
                                                  addr_mode: AddressMode::IndirectY,
                                                  dest_addr: Some(0),
                                                  page_boundary_cycle: false,
                                              },

                                              // 34
                                              Instruction {
                                                  name: "*DOP",
                                                  bytes: 2,
                                                  operand: 0,
                                                  ticks: 4,
                                                  addr_mode: AddressMode::ZeropageX,
                                                  dest_addr: None,
                                                  page_boundary_cycle: false,
                                              },

                                              // 35
                                              Instruction {
                                                  name: "AND",
                                                  bytes: 2,
                                                  operand: 0,
                                                  ticks: 4,
                                                  addr_mode: AddressMode::ZeropageX,
                                                  dest_addr: None,
                                                  page_boundary_cycle: false,
                                              },

                                              // 36
                                              Instruction {
                                                  name: "ROL",
                                                  bytes: 2,
                                                  operand: 0,
                                                  ticks: 6,
                                                  addr_mode: AddressMode::ZeropageX,
                                                  dest_addr: Some(0),
                                                  page_boundary_cycle: false,
                                              },

                                              // 37
                                              Instruction {
                                                  name: "*RLA",
                                                  bytes: 2,
                                                  operand: 0,
                                                  ticks: 6,
                                                  addr_mode: AddressMode::ZeropageX,
                                                  dest_addr: Some(0),
                                                  page_boundary_cycle: false,
                                              },

                                              // 38
                                              Instruction {
                                                  name: "SEC",
                                                  bytes: 1,
                                                  operand: 0,
                                                  ticks: 2,
                                                  addr_mode: AddressMode::Implied,
                                                  dest_addr: None,
                                                  page_boundary_cycle: false,
                                              },

                                              // 39
                                              Instruction {
                                                  name: "AND",
                                                  bytes: 3,
                                                  operand: 0,
                                                  ticks: 4,
                                                  addr_mode: AddressMode::AbsoluteY,
                                                  dest_addr: None,
                                                  page_boundary_cycle: true,
                                              },

                                              // 3A
                                              Instruction {
                                                  name: "*NOP",
                                                  bytes: 1,
                                                  operand: 0,
                                                  ticks: 2,
                                                  addr_mode: AddressMode::Implied,
                                                  dest_addr: None,
                                                  page_boundary_cycle: false,
                                              },

                                              // 3B
                                              Instruction {
                                                  name: "*RLA",
                                                  bytes: 3,
                                                  operand: 0,
                                                  ticks: 7,
                                                  addr_mode: AddressMode::AbsoluteY,
                                                  dest_addr: Some(0),
                                                  page_boundary_cycle: false,
                                              },

                                              // 3C
                                              Instruction {
                                                  name: "*TOP",
                                                  bytes: 3,
                                                  operand: 0,
                                                  ticks: 4,
                                                  addr_mode: AddressMode::AbsoluteX,
                                                  dest_addr: None,
                                                  page_boundary_cycle: true,
                                              },

                                              // 3D
                                              Instruction {
                                                  name: "AND",
                                                  bytes: 3,
                                                  operand: 0,
                                                  ticks: 4,
                                                  addr_mode: AddressMode::AbsoluteX,
                                                  dest_addr: None,
                                                  page_boundary_cycle: true,
                                              },

                                              // 3E
                                              Instruction {
                                                  name: "ROL",
                                                  bytes: 3,
                                                  operand: 0,
                                                  ticks: 7,
                                                  addr_mode: AddressMode::AbsoluteX,
                                                  dest_addr: Some(0),
                                                  page_boundary_cycle: false,
                                              },

                                              // 3F
                                              Instruction {
                                                  name: "*RLA",
                                                  bytes: 3,
                                                  operand: 0,
                                                  ticks: 7,
                                                  addr_mode: AddressMode::AbsoluteX,
                                                  dest_addr: Some(0),
                                                  page_boundary_cycle: false,
                                              },

                                              // 40
                                              Instruction {
                                                  name: "RTI",
                                                  bytes: 1,
                                                  operand: 0,
                                                  ticks: 6,
                                                  addr_mode: AddressMode::Implied,
                                                  dest_addr: None,
                                                  page_boundary_cycle: false,
                                              },

                                              // 41
                                              Instruction {
                                                  name: "EOR",
                                                  bytes: 2,
                                                  operand: 0,
                                                  ticks: 6,
                                                  addr_mode: AddressMode::XIndirect,
                                                  dest_addr: None,
                                                  page_boundary_cycle: false,
                                              },

                                              // 42
                                              Instruction {
                                                  name: "*KIL",
                                                  bytes: 1,
                                                  operand: 0,
                                                  ticks: 0,
                                                  addr_mode: AddressMode::Implied,
                                                  dest_addr: None,
                                                  page_boundary_cycle: false,
                                              },

                                              // 43
                                              Instruction {
                                                  name: "*SRE",
                                                  bytes: 2,
                                                  operand: 0,
                                                  ticks: 8,
                                                  addr_mode: AddressMode::XIndirect,
                                                  dest_addr: Some(0),
                                                  page_boundary_cycle: false,
                                              },

                                              // 44
                                              Instruction {
                                                  name: "*DOP",
                                                  bytes: 2,
                                                  operand: 0,
                                                  ticks: 3,
                                                  addr_mode: AddressMode::Zeropage,
                                                  dest_addr: None,
                                                  page_boundary_cycle: false,
                                              },

                                              // 45
                                              Instruction {
                                                  name: "EOR",
                                                  bytes: 2,
                                                  operand: 0,
                                                  ticks: 3,
                                                  addr_mode: AddressMode::Zeropage,
                                                  dest_addr: None,
                                                  page_boundary_cycle: false,
                                              },

                                              // 46
                                              Instruction {
                                                  name: "LSR",
                                                  bytes: 2,
                                                  operand: 0,
                                                  ticks: 5,
                                                  addr_mode: AddressMode::Zeropage,
                                                  dest_addr: Some(0),
                                                  page_boundary_cycle: false,
                                              },

                                              // 47
                                              Instruction {
                                                  name: "*SRE",
                                                  bytes: 2,
                                                  operand: 0,
                                                  ticks: 5,
                                                  addr_mode: AddressMode::Zeropage,
                                                  dest_addr: Some(0),
                                                  page_boundary_cycle: false,
                                              },

                                              // 48
                                              Instruction {
                                                  name: "PHA",
                                                  bytes: 1,
                                                  operand: 0,
                                                  ticks: 3,
                                                  addr_mode: AddressMode::Implied,
                                                  dest_addr: None,
                                                  page_boundary_cycle: false,
                                              },

                                              // 49
                                              Instruction {
                                                  name: "EOR",
                                                  bytes: 2,
                                                  operand: 0,
                                                  ticks: 2,
                                                  addr_mode: AddressMode::Immediate,
                                                  dest_addr: None,
                                                  page_boundary_cycle: false,
                                              },

                                              // 4A
                                              Instruction {
                                                  name: "LSR",
                                                  bytes: 1,
                                                  operand: 0,
                                                  ticks: 2,
                                                  addr_mode: AddressMode::Accumulator,
                                                  dest_addr: None,
                                                  page_boundary_cycle: false,
                                              },

                                              // 4B
                                              Instruction {
                                                  name: "*ASR",
                                                  bytes: 2,
                                                  operand: 0,
                                                  ticks: 2,
                                                  addr_mode: AddressMode::Immediate,
                                                  dest_addr: None,
                                                  page_boundary_cycle: false,
                                              },

                                              // 4C
                                              Instruction {
                                                  name: "JMP",
                                                  bytes: 3,
                                                  operand: 0,
                                                  ticks: 3,
                                                  addr_mode: AddressMode::Absolute,
                                                  dest_addr: None,
                                                  page_boundary_cycle: false,
                                              },

                                              // 4D
                                              Instruction {
                                                  name: "EOR",
                                                  bytes: 3,
                                                  operand: 0,
                                                  ticks: 4,
                                                  addr_mode: AddressMode::Absolute,
                                                  dest_addr: None,
                                                  page_boundary_cycle: false,
                                              },

                                              // 4E
                                              Instruction {
                                                  name: "LSR",
                                                  bytes: 3,
                                                  operand: 0,
                                                  ticks: 6,
                                                  addr_mode: AddressMode::Absolute,
                                                  dest_addr: Some(0),
                                                  page_boundary_cycle: false,
                                              },

                                              // 4F
                                              Instruction {
                                                  name: "*SRE",
                                                  bytes: 3,
                                                  operand: 0,
                                                  ticks: 6,
                                                  addr_mode: AddressMode::Absolute,
                                                  dest_addr: Some(0),
                                                  page_boundary_cycle: false,
                                              },

                                              // 50
                                              Instruction {
                                                  name: "BVC",
                                                  bytes: 2,
                                                  operand: 0,
                                                  ticks: 2,
                                                  addr_mode: AddressMode::Relative,
                                                  dest_addr: None,
                                                  page_boundary_cycle: false,
                                              },

                                              // 51
                                              Instruction {
                                                  name: "EOR",
                                                  bytes: 2,
                                                  operand: 0,
                                                  ticks: 5,
                                                  addr_mode: AddressMode::IndirectY,
                                                  dest_addr: None,
                                                  page_boundary_cycle: true,
                                              },

                                              // 52
                                              Instruction {
                                                  name: "KIL",
                                                  bytes: 1,
                                                  operand: 0,
                                                  ticks: 0,
                                                  addr_mode: AddressMode::Implied,
                                                  dest_addr: None,
                                                  page_boundary_cycle: false,
                                              },

                                              // 53
                                              Instruction {
                                                  name: "*SRE",
                                                  bytes: 2,
                                                  operand: 0,
                                                  ticks: 8,
                                                  addr_mode: AddressMode::IndirectY,
                                                  dest_addr: Some(0),
                                                  page_boundary_cycle: false,
                                              },

                                              // 54
                                              Instruction {
                                                  name: "DOP",
                                                  bytes: 2,
                                                  operand: 0,
                                                  ticks: 4,
                                                  addr_mode: AddressMode::ZeropageX,
                                                  dest_addr: None,
                                                  page_boundary_cycle: false,
                                              },

                                              // 55
                                              Instruction {
                                                  name: "EOR",
                                                  bytes: 2,
                                                  operand: 0,
                                                  ticks: 4,
                                                  addr_mode: AddressMode::ZeropageX,
                                                  dest_addr: None,
                                                  page_boundary_cycle: false,
                                              },

                                              // 56
                                              Instruction {
                                                  name: "LSR",
                                                  bytes: 2,
                                                  operand: 0,
                                                  ticks: 6,
                                                  addr_mode: AddressMode::ZeropageX,
                                                  dest_addr: Some(0),
                                                  page_boundary_cycle: false,
                                              },

                                              // 57
                                              Instruction {
                                                  name: "*SRE",
                                                  bytes: 2,
                                                  operand: 0,
                                                  ticks: 6,
                                                  addr_mode: AddressMode::ZeropageX,
                                                  dest_addr: Some(0),
                                                  page_boundary_cycle: false,
                                              },

                                              // 58
                                              Instruction {
                                                  name: "CLI",
                                                  bytes: 1,
                                                  operand: 0,
                                                  ticks: 2,
                                                  addr_mode: AddressMode::Implied,
                                                  dest_addr: None,
                                                  page_boundary_cycle: false,
                                              },

                                              // 59
                                              Instruction {
                                                  name: "EOR",
                                                  bytes: 3,
                                                  operand: 0,
                                                  ticks: 4,
                                                  addr_mode: AddressMode::AbsoluteY,
                                                  dest_addr: None,
                                                  page_boundary_cycle: true,
                                              },

                                              // 5A
                                              Instruction {
                                                  name: "*NOP",
                                                  bytes: 1,
                                                  operand: 0,
                                                  ticks: 2,
                                                  addr_mode: AddressMode::Implied,
                                                  dest_addr: None,
                                                  page_boundary_cycle: false,
                                              },

                                              // 5B
                                              Instruction {
                                                  name: "*SRE",
                                                  bytes: 3,
                                                  operand: 0,
                                                  ticks: 7,
                                                  addr_mode: AddressMode::AbsoluteY,
                                                  dest_addr: Some(0),
                                                  page_boundary_cycle: false,
                                              },

                                              // 5C
                                              Instruction {
                                                  name: "*TOP",
                                                  bytes: 3,
                                                  operand: 0,
                                                  ticks: 4,
                                                  addr_mode: AddressMode::AbsoluteX,
                                                  dest_addr: None,
                                                  page_boundary_cycle: true,
                                              },

                                              // 5D
                                              Instruction {
                                                  name: "EOR",
                                                  bytes: 3,
                                                  operand: 0,
                                                  ticks: 4,
                                                  addr_mode: AddressMode::AbsoluteX,
                                                  dest_addr: None,
                                                  page_boundary_cycle: true,
                                              },

                                              // 5E
                                              Instruction {
                                                  name: "LSR",
                                                  bytes: 3,
                                                  operand: 0,
                                                  ticks: 7,
                                                  addr_mode: AddressMode::AbsoluteX,
                                                  dest_addr: Some(0),
                                                  page_boundary_cycle: false,
                                              },

                                              // 5F
                                              Instruction {
                                                  name: "*SRE",
                                                  bytes: 3,
                                                  operand: 0,
                                                  ticks: 7,
                                                  addr_mode: AddressMode::AbsoluteX,
                                                  dest_addr: Some(0),
                                                  page_boundary_cycle: false,
                                              },

                                              // 60
                                              Instruction {
                                                  name: "RTS",
                                                  bytes: 1,
                                                  operand: 0,
                                                  ticks: 6,
                                                  addr_mode: AddressMode::Implied,
                                                  dest_addr: None,
                                                  page_boundary_cycle: false,
                                              },

                                              // 61
                                              Instruction {
                                                  name: "ADC",
                                                  bytes: 2,
                                                  operand: 0,
                                                  ticks: 6,
                                                  addr_mode: AddressMode::XIndirect,
                                                  dest_addr: None,
                                                  page_boundary_cycle: false,
                                              },

                                              // 62
                                              Instruction {
                                                  name: "KIL",
                                                  bytes: 1,
                                                  operand: 0,
                                                  ticks: 0,
                                                  addr_mode: AddressMode::Implied,
                                                  dest_addr: None,
                                                  page_boundary_cycle: false,
                                              },

                                              // 63
                                              Instruction {
                                                  name: "RRA",
                                                  bytes: 2,
                                                  operand: 0,
                                                  ticks: 8,
                                                  addr_mode: AddressMode::XIndirect,
                                                  dest_addr: Some(0),
                                                  page_boundary_cycle: false,
                                              },

                                              // 64
                                              Instruction {
                                                  name: "DOP",
                                                  bytes: 2,
                                                  operand: 0,
                                                  ticks: 3,
                                                  addr_mode: AddressMode::Zeropage,
                                                  dest_addr: None,
                                                  page_boundary_cycle: false,
                                              },

                                              // 65
                                              Instruction {
                                                  name: "ADC",
                                                  bytes: 2,
                                                  operand: 0,
                                                  ticks: 3,
                                                  addr_mode: AddressMode::Zeropage,
                                                  dest_addr: None,
                                                  page_boundary_cycle: false,
                                              },

                                              // 66
                                              Instruction {
                                                  name: "ROR",
                                                  bytes: 2,
                                                  operand: 0,
                                                  ticks: 5,
                                                  addr_mode: AddressMode::Zeropage,
                                                  dest_addr: Some(0),
                                                  page_boundary_cycle: false,
                                              },

                                              // 67
                                              Instruction {
                                                  name: "RRA",
                                                  bytes: 2,
                                                  operand: 0,
                                                  ticks: 5,
                                                  addr_mode: AddressMode::Zeropage,
                                                  dest_addr: Some(0),
                                                  page_boundary_cycle: false,
                                              },

                                              // 68
                                              Instruction {
                                                  name: "PLA",
                                                  bytes: 1,
                                                  operand: 0,
                                                  ticks: 4,
                                                  addr_mode: AddressMode::Implied,
                                                  dest_addr: None,
                                                  page_boundary_cycle: false,
                                              },

                                              // 69
                                              Instruction {
                                                  name: "ADC",
                                                  bytes: 2,
                                                  operand: 0,
                                                  ticks: 2,
                                                  addr_mode: AddressMode::Immediate,
                                                  dest_addr: None,
                                                  page_boundary_cycle: false,
                                              },

                                              // 6A
                                              Instruction {
                                                  name: "ROR",
                                                  bytes: 1,
                                                  operand: 0,
                                                  ticks: 2,
                                                  addr_mode: AddressMode::Accumulator,
                                                  dest_addr: None,
                                                  page_boundary_cycle: false,
                                              },

                                              // 6B
                                              Instruction {
                                                  name: "*ARR",
                                                  bytes: 2,
                                                  operand: 0,
                                                  ticks: 2,
                                                  addr_mode: AddressMode::Immediate,
                                                  dest_addr: None,
                                                  page_boundary_cycle: false,
                                              },

                                              // 6C
                                              Instruction {
                                                  name: "JMP",
                                                  bytes: 3,
                                                  operand: 0,
                                                  ticks: 5,
                                                  addr_mode: AddressMode::Indirect,
                                                  dest_addr: None,
                                                  page_boundary_cycle: false,
                                              },

                                              // 6D
                                              Instruction {
                                                  name: "ADC",
                                                  bytes: 3,
                                                  operand: 0,
                                                  ticks: 4,
                                                  addr_mode: AddressMode::Absolute,
                                                  dest_addr: None,
                                                  page_boundary_cycle: false,
                                              },

                                              // 6E
                                              Instruction {
                                                  name: "ROR",
                                                  bytes: 3,
                                                  operand: 0,
                                                  ticks: 6,
                                                  addr_mode: AddressMode::Absolute,
                                                  dest_addr: Some(0),
                                                  page_boundary_cycle: false,
                                              },

                                              // 6F
                                              Instruction {
                                                  name: "*RRA",
                                                  bytes: 3,
                                                  operand: 0,
                                                  ticks: 6,
                                                  addr_mode: AddressMode::Absolute,
                                                  dest_addr: Some(0),
                                                  page_boundary_cycle: false,
                                              },

                                              // 70
                                              Instruction {
                                                  name: "BVS",
                                                  bytes: 2,
                                                  operand: 0,
                                                  ticks: 2,
                                                  addr_mode: AddressMode::Relative,
                                                  dest_addr: None,
                                                  page_boundary_cycle: false,
                                              },

                                              // 71
                                              Instruction {
                                                  name: "ADC",
                                                  bytes: 2,
                                                  operand: 0,
                                                  ticks: 5,
                                                  addr_mode: AddressMode::IndirectY,
                                                  dest_addr: None,
                                                  page_boundary_cycle: false,
                                              },

                                              // 72
                                              Instruction {
                                                  name: "*KIL",
                                                  bytes: 1,
                                                  operand: 0,
                                                  ticks: 0,
                                                  addr_mode: AddressMode::Implied,
                                                  dest_addr: None,
                                                  page_boundary_cycle: false,
                                              },

                                              // 73
                                              Instruction {
                                                  name: "*RRA",
                                                  bytes: 2,
                                                  operand: 0,
                                                  ticks: 8,
                                                  addr_mode: AddressMode::IndirectY,
                                                  dest_addr: Some(0),
                                                  page_boundary_cycle: false,
                                              },

                                              // 74
                                              Instruction {
                                                  name: "*DOP",
                                                  bytes: 2,
                                                  operand: 0,
                                                  ticks: 4,
                                                  addr_mode: AddressMode::ZeropageX,
                                                  dest_addr: None,
                                                  page_boundary_cycle: false,
                                              },

                                              // 75
                                              Instruction {
                                                  name: "ADC",
                                                  bytes: 2,
                                                  operand: 0,
                                                  ticks: 4,
                                                  addr_mode: AddressMode::ZeropageX,
                                                  dest_addr: None,
                                                  page_boundary_cycle: false,
                                              },

                                              // 76
                                              Instruction {
                                                  name: "ROR",
                                                  bytes: 2,
                                                  operand: 0,
                                                  ticks: 6,
                                                  addr_mode: AddressMode::ZeropageX,
                                                  dest_addr: Some(0),
                                                  page_boundary_cycle: false,
                                              },

                                              // 77
                                              Instruction {
                                                  name: "*RRA",
                                                  bytes: 2,
                                                  operand: 0,
                                                  ticks: 6,
                                                  addr_mode: AddressMode::ZeropageX,
                                                  dest_addr: Some(0),
                                                  page_boundary_cycle: false,
                                              },

                                              // 78
                                              Instruction {
                                                  name: "SEI",
                                                  bytes: 1,
                                                  operand: 0,
                                                  ticks: 2,
                                                  addr_mode: AddressMode::Implied,
                                                  dest_addr: None,
                                                  page_boundary_cycle: false,
                                              },

                                              // 79
                                              Instruction {
                                                  name: "ADC",
                                                  bytes: 3,
                                                  operand: 0,
                                                  ticks: 4,
                                                  addr_mode: AddressMode::AbsoluteY,
                                                  dest_addr: None,
                                                  page_boundary_cycle: true,
                                              },

                                              // 7A
                                              Instruction {
                                                  name: "*NOP",
                                                  bytes: 1,
                                                  operand: 0,
                                                  ticks: 2,
                                                  addr_mode: AddressMode::Implied,
                                                  dest_addr: None,
                                                  page_boundary_cycle: false,
                                              },

                                              // 7B
                                              Instruction {
                                                  name: "*RRA",
                                                  bytes: 3,
                                                  operand: 0,
                                                  ticks: 7,
                                                  addr_mode: AddressMode::AbsoluteY,
                                                  dest_addr: Some(0),
                                                  page_boundary_cycle: false,
                                              },

                                              // 7C
                                              Instruction {
                                                  name: "*TOP",
                                                  bytes: 3,
                                                  operand: 0,
                                                  ticks: 4,
                                                  addr_mode: AddressMode::AbsoluteX,
                                                  dest_addr: None,
                                                  page_boundary_cycle: true,
                                              },

                                              // 7D
                                              Instruction {
                                                  name: "ADC",
                                                  bytes: 3,
                                                  operand: 0,
                                                  ticks: 4,
                                                  addr_mode: AddressMode::AbsoluteX,
                                                  dest_addr: None,
                                                  page_boundary_cycle: true,
                                              },

                                              // 7E
                                              Instruction {
                                                  name: "ROR",
                                                  bytes: 3,
                                                  operand: 0,
                                                  ticks: 7,
                                                  addr_mode: AddressMode::AbsoluteX,
                                                  dest_addr: Some(0),
                                                  page_boundary_cycle: false,
                                              },

                                              // 7F
                                              Instruction {
                                                  name: "*RRA",
                                                  bytes: 3,
                                                  operand: 0,
                                                  ticks: 7,
                                                  addr_mode: AddressMode::AbsoluteX,
                                                  dest_addr: Some(0),
                                                  page_boundary_cycle: false,
                                              },

                                              // 80
                                              Instruction {
                                                  name: "*DOP",
                                                  bytes: 2,
                                                  operand: 0,
                                                  ticks: 2,
                                                  addr_mode: AddressMode::Immediate,
                                                  dest_addr: None,
                                                  page_boundary_cycle: false,
                                              },

                                              // 81
                                              Instruction {
                                                  name: "*STA",
                                                  bytes: 2,
                                                  operand: 0,
                                                  ticks: 6,
                                                  addr_mode: AddressMode::XIndirect,
                                                  dest_addr: Some(0),
                                                  page_boundary_cycle: false,
                                              },

                                              // 82
                                              Instruction {
                                                  name: "*DOP",
                                                  bytes: 2,
                                                  operand: 0,
                                                  ticks: 2,
                                                  addr_mode: AddressMode::Immediate,
                                                  dest_addr: None,
                                                  page_boundary_cycle: false,
                                              },

                                              // 83
                                              Instruction {
                                                  name: "*AAX",
                                                  bytes: 2,
                                                  operand: 0,
                                                  ticks: 6,
                                                  addr_mode: AddressMode::XIndirect,
                                                  dest_addr: Some(0),
                                                  page_boundary_cycle: false,
                                              },

                                              // 84
                                              Instruction {
                                                  name: "STY",
                                                  bytes: 2,
                                                  operand: 0,
                                                  ticks: 3,
                                                  addr_mode: AddressMode::Zeropage,
                                                  dest_addr: Some(0),
                                                  page_boundary_cycle: false,
                                              },

                                              // 85
                                              Instruction {
                                                  name: "STA",
                                                  bytes: 2,
                                                  operand: 0,
                                                  ticks: 3,
                                                  addr_mode: AddressMode::Zeropage,
                                                  dest_addr: Some(0),
                                                  page_boundary_cycle: false,
                                              },

                                              // 86
                                              Instruction {
                                                  name: "STX",
                                                  bytes: 2,
                                                  operand: 0,
                                                  ticks: 3,
                                                  addr_mode: AddressMode::Zeropage,
                                                  dest_addr: Some(0),
                                                  page_boundary_cycle: false,
                                              },

                                              // 87
                                              Instruction {
                                                  name: "*AAX",
                                                  bytes: 2,
                                                  operand: 0,
                                                  ticks: 3,
                                                  addr_mode: AddressMode::Zeropage,
                                                  dest_addr: Some(0),
                                                  page_boundary_cycle: false,
                                              },

                                              // 88
                                              Instruction {
                                                  name: "DEY",
                                                  bytes: 1,
                                                  operand: 0,
                                                  ticks: 2,
                                                  addr_mode: AddressMode::Implied,
                                                  dest_addr: None,
                                                  page_boundary_cycle: false,
                                              },

                                              // 89
                                              Instruction {
                                                  name: "*DOP",
                                                  bytes: 2,
                                                  operand: 0,
                                                  ticks: 2,
                                                  addr_mode: AddressMode::Immediate,
                                                  dest_addr: None,
                                                  page_boundary_cycle: false,
                                              },

                                              // 8A
                                              Instruction {
                                                  name: "TXA",
                                                  bytes: 1,
                                                  operand: 0,
                                                  ticks: 2,
                                                  addr_mode: AddressMode::Implied,
                                                  dest_addr: None,
                                                  page_boundary_cycle: false,
                                              },

                                              // 8B
                                              Instruction {
                                                  name: "*XAA",
                                                  bytes: 2,
                                                  operand: 0,
                                                  ticks: 2,
                                                  addr_mode: AddressMode::Immediate,
                                                  dest_addr: None,
                                                  page_boundary_cycle: false,
                                              },

                                              // 8C
                                              Instruction {
                                                  name: "STY",
                                                  bytes: 3,
                                                  operand: 0,
                                                  ticks: 4,
                                                  addr_mode: AddressMode::Absolute,
                                                  dest_addr: Some(0),
                                                  page_boundary_cycle: false,
                                              },

                                              // 8D
                                              Instruction {
                                                  name: "STA",
                                                  bytes: 3,
                                                  operand: 0,
                                                  ticks: 4,
                                                  addr_mode: AddressMode::Absolute,
                                                  dest_addr: Some(0),
                                                  page_boundary_cycle: false,
                                              },

                                              // 8E
                                              Instruction {
                                                  name: "STX",
                                                  bytes: 3,
                                                  operand: 0,
                                                  ticks: 4,
                                                  addr_mode: AddressMode::Absolute,
                                                  dest_addr: Some(0),
                                                  page_boundary_cycle: false,
                                              },

                                              // 8F
                                              Instruction {
                                                  name: "*AAX",
                                                  bytes: 3,
                                                  operand: 0,
                                                  ticks: 4,
                                                  addr_mode: AddressMode::Absolute,
                                                  dest_addr: Some(0),
                                                  page_boundary_cycle: false,
                                              },

                                              // 90
                                              Instruction {
                                                  name: "BCC",
                                                  bytes: 2,
                                                  operand: 0,
                                                  ticks: 2,
                                                  addr_mode: AddressMode::Relative,
                                                  dest_addr: None,
                                                  page_boundary_cycle: false,
                                              },

                                              // 91
                                              Instruction {
                                                  name: "STA",
                                                  bytes: 2,
                                                  operand: 0,
                                                  ticks: 6,
                                                  addr_mode: AddressMode::IndirectY,
                                                  dest_addr: Some(0),
                                                  page_boundary_cycle: false,
                                              },

                                              // 92
                                              Instruction {
                                                  name: "KIL",
                                                  bytes: 1,
                                                  operand: 0,
                                                  ticks: 0,
                                                  addr_mode: AddressMode::Implied,
                                                  dest_addr: None,
                                                  page_boundary_cycle: false,
                                              },

                                              // 93
                                              Instruction {
                                                  name: "*AXA",
                                                  bytes: 2,
                                                  operand: 0,
                                                  ticks: 6,
                                                  addr_mode: AddressMode::IndirectY,
                                                  dest_addr: Some(0),
                                                  page_boundary_cycle: false,
                                              },

                                              // 94
                                              Instruction {
                                                  name: "STY",
                                                  bytes: 2,
                                                  operand: 0,
                                                  ticks: 4,
                                                  addr_mode: AddressMode::ZeropageX,
                                                  dest_addr: Some(0),
                                                  page_boundary_cycle: false,
                                              },

                                              // 95
                                              Instruction {
                                                  name: "STA",
                                                  bytes: 2,
                                                  operand: 0,
                                                  ticks: 4,
                                                  addr_mode: AddressMode::ZeropageX,
                                                  dest_addr: Some(0),
                                                  page_boundary_cycle: false,
                                              },

                                              // 96
                                              Instruction {
                                                  name: "STX",
                                                  bytes: 2,
                                                  operand: 0,
                                                  ticks: 4,
                                                  addr_mode: AddressMode::ZeropageY,
                                                  dest_addr: Some(0),
                                                  page_boundary_cycle: false,
                                              },

                                              // 97
                                              Instruction {
                                                  name: "*AAX",
                                                  bytes: 2,
                                                  operand: 0,
                                                  ticks: 4,
                                                  addr_mode: AddressMode::ZeropageY,
                                                  dest_addr: Some(0),
                                                  page_boundary_cycle: false,
                                              },

                                              // 98
                                              Instruction {
                                                  name: "TYA",
                                                  bytes: 1,
                                                  operand: 0,
                                                  ticks: 2,
                                                  addr_mode: AddressMode::Implied,
                                                  dest_addr: None,
                                                  page_boundary_cycle: false,
                                              },

                                              // 99
                                              Instruction {
                                                  name: "STA",
                                                  bytes: 3,
                                                  operand: 0,
                                                  ticks: 5,
                                                  addr_mode: AddressMode::AbsoluteY,
                                                  dest_addr: Some(0),
                                                  page_boundary_cycle: false,
                                              },

                                              // 9A
                                              Instruction {
                                                  name: "TXS",
                                                  bytes: 1,
                                                  operand: 0,
                                                  ticks: 2,
                                                  addr_mode: AddressMode::Implied,
                                                  dest_addr: None,
                                                  page_boundary_cycle: false,
                                              },

                                              // 9B
                                              Instruction {
                                                  name: "*XAS",
                                                  bytes: 3,
                                                  operand: 0,
                                                  ticks: 5,
                                                  addr_mode: AddressMode::AbsoluteY,
                                                  dest_addr: Some(0),
                                                  page_boundary_cycle: false,
                                              },

                                              // 9C
                                              Instruction {
                                                  name: "*SYA",
                                                  bytes: 3,
                                                  operand: 0,
                                                  ticks: 6,
                                                  addr_mode: AddressMode::AbsoluteX,
                                                  dest_addr: Some(0),
                                                  page_boundary_cycle: false,
                                              },

                                              // 9D
                                              Instruction {
                                                  name: "STA",
                                                  bytes: 3,
                                                  operand: 0,
                                                  ticks: 5,
                                                  addr_mode: AddressMode::AbsoluteX,
                                                  dest_addr: Some(0),
                                                  page_boundary_cycle: false,
                                              },

                                              // 9E
                                              Instruction {
                                                  name: "*SXA",
                                                  bytes: 3,
                                                  operand: 0,
                                                  ticks: 5,
                                                  addr_mode: AddressMode::AbsoluteY,
                                                  dest_addr: Some(0),
                                                  page_boundary_cycle: false,
                                              },

                                              // 9F
                                              Instruction {
                                                  name: "*AXA",
                                                  bytes: 3,
                                                  operand: 0,
                                                  ticks: 5,
                                                  addr_mode: AddressMode::AbsoluteY,
                                                  dest_addr: Some(0),
                                                  page_boundary_cycle: false,
                                              },

                                              // A0
                                              Instruction {
                                                  name: "LDY",
                                                  bytes: 2,
                                                  operand: 0,
                                                  ticks: 2,
                                                  addr_mode: AddressMode::Immediate,
                                                  dest_addr: None,
                                                  page_boundary_cycle: false,
                                              },

                                              // A1
                                              Instruction {
                                                  name: "LDA",
                                                  bytes: 2,
                                                  operand: 0,
                                                  ticks: 6,
                                                  addr_mode: AddressMode::XIndirect,
                                                  dest_addr: None,
                                                  page_boundary_cycle: false,
                                              },

                                              // A2
                                              Instruction {
                                                  name: "LDX",
                                                  bytes: 2,
                                                  operand: 0,
                                                  ticks: 2,
                                                  addr_mode: AddressMode::Immediate,
                                                  dest_addr: None,
                                                  page_boundary_cycle: false,
                                              },

                                              // A3
                                              Instruction {
                                                  name: "*LAX",
                                                  bytes: 2,
                                                  operand: 0,
                                                  ticks: 6,
                                                  addr_mode: AddressMode::XIndirect,
                                                  dest_addr: None,
                                                  page_boundary_cycle: false,
                                              },

                                              // A4
                                              Instruction {
                                                  name: "LDY",
                                                  bytes: 2,
                                                  operand: 0,
                                                  ticks: 3,
                                                  addr_mode: AddressMode::Zeropage,
                                                  dest_addr: None,
                                                  page_boundary_cycle: false,
                                              },

                                              // A5
                                              Instruction {
                                                  name: "LDA",
                                                  bytes: 2,
                                                  operand: 0,
                                                  ticks: 3,
                                                  addr_mode: AddressMode::Zeropage,
                                                  dest_addr: None,
                                                  page_boundary_cycle: false,
                                              },

                                              // A6
                                              Instruction {
                                                  name: "LDX",
                                                  bytes: 2,
                                                  operand: 0,
                                                  ticks: 3,
                                                  addr_mode: AddressMode::Zeropage,
                                                  dest_addr: None,
                                                  page_boundary_cycle: false,
                                              },

                                              // A7
                                              Instruction {
                                                  name: "*LAX",
                                                  bytes: 2,
                                                  operand: 0,
                                                  ticks: 3,
                                                  addr_mode: AddressMode::Zeropage,
                                                  dest_addr: None,
                                                  page_boundary_cycle: false,
                                              },

                                              // A8
                                              Instruction {
                                                  name: "TAY",
                                                  bytes: 1,
                                                  operand: 0,
                                                  ticks: 2,
                                                  addr_mode: AddressMode::Accumulator,
                                                  dest_addr: None,
                                                  page_boundary_cycle: false,
                                              },

                                              // A9
                                              Instruction {
                                                  name: "LDA",
                                                  bytes: 2,
                                                  operand: 0,
                                                  ticks: 2,
                                                  addr_mode: AddressMode::Immediate,
                                                  dest_addr: None,
                                                  page_boundary_cycle: false,
                                              },

                                              // AA
                                              Instruction {
                                                  name: "TAX",
                                                  bytes: 1,
                                                  operand: 0,
                                                  ticks: 2,
                                                  addr_mode: AddressMode::Accumulator,
                                                  dest_addr: None,
                                                  page_boundary_cycle: false,
                                              },

                                              // AB
                                              Instruction {
                                                  name: "*ATX",
                                                  bytes: 2,
                                                  operand: 0,
                                                  ticks: 2,
                                                  addr_mode: AddressMode::Immediate,
                                                  dest_addr: None,
                                                  page_boundary_cycle: false,
                                              },

                                              // AC
                                              Instruction {
                                                  name: "LDY",
                                                  bytes: 3,
                                                  operand: 0,
                                                  ticks: 4,
                                                  addr_mode: AddressMode::Absolute,
                                                  dest_addr: None,
                                                  page_boundary_cycle: false,
                                              },

                                              // AD
                                              Instruction {
                                                  name: "LDA",
                                                  bytes: 3,
                                                  operand: 0,
                                                  ticks: 4,
                                                  addr_mode: AddressMode::Absolute,
                                                  dest_addr: None,
                                                  page_boundary_cycle: false,
                                              },

                                              // AE
                                              Instruction {
                                                  name: "LDX",
                                                  bytes: 3,
                                                  operand: 0,
                                                  ticks: 4,
                                                  addr_mode: AddressMode::Absolute,
                                                  dest_addr: None,
                                                  page_boundary_cycle: false,
                                              },

                                              // AF
                                              Instruction {
                                                  name: "*LAX",
                                                  bytes: 3,
                                                  operand: 0,
                                                  ticks: 4,
                                                  addr_mode: AddressMode::Absolute,
                                                  dest_addr: None,
                                                  page_boundary_cycle: false,
                                              },

                                              // B0
                                              Instruction {
                                                  name: "BCS",
                                                  bytes: 2,
                                                  operand: 0,
                                                  ticks: 2,
                                                  addr_mode: AddressMode::Relative,
                                                  dest_addr: None,
                                                  page_boundary_cycle: false,
                                              },

                                              // B1
                                              Instruction {
                                                  name: "LDA",
                                                  bytes: 2,
                                                  operand: 0,
                                                  ticks: 5,
                                                  addr_mode: AddressMode::IndirectY,
                                                  dest_addr: None,
                                                  page_boundary_cycle: true,
                                              },

                                              // B2 -- not implemented
                                              Instruction {
                                                  name: "*KIL",
                                                  bytes: 1,
                                                  operand: 0,
                                                  ticks: 0,
                                                  addr_mode: AddressMode::Implied,
                                                  dest_addr: None,
                                                  page_boundary_cycle: false,
                                              },

                                              // B3
                                              Instruction {
                                                  name: "*LAX",
                                                  bytes: 2,
                                                  operand: 0,
                                                  ticks: 5,
                                                  addr_mode: AddressMode::IndirectY,
                                                  dest_addr: None,
                                                  page_boundary_cycle: true,
                                              },

                                              // B4
                                              Instruction {
                                                  name: "LDY",
                                                  bytes: 2,
                                                  operand: 0,
                                                  ticks: 4,
                                                  addr_mode: AddressMode::ZeropageX,
                                                  dest_addr: None,
                                                  page_boundary_cycle: false,
                                              },

                                              // B5
                                              Instruction {
                                                  name: "LDA",
                                                  bytes: 2,
                                                  operand: 0,
                                                  ticks: 4,
                                                  addr_mode: AddressMode::ZeropageX,
                                                  dest_addr: None,
                                                  page_boundary_cycle: false,
                                              },

                                              // B6
                                              Instruction {
                                                  name: "LDX",
                                                  bytes: 2,
                                                  operand: 0,
                                                  ticks: 4,
                                                  addr_mode: AddressMode::ZeropageY,
                                                  dest_addr: None,
                                                  page_boundary_cycle: false,
                                              },

                                              // B7
                                              Instruction {
                                                  name: "LAX",
                                                  bytes: 2,
                                                  operand: 0,
                                                  ticks: 4,
                                                  addr_mode: AddressMode::ZeropageY,
                                                  dest_addr: None,
                                                  page_boundary_cycle: false,
                                              },

                                              // B8
                                              Instruction {
                                                  name: "CLV",
                                                  bytes: 1,
                                                  operand: 0,
                                                  ticks: 2,
                                                  addr_mode: AddressMode::Implied,
                                                  dest_addr: None,
                                                  page_boundary_cycle: false,
                                              },

                                              // B9
                                              Instruction {
                                                  name: "LDA",
                                                  bytes: 3,
                                                  operand: 0,
                                                  ticks: 4,
                                                  addr_mode: AddressMode::AbsoluteY,
                                                  dest_addr: None,
                                                  page_boundary_cycle: true,
                                              },

                                              // BA
                                              Instruction {
                                                  name: "TSX",
                                                  bytes: 1,
                                                  operand: 0,
                                                  ticks: 2,
                                                  addr_mode: AddressMode::Implied,
                                                  dest_addr: None,
                                                  page_boundary_cycle: false,
                                              },

                                              // BB -- not implemented
                                              Instruction {
                                                  name: "*LAR",
                                                  bytes: 3,
                                                  operand: 0,
                                                  ticks: 4,
                                                  addr_mode: AddressMode::AbsoluteY,
                                                  dest_addr: None,
                                                  page_boundary_cycle: true,
                                              },

                                              // BC
                                              Instruction {
                                                  name: "LDY",
                                                  bytes: 3,
                                                  operand: 0,
                                                  ticks: 4,
                                                  addr_mode: AddressMode::AbsoluteX,
                                                  dest_addr: None,
                                                  page_boundary_cycle: true,
                                              },

                                              // BD
                                              Instruction {
                                                  name: "LDA",
                                                  bytes: 3,
                                                  operand: 0,
                                                  ticks: 4,
                                                  addr_mode: AddressMode::AbsoluteX,
                                                  dest_addr: None,
                                                  page_boundary_cycle: true,
                                              },

                                              // BE
                                              Instruction {
                                                  name: "LDX",
                                                  bytes: 3,
                                                  operand: 0,
                                                  ticks: 4,
                                                  addr_mode: AddressMode::AbsoluteY,
                                                  dest_addr: None,
                                                  page_boundary_cycle: true,
                                              },

                                              // BF
                                              Instruction {
                                                  name: "*LAX",
                                                  bytes: 3,
                                                  operand: 0,
                                                  ticks: 4,
                                                  addr_mode: AddressMode::AbsoluteY,
                                                  dest_addr: None,
                                                  page_boundary_cycle: true,
                                              },

                                              // C0
                                              Instruction {
                                                  name: "CPY",
                                                  bytes: 2,
                                                  operand: 0,
                                                  ticks: 2,
                                                  addr_mode: AddressMode::Immediate,
                                                  dest_addr: None,
                                                  page_boundary_cycle: false,
                                              },

                                              // C1
                                              Instruction {
                                                  name: "CMP",
                                                  bytes: 2,
                                                  operand: 0,
                                                  ticks: 6,
                                                  addr_mode: AddressMode::XIndirect,
                                                  dest_addr: None,
                                                  page_boundary_cycle: false,
                                              },

                                              // C2
                                              Instruction {
                                                  name: "*DOP",
                                                  bytes: 2,
                                                  operand: 0,
                                                  ticks: 2,
                                                  addr_mode: AddressMode::Immediate,
                                                  dest_addr: None,
                                                  page_boundary_cycle: false,
                                              },

                                              // C3
                                              Instruction {
                                                  name: "*DCP",
                                                  bytes: 2,
                                                  operand: 0,
                                                  ticks: 8,
                                                  addr_mode: AddressMode::XIndirect,
                                                  dest_addr: Some(0),
                                                  page_boundary_cycle: false,
                                              },

                                              // C4
                                              Instruction {
                                                  name: "CPY",
                                                  bytes: 2,
                                                  operand: 0,
                                                  ticks: 3,
                                                  addr_mode: AddressMode::Zeropage,
                                                  dest_addr: None,
                                                  page_boundary_cycle: false,
                                              },

                                              // C5
                                              Instruction {
                                                  name: "CMP",
                                                  bytes: 2,
                                                  operand: 0,
                                                  ticks: 3,
                                                  addr_mode: AddressMode::Zeropage,
                                                  dest_addr: None,
                                                  page_boundary_cycle: false,
                                              },

                                              // C6
                                              Instruction {
                                                  name: "DEC",
                                                  bytes: 2,
                                                  operand: 0,
                                                  ticks: 5,
                                                  addr_mode: AddressMode::Zeropage,
                                                  dest_addr: Some(0),
                                                  page_boundary_cycle: false,
                                              },

                                              // C7
                                              Instruction {
                                                  name: "*DCP",
                                                  bytes: 2,
                                                  operand: 0,
                                                  ticks: 5,
                                                  addr_mode: AddressMode::Zeropage,
                                                  dest_addr: Some(0),
                                                  page_boundary_cycle: false,
                                              },

                                              // C8
                                              Instruction {
                                                  name: "INY",
                                                  bytes: 1,
                                                  operand: 0,
                                                  ticks: 2,
                                                  addr_mode: AddressMode::Implied,
                                                  dest_addr: None,
                                                  page_boundary_cycle: false,
                                              },

                                              // C9
                                              Instruction {
                                                  name: "CMP",
                                                  bytes: 2,
                                                  operand: 0,
                                                  ticks: 2,
                                                  addr_mode: AddressMode::Immediate,
                                                  dest_addr: None,
                                                  page_boundary_cycle: false,
                                              },

                                              // CA
                                              Instruction {
                                                  name: "DEX",
                                                  bytes: 1,
                                                  operand: 0,
                                                  ticks: 2,
                                                  addr_mode: AddressMode::Implied,
                                                  dest_addr: None,
                                                  page_boundary_cycle: false,
                                              },

                                              // CB -- Not implemented
                                              Instruction {
                                                  name: "*AXS",
                                                  bytes: 2,
                                                  operand: 0,
                                                  ticks: 2,
                                                  addr_mode: AddressMode::Accumulator,
                                                  dest_addr: None,
                                                  page_boundary_cycle: false,
                                              },

                                              // CC
                                              Instruction {
                                                  name: "CPY",
                                                  bytes: 3,
                                                  operand: 0,
                                                  ticks: 4,
                                                  addr_mode: AddressMode::Absolute,
                                                  dest_addr: None,
                                                  page_boundary_cycle: false,
                                              },

                                              // CD
                                              Instruction {
                                                  name: "CMP",
                                                  bytes: 3,
                                                  operand: 0,
                                                  ticks: 4,
                                                  addr_mode: AddressMode::Absolute,
                                                  dest_addr: None,
                                                  page_boundary_cycle: false,
                                              },

                                              // CE
                                              Instruction {
                                                  name: "DEC",
                                                  bytes: 3,
                                                  operand: 0,
                                                  ticks: 6,
                                                  addr_mode: AddressMode::Absolute,
                                                  dest_addr: Some(0),
                                                  page_boundary_cycle: false,
                                              },

                                              // CF
                                              Instruction {
                                                  name: "*DCP",
                                                  bytes: 3,
                                                  operand: 0,
                                                  ticks: 6,
                                                  addr_mode: AddressMode::Absolute,
                                                  dest_addr: Some(0),
                                                  page_boundary_cycle: false,
                                              },

                                              // D0
                                              Instruction {
                                                  name: "BNE",
                                                  bytes: 2,
                                                  operand: 0,
                                                  ticks: 2,
                                                  addr_mode: AddressMode::Relative,
                                                  dest_addr: None,
                                                  page_boundary_cycle: false,
                                              },

                                              // D1
                                              Instruction {
                                                  name: "CMP",
                                                  bytes: 2,
                                                  operand: 0,
                                                  ticks: 5,
                                                  addr_mode: AddressMode::IndirectY,
                                                  dest_addr: None,
                                                  page_boundary_cycle: true,
                                              },

                                              // D2
                                              Instruction {
                                                  name: "KIL",
                                                  bytes: 1,
                                                  operand: 0,
                                                  ticks: 0,
                                                  addr_mode: AddressMode::Implied,
                                                  dest_addr: None,
                                                  page_boundary_cycle: false,
                                              },

                                              // D3
                                              Instruction {
                                                  name: "*DCP",
                                                  bytes: 2,
                                                  operand: 0,
                                                  ticks: 8,
                                                  addr_mode: AddressMode::IndirectY,
                                                  dest_addr: Some(0),
                                                  page_boundary_cycle: false,
                                              },

                                              // D4
                                              Instruction {
                                                  name: "*DOP",
                                                  bytes: 2,
                                                  operand: 0,
                                                  ticks: 4,
                                                  addr_mode: AddressMode::ZeropageX,
                                                  dest_addr: None,
                                                  page_boundary_cycle: false,
                                              },

                                              // D5
                                              Instruction {
                                                  name: "CMP",
                                                  bytes: 2,
                                                  operand: 0,
                                                  ticks: 4,
                                                  addr_mode: AddressMode::ZeropageX,
                                                  dest_addr: None,
                                                  page_boundary_cycle: false,
                                              },

                                              // D6
                                              Instruction {
                                                  name: "DEC",
                                                  bytes: 2,
                                                  operand: 0,
                                                  ticks: 6,
                                                  addr_mode: AddressMode::ZeropageX,
                                                  dest_addr: Some(0),
                                                  page_boundary_cycle: false,
                                              },

                                              // D7
                                              Instruction {
                                                  name: "*DCP",
                                                  bytes: 2,
                                                  operand: 0,
                                                  ticks: 6,
                                                  addr_mode: AddressMode::ZeropageX,
                                                  dest_addr: Some(0),
                                                  page_boundary_cycle: false,
                                              },

                                              // D8
                                              Instruction {
                                                  name: "CLD",
                                                  bytes: 1,
                                                  operand: 0,
                                                  ticks: 2,
                                                  addr_mode: AddressMode::Implied,
                                                  dest_addr: None,
                                                  page_boundary_cycle: false,
                                              },

                                              // D9
                                              Instruction {
                                                  name: "CMP",
                                                  bytes: 3,
                                                  operand: 0,
                                                  ticks: 4,
                                                  addr_mode: AddressMode::AbsoluteY,
                                                  dest_addr: None,
                                                  page_boundary_cycle: true,
                                              },

                                              // DA
                                              Instruction {
                                                  name: "*NOP",
                                                  bytes: 1,
                                                  operand: 0,
                                                  ticks: 2,
                                                  addr_mode: AddressMode::Implied,
                                                  dest_addr: None,
                                                  page_boundary_cycle: false,
                                              },

                                              // DB
                                              Instruction {
                                                  name: "*DCP",
                                                  bytes: 3,
                                                  operand: 0,
                                                  ticks: 7,
                                                  addr_mode: AddressMode::AbsoluteY,
                                                  dest_addr: Some(0),
                                                  page_boundary_cycle: false,
                                              },

                                              // DC
                                              Instruction {
                                                  name: "*TOP",
                                                  bytes: 3,
                                                  operand: 0,
                                                  ticks: 4,
                                                  addr_mode: AddressMode::AbsoluteX,
                                                  dest_addr: None,
                                                  page_boundary_cycle: true,
                                              },

                                              // DD
                                              Instruction {
                                                  name: "CMP",
                                                  bytes: 3,
                                                  operand: 0,
                                                  ticks: 4,
                                                  addr_mode: AddressMode::AbsoluteX,
                                                  dest_addr: None,
                                                  page_boundary_cycle: true,
                                              },

                                              // DE
                                              Instruction {
                                                  name: "DEC",
                                                  bytes: 3,
                                                  operand: 0,
                                                  ticks: 7,
                                                  addr_mode: AddressMode::AbsoluteX,
                                                  dest_addr: Some(0),
                                                  page_boundary_cycle: false,
                                              },

                                              // DF
                                              Instruction {
                                                  name: "*DCP",
                                                  bytes: 3,
                                                  operand: 0,
                                                  ticks: 7,
                                                  addr_mode: AddressMode::AbsoluteX,
                                                  dest_addr: Some(0),
                                                  page_boundary_cycle: false,
                                              },

                                              // E0
                                              Instruction {
                                                  name: "CPX",
                                                  bytes: 2,
                                                  operand: 0,
                                                  ticks: 2,
                                                  addr_mode: AddressMode::Immediate,
                                                  dest_addr: None,
                                                  page_boundary_cycle: false,
                                              },

                                              // E1
                                              Instruction {
                                                  name: "SBC",
                                                  bytes: 2,
                                                  operand: 0,
                                                  ticks: 6,
                                                  addr_mode: AddressMode::XIndirect,
                                                  dest_addr: None,
                                                  page_boundary_cycle: false,
                                              },

                                              // E2
                                              Instruction {
                                                  name: "*DOP",
                                                  bytes: 2,
                                                  operand: 0,
                                                  ticks: 2,
                                                  addr_mode: AddressMode::Immediate,
                                                  dest_addr: None,
                                                  page_boundary_cycle: false,
                                              },

                                              // E3
                                              Instruction {
                                                  name: "*ISC",
                                                  bytes: 2,
                                                  operand: 0,
                                                  ticks: 8,
                                                  addr_mode: AddressMode::XIndirect,
                                                  dest_addr: Some(0),
                                                  page_boundary_cycle: false,
                                              },

                                              // E4
                                              Instruction {
                                                  name: "CPX",
                                                  bytes: 2,
                                                  operand: 0,
                                                  ticks: 3,
                                                  addr_mode: AddressMode::Zeropage,
                                                  dest_addr: None,
                                                  page_boundary_cycle: false,
                                              },

                                              // E5
                                              Instruction {
                                                  name: "CPX",
                                                  bytes: 2,
                                                  operand: 0,
                                                  ticks: 3,
                                                  addr_mode: AddressMode::Zeropage,
                                                  dest_addr: None,
                                                  page_boundary_cycle: false,
                                              },

                                              // E6
                                              Instruction {
                                                  name: "INC",
                                                  bytes: 2,
                                                  operand: 0,
                                                  ticks: 5,
                                                  addr_mode: AddressMode::Zeropage,
                                                  dest_addr: Some(0),
                                                  page_boundary_cycle: false,
                                              },

                                              // E7
                                              Instruction {
                                                  name: "*ISC",
                                                  bytes: 2,
                                                  operand: 0,
                                                  ticks: 5,
                                                  addr_mode: AddressMode::Zeropage,
                                                  dest_addr: Some(0),
                                                  page_boundary_cycle: false,
                                              },

                                              // E8
                                              Instruction {
                                                  name: "INX",
                                                  bytes: 1,
                                                  operand: 0,
                                                  ticks: 2,
                                                  addr_mode: AddressMode::Implied,
                                                  dest_addr: None,
                                                  page_boundary_cycle: false,
                                              },

                                              // E9
                                              Instruction {
                                                  name: "SBC",
                                                  bytes: 2,
                                                  operand: 0,
                                                  ticks: 2,
                                                  addr_mode: AddressMode::Immediate,
                                                  dest_addr: None,
                                                  page_boundary_cycle: false,
                                              },

                                              // EA
                                              Instruction {
                                                  name: "NOP",
                                                  bytes: 1,
                                                  operand: 0,
                                                  ticks: 2,
                                                  addr_mode: AddressMode::Implied,
                                                  dest_addr: None,
                                                  page_boundary_cycle: false,
                                              },

                                              // EB
                                              Instruction {
                                                  name: "*SBC",
                                                  bytes: 2,
                                                  operand: 0,
                                                  ticks: 2,
                                                  addr_mode: AddressMode::Immediate,
                                                  dest_addr: None,
                                                  page_boundary_cycle: false,
                                              },

                                              // EC
                                              Instruction {
                                                  name: "CPX",
                                                  bytes: 3,
                                                  operand: 0,
                                                  ticks: 4,
                                                  addr_mode: AddressMode::Absolute,
                                                  dest_addr: None,
                                                  page_boundary_cycle: false,
                                              },

                                              // ED
                                              Instruction {
                                                  name: "SBC",
                                                  bytes: 3,
                                                  operand: 0,
                                                  ticks: 4,
                                                  addr_mode: AddressMode::Absolute,
                                                  dest_addr: None,
                                                  page_boundary_cycle: false,
                                              },

                                              // EE
                                              Instruction {
                                                  name: "INC",
                                                  bytes: 3,
                                                  operand: 0,
                                                  ticks: 6,
                                                  addr_mode: AddressMode::Absolute,
                                                  dest_addr: Some(0),
                                                  page_boundary_cycle: false,
                                              },

                                              // EF
                                              Instruction {
                                                  name: "*ISC",
                                                  bytes: 3,
                                                  operand: 0,
                                                  ticks: 6,
                                                  addr_mode: AddressMode::Absolute,
                                                  dest_addr: Some(0),
                                                  page_boundary_cycle: false,
                                              },

                                              // F0
                                              Instruction {
                                                  name: "BEQ",
                                                  bytes: 2,
                                                  operand: 0,
                                                  ticks: 2,
                                                  addr_mode: AddressMode::Relative,
                                                  dest_addr: None,
                                                  page_boundary_cycle: false,
                                              },

                                              // F1
                                              Instruction {
                                                  name: "SBC",
                                                  bytes: 2,
                                                  operand: 0,
                                                  ticks: 5,
                                                  addr_mode: AddressMode::IndirectY,
                                                  dest_addr: None,
                                                  page_boundary_cycle: true,
                                              },

                                              // F2
                                              Instruction {
                                                  name: "*KIL",
                                                  bytes: 1,
                                                  operand: 0,
                                                  ticks: 0,
                                                  addr_mode: AddressMode::Implied,
                                                  dest_addr: None,
                                                  page_boundary_cycle: false,
                                              },

                                              // F3
                                              Instruction {
                                                  name: "*ISC",
                                                  bytes: 2,
                                                  operand: 0,
                                                  ticks: 8,
                                                  addr_mode: AddressMode::IndirectY,
                                                  dest_addr: Some(0),
                                                  page_boundary_cycle: false,
                                              },

                                              // F4
                                              Instruction {
                                                  name: "*DOP",
                                                  bytes: 2,
                                                  operand: 0,
                                                  ticks: 4,
                                                  addr_mode: AddressMode::ZeropageX,
                                                  dest_addr: None,
                                                  page_boundary_cycle: false,
                                              },

                                              // F5
                                              Instruction {
                                                  name: "SBC",
                                                  bytes: 2,
                                                  operand: 0,
                                                  ticks: 4,
                                                  addr_mode: AddressMode::ZeropageX,
                                                  dest_addr: None,
                                                  page_boundary_cycle: false,
                                              },

                                              // F6
                                              Instruction {
                                                  name: "INC",
                                                  bytes: 2,
                                                  operand: 0,
                                                  ticks: 6,
                                                  addr_mode: AddressMode::ZeropageX,
                                                  dest_addr: Some(0),
                                                  page_boundary_cycle: false,
                                              },

                                              // F7
                                              Instruction {
                                                  name: "*ISC",
                                                  bytes: 2,
                                                  operand: 0,
                                                  ticks: 6,
                                                  addr_mode: AddressMode::ZeropageX,
                                                  dest_addr: Some(0),
                                                  page_boundary_cycle: false,
                                              },

                                              // F8
                                              Instruction {
                                                  name: "SED",
                                                  bytes: 1,
                                                  operand: 0,
                                                  ticks: 2,
                                                  addr_mode: AddressMode::Implied,
                                                  dest_addr: None,
                                                  page_boundary_cycle: false,
                                              },

                                              // F9
                                              Instruction {
                                                  name: "SBC",
                                                  bytes: 3,
                                                  operand: 0,
                                                  ticks: 4,
                                                  addr_mode: AddressMode::AbsoluteY,
                                                  dest_addr: None,
                                                  page_boundary_cycle: true,
                                              },

                                              // FA
                                              Instruction {
                                                  name: "*NOP",
                                                  bytes: 1,
                                                  operand: 0,
                                                  ticks: 2,
                                                  addr_mode: AddressMode::Implied,
                                                  dest_addr: None,
                                                  page_boundary_cycle: false,
                                              },

                                              // FB
                                              Instruction {
                                                  name: "*ISC",
                                                  bytes: 3,
                                                  operand: 0,
                                                  ticks: 7,
                                                  addr_mode: AddressMode::AbsoluteY,
                                                  dest_addr: Some(0),
                                                  page_boundary_cycle: false,
                                              },

                                              // FC
                                              Instruction {
                                                  name: "*TOP",
                                                  bytes: 3,
                                                  operand: 0,
                                                  ticks: 4,
                                                  addr_mode: AddressMode::AbsoluteX,
                                                  dest_addr: None,
                                                  page_boundary_cycle: true,
                                              },

                                              // FD
                                              Instruction {
                                                  name: "SBC",
                                                  bytes: 3,
                                                  operand: 0,
                                                  ticks: 4,
                                                  addr_mode: AddressMode::AbsoluteX,
                                                  dest_addr: None,
                                                  page_boundary_cycle: true,
                                              },

                                              // FE
                                              Instruction {
                                                  name: "INC",
                                                  bytes: 3,
                                                  operand: 0,
                                                  ticks: 7,
                                                  addr_mode: AddressMode::AbsoluteX,
                                                  dest_addr: Some(0),
                                                  page_boundary_cycle: false,
                                              },

                                              // FF
                                              Instruction {
                                                  name: "*ISC",
                                                  bytes: 3,
                                                  operand: 0,
                                                  ticks: 7,
                                                  addr_mode: AddressMode::AbsoluteX,
                                                  dest_addr: Some(0),
                                                  page_boundary_cycle: false,
                                              }];











// #[derive(Debug, Clone, Copy, PartialEq)]
// pub enum DestAddr {
// Memory(u16),
// Accumulator,
// IndexX,
// IndexY,
// Na,
// }
//

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum AddressMode {
    Accumulator,
    Absolute,
    AbsoluteX,
    AbsoluteY,
    Immediate,
    XIndirect,
    IndirectY,
    Zeropage,
    ZeropageX,
    ZeropageY,
    Implied,
    Indirect,
    Relative,
}

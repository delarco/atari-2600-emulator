use super::{Flags, Registers};

pub struct CPU {
    registers: Registers,
    flags: Flags,
}

impl CPU {
    pub fn new() -> Self {
        log::info!("CPU::new");
        Self {
            registers: Registers::new(),
            flags: Flags::new(),
        }
    }
}

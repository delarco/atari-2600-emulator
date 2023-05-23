use super::{Registers, Flags};

pub struct CPU {

    registers: Registers,
    flags: Flags,

}

impl CPU {

    pub fn new() -> Self {
        Self {
           registers: Registers::new(),
           flags: Flags::new(),
        }
    }
}

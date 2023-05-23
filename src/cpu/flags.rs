pub struct Flags {
    /// Carry (0=No Carry, 1=Carry)
    pub C: u8,

    /// Zero (0=Nonzero, 1=Zero)
    pub Z: u8,

    /// IRQ Disable (0=IRQ Enable, 1=IRQ Disable)
    pub I: u8,

    /// Decimal Mode (0=Normal, 1=BCD Mode for ADC/SBC opcodes)
    pub D: u8,

    /// Break Flag (0=IRQ/NMI, 1=RESET or BRK/PHP opcode)
    pub B: u8,

    /// Not used (Always 1)
    pub not_used: u8,

    /// Overflow (0=No Overflow, 1=Overflow)
    pub V: u8,

    /// Negative/Sign (0=Positive, 1=Negative)
    pub N: u8,
}

impl Flags {
    pub fn new() -> Self {
        Self {
            C: 0,
            Z: 0,
            I: 0,
            D: 0,
            B: 0,
            not_used: 1,
            V: 0,
            N: 0,
        }
    }
}

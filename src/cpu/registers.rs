pub struct Registers {
    /// Accumulator
    pub A: u8,

    /// Index Register X
    pub X: u8,

    /// Index Register Y
    pub Y: u8,

    /// Program Counter
    pub PC: u16,

    /// Stack Pointer
    pub S: u8,

    /// Processor Status Register
    pub P: u8,
}

impl Registers {
    pub fn new() -> Self {
        log::info!("Registers::new");
        Self {
            A: 0,
            X: 0,
            Y: 0,
            PC: 0,
            S: 0,
            P: 0,
        }
    }
}

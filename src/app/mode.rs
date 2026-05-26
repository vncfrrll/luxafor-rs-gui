#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Mode {
    Static,
    Fade,
    Strobe,
    Wave,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WaveType {
    Short,
    Long,
    OverlappingShort,
    OverlappingLong,
    Wave5,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WaveActiveColor {
    A,
    B
}

impl WaveType {
    pub fn to_byte(&self) -> u8 {
        match self {
            WaveType::Short => 1,
            WaveType::Long => 2,
            WaveType::OverlappingShort => 3,
            WaveType::OverlappingLong => 4,
            WaveType::Wave5 => 5,
        }
    }

    pub fn label(&self) -> &str {
        match self {
            WaveType::Short => "Short",
            WaveType::Long => "Long",
            WaveType::OverlappingShort => "Overlapping Short",
            WaveType::OverlappingLong => "Overlapping Long",
            WaveType::Wave5 => "Pulse Strobe",
        }
    }

    pub fn all() -> Vec<WaveType> {
        vec![
            WaveType::Short,
            WaveType::Long,
            WaveType::OverlappingShort,
            WaveType::OverlappingLong,
            WaveType::Wave5,
        ]
    }

    pub fn uses_two_colors(&self) -> bool {
        match self {
            WaveType::Short | WaveType::Long => false,
            WaveType::OverlappingShort | WaveType::OverlappingLong | WaveType::Wave5 => true,
        }
    }
}
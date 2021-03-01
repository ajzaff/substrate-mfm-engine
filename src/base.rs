use num_derive::FromPrimitive;
use num_traits::FromPrimitive;
use std::fmt;

#[derive(Copy, Clone, Debug, PartialEq, PartialOrd, FromPrimitive)]
pub enum Register {
    R0,
    R1,
    R2,
    R3,
    R4,
    R5,
    R6,
    R7,
    R8,
    R9,
    R10,
    R11,
    R12,
    R13,
    R14,
    RRand,
}

impl Register {
    pub fn from_usize(x: usize) -> Option<Self> {
        FromPrimitive::from_usize(x)
    }
}

impl fmt::Display for Register {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::R0 => "r0",
                Self::R1 => "r1",
                Self::R2 => "r2",
                Self::R3 => "r3",
                Self::R4 => "r4",
                Self::R5 => "r5",
                Self::R6 => "r6",
                Self::R7 => "r7",
                Self::R8 => "r8",
                Self::R9 => "r9",
                Self::R10 => "r10",
                Self::R11 => "r11",
                Self::R12 => "r12",
                Self::R13 => "r13",
                Self::R14 => "r14",
                Self::RRand => "r?",
            }
        )
    }
}

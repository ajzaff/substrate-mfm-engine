use std::fmt;
use std::num::ParseIntError;
use std::ops::{Add, Div, Mul, Neg, Rem, Sub};
use std::ops::{AddAssign, DivAssign, MulAssign, RemAssign, SubAssign};
use std::ops::{BitAnd, BitOr, BitXor};
use std::ops::{BitAndAssign, BitOrAssign, BitXorAssign};

#[derive(Copy, Clone, Debug)]
pub struct U96(pub u128);

impl U96 {
    pub const MIN: Self = Self(0);
    pub const MAX: Self = Self(1 << 96 - 1);

    fn is_valid(self) -> bool {
        self.0 < Self::MAX.0 // FIXME
    }

    fn truncate(self) -> Self {
        Self(self.0 & Self::MAX.0) // FIXME
    }

    pub fn from_str_radix(s: &str, radix: u32) -> Result<Self, ParseIntError> {
        u128::from_str_radix(s, radix).map(|x| Self(x))
    }
}

impl fmt::Display for U96 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "U96({})", self.0)
    }
}

impl Add for U96 {
    type Output = Self;

    fn add(self, rhs: Self) -> Self {
        Self(self.0 + rhs.0)
    }
}

impl Sub for U96 {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self {
        Self(self.0 - rhs.0)
    }
}

#[derive(Copy, Clone, Debug)]
pub struct I96(pub i128);

impl I96 {
    pub const MIN: Self = Self(-1 << 95 - 1);
    pub const MAX: Self = Self(1 << 95 - 1);

    fn is_valid(self) -> bool {
        self.0 > Self::MIN.0 && self.0 < Self::MAX.0 // FIXME
    }

    fn truncate(self) -> Self {
        Self(self.0 & Self::MAX.0) // FIXME
    }

    pub fn from_str_radix(s: &str, radix: u32) -> Result<Self, ParseIntError> {
        i128::from_str_radix(s, radix).map(|x| Self(x))
    }
}

impl fmt::Display for I96 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "I96({})", self.0)
    }
}

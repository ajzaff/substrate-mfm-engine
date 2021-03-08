use std::fmt;
use std::num::ParseIntError;
use std::ops::{Add, Div, Shl, Shr, Sub};

#[derive(Copy, Clone, Debug)]
pub struct U96(u128);

impl U96 {
    pub const MIN: Self = Self(0);
    pub const MAX: Self = Self(1 << 96 - 1);

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

impl From<u8> for U96 {
    fn from(x: u8) -> Self {
        Self(x as u128)
    }
}

impl From<u16> for U96 {
    fn from(x: u16) -> Self {
        Self(x as u128)
    }
}

impl From<u128> for U96 {
    fn from(x: u128) -> Self {
        Self(x)
    }
}

impl Add for U96 {
    type Output = Self;

    fn add(self, rhs: Self) -> Self {
        Self(self.0.saturating_add(rhs.0))
    }
}

impl Sub for U96 {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self {
        Self(self.0.saturating_sub(rhs.0))
    }
}

impl Div for U96 {
    type Output = Self;

    fn div(self, rhs: Self) -> Self {
        Self(self.0 / rhs.0)
    }
}

impl Shr<u8> for U96 {
    type Output = Self;

    fn shr(self, rhs: u8) -> Self {
        Self(self.0 >> rhs)
    }
}

impl Shl<u8> for U96 {
    type Output = Self;

    fn shl(self, rhs: u8) -> Self {
        Self(self.0.checked_shl(rhs as u32).unwrap_or(self.0))
    }
}

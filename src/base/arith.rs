use std::fmt;
use std::num::ParseIntError;
use std::ops::{Add, BitAnd, Div, Shl, Shr, Sub};

#[derive(Copy, Clone, Debug)]
pub enum Const {
    Unsigned(u128),
    Signed(i128),
}

impl Const {
    pub fn from_str_radix(s: &str, radix: u32) -> Result<Self, ParseIntError> {
        u128::from_str_radix(s, radix).map(|x| Self::Unsigned(x))
    }

    pub fn from_str_signed_radix(s: &str, radix: u32) -> Result<Self, ParseIntError> {
        i128::from_str_radix(s, radix).map(|x| Self::Signed(x))
    }

    pub fn as_u128(self) -> u128 {
        match self {
            Self::Unsigned(x) => x,
            Self::Signed(x) => x as u128,
        }
    }

    pub fn as_i128(self) -> i128 {
        match self {
            Self::Unsigned(x) => x as i128,
            Self::Signed(x) => x,
        }
    }
}

impl fmt::Display for Const {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Unsigned(x) => write!(f, "U96({})", x),
            Self::Signed(x) => write!(f, "I96({})", x),
        }
    }
}

impl From<u8> for Const {
    fn from(x: u8) -> Self {
        Self::Unsigned(x as u128)
    }
}

impl From<u16> for Const {
    fn from(x: u16) -> Self {
        Self::Unsigned(x as u128)
    }
}

impl From<u32> for Const {
    fn from(x: u32) -> Self {
        Self::Unsigned(x as u128)
    }
}

impl From<u64> for Const {
    fn from(x: u64) -> Self {
        Self::Unsigned(x as u128)
    }
}

impl From<u128> for Const {
    fn from(x: u128) -> Self {
        Self::Unsigned(x)
    }
}

impl From<i8> for Const {
    fn from(x: i8) -> Self {
        Self::Signed(x as i128)
    }
}

impl From<i16> for Const {
    fn from(x: i16) -> Self {
        Self::Signed(x as i128)
    }
}

impl From<i32> for Const {
    fn from(x: i32) -> Self {
        Self::Signed(x as i128)
    }
}

impl From<i64> for Const {
    fn from(x: i64) -> Self {
        Self::Signed(x as i128)
    }
}

impl From<i128> for Const {
    fn from(x: i128) -> Self {
        Self::Signed(x)
    }
}

impl Add for Const {
    type Output = Self;

    fn add(self, rhs: Self) -> Self {
        match self {
            Self::Unsigned(x) => Self::Unsigned(x.saturating_add(rhs.as_u128())),
            Self::Signed(x) => Self::Signed(x.saturating_add(rhs.as_i128())),
        }
    }
}

impl Sub for Const {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self {
        match self {
            Self::Unsigned(x) => Self::Unsigned(x.saturating_sub(rhs.as_u128())),
            Self::Signed(x) => Self::Signed(x.saturating_sub(rhs.as_i128())),
        }
    }
}

impl Div for Const {
    type Output = Self;

    fn div(self, rhs: Self) -> Self {
        match self {
            Self::Unsigned(x) => Self::Unsigned(x / rhs.as_u128()),
            Self::Signed(x) => Self::Signed(x / rhs.as_i128()),
        }
    }
}

impl Shr<u8> for Const {
    type Output = Self;

    fn shr(self, rhs: u8) -> Self {
        match self {
            Self::Unsigned(x) => Self::Unsigned(x >> rhs),
            Self::Signed(x) => Self::Signed(x >> rhs),
        }
    }
}

impl Shl<u8> for Const {
    type Output = Self;

    fn shl(self, rhs: u8) -> Self {
        match self {
            Self::Unsigned(x) => Self::Unsigned(x << rhs),
            Self::Signed(x) => Self::Signed(x << rhs),
        }
    }
}

impl BitAnd for Const {
    type Output = Self;

    fn bitand(self, rhs: Self) -> Self {
        match self {
            Self::Unsigned(x) => Self::Unsigned(x & rhs.as_u128()),
            Self::Signed(x) => Self::Signed(x & rhs.as_i128()),
        }
    }
}

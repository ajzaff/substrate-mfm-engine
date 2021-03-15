use crate::base::FieldSelector;
use std::num::ParseIntError;
use std::ops::{Add, BitAnd, BitOr, BitXor, Div, Mul, Neg, Rem, Shl, Shr, Sub};

#[derive(Copy, Clone, Debug, PartialEq, PartialOrd)]
pub enum Const {
    Unsigned(u128),
    Signed(i128),
}

impl Const {
    pub fn is_zero(&self) -> bool {
        match self {
            Self::Unsigned(x) => *x == 0,
            Self::Signed(x) => *x == 0,
        }
    }

    pub fn count_ones(&self) -> u32 {
        match self {
            Self::Unsigned(x) => x.count_ones(),
            Self::Signed(x) => x.count_ones(),
        }
    }

    fn is_neg(&self) -> bool {
        match self {
            Self::Unsigned(_) => false,
            Self::Signed(x) => *x < 0,
        }
    }

    fn as_u128_bits(&self) -> u128 {
        match self {
            Self::Unsigned(x) => *x,
            Self::Signed(x) => *x as u128,
        }
    }

    fn as_u128_saturating(&self) -> u128 {
        match self {
            Self::Unsigned(x) => *x,
            Self::Signed(x) => {
                if *x < 0 {
                    0
                } else {
                    *x as u128
                }
            }
        }
    }

    fn neg_saturating(x: i128) -> i128 {
        if x == i128::MIN {
            i128::MAX
        } else {
            -x
        }
    }

    fn i128_saturating(x: u128) -> i128 {
        if x > i128::MAX as u128 {
            i128::MAX
        } else {
            x as i128
        }
    }

    fn as_i128_saturating(&self) -> i128 {
        match self {
            Self::Unsigned(x) => Self::i128_saturating(*x),
            Self::Signed(x) => *x,
        }
    }

    /// truncate the Const to i bits saturating the underflow or overflow.
    fn truncate(&mut self, i: u8) {
        assert_ne!(i, 0);

        let is_neg = self.is_neg();
        match self {
            Self::Unsigned(x) => {
                let ulimit = (1u128 << i) - 1;
                if *x > ulimit {
                    *x = ulimit;
                }
            }
            Self::Signed(x) => {
                let ulimit = (1u128 << (i - 1)) - 1;
                if is_neg {
                    let u = !(*x as u128) + 1;
                    if u > ulimit {
                        *x = -(ulimit as i128) - 1;
                    }
                } else {
                    if *x as u128 > ulimit {
                        *x = ulimit as i128;
                    }
                }
            }
        }
    }

    pub fn apply(self, f: FieldSelector) -> Const {
        let mut x = self >> f.offset;
        x.truncate(f.length);
        x
    }

    pub fn from_str_radix(src: &str, radix: u32) -> Result<Self, ParseIntError> {
        if src.starts_with("-") || src.starts_with("+") {
            Ok(Self::Signed(i128::from_str_radix(src, radix)?))
        } else {
            Ok(Self::Unsigned(u128::from_str_radix(src, radix)?))
        }
    }
}

macro_rules! from_numeric_uimpl {
    ($i:ident) => {
        impl From<$i> for Const {
            fn from(x: $i) -> Self {
                Self::Unsigned(x as u128)
            }
        }
    };
}

macro_rules! from_numeric_simpl {
    ($i:ident) => {
        impl From<$i> for Const {
            fn from(x: $i) -> Self {
                Self::Signed(x as i128)
            }
        }
    };
}

from_numeric_uimpl!(u8);
from_numeric_uimpl!(u16);
from_numeric_uimpl!(u32);
from_numeric_uimpl!(u64);
from_numeric_uimpl!(usize);
from_numeric_uimpl!(u128);

from_numeric_simpl!(i8);
from_numeric_simpl!(i16);
from_numeric_simpl!(i32);
from_numeric_simpl!(i64);
from_numeric_simpl!(isize);
from_numeric_simpl!(i128);

macro_rules! from_const_impl {
    ($i:ident) => {
        impl From<Const> for $i {
            fn from(x: Const) -> Self {
                match x {
                    Const::Unsigned(x) => x as $i,
                    Const::Signed(x) => x as $i,
                }
            }
        }
    };
}

from_const_impl!(u8);
from_const_impl!(u16);
from_const_impl!(u32);
from_const_impl!(u64);
from_const_impl!(usize);
from_const_impl!(u128);

from_const_impl!(i8);
from_const_impl!(i16);
from_const_impl!(i32);
from_const_impl!(i64);
from_const_impl!(isize);
from_const_impl!(i128);

impl Add for Const {
    type Output = Self;

    fn add(self, rhs: Self) -> Self {
        match self {
            Self::Unsigned(x) => match rhs {
                Self::Unsigned(y) => Self::Unsigned(x.saturating_add(y)),
                Self::Signed(y) => Self::Signed(Self::i128_saturating(x).saturating_add(y)),
            },
            Self::Signed(x) => Self::Signed(x.saturating_add(rhs.as_i128_saturating())),
        }
    }
}

impl Sub for Const {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self {
        match self {
            Self::Unsigned(x) => match rhs {
                Self::Unsigned(y) => Self::Unsigned(x.saturating_sub(y)),
                Self::Signed(y) => Self::Signed(Self::i128_saturating(x).saturating_sub(y)),
            },
            Self::Signed(x) => Self::Signed(x.saturating_sub(rhs.as_i128_saturating())),
        }
    }
}

impl Mul for Const {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self {
        match self {
            Self::Unsigned(x) => match rhs {
                Self::Unsigned(y) => Self::Unsigned(x.saturating_mul(y)),
                Self::Signed(y) => Self::Signed(Self::i128_saturating(x).saturating_mul(y)),
            },
            Self::Signed(x) => Self::Signed(x.saturating_mul(rhs.as_i128_saturating())),
        }
    }
}

impl Div for Const {
    type Output = Self;

    fn div(self, rhs: Self) -> Self {
        match self {
            Self::Unsigned(x) => match rhs {
                Self::Unsigned(y) => Self::Unsigned(x / y),
                Self::Signed(y) => Self::Signed(Self::i128_saturating(x) / y),
            },
            Self::Signed(x) => Self::Signed(x / rhs.as_i128_saturating()),
        }
    }
}

impl Rem for Const {
    type Output = Self;

    fn rem(self, rhs: Self) -> Self {
        match self {
            Self::Unsigned(x) => match rhs {
                Self::Unsigned(y) => Self::Unsigned(x % y),
                Self::Signed(y) => Self::Signed(Self::i128_saturating(x) % y),
            },
            Self::Signed(x) => Self::Signed(x % rhs.as_i128_saturating()),
        }
    }
}

impl Neg for Const {
    type Output = Self;

    fn neg(self) -> Self {
        match self {
            Self::Unsigned(x) => Self::Signed(Self::neg_saturating(Self::i128_saturating(x))),
            Self::Signed(x) => Self::Signed(Self::neg_saturating(x)),
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
            Self::Unsigned(x) => Self::Unsigned(x & rhs.as_u128_bits()),
            Self::Signed(x) => Self::Signed((x as u128 & rhs.as_u128_bits()) as i128),
        }
    }
}

impl BitOr for Const {
    type Output = Self;

    fn bitor(self, rhs: Self) -> Self {
        match self {
            Self::Unsigned(x) => Self::Unsigned(x | rhs.as_u128_bits()),
            Self::Signed(x) => Self::Signed((x as u128 | rhs.as_u128_bits()) as i128),
        }
    }
}

impl BitXor for Const {
    type Output = Self;

    fn bitxor(self, rhs: Self) -> Self {
        match self {
            Self::Unsigned(x) => Self::Unsigned(x ^ rhs.as_u128_bits()),
            Self::Signed(x) => Self::Signed((x as u128 ^ rhs.as_u128_bits()) as i128),
        }
    }
}

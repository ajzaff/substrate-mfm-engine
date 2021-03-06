use crate::base::FieldSelector;
use std::cmp::{Eq, Ordering};
use std::num::ParseIntError;
use std::ops::{Add, BitAnd, BitOr, BitXor, Div, Mul, Neg, Rem, Shl, Shr, Sub};

const BIT_SIZE: u8 = 128;

#[derive(Copy, Clone, Debug)]
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

    pub fn bitscanforward(&self) -> u32 {
        match self {
            Self::Unsigned(x) => x.trailing_zeros(),
            Self::Signed(x) => x.trailing_zeros(),
        }
    }

    pub fn bitscanreverse(&self) -> u32 {
        match self {
            Self::Unsigned(x) => x.leading_zeros(),
            Self::Signed(x) => x.leading_zeros(),
        }
    }

    pub fn is_neg(&self) -> bool {
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

    pub fn abs(&self) -> Const {
        match self {
            Self::Unsigned(_) => *self,
            Self::Signed(x) => {
                if *x < 0 {
                    (0i128.saturating_sub(*x) as u128).into()
                } else {
                    (*x as u128).into()
                }
            }
        }
    }

    pub fn apply(self, f: &FieldSelector) -> Const {
        match self {
            Self::Unsigned(mut x) => {
                if f.length == 0 {
                    0u128.into()
                } else {
                    x <<= BIT_SIZE - f.offset - f.length;
                    x >>= BIT_SIZE - f.length;
                    x &= (1u128 << f.length) - 1;
                    x.into()
                }
            }
            Self::Signed(mut x) => {
                if f.length <= 1 {
                    0i128.into()
                } else {
                    x <<= BIT_SIZE - f.offset - f.length;
                    x >>= BIT_SIZE - f.length;
                    let sign = x & (1i128 << (f.offset + f.length - 1) as i128) != 0;
                    x &= (1i128 << f.length - 1) - 1;
                    if sign {
                        x = -x;
                    }
                    x.into()
                }
            }
        }
    }

    pub fn store(&mut self, x: Const, f: &FieldSelector) {
        let mut a = self.as_u128_bits();
        let mut mask = (1u128 << f.length) - 1;
        let mut b = x.as_u128_bits() & mask;
        mask <<= f.offset;
        b <<= f.offset;
        // From https://graphics.stanford.edu/~seander/bithacks.html#MaskedMerge.
        a ^= (a ^ b) & mask;
        match self {
            Self::Unsigned(x) => *x = a,
            Self::Signed(x) => *x = a as i128,
        }
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

from_numeric_uimpl!(u8);
from_numeric_uimpl!(u16);
from_numeric_uimpl!(u32);
from_numeric_uimpl!(u64);
from_numeric_uimpl!(usize);
from_numeric_uimpl!(u128);

macro_rules! from_numeric_simpl {
    ($i:ident) => {
        impl From<$i> for Const {
            fn from(x: $i) -> Self {
                Self::Signed(x as i128)
            }
        }
    };
}

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
            Self::Unsigned(x) => Self::Signed(0i128.saturating_sub(Self::i128_saturating(x))),
            Self::Signed(x) => Self::Signed(0i128.saturating_sub(x)),
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

impl PartialEq for Const {
    fn eq(&self, rhs: &Self) -> bool {
        match self {
            Self::Unsigned(x) => {
                if rhs.is_neg() {
                    false
                } else {
                    *x == rhs.as_u128_bits()
                }
            }
            Self::Signed(x) => match rhs {
                Self::Unsigned(y) => {
                    if *x < 0 {
                        false
                    } else {
                        *x as u128 == *y
                    }
                }
                Self::Signed(y) => *x == *y,
            },
        }
    }
}

impl Eq for Const {}

impl Ord for Const {
    fn cmp(&self, other: &Self) -> Ordering {
        match self {
            Self::Unsigned(x) => {
                if other.is_neg() {
                    Ordering::Greater
                } else {
                    x.cmp(&other.as_u128_bits())
                }
            }
            Self::Signed(x) => match other {
                Self::Unsigned(y) => {
                    if *x < 0 {
                        Ordering::Less
                    } else {
                        (*x as u128).cmp(y)
                    }
                }
                Self::Signed(y) => x.cmp(y),
            },
        }
    }
}

impl PartialOrd for Const {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_zero() {
        assert!(Const::Unsigned(0).is_zero());
        assert!(Const::Signed(0).is_zero());
        assert!(!Const::Unsigned(1).is_zero());
        assert!(!Const::Signed(1).is_zero());
        assert!(!Const::Signed(-1).is_zero());
    }

    #[test]
    fn test_count_ones() {
        assert_eq!(Const::Unsigned(0).count_ones(), 0);
        assert_eq!(Const::Signed(0).count_ones(), 0);
        assert_eq!(Const::Unsigned(1).count_ones(), 1);
        assert_eq!(Const::Signed(1).count_ones(), 1);
    }

    #[test]
    fn test_bitscanforward() {
        assert_eq!(Const::Unsigned(0).bitscanforward(), 128);
        assert_eq!(Const::Signed(0).bitscanforward(), 128);
        assert_eq!(Const::Unsigned(3).bitscanforward(), 0);
        assert_eq!(Const::Signed(3).bitscanforward(), 0);
    }

    #[test]
    fn test_bitscanreverse() {
        assert_eq!(Const::Unsigned(0).bitscanreverse(), 128);
        assert_eq!(Const::Signed(0).bitscanreverse(), 128);
        assert_eq!(Const::Unsigned(3).bitscanreverse(), 126);
        assert_eq!(Const::Signed(3).bitscanreverse(), 126);
    }

    #[test]
    fn test_is_neg() {
        assert!(!Const::Unsigned(0).is_neg());
        assert!(!Const::Unsigned(1).is_neg());
        assert!(!Const::Signed(0).is_neg());
        assert!(!Const::Signed(1).is_neg());
        assert!(Const::Signed(-1).is_neg());
    }

    #[test]
    fn test_abs() {
        assert_eq!(Const::Unsigned(0).abs(), Const::Unsigned(0));
        assert_eq!(Const::Unsigned(1).abs(), Const::Unsigned(1));
        assert_eq!(Const::Signed(0).abs(), Const::Unsigned(0));
        assert_eq!(Const::Signed(1).abs(), Const::Unsigned(1));
        assert_eq!(Const::Signed(-1).abs(), Const::Unsigned(1));
        assert_eq!(
            Const::Signed(-1 << 127).abs(),
            Const::Unsigned((1 << 127) - 1)
        );
    }

    #[test]
    fn test_apply_unsigned() {
        let mut x = Const::Unsigned(1).apply(&FieldSelector {
            offset: 0,
            length: 0,
        });
        assert_eq!(x, Const::Unsigned(0));

        x = Const::Unsigned(1).apply(&FieldSelector {
            offset: 0,
            length: 1,
        });
        assert_eq!(x, Const::Unsigned(1));

        x = Const::Unsigned(2).apply(&FieldSelector {
            offset: 0,
            length: 1,
        });
        assert_eq!(x, Const::Unsigned(0));

        x = Const::Unsigned(2).apply(&FieldSelector {
            offset: 0,
            length: 3,
        });
        assert_eq!(x, Const::Unsigned(2));

        x = Const::Unsigned(1 << 64).apply(&FieldSelector {
            offset: 0,
            length: 20,
        });
        assert_eq!(x, Const::Unsigned(0));

        x = Const::Unsigned(1208925819614629174706176).apply(&FieldSelector::TYPE); // type = 1
        assert_eq!(x, Const::Unsigned(1));
    }

    #[test]
    fn test_apply_signed_offset0() {
        let mut x = Const::Signed(-1).apply(&FieldSelector {
            offset: 0,
            length: 0,
        });
        assert_eq!(x, Const::Signed(0));

        x = Const::Signed(-1).apply(&FieldSelector {
            offset: 0,
            length: 1,
        });
        assert_eq!(x, Const::Signed(0));

        x = Const::Signed(2).apply(&FieldSelector {
            offset: 0,
            length: 1,
        });
        assert_eq!(x, Const::Signed(0));

        x = Const::Signed(2).apply(&FieldSelector {
            offset: 0,
            length: 3,
        });
        assert_eq!(x, Const::Signed(2));

        x = Const::Signed(1 << 64).apply(&FieldSelector {
            offset: 0,
            length: 20,
        });
        assert_eq!(x, Const::Signed(0));

        x = Const::Signed(-1).apply(&FieldSelector {
            offset: 0,
            length: 1,
        });
        assert_eq!(x, Const::Signed(0));

        x = Const::Signed(-2).apply(&FieldSelector {
            offset: 0,
            length: 1,
        });
        assert_eq!(x, Const::Signed(0));

        x = Const::Signed(-2).apply(&FieldSelector {
            offset: 0,
            length: 3,
        });
        assert_eq!(x, Const::Signed(-2));

        x = Const::Signed(-1 << 64).apply(&FieldSelector {
            offset: 0,
            length: 20,
        });
        assert_eq!(x, Const::Signed(0));
    }

    #[test]
    fn test_store_unsigned() {
        let mut x = Const::Unsigned(1);
        x.store(
            Const::Unsigned(0),
            &FieldSelector {
                offset: 0,
                length: 1,
            },
        );
        assert_eq!(x, Const::Unsigned(0));

        let mut x = Const::Unsigned(1);
        x.store(
            Const::Unsigned(1),
            &FieldSelector {
                offset: 1,
                length: 1,
            },
        );
        assert_eq!(x, Const::Unsigned(3));

        let mut x = Const::Unsigned(0b110101);
        x.store(
            Const::Unsigned(0b10101101),
            &FieldSelector {
                offset: 1,
                length: 4,
            },
        );
        assert_eq!(x, Const::Unsigned(0b111011));
    }
}

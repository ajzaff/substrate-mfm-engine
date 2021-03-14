pub mod arith;

use bitflags::bitflags;
use std::fmt;
use std::str::FromStr;

#[derive(Copy, Clone, Debug, PartialEq, PartialOrd)]
pub struct SiteNumber(pub u8);

impl fmt::Display for SiteNumber {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "#{}", self.0)
    }
}

#[derive(Copy, Clone, Debug, PartialEq, PartialOrd)]
pub struct FieldSelector {
    pub offset: u8,
    pub length: u8,
}

impl FieldSelector {
    pub const TYPE: Self = Self {
        offset: 80,
        length: 16,
    };
    pub const HEADER: Self = Self {
        offset: 71,
        length: 25,
    };
    pub const DATA: Self = Self {
        offset: 0,
        length: 71,
    };

    pub fn as_u16(&self) -> u16 {
        (self.offset as u16) | (self.length as u16) << 8
    }
}

impl From<u16> for FieldSelector {
    fn from(x: u16) -> Self {
        Self {
            offset: x as u8,
            length: (x >> 8) as u8,
        }
    }
}

bitflags! {
  pub struct Symmetries: u8 {
    const R000L = 0x1; // Normal.
    const R090L = 0x2;
    const R180L = 0x4; // Flip_XY.
    const R270L = 0x8;
    const R000R = 0x10; // Flip_Y.
    const R090R = 0x20; // Flip_X.
    const R180R = 0x40;
    const R270R = 0x80;
  }
}

impl FromStr for Symmetries {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "NONE" => Ok(0.into()),
            "R000L" => Ok(Symmetries::R000L),
            "R090L" => Ok(Symmetries::R090L),
            "R180L" => Ok(Symmetries::R180L),
            "R270L" => Ok(Symmetries::R270L),
            "R000R" => Ok(Symmetries::R000R),
            "R090R" => Ok(Symmetries::R090R),
            "R180R" => Ok(Symmetries::R180R),
            "R270R" => Ok(Symmetries::R270R),
            "ALL" => Ok(0xff.into()),
            _ => Err(()),
        }
    }
}
impl From<u8> for Symmetries {
    fn from(x: u8) -> Self {
        Self { bits: x }
    }
}

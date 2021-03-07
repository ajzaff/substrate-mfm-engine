pub mod arith;
pub mod op;

use arith::{I96, U96};
use bitflags::bitflags;
use byteorder::LittleEndian;
use byteorder::WriteBytesExt;
use std::fmt;
use std::io;
use std::str::FromStr;

#[derive(Copy, Clone, Debug, PartialEq, PartialOrd)]
pub struct SiteNumber(pub u8);

impl fmt::Display for SiteNumber {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "#{}", self.0)
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct FieldSelector {
    pub offset: u8,
    pub length: u8,
}

impl FieldSelector {
    pub const ALL: Self = Self {
        offset: 0,
        length: 96,
    };
}

impl fmt::Display for FieldSelector {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{{{}:{}}}", self.offset, self.length)
    }
}

bitflags! {
  pub struct Symmetries: u8 {
    const NONE  = 0x0;
    const R000L = 0x1; // Normal.
    const R090L = 0x2;
    const R180L = 0x4; // Flip_XY.
    const R270L = 0x8;
    const R000R = 0x10; // Flip_Y.
    const R090R = 0x20; // Flip_X.
    const R180R = 0x40;
    const R270R = 0x80;
    const ALL   = 0xff;
  }
}

impl FromStr for Symmetries {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "NONE" => Ok(Symmetries::NONE),
            "R000L" => Ok(Symmetries::R000L),
            "R090L" => Ok(Symmetries::R090L),
            "R180L" => Ok(Symmetries::R180L),
            "R270L" => Ok(Symmetries::R270L),
            "R000R" => Ok(Symmetries::R000R),
            "R090R" => Ok(Symmetries::R090R),
            "R180R" => Ok(Symmetries::R180R),
            "R270R" => Ok(Symmetries::R270R),
            "ALL" => Ok(Symmetries::ALL),
            _ => Err(()),
        }
    }
}

impl Symmetries {
    fn to_u8(&self) -> u8 {
        self.bits as u8
    }
}

#[derive(Copy, Clone, Debug)]
pub enum Const {
    Unsigned(U96),
    Signed(I96),
}

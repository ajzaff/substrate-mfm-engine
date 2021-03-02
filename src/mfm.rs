use bitflags::bitflags;
use std::fmt;

bitflags! {
  pub struct Symmetries: u8 {
    const NONE  = 0x0;
    const R000L = 0x1; // Normal.
    const R090L = 0x2;
    const R180L = 0x4; // Flip_XY.
    const R270L = 0x8;
    const R000R = 0x10; // Flip_Y.
    const R090R = 0x20;
    const R180R = 0x40; // Flip_X.
    const R270R = 0x80;
    const ALL   = 0xff;
  }
}

#[derive(Debug)]
pub struct ElementEntry<T>(pub &'static str, pub T);

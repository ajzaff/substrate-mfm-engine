use crate::base;
use crate::base::arith::U96;

#[derive(Clone, Debug)]
pub struct Metadata {
  pub name: String,
  pub symbol: String,
  pub descs: Vec<String>,
  pub authors: Vec<String>,
  pub licenses: Vec<String>,
  pub radius: u8,
  pub bg_color: String,
  pub fg_color: String,
  pub symmetries: base::Symmetries,
}

impl Metadata {
  pub fn new() -> Self {
    Self {
      name: "".to_string(),
      symbol: "".to_string(),
      descs: Vec::new(),
      authors: Vec::new(),
      licenses: Vec::new(),
      radius: 0,
      bg_color: "".to_string(),
      fg_color: "".to_string(),
      symmetries: base::Symmetries::R000L,
    }
  }
}

pub struct EventWindow {
  data: [U96; 41],
}

impl EventWindow {
  fn new() -> Self {
    Self { data: [U96(0); 41] }
  }
}

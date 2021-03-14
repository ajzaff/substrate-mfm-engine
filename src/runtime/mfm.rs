use crate::base;
use crate::base::arith::Const;
use std::collections::HashMap;

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
  pub field_map: HashMap<String, base::FieldSelector>,
  pub parameter_map: HashMap<String, Const>,
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
      field_map: HashMap::new(),
      parameter_map: HashMap::new(),
    }
  }
}

pub struct EventWindow {
  data: [Const; 41],
  paint: [u32; 41],
}

impl EventWindow {
  pub fn new() -> Self {
    Self {
      data: [0u128.into(); 41],
      paint: [0; 41],
    }
  }

  pub fn get(&self, i: usize) -> Option<&Const> {
    self.data.get(i)
  }

  pub fn get_mut(&mut self, i: usize) -> Option<&mut Const> {
    self.data.get_mut(i)
  }

  pub fn get_paint(&self, i: usize) -> Option<&u32> {
    self.paint.get(i)
  }

  pub fn get_paint_mut(&mut self, i: usize) -> Option<&mut u32> {
    self.paint.get_mut(i)
  }
}

use crate::lib::Element;

#[derive(Debug)]
pub struct Model<'a> {
  elems: &'a [&'a Element<'a>],
  bounds: (u16, u16),
}

struct State<'a> {
  elem: &'a Element<'a>,
  value: u64,
  ip: u64,
}

impl Model<'_> {
  pub fn new<'a>(elems: &'a [&'a Element<'a>], bounds: (u16, u16)) -> Model {
    Model {
      elems: elems,
      bounds: bounds,
    }
  }

  pub fn set_element(x: i8, y: i8, e: Element) {}

  pub fn step() {}
}

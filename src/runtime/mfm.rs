use crate::base::arith::U96;

pub struct EventWindow {
  data: [U96; 41],
}

impl EventWindow {
  fn new() -> Self {
    Self { data: [U96(0); 41] }
  }
}

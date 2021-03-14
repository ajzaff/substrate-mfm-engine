use crate::base;
use crate::base::arith::Const;
use crate::base::color::Color;
use lazy_static::lazy_static;
use std::collections::HashMap;
use std::fmt;

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

#[derive(Debug)]
pub struct EventWindow<'a> {
    data: [Const; 41],
    paint: [Color; 41],
    type_data: Option<&'a HashMap<u16, Metadata>>,
}

impl<'a> EventWindow<'a> {
    pub fn new_with_const(x: Const) -> Self {
        let mut ew = Self::new();
        ew.data[0] = x;
        ew
    }

    pub fn new() -> Self {
        Self {
            data: [0u128.into(); 41],
            paint: [0.into(); 41],
            type_data: None,
        }
    }

    pub fn get(&self, i: usize) -> Option<&Const> {
        self.data.get(i)
    }

    pub fn get_mut(&mut self, i: usize) -> Option<&mut Const> {
        self.data.get_mut(i)
    }

    pub fn get_paint(&self, i: usize) -> Option<&Color> {
        self.paint.get(i)
    }

    pub fn get_paint_mut(&mut self, i: usize) -> Option<&mut Color> {
        self.paint.get_mut(i)
    }
}

const VOID: char = ' ';
const EMPTY: char = '.';
const OCCUPIED: char = 'x';

impl fmt::Display for EventWindow<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        lazy_static! {
            static ref INDICES: [usize; 41] = [
                38, 31, 22, 33, 25, 15, 10, 17, 27, 29, 13, 5, 2, 7, 19, 35, 37, 21, 9, 1, 0, 4,
                12, 24, 40, 30, 14, 6, 3, 8, 20, 36, 26, 16, 11, 18, 28, 32, 23, 34, 39,
            ];
        }
        let mut s = String::new();
        let mut cols = 0;
        let mut idx = 0;

        macro_rules! print_row {
            ($cols:ident) => {
                for _ in 0..4 - $cols {
                    s.push(VOID);
                }
                for _ in 0..2 * $cols + 1 {
                    if let Some(x) = self.data.get(INDICES[idx]) {
                        if x.as_u128() == 0u128 {
                            s.push(EMPTY);
                        } else {
                            s.push(OCCUPIED);
                        }
                    }
                    idx += 1;
                }
                for _ in 0..4 - $cols {
                    s.push(VOID);
                }
            };
        }
        for _ in 0..4 {
            print_row!(cols);
            s.push('\n');
            cols += 1;
        }
        for _ in 4..9 {
            print_row!(cols);
            s.push('\n');
            cols -= 1;
        }

        write!(f, "{}", s)
    }
}

use crate::base;
use crate::base::arith::Const;
use crate::base::color;
use crate::base::FieldSelector;
use colored::*;
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
    pub bg_color: color::Color,
    pub fg_color: color::Color,
    pub symmetries: base::Symmetries,
    pub field_map: HashMap<String, base::FieldSelector>,
    pub parameter_map: HashMap<String, Const>,
}

const VOID: char = ' ';
const EMPTY: char = '.';
const OCCUPIED: char = 'x';
const UNKNOWN: char = '?';
const BAD: char = '!';

impl Metadata {
    pub fn new() -> Self {
        Self {
            name: "???".to_string(),
            symbol: "?".to_string(),
            descs: Vec::new(),
            authors: Vec::new(),
            licenses: Vec::new(),
            radius: 0,
            fg_color: 0xffffffffu32.into(),
            bg_color: 0u32.into(),
            symmetries: 0.into(),
            field_map: HashMap::new(),
            parameter_map: HashMap::new(),
        }
    }
}

#[derive(Debug)]
pub struct EventWindow<'input> {
    data: [Const; 41],
    paint: [color::Color; 41],
    type_map: Option<&'input HashMap<u16, Metadata>>,
}

impl<'input> EventWindow<'input> {
    pub fn new() -> Self {
        Self {
            data: [0.into(); 41],
            paint: [0.into(); 41],
            type_map: None,
        }
    }

    pub fn set_type_map(&mut self, type_map: &'input HashMap<u16, Metadata>) {
        self.type_map = Some(type_map)
    }

    pub fn get(&self, i: usize) -> Option<&Const> {
        self.data.get(i)
    }

    pub fn get_mut(&mut self, i: usize) -> Option<&mut Const> {
        self.data.get_mut(i)
    }

    pub fn swap(&mut self, i: usize, j: usize) {
        self.data.swap(i, j);
    }

    pub fn get_paint(&self, i: usize) -> Option<&color::Color> {
        self.paint.get(i)
    }

    pub fn get_paint_mut(&mut self, i: usize) -> Option<&mut color::Color> {
        self.paint.get_mut(i)
    }

    pub fn reset(&mut self) {
        self.data.iter_mut().for_each(|x| *x = 0.into());
        self.paint.iter_mut().for_each(|x| *x = 0.into());
    }
}

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
                        if let Some(type_map) = self.type_map {
                            let typ: u16 = x.apply(&FieldSelector::TYPE).into();
                            let meta = type_map.get(&typ);
                            if let Some(meta) = meta {
                                let (r, g, b) = meta.fg_color.components();
                                let (b_r, b_g, b_b) = meta.bg_color.components();
                                s.push_str(
                                    format!(
                                        "{}",
                                        meta.symbol.truecolor(r, g, b).on_truecolor(b_r, b_g, b_b)
                                    )
                                    .as_str(),
                                );
                            } else {
                                s.push(UNKNOWN);
                            }
                        } else {
                            if x.is_zero() {
                                s.push(EMPTY);
                            } else {
                                s.push(OCCUPIED);
                            }
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

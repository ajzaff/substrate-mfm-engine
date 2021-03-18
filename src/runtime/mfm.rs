use crate::base;
use crate::base::arith::Const;
use crate::base::color;
use crate::base::color::Color;
use crate::base::FieldSelector;
use colored::*;
use lazy_static::lazy_static;
use rand::Rng;
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

pub trait EventWindow {
    /// Reset moves the center of the event window to a new arbitrarily selected site.
    fn reset(&mut self);

    fn get(&self, i: usize) -> Option<&Const>;

    fn get_mut(&mut self, i: usize) -> Option<&mut Const>;

    fn swap(&mut self, i: usize, j: usize);

    fn get_paint(&self) -> Option<&color::Color>;

    fn get_paint_mut(&mut self) -> Option<&mut color::Color>;
}

pub struct MinimalEventWindow {
    data: [Const; 41],
    paint: [color::Color; 41],
}

impl MinimalEventWindow {
    pub fn new() -> Self {
        Self {
            data: [0.into(); 41],
            paint: [0.into(); 41],
        }
    }
}

impl EventWindow for MinimalEventWindow {
    fn reset(&mut self) {}

    fn get(&self, i: usize) -> Option<&Const> {
        self.data.get(i)
    }

    fn get_mut(&mut self, i: usize) -> Option<&mut Const> {
        self.data.get_mut(i)
    }

    fn swap(&mut self, i: usize, j: usize) {
        self.data.swap(i, j)
    }

    fn get_paint(&self) -> Option<&color::Color> {
        self.paint.get(0)
    }

    fn get_paint_mut(&mut self) -> Option<&mut color::Color> {
        self.paint.get_mut(0)
    }
}

lazy_static! {
    static ref WINDOW_INDICES: [usize; 41] = [
        38, 31, 22, 33, 25, 15, 10, 17, 27, 29, 13, 5, 2, 7, 19, 35, 37, 21, 9, 1, 0, 4, 12, 24,
        40, 30, 14, 6, 3, 8, 20, 36, 26, 16, 11, 18, 28, 32, 23, 34, 39,
    ];
    static ref WINDOW_OFFSETS: [(isize, isize); 41] = [
        (0, -4),
        (-1, -3),
        (0, -3),
        (1, -3),
        (-2, -2),
        (-1, -2),
        (0, -2),
        (1, -2),
        (2, -2),
        (-3, -1),
        (-2, -1),
        (-1, -1),
        (0, -1),
        (1, -1),
        (2, -1),
        (3, -1),
        (-4, 0),
        (-3, 0),
        (-2, 0),
        (-1, 0),
        (0, 0),
        (1, 0),
        (2, 0),
        (3, 0),
        (4, 0),
        (-3, 1),
        (-2, 1),
        (-1, 1),
        (0, 1),
        (1, 1),
        (2, 1),
        (3, 1),
        (-2, 2),
        (-1, 2),
        (0, 2),
        (1, 2),
        (2, 2),
        (-1, 3),
        (0, 3),
        (1, 3),
        (0, 4),
    ];
}

pub fn debug_event_window<T: EventWindow>(
    ew: &T,
    w: &mut std::io::Write,
    type_map: &HashMap<u16, Metadata>,
) -> std::io::Result<()> {
    let mut s = String::new();
    let mut cols = 0;
    let mut idx = 0;

    macro_rules! print_row {
        ($cols:ident) => {
            for _ in 0..4 - $cols {
                s.push(VOID);
            }
            for _ in 0..2 * $cols + 1 {
                if let Some(x) = ew.get(WINDOW_INDICES[idx]) {
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

    w.write_all(s.as_bytes())
}

#[derive(Copy, Clone, Debug)]
struct Bounds {
    pub width: usize,
    pub height: usize,
}

impl From<(usize, usize)> for Bounds {
    fn from(b: (usize, usize)) -> Self {
        Self {
            width: b.0,
            height: b.1,
        }
    }
}

pub struct DenseGridStorage<R: Rng> {
    data: Vec<Const>,
    paint: Vec<Color>,
    size: Bounds,
    scale: usize,
    rng: R,
}

impl<R: Rng> DenseGridStorage<R> {
    pub fn new(rng: R, size: (usize, usize)) -> Self {
        Self::with_scale(rng, 1, size)
    }

    pub fn with_scale(rng: R, scale: usize, size: (usize, usize)) -> Self {
        Self {
            data: vec![0.into(); size.0 * size.1], // TODO: fix the capacity
            paint: vec![0.into(); scale * size.0 * size.1], // TODO: fix the capacity
            size: size.into(),
            scale: scale,
            rng: rng,
        }
    }
}

impl<R: Rng> EventWindow for DenseGridStorage<R> {
    fn reset(&mut self) {}

    fn get(&self, i: usize) -> Option<&Const> {
        self.data.get(i)
    }

    fn get_mut(&mut self, i: usize) -> Option<&mut Const> {
        self.data.get_mut(i)
    }

    fn swap(&mut self, i: usize, j: usize) {
        self.data.swap(i, j);
    }

    fn get_paint(&self) -> Option<&color::Color> {
        self.paint.get(0)
    }

    fn get_paint_mut(&mut self) -> Option<&mut color::Color> {
        self.paint.get_mut(0)
    }
}

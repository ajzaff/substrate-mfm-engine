use crate::base;
use crate::base::arith::Const;
use crate::base::color;
use crate::base::color::Color;
use crate::base::FieldSelector;
use colored::*;
use lazy_static::lazy_static;
use log::trace;
use rand::RngCore;
use std::collections::HashMap;

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

    fn get(&self, i: usize) -> Const;

    fn get_mut(&mut self, i: usize) -> Option<&mut Const>;

    fn swap(&mut self, i: usize, j: usize);

    fn get_paint(&self) -> color::Color;

    fn get_paint_mut(&mut self) -> &mut color::Color;
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

    fn get(&self, i: usize) -> Const {
        self.data.get(i).map(|x| *x).unwrap_or(0.into())
    }

    fn get_mut(&mut self, i: usize) -> Option<&mut Const> {
        self.data.get_mut(i)
    }

    fn swap(&mut self, i: usize, j: usize) {
        self.data.swap(i, j)
    }

    fn get_paint(&self) -> color::Color {
        self.paint.get(0).map(|x| *x).unwrap()
    }

    fn get_paint_mut(&mut self) -> &mut color::Color {
        self.paint.get_mut(0).unwrap()
    }
}

lazy_static! {
    static ref WINDOW_OFFSETS: [(isize, isize); 41] = [
        /*  0 = */ (0, 0),
        /*  1 = */ (-1, 0),
        /*  2 = */ (0, -1),
        /*  3 = */ (0, 1),
        /*  4 = */ (1, 0),
        /*  5 = */ (-1, -1),
        /*  6 = */ (-1, 1),
        /*  7 = */ (1, -1),
        /*  8 = */ (1, 1),
        /*  9 = */ (-2, 0),
        /* 10 = */ (0, -2),
        /* 11 = */ (0, 2),
        /* 12 = */ (2, 0),
        /* 13 = */ (2, -1),
        /* 14 = */ (2, 1),
        /* 15 = */ (-1, -2),
        /* 16 = */ (-1, 2),
        /* 17 = */ (1, -2),
        /* 18 = */ (1, 2),
        /* 19 = */ (2, -1),
        /* 20 = */ (2, 1),
        /* 21 = */ (-3, 0),
        /* 22 = */ (0, -3),
        /* 23 = */ (0, 3),
        /* 24 = */ (3, 0),
        /* 25 = */ (-2, -2),
        /* 26 = */ (-2, 2),
        /* 27 = */ (2, -2),
        /* 28 = */ (2, 2),
        /* 29 = */ (-3, -1),
        /* 30 = */ (-3, 1),
        /* 31 = */ (-1, -3),
        /* 32 = */ (-1, 3),
        /* 33 = */ (1, -3),
        /* 34 = */ (1, 3),
        /* 35 = */ (3, -1),
        /* 36 = */ (3, 1),
        /* 37 = */ (-4, 0),
        /* 38 = */ (0, -4),
        /* 39 = */ (0, 4),
        /* 40 = */ (4, 0),
    ];
}

pub fn debug_event_window<T: EventWindow>(
    ew: &T,
    w: &mut std::io::Write,
    type_map: &HashMap<u16, Metadata>,
) -> std::io::Result<()> {
    lazy_static! {
        static ref PRINT_INDICES: [usize; 41] = [
            38, 31, 22, 33, 25, 15, 10, 17, 27, 29, 13, 5, 2, 7, 19, 35, 37, 21, 9, 1, 0, 4, 12,
            24, 40, 30, 14, 6, 3, 8, 20, 36, 26, 16, 11, 18, 28, 32, 23, 34, 39,
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
                let x = ew.get(PRINT_INDICES[idx]);
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

pub struct DenseGrid<'a, R: RngCore> {
    data: Vec<Const>,
    paint: Vec<Color>,
    size: Bounds,
    scale: usize,
    origin: usize,
    rng: &'a mut R,
}

impl<'a, R: RngCore> DenseGrid<'a, R> {
    pub fn new(rng: &'a mut R, size: (usize, usize)) -> Self {
        Self::with_scale(rng, 1, size)
    }

    pub fn with_scale(rng: &'a mut R, scale: usize, size: (usize, usize)) -> Self {
        Self {
            data: {
                let mut v = Vec::with_capacity(size.0 * size.1);
                (0..size.0 * size.1).for_each(|_| v.push(0.into()));
                v
            },
            paint: {
                let mut v = Vec::with_capacity(size.0 * size.1);
                (0..size.0 * size.1).for_each(|_| v.push(0.into()));
                v
            },
            size: size.into(),
            scale: scale,
            origin: rng.next_u64() as usize % (size.0 * size.1),
            rng: rng,
        }
    }
}

impl<R: RngCore> EventWindow for DenseGrid<'_, R> {
    fn reset(&mut self) {
        self.origin = self.rng.next_u64() as usize % self.data.len()
    }

    fn get(&self, i: usize) -> Const {
        if let Some(wi) = WINDOW_OFFSETS.get(i) {
            let i = (self.origin as isize) + wi.1 * self.size.width as isize + wi.0;
            if i >= 0 {
                return self.data.get(i as usize).map(|x| *x).unwrap_or(0.into());
            }
        }
        0.into()
    }

    fn get_mut(&mut self, i: usize) -> Option<&mut Const> {
        let wi = WINDOW_OFFSETS.get(i)?;
        let i = (self.origin as isize) + wi.1 * self.size.width as isize + wi.0;
        if i >= 0 {
            self.data.get_mut(i as usize)
        } else {
            None
        }
    }

    fn swap(&mut self, i: usize, j: usize) {
        let wi = WINDOW_OFFSETS.get(i);
        if wi == None {
            return;
        }
        let wj = WINDOW_OFFSETS.get(j);
        if wj == None {
            return;
        }
        let (w1, w2) = (wi.unwrap(), wj.unwrap());
        let i1 = (self.origin as isize) + w1.1 * self.size.width as isize + w1.0;
        if i1 < 0 {
            return;
        }
        let i2 = (self.origin as isize) + w2.1 * self.size.width as isize + w2.0;
        if i2 >= 0 {
            self.data.swap(i1 as usize, i2 as usize);
        }
    }

    fn get_paint(&self) -> color::Color {
        self.paint.get(self.origin).map(|x| *x).unwrap_or(0.into())
    }

    fn get_paint_mut(&mut self) -> &mut color::Color {
        self.paint.get_mut(self.origin).unwrap()
    }
}

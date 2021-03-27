use crate::base;
use crate::base::arith::Const;
use crate::base::color;
use crate::base::color::Color;
use crate::base::{FieldSelector, Symmetries};
use colored::*;
use image::RgbaImage;
use indexmap::map::Entry;
use indexmap::IndexMap;
use lazy_static::lazy_static;
use log::trace;
use rand;
use rand::RngCore;
use std::cmp::min;
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
    pub symmetries: Symmetries,
    pub field_map: HashMap<String, base::FieldSelector>,
    pub parameter_map: HashMap<String, Const>,
    pub type_num: u16,
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
            fg_color: 0xffffffff.into(),
            bg_color: 0.into(),
            symmetries: 0.into(),
            field_map: HashMap::new(),
            parameter_map: HashMap::new(),
            type_num: 0,
        }
    }

    pub fn new_atom(&self) -> Const {
        let mut a = Const::Unsigned(0);
        a.store(self.type_num.into(), &FieldSelector::TYPE);
        a
    }
}

pub trait EventWindow {
    /// Reset moves the center of the event window to a new arbitrarily selected site.
    fn reset(&mut self);

    fn get(&self, i: usize) -> Const;

    fn set(&mut self, i: usize, v: Const);

    fn swap(&mut self, i: usize, j: usize);

    fn get_paint(&self) -> color::Color;

    fn set_paint(&mut self, c: color::Color);
}

pub struct MinimalEventWindow<'a, R: RngCore> {
    data: [Const; 41],
    paint: [color::Color; 41],
    rng: &'a mut R,
}

impl<'a, R: RngCore> MinimalEventWindow<'a, R> {
    pub fn new(rng: &'a mut R) -> Self {
        Self {
            data: [0.into(); 41],
            paint: [0.into(); 41],
            rng: rng,
        }
    }
}

impl<R: RngCore> EventWindow for MinimalEventWindow<'_, R> {
    fn reset(&mut self) {}

    fn get(&self, i: usize) -> Const {
        self.data.get(i).map(|x| *x).unwrap_or(0.into())
    }

    fn set(&mut self, i: usize, v: Const) {
        if let Some(site) = self.data.get_mut(i) {
            *site = v;
        }
    }

    fn swap(&mut self, i: usize, j: usize) {
        let n = self.data.len();
        if i != j && i < n && j < n {
            self.data.swap(i, j)
        }
    }

    fn get_paint(&self) -> color::Color {
        *self.paint.get(0).unwrap_or(&0.into())
    }

    fn set_paint(&mut self, c: color::Color) {
        if let Some(color) = self.paint.get_mut(0) {
            *color = c;
        }
    }
}

pub trait Rand {
    fn rand_u32(&mut self) -> u32;
    fn rand(&mut self) -> Const;
}

impl<'a, R: RngCore> Rand for MinimalEventWindow<'a, R> {
    fn rand_u32(&mut self) -> u32 {
        self.rng.next_u32()
    }
    fn rand(&mut self) -> Const {
        let mut a: u128 = (self.rng.next_u64() as u128) << 64;
        a |= self.rng.next_u32() as u128;
        a.into()
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

fn offset_to_site(offset: &(isize, isize)) -> u8 {
    match offset {
        (0, 0) => 0,
        (-1, 0) => 1,
        (0, -1) => 2,
        (0, 1) => 3,
        (1, 0) => 4,
        (-1, -1) => 5,
        (-1, 1) => 6,
        (1, -1) => 7,
        (1, 1) => 8,
        (-2, 0) => 9,
        (0, -2) => 10,
        (0, 2) => 11,
        (2, 0) => 12,
        (2, -1) => 13,
        (2, 1) => 14,
        (-1, -2) => 15,
        (-1, 2) => 16,
        (1, -2) => 17,
        (1, 2) => 18,
        (2, -1) => 19,
        (2, 1) => 20,
        (-3, 0) => 21,
        (0, -3) => 22,
        (0, 3) => 23,
        (3, 0) => 24,
        (-2, -2) => 25,
        (-2, 2) => 26,
        (2, -2) => 27,
        (2, 2) => 28,
        (-3, -1) => 29,
        (-3, 1) => 30,
        (-1, -3) => 31,
        (-1, 3) => 32,
        (1, -3) => 33,
        (1, 3) => 34,
        (3, -1) => 35,
        (3, 1) => 36,
        (-4, 0) => 37,
        (0, -4) => 38,
        (0, 4) => 39,
        (4, 0) => 40,
        i => panic!("bad offset: {:?}", i),
    }
}

pub fn map_site(x: u8, s: Symmetries) -> u8 {
    if let Some(mut wo) = WINDOW_OFFSETS.get(x as usize) {
        let offset = match s {
            Symmetries::R000L => *wo,
            Symmetries::R090L => (wo.1, -wo.0),
            Symmetries::R180L => (-wo.0, wo.1),
            Symmetries::R270L => (wo.1, wo.0),
            Symmetries::R000R => (-wo.0, wo.1),
            Symmetries::R090R => (-wo.1, -wo.0),
            Symmetries::R180R => (wo.0, wo.1),
            Symmetries::R270R => (-wo.1, wo.0),
            i => panic!("map_site: bad symmetries: {:?}", i),
        };
        offset_to_site(&offset)
    } else {
        panic!("map_site: bad site: {}", x)
    }
}

pub fn select_symmetries(r: u32, s: Symmetries) -> Symmetries {
    if s.is_empty() {
        Symmetries::R000L
    } else {
        let i = s.bits().count_ones();
        if i == 1 {
            s
        } else {
            let mut v = s.bits();
            let mut z = 0;
            let mut x = r % i;

            for _ in 0..8 {
                let b = v.trailing_zeros();
                z += b;
                if x == 0 {
                    return (1u8 << z).into();
                } else {
                    z += 1;
                    x -= 1;
                    v >>= b + 1;
                }
            }

            unreachable!();
        }
    }
}

pub fn sample_symmetries<R: RngCore>(r: &mut R, s: Symmetries) -> Symmetries {
    select_symmetries(r.next_u32(), s)
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
                    let (r, g, b, _) = meta.fg_color.components();
                    let (b_r, b_g, b_b, _) = meta.bg_color.components();
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
                return *self.data.get(i as usize).unwrap_or(&0.into());
            }
        }
        0.into()
    }

    fn set(&mut self, i: usize, v: Const) {
        if let Some(wi) = WINDOW_OFFSETS.get(i) {
            let i = (self.origin as isize) + wi.1 * self.size.width as isize + wi.0;
            if i >= 0 {
                if let Some(site) = self.data.get_mut(i as usize) {
                    *site = v;
                }
            }
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
        let n = self.data.len() as isize;
        if i1 != i2 && i2 >= 0 && i1 < n && i2 < n {
            self.data.swap(i1 as usize, i2 as usize);
        }
    }

    fn get_paint(&self) -> color::Color {
        *self.paint.get(self.origin).unwrap_or(&0.into())
    }

    fn set_paint(&mut self, c: color::Color) {
        if let Some(color) = self.paint.get_mut(self.origin) {
            *color = c;
        }
    }
}

impl<'a, R: RngCore> Rand for DenseGrid<'a, R> {
    fn rand_u32(&mut self) -> u32 {
        self.rng.next_u32()
    }
    fn rand(&mut self) -> Const {
        let mut a: u128 = (self.rng.next_u64() as u128) << 64;
        a |= self.rng.next_u32() as u128;
        a.into()
    }
}

pub trait Blit {
    fn blit_image(&mut self, im: &RgbaImage);

    fn unblit_image(&self, im: &mut RgbaImage);
}

impl<R: RngCore> Blit for DenseGrid<'_, R> {
    fn blit_image(&mut self, im: &RgbaImage) {
        let (width, height) = im.dimensions();
        for x in 0..min(self.size.width, width as usize) {
            for y in 0..min(self.size.height, height as usize) {
                let pix = im.get_pixel(x as u32, y as u32);
                let mut c = (pix.0[0] as u32) << 24;
                c |= (pix.0[1] as u32) << 16;
                c |= (pix.0[2] as u32) << 8;
                c |= pix.0[3] as u32;
                self.paint[y * self.size.width + x] = c.into();
            }
        }
    }

    fn unblit_image(&self, im: &mut RgbaImage) {
        let (width, height) = im.dimensions();
        for x in 0..min(self.size.width, width as usize) {
            for y in 0..min(self.size.height, height as usize) {
                let (r, g, b, a) = self.paint[y * self.size.width + x].components();
                *im.get_pixel_mut(x as u32, y as u32) = [r, g, b, a].into();
            }
        }
    }
}

pub struct SparseGrid<'a, R: RngCore> {
    data: IndexMap<usize, Const>,
    paint: IndexMap<usize, Color>,
    size: Bounds,
    scale: usize,
    origin: usize,
    rng: &'a mut R,
}

impl<'a, R: RngCore> SparseGrid<'a, R> {
    pub fn new(rng: &'a mut R, size: (usize, usize)) -> Self {
        Self::with_scale(rng, 1, size)
    }

    pub fn with_scale(rng: &'a mut R, scale: usize, size: (usize, usize)) -> Self {
        Self {
            data: IndexMap::new(),
            paint: IndexMap::new(),
            size: size.into(),
            scale: scale,
            origin: rng.next_u64() as usize % (size.0 * size.1),
            rng: rng,
        }
    }
}

impl<R: RngCore> EventWindow for SparseGrid<'_, R> {
    fn reset(&mut self) {
        if self.data.len() > 0 {
            let i = self.rng.next_u64() as usize % self.data.len();
            if let Some((k, _)) = self.data.get_index(i) {
                self.origin = *k;
            }
        }
    }

    fn get(&self, i: usize) -> Const {
        if let Some(wi) = WINDOW_OFFSETS.get(i) {
            let i = (self.origin as isize) + wi.1 * self.size.width as isize + wi.0;
            if i >= 0 {
                return *self.data.get(&(i as usize)).unwrap_or(&0.into());
            }
        }
        0.into()
    }

    fn set(&mut self, i: usize, v: Const) {
        if let Some(wi) = WINDOW_OFFSETS.get(i) {
            let i = (self.origin as isize) + wi.1 * self.size.width as isize + wi.0;
            if i >= 0 {
                if v.is_zero() {
                    self.data.remove(&(i as usize));
                } else {
                    match self.data.entry(i as usize) {
                        Entry::Occupied(o) => *o.into_mut() = v,
                        Entry::Vacant(e) => {
                            e.insert(v);
                        }
                    }
                }
            }
        }
    }

    fn swap(&mut self, i: usize, j: usize) {
        let t = self.get(i);
        self.set(i, self.get(j));
        self.set(j, t);
    }

    fn get_paint(&self) -> color::Color {
        self.paint.get(&self.origin).map(|x| *x).unwrap_or(0.into())
    }

    fn set_paint(&mut self, c: color::Color) {
        if c.bits() == 0 {
            self.paint.remove(&self.origin);
        } else {
            match self.paint.entry(self.origin) {
                Entry::Occupied(o) => *o.into_mut() = c,
                Entry::Vacant(v) => {
                    v.insert(c);
                }
            }
        }
    }
}

impl<'a, R: RngCore> Rand for SparseGrid<'a, R> {
    fn rand_u32(&mut self) -> u32 {
        self.rng.next_u32()
    }
    fn rand(&mut self) -> Const {
        let mut a: u128 = (self.rng.next_u64() as u128) << 64;
        a |= self.rng.next_u32() as u128;
        a.into()
    }
}

impl<R: RngCore> Blit for SparseGrid<'_, R> {
    fn blit_image(&mut self, im: &RgbaImage) {
        let (width, height) = im.dimensions();
        for x in 0..min(self.size.width, width as usize) {
            for y in 0..min(self.size.height, height as usize) {
                let pix = im.get_pixel(x as u32, y as u32);
                let mut c = (pix.0[0] as u32) << 24;
                c |= (pix.0[1] as u32) << 16;
                c |= (pix.0[2] as u32) << 8;
                c |= pix.0[3] as u32;
                match self.paint.entry(y * self.size.width + x) {
                    Entry::Occupied(o) => *o.into_mut() = c.into(),
                    Entry::Vacant(v) => *v.insert(0.into()) = c.into(),
                }
            }
        }
    }

    fn unblit_image(&self, im: &mut RgbaImage) {
        let (width, height) = im.dimensions();
        for x in 0..min(self.size.width, width as usize) {
            for y in 0..min(self.size.height, height as usize) {
                if let Some(c) = self.paint.get(&(y * self.size.width + x)) {
                    let (r, g, b, a) = c.components();
                    *im.get_pixel_mut(x as u32, y as u32) = [r, g, b, a].into();
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sample_none_symmetries() {
        let mut rng = rand::rngs::mock::StepRng::new(0, 1);
        assert_eq!(sample_symmetries(&mut rng, 0.into()), Symmetries::R000L);
    }

    #[test]
    fn test_sample_one_symmetries() {
        let mut rng = rand::rngs::mock::StepRng::new(0, 1);
        assert_eq!(
            sample_symmetries(&mut rng, Symmetries::R090L),
            Symmetries::R090L
        );
        assert_eq!(
            sample_symmetries(&mut rng, Symmetries::R090R),
            Symmetries::R090R
        );
        assert_eq!(
            sample_symmetries(&mut rng, Symmetries::R270R),
            Symmetries::R270R
        );
    }

    #[test]
    fn test_sample_some_symmetries() {
        let mut rng = rand::rngs::mock::StepRng::new(0, 1);
        assert_eq!(
            sample_symmetries(
                &mut rng,
                Symmetries::R000L | Symmetries::R090L | Symmetries::R180L
            ),
            Symmetries::R000L
        );
        assert_eq!(
            sample_symmetries(&mut rng, Symmetries::R180L | Symmetries::R180R),
            Symmetries::R180R
        );
        assert_eq!(
            sample_symmetries(&mut rng, Symmetries::R000R | Symmetries::R090R),
            Symmetries::R000R
        );
    }

    #[test]
    fn test_sample_all_symmetries() {
        let mut rng = rand::rngs::mock::StepRng::new(0, 1);
        assert_eq!(sample_symmetries(&mut rng, 255.into()), Symmetries::R000L);
        assert_eq!(sample_symmetries(&mut rng, 255.into()), Symmetries::R090L);
        assert_eq!(sample_symmetries(&mut rng, 255.into()), Symmetries::R180L);
        assert_eq!(sample_symmetries(&mut rng, 255.into()), Symmetries::R270L);
        assert_eq!(sample_symmetries(&mut rng, 255.into()), Symmetries::R000R);
        assert_eq!(sample_symmetries(&mut rng, 255.into()), Symmetries::R090R);
        assert_eq!(sample_symmetries(&mut rng, 255.into()), Symmetries::R180R);
        assert_eq!(sample_symmetries(&mut rng, 255.into()), Symmetries::R270R);
    }
}

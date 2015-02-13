use ext_color::Hsv;
use ext_color::ToRgb;
use ext_color::Color3;


/// A `Color` is any object that can be converted to a length-4 array of f32s.
///
/// The values of these floats range from 0.0 to 1.0 and represent [r, g, b, a].
pub trait Color {
    fn to_rgba(self) -> [f32; 4];
}

pub trait NormalizeTo1: Sized {
    fn norm(&self) -> f32;
}

pub fn hex_rgb(mut v: u32) -> [f32; 4] {
    let b = v & 0xff;
    v = v << 4;
    let g = v & 0xff;
    v = v << 4;
    let r = v & 0xff;
    [r, g, b].to_rgba()
}

pub fn hex_rgba(mut v: u32) -> [f32; 4] {
    let a = v & 0xff;
    v = v << 4;
    let b = v & 0xff;
    v = v << 4;
    let g = v & 0xff;
    v = v << 4;
    let r = v & 0xff;
    [r, g, b, a].to_rgba()
}

pub fn rgb<T: NormalizeTo1>(r: T, g: T, b: T) -> [f32; 4] {
    [r, g, b].to_rgba()
}

pub fn rgba<T: NormalizeTo1>(r: T, g: T, b: T, a: T) -> [f32; 4] {
    [r, g, b, a].to_rgba()
}

pub fn hsv<T: NormalizeTo1>(h: T, s: T, v: T) -> [f32; 4] {
    let v: [f32; 3] = Hsv::new(h.norm(), s.norm(), v.norm()).to_rgb().into_fixed();
    v.to_rgba()
}

pub fn hsva<T: NormalizeTo1>(h: T, s: T, v: T, a: T) -> [f32; 4] {
    let rgb = Hsv::new(h.norm(), s.norm(), v.norm()).to_rgb().into_fixed();
    rgba(rgb[0], rgb[1], rgb[2], a.norm())
}

impl <T: NormalizeTo1> Color for [T; 4] {
    fn to_rgba(self) -> [f32; 4] {
        match self {
            [ref r, ref g, ref b, ref a] => [r.norm(), g.norm(), b.norm(), a.norm()]
        }
    }
}

impl <T: NormalizeTo1> Color for [T; 3] {
    fn to_rgba(self) -> [f32; 4] {
        match self {
            [ref r, ref g, ref b] => [r.norm(), g.norm(), b.norm(), 1.0]
        }
    }
}

impl NormalizeTo1 for u8 {
    fn norm(&self) -> f32 {
        *self as f32 / 255.0
    }
}

impl NormalizeTo1 for u16 {
    fn norm(&self) -> f32 {
        *self as f32 / 255.0
    }
}

impl NormalizeTo1 for u32 {
    fn norm(&self) -> f32 {
        *self as f32 / 255.0
    }
}

impl NormalizeTo1 for u64 {
    fn norm(&self) -> f32 {
        *self as f32 / 255.0
    }
}

impl NormalizeTo1 for i8 {
    fn norm(&self) -> f32 {
        *self as f32 / 255.0
    }
}

impl NormalizeTo1 for i16 {
    fn norm(&self) -> f32 {
        *self as f32 / 255.0
    }
}

impl NormalizeTo1 for i32 {
    fn norm(&self) -> f32 {
        *self as f32 / 255.0
    }
}

impl NormalizeTo1 for i64 {
    fn norm(&self) -> f32 {
        *self as f32 / 255.0
    }
}

impl NormalizeTo1 for f32 {
    fn norm(&self) -> f32 {
        *self
    }
}

impl NormalizeTo1 for f64 {
    fn norm(&self) -> f32 {
        *self as f32
    }
}

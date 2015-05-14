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

pub fn hsv<T>(h: T, s: T, v: T) -> [f32; 4] where T: NormalizeTo1 {
    let h = h.norm();
    let s = s.norm();
    let v = v.norm();

    let chr = v * s;
    let h = h / 60.0;

    // the 2nd largest component
    let x = chr * (1.0 - ((h % 2.0f32) - 1.0).abs());

    let mut rgb =
        if      h < 1.0 { (chr, x, 0.0) }
        else if h < 2.0 { (x, chr, 0.0) }
        else if h < 3.0 { (0.0, chr, x) }
        else if h < 4.0 { (0.0, x, chr) }
        else if h < 5.0 { (x, 0.0, chr) }
        else if h < 6.0 { (chr, 0.0, x) }
        else            { (0.0, 0.0, 0.0)       };

    // match the value by adding the same amount to each component
    let mn = v - chr;

    rgb.0 = rgb.0 + mn;
    rgb.1 = rgb.1 + mn;
    rgb.2 = rgb.2 + mn;

    [rgb.0, rgb.1, rgb.2, 1.0]
}

pub fn hsva<T: NormalizeTo1>(h: T, s: T, v: T, a: T) -> [f32; 4] {
    let mut r = hsv(h, s, v);
    r[3] = a.norm();
    r
}

impl <T: NormalizeTo1> Color for [T; 4] {
    fn to_rgba(self) -> [f32; 4] {
        [self[0].norm(), self[1].norm(), self[2].norm(), self[3].norm()]
    }
}

impl <T: NormalizeTo1> Color for [T; 3] {
    fn to_rgba(self) -> [f32; 4] {
        [self[0].norm(), self[1].norm(), self[2].norm(), 1.0]
    }
}

impl <T: NormalizeTo1> Color for (T, T, T, T) {
    fn to_rgba(self) -> [f32; 4] {
        [self.0.norm(), self.1.norm(), self.2.norm(), self.3.norm()]
    }
}

impl <T: NormalizeTo1> Color for (T, T, T) {
    fn to_rgba(self) -> [f32; 4] {
        [self.0.norm(), self.1.norm(), self.2.norm(), 1.0]
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

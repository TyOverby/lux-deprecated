/// A `Color` is any object that can be converted to a length-4 array of f32s.
///
/// The values of these floats range from 0.0 to 1.0 and represent [r, g, b, a].
pub trait Color {
    /// Converts this object to a length-4 array of floats.
    fn to_rgba(self) -> [f32; 4];
}

/// A number that can be converted to a floating point number within the range
/// `0.0` to `1.0`
pub trait ToColorComponent {
    fn norm(&self) -> f32;
}

/// Convertes a u32 to a color by treating the last 3 bytes as rgb tripples.
///
/// ```ignore rust
/// hex_rgb(0xFF5500); // rgb(0xFF, 0x55, 0x00)
/// ```
pub fn hex_rgb(mut v: u32) -> [f32; 4] {
    let b = v & 0xff;
    v = v << 4;
    let g = v & 0xff;
    v = v << 4;
    let r = v & 0xff;
    [r as u8, g as u8, b as u8].to_rgba()
}

/// Convertes a u32 to a color by treating the 4 bytes as rgb tripples.
///
/// ```ignore rust
/// hex_rgb(0xFF5500AA); // rgba(0xFF, 0x55, 0x00, 0xAA)
/// ```
pub fn hex_rgba(mut v: u32) -> [f32; 4] {
    let a = v & 0xff;
    v = v << 4;
    let b = v & 0xff;
    v = v << 4;
    let g = v & 0xff;
    v = v << 4;
    let r = v & 0xff;
    [r as u8, g as u8, b as u8, a as u8].to_rgba()
}

/// Constructs a color from R, G, and B components.
///
/// Alpha is set to 100%.
///
/// If the numbers are u8s the scale is from `0` to `255`.  If the numbers
/// are floating point, the scale is from `0.0` to `1.0`.
pub fn rgb<T: ToColorComponent>(r: T, g: T, b: T) -> [f32; 4] {
    [r, g, b].to_rgba()
}

/// Constructs a color from R, G, B, and A components.
///
/// If the numbers are u8s the scale is from `0` to `255`.  If the numbers
/// are floating point, the scale is from `0.0` to `1.0`.
pub fn rgba<T: ToColorComponent>(r: T, g: T, b: T, a: T) -> [f32; 4] {
    [r, g, b, a].to_rgba()
}


/// Constructs a color from Hue, Saturation and Value components.
///
/// `h` is in the range of 0.0 to 360.0.  `s` and `v` are in the range of
/// `0.0` to `1.0`.
pub fn hsv(h: f32, s: f32, v: f32) -> [f32; 4] {
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

/// Same as `hsv` but with an alpha component.
///
/// `a` is in the range of `0.0` to `1.0`.
pub fn hsva(h: f32, s: f32, v: f32, a: f32) -> [f32; 4] {
    let mut r = hsv(h, s, v);
    r[3] = a.norm();
    r
}

impl <T: ToColorComponent> Color for [T; 4] {
    fn to_rgba(self) -> [f32; 4] {
        [self[0].norm(), self[1].norm(), self[2].norm(), self[3].norm()]
    }
}

impl <T: ToColorComponent> Color for [T; 3] {
    fn to_rgba(self) -> [f32; 4] {
        [self[0].norm(), self[1].norm(), self[2].norm(), 1.0]
    }
}

impl <T: ToColorComponent> Color for (T, T, T, T) {
    fn to_rgba(self) -> [f32; 4] {
        [self.0.norm(), self.1.norm(), self.2.norm(), self.3.norm()]
    }
}

impl <T: ToColorComponent> Color for (T, T, T) {
    fn to_rgba(self) -> [f32; 4] {
        [self.0.norm(), self.1.norm(), self.2.norm(), 1.0]
    }
}

impl ToColorComponent for u8 {
    fn norm(&self) -> f32 {
        *self as f32 / 255.0
    }
}

impl ToColorComponent for i32 {
    fn norm(&self) -> f32 {
        *self as f32 / 255.0
    }
}

impl ToColorComponent for f32 {
    fn norm(&self) -> f32 {
        *self
    }
}

impl ToColorComponent for f64 {
    fn norm(&self) -> f32 {
        *self as f32
    }
}

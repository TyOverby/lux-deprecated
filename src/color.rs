/// A `Color` is any object that can be converted to a length-4 array of f32s.
///
/// The values of these floats range from 0.0 to 1.0 and represent [r, g, b, a].
pub trait Color {
    fn to_rgba(self) -> [f32; 4];
}

impl Color for [f32; 4] {
    fn to_rgba(self) -> [f32; 4] {
        self
    }
}

impl Color for [f32; 3] {
    fn to_rgba(self) -> [f32; 4] {
        match self {
            [r, g, b] => [r, g, b, 1.0]
        }
    }
}

pub trait Color {
    fn to_rgba(self) -> [f32, ..4];
}

impl Color for [f32, ..4] {
    fn to_rgba(self) -> [f32, ..4] {
        self
    }
}

impl Color for [f32, ..3] {
    fn to_rgba(self) -> [f32, ..4] {
        match self {
            [r,g,b] => [r,g,b,1.0]
        }
    }
}

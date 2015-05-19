extern crate lux;

use lux::prelude::*;
use lux::color;
use std::path::Path;

fn main() {
    let mut lux = Window::new().unwrap();

    lux.load_font("Pacifico", &Path::new("./resources/Pacifico.ttf")).unwrap();
    lux.load_font("cbt", &Path::new("./resources/cbt.ttf")).unwrap();

    while lux.is_open() {
        let mut frame = lux.cleared_frame(color::WHITE);
        let mut y = 0.0;

        for pt in 1 .. 6 {
            frame.text("Hello World", 0.0, y)
                  .size(pt * 10)
                  .font("Pacifico")
                  .draw().unwrap();
            y += (pt * 15) as f32;
        }

        for pt in 1 .. 6 {
            frame.text("Hello World", 0.0, y)
                  .size(pt * 10)
                  .font("cbt")
                  .draw().unwrap();
            y += (pt * 15) as f32;
        }

        for pt in 1 .. 6 {
            frame.text("Hello World", 0.0, y)
                  .size(pt * 10)
                  .font("SourceCodePro")
                  .draw().unwrap();
            y += (pt * 15) as f32;
        }

    }
}

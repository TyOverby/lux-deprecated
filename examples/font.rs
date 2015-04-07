extern crate lux;
extern crate glium;
extern crate image;
extern crate freetype;

use lux::prelude::*;
use std::path::Path;

fn main() {
    let mut lux = Window::new().unwrap();

    lux.load_font("Pacifico", &Path::new("./resources/Pacifico.ttf")).unwrap();
    lux.load_font("cbt", &Path::new("./resources/cbt.ttf")).unwrap();

    while lux.is_open() {
        let mut frame = lux.cleared_frame(colors::WHITE);
        let mut y = 0.0;

        for pt in 1 .. 6 {
            frame.set_font("Pacifico", pt * 10).unwrap();
            frame.draw_text("Hello World", 0.5, y + 0.5).unwrap();
            y += (pt * 15) as f32;
        }

        for pt in 1 .. 6 {
            frame.set_font("cbt", pt * 10).unwrap();
            frame.draw_text("Hello World", 0.5, y + 0.5).unwrap();
            y += (pt * 15) as f32;
        }

        for pt in 1 .. 6 {
            frame.set_font("SourceCodePro", pt * 10).unwrap();
            frame.draw_text("Hello World", 0.5, y + 0.5).unwrap();
            y += (pt * 15) as f32;
        }

    }
}

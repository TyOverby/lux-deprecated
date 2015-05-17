/*
extern crate lux;
extern crate glium;
extern crate image;
extern crate freetype;

use lux::prelude::*;
use lux::color;
use lux::loader::*;
use std::path::Path;

fn main() {
    let lux = Window::new().unwrap();

    let mut loader = GraphicalLoader::new(lux, ());

    loader.load("Loading font `Pacifico`", |window, _| {
        window.load_font("Pacifico", &Path::new("./resources/Pacifico.ttf")).unwrap();
    });

    loader.load("Loading font `cbt`", |window, _| {
        window.load_font("cbt", &Path::new("./resources/cbt.ttf")).unwrap();
    });

    for font in vec!["Pacifico", "cbt", "SourceCodePro"] {
        for size in 1 ..6 {
            let mp = format!("Rendering font {} at size {}", font, size * 10);
            loader.load(mp, move |window, _| {
                window.preload_font(font, size * 10).unwrap();
            });
        }
    }

    let (mut lux, _) = loader.run();


    while lux.is_open() {
        let mut frame = lux.cleared_frame(color::WHITE);
        let mut y = 0.0;

        for pt in 1 .. 6 {
            frame.text("Hello World", 0.0, y)
                 .font("Pacifico")
                 .size(pt * 10)
                 .draw().unwrap();
            y += (pt * 15) as f32;
        }

        for pt in 1 .. 6 {
            frame.text("Hello World", 0.0, y)
                 .font("cbt")
                 .size(pt * 10)
                 .draw().unwrap();
            y += (pt * 15) as f32;
        }

        for pt in 1 .. 6 {
            frame.text("Hello World", 0.0, y)
                 .font("SourceCodePro")
                 .size(pt * 10)
                 .draw().unwrap();
            y += (pt * 15) as f32;
        }

    }
}
*/

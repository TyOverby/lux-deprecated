#![allow(unstable)]
extern crate lux;
extern crate glium;
extern crate image;

use lux::*;
use std::path::Path;

fn main() {
    let mut lux = Window::new().unwrap();

    // A sprite made from an image
    let texture = lux.load_sprite(&Path::new("./test/minecraft_fixedwidth_font.png")).unwrap();
    let sheet   = texture.as_sprite_sheet(16, 16);

    while lux.is_open() {
        let mut frame = lux.cleared_frame(colors::WHITE);
        let (x, y) = lux.mouse_pos();

        let s1 = sheet.get(1, 1);

        frame.draw_sprite(&s1, (0.0, 0.0), s1.ideal_size());
    }
}

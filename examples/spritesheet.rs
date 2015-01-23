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
    let rgb_test = lux.sprite_from_pixels(
        vec![vec![colors::RED, colors::BLUE],
             vec![colors::GREEN, colors::BLACK]]);

    while lux.is_open() {
        let mut frame = lux.cleared_frame(colors::BLACK);
        let (x, y) = lux.mouse_pos();

        let s1 = sheet.get(1, 4);

        frame.draw_sprite(&texture, (0.0, 0.0), texture.ideal_size());
        //frame.draw_sprite(&rgb_test, (0.0, 0.0), (100.0, 100.0));
        frame.draw_sprite(&s1, (x as f32, y as f32), s1.ideal_size());
    }
}

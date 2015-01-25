#![allow(unstable)]
extern crate lux;
extern crate glium;
extern crate image;

use lux::*;
use std::path::Path;

fn main() {
    let mut lux = Window::new().unwrap();

    // A full sprite
    let sprite1 = lux.load_sprite(&Path::new("./test/test.png")).unwrap();
    // A sprite made by chopping off parts of the other one.
    let sprite2 = sprite1.sub_sprite((0, 0), (256 / 2, 255)).unwrap();

    while lux.is_open() {
        let mut frame = lux.cleared_frame(colors::WHITE);
        let (x, y) = lux.mouse_pos();

        frame.draw_sprite(&sprite1, (0.0, 0.0), sprite1.ideal_size());
        frame.draw_sprite(&sprite2, (x as f32, y as f32), sprite2.ideal_size());
    }
}

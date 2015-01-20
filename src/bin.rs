#![allow(unstable)]
extern crate lux;
extern crate glium;
extern crate image;

use lux::*;
use std::path::Path;

fn main() {
    let mut lux = Window::new().unwrap();

    let texture = lux.load_sprite(&Path::new("./test.png")).unwrap();

    while lux.is_open() {
        let mut frame = lux.cleared_frame(colors::WHITE);
        let (x, y) = lux.mouse_pos();
        frame.draw_sprite(&texture, (x as f32, y as f32), (255.0, 255.0));
    }
}

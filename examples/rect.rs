extern crate lux;

use lux::prelude::*;
use lux::color;

fn main() {
    let mut window = Window::new_with_defaults().unwrap();
    while window.is_open() {
        let mut frame = window.cleared_frame(rgb(0.9, 0.9, 0.9));
        frame.rect(10.0, 20.0, 15.0, 25.0).fill();
        frame.with_color(color::BLUE, |frame| {
            frame.rect(20.0, 10.0, 10.0, 10.0).fill();
            frame.rect(50.0, 10.0, 10.0, 10.0).border(3.0, rgb(255, 0, 0)).fill_and_stroke()
        });
    }
}

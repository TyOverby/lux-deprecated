extern crate lux;
extern crate num;

use lux::prelude::*;
use lux::color;

use num::Float;

fn main() {
    let mut window = Window::new().unwrap();
    let mut theta:f32 = 0.0;
    while window.is_open() {
        let mut frame = window.cleared_frame(rgb(0.9, 0.9, 0.9));
        theta += 0.01;

        let size = 10.0;
        let dist = (2.0 * size * size).sqrt();

        frame.rotate(0.3);

        frame.with_scissor(200, 200, 500, 500, |frame| {
            for x in 0 .. 100 {
                for y in 0 .. 100 {
                    let x = x as f32 * dist;
                    let y = y as f32 * dist;
                    frame.rect(x, y, size, size)
                       .rotate_around((size / 2.0, size / 2.0), theta)
                       .color(color::RED)
                       .fill();
                }
            }
        });
    }
}

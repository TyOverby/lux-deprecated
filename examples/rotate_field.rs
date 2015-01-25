extern crate lux;
use lux::*;

use std::num::Float;

fn main() {
    let mut window = Window::new().unwrap();
    let mut theta:f32 = 0.0;
    while window.is_open() {
        let mut frame = window.cleared_frame(rgb(0.9, 0.9, 0.9));
        theta += 0.01;

        let size = 10.0;
        let dist = (2.0*size*size).sqrt();

        for x in 0u32..100 {
            for y in 0u32..100 {
                let x = x as f32 * dist;
                let y = y as f32 * dist;
                frame.rect(x, y, size, size)
                   .rotate_around((5.0, 5.0), theta)
                   .fill_color(colors::RED)
                   .fill();
            }
        }
    }
}

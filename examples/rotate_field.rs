extern crate lux;
use lux::*;

use std::num::Float;

fn main() {
    let mut window = Window::new().unwrap();
    let mut theta:f32 = 0.0;
    while window.is_open() {
        let mut frame = window.cleared_frame([0.9, 0.9, 0.9]);
        theta += 0.01;

        let size = 10.0;
        let dist = (2.0*size*size).sqrt();

        for x in range(0u, 100) {
            for y in range(0u, 100) {
                let x = x as f32 * dist;
                let y = y as f32 * dist;
                frame.draw(
                    rect((x, y), (size, size))
                       .fill_color(colors::BLUE)
                       .rotate_around((5.0, 5.0), theta))
            }
        }
    }
}

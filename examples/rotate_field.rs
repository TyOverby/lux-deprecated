extern crate lux;

use lux::{LuxCanvas, LuxWindow, Window, StackedTransform, Colored, Transform};
use lux::colors;
use std::num::Float;

fn main() {
    let mut lux = Window::new().unwrap();
    let mut theta:f32 = 0.0;
    while lux.is_open() {
        lux.clear([0.9, 0.9, 0.9]);
        theta += 0.01;

        let size = 10.0;
        let dist = (2.0*size*size).sqrt();

        for x in range(0u, 100) {
            for y in range(0u, 100) {
                let x = x as f32 * dist;
                let y = y as f32 * dist;
                lux.rect((x, y), (size, size))
                   .rotate_around((5.0, 5.0), theta)
                   .fill_color(colors::RED)
                   .fill();
            }
        }

        lux.render();
    }
}

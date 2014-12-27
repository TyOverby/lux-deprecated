extern crate lux;

use lux::{LuxCanvas, LuxWindow, Window, StackedTransform, Colored, Transform};
use lux::colors;
use std::num::Float;

fn main() {
    let mut lux = Window::new().unwrap();
    let mut delta = 0.0f32;

    while lux.is_open() {
        lux.clear([1.0, 0.0, 0.0, 0.1]);
        delta += 0.1;
        println!("{}", delta);

        for x in range(0u, 100) {
            for y in range(0u, 100) {
                let (x, y) = (x as f32 * 40.0, y as f32 * 40.0);
                lux.rect((x, y), (30.0, 30.0))
                   .fill_color(colors::BLUE)
                   .fill();
            }
        }


        lux.render();
    }
}

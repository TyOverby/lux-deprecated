extern crate lux;
extern crate noise;
extern crate nd_iter;

use lux::{Window, LuxCanvas, Interactive, colors};
use nd_iter::iter_2d;

const DIV: f32 = 20.0;

fn main() {
    let mut window = Window::new().unwrap();

    let seed = noise::Seed::new(0);
    let mut z = 0.0;

    while window.is_open() {
        z += 1.0;
        let mut frame = window.frame(colors::RED);

        frame.draw_pixels(
            iter_2d(range(0, 256), range(0, 245)).map(|(x, y)| {
                let (x, y) = (x as f32, y as f32);
                let value = noise::simplectic3(&seed, &[x / DIV, y / DIV, z / DIV]);
                let value = (value + 1.0) / 2.0;

                ((x, y), [value, value, value])
            })
        );
    }
}

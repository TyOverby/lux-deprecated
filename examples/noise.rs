extern crate lux;
extern crate noise;
extern crate nd_iter;

use lux::prelude::*;
use lux::graphics::ColorVertex;
use lux::color;
use nd_iter::iter_2d;

const DIV: f32 = 20.0;

fn main() {
    let mut window = Window::new().unwrap();

    let seed = noise::Seed::new(0);
    let mut z = 0.0;


    let mut points: Vec<_> = iter_2d(0u32..256, 0u32..256).map(|(x, y)| {
        let (x, y) = (x as f32, y as f32);
        ColorVertex {
            pos: [x, y],
            color: rgb(0, 0, 0)
        }
    }).collect();

    while window.is_open() {
        z += 1.0;

        for pt in &mut points {
            let (x, y) = (pt.pos[0], pt.pos[1]);
            let value = noise::perlin3(&seed, &[x / DIV, y / DIV, z / DIV]);
            let value = (value + 1.0) / 2.0;
            pt.color = rgb(value, value, value);
        }

        let mut frame = window.cleared_frame(color::WHITE);
        frame.draw_points(&points);
    }
}

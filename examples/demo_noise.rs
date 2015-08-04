extern crate lux;
extern crate noise;

use lux::prelude::*;
use lux::graphics::ColorVertex;
use lux::color;

const SCALE: f32 = 20.0;

fn main() {
    let mut window = Window::new().unwrap();
    let seed = noise::Seed::new(0);
    let mut t = 0.0;

    // Set up the point buffer
    let mut points = Vec::with_capacity(255 * 255);
    for x in 0 .. 255 {
        for y in 0 .. 255 {
            points.push(ColorVertex {
                pos: [x as f32, y as f32],
                color: rgb(0, 0, 0)
            });
        }
    }

    while window.is_open() {
        let mut frame = window.cleared_frame(color::WHITE);

        // Update the point buffer with a new noise pattern
        for pt in &mut points {
            let value = noise::perlin3(&seed, &[pt.pos[0] / SCALE, pt.pos[1] / SCALE, t / SCALE]);
            let value = (value + 1.0) / 2.0;
            pt.color = hsv(value * 360.0, 1.0, 1.0);
        }

        frame.draw_points(&points);
        t += 1.0;
    }
}

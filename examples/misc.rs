extern crate lux;

use lux::prelude::*;
use lux::color;
use lux::graphics::{PrimitiveCanvas, ColorVertex};
use lux::graphics::TrianglesList;

fn main() {
    let mut lux = Window::new_with_defaults().unwrap();
    let mut delta = 0.0f32;

    // Create a series of points that will be laid out in a diagonal line.
    let points: Vec<_> = (0..100).map(|i| ColorVertex {
        pos: [i as f32, i as f32],
        color: color::BLUE
    }).collect();

    while lux.is_open() {
        let mut frame = lux.cleared_frame(color::RED);
        delta += 0.1;

        frame.with_rotation(delta, |frame|{
            for x in 0..40 {
                for y in 0..40 {
                    let (x, y) = (x as f32 * 40.0, y as f32 * 40.0);
                    frame.rect(x, y, 30.0, 30.0)
                         .color(color::BLUE)
                         .fill();
                }
            }

            // Create some vertices for a triangle.
            let vtxs = [
                ColorVertex {pos: [0.0, 0.0], color: rgb(1.0, 0.0, 0.0)},
                ColorVertex {pos: [0.0, 200.0], color: rgb(1.0, 0.0, 1.0)},
                ColorVertex {pos: [200.0, 0.0], color: rgb(0.0, 1.0, 0.0)},
            ];

            let idxs = [0, 1, 2];
            frame.draw_colored(TrianglesList, &vtxs[..], Some(&idxs[..]), None).unwrap();
        });

        frame.draw_points(&points);
        frame.rect(100.0, 100.0, 50.0, 50.0).color(color::GREEN).fill();
        frame.draw_point(110.5, 110.5, color::RED);
    }
}

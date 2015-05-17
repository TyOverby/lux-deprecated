extern crate lux;
extern crate nd_iter;

use lux::prelude::*;
use lux::color;
use lux::graphics::{PrimitiveCanvas, ColorVertex};
use lux::graphics::TrianglesList;
use nd_iter::iter_2d;

fn main() {
    let mut lux = Window::new().unwrap();
    let mut delta = 0.0f32;

    let points: Vec<_> = (0..100).map(|i| ColorVertex {
        pos: [i as f32, i as f32],
        color: color::BLUE
    }).collect();

    while lux.is_open() {
        delta += 0.1;

        let mut frame = lux.cleared_frame(color::RED);

        frame.with_rotation(delta, |frame|{
            for (x, y) in iter_2d(0u32..100, 0u32..100) {
                let (x, y) = (x as f32 * 40.0, y as f32 * 40.0);
                frame.rect(x, y, 30.0, 30.0)
                   .color(color::BLUE)
                   .fill();
            }

            let vtxs = [
                ColorVertex {pos: [0.0, 0.0], color: rgb(1.0, 0.0, 0.0)},
                ColorVertex {pos: [0.0, 200.0], color: rgb(1.0, 0.0, 1.0)},
                ColorVertex {pos: [200.0, 0.0], color: rgb(0.0, 1.0, 0.0)},
            ];

            let idxs = [0, 1, 2];
            frame.draw_colored(TrianglesList, &vtxs[..], Some(&idxs[..]), None);
        });

        frame.rect(101.0, 100.0, 50.0, 50.0).color(color::GREEN).fill();

        frame.draw_point(101.5, 100.5, color::RED);
        frame.draw_points(&points);

        frame.rect(100.0, 100.0, 50.0, 50.0).color(color::GREEN).fill();

    }
}

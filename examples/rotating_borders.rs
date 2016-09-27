extern crate lux;

use lux::prelude::*;

fn main() {
    let mut rot = 0.0;
    let mut window = Window::new_with_defaults().unwrap();
    while window.is_open() {
        rot += 0.05;
        let mut frame = window.cleared_frame([0.9, 0.9, 0.9, 0.001]);

        frame.translate(200.0, 200.0);

        for i in 0 .. 5 {
            let pos = i as f32 * 100.0 + 50.0;
            frame.draw(
                Square {
                    x: pos, y: 50.0,
                    size: 50.0,
                    transform: Some(*mat4_id().rotate_around((50.0, 50.0), rot)),
                    .. Default::default()
                }).unwrap();
        }
    }
}

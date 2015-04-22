extern crate lux;
use lux::prelude::*;

fn main() {
    let mut rot = 0.0;
    let mut window = Window::new().unwrap();
    while window.is_open() {
        rot += 0.05;
        let mut frame = window.cleared_frame([0.9, 0.9, 0.9, 0.001]);

        frame.translate(200.0, 200.0);

        frame.set_color(rgb(255, 0, 0));

        for i in (0 .. 5) {
            let border = i as f32 * 10.0;
            let pos = i as f32 * 100.0;
            frame.square(pos, 0.0, 50.0)
                 .border(border / 2.0, rgba(0, 0, 255, 255))
                 .rotate_around((25.0, 25.0), rot)
                 .fill_and_stroke();
        }
    }
}

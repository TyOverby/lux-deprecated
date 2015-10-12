extern crate lux;
use lux::prelude::*;

fn main() {
    let mut t = 0.0;
    let mut window = Window::new_with_defaults().unwrap();
    let logo = window.load_texture_file("./test/test.png").unwrap().into_sprite();
    let half = logo.width() / 2.0; // image is square, so this is fine

    while window.is_open() {
        let mut frame = window.cleared_frame(lux::color::WHITE);
        let (x, y) = window.mouse_pos();
        frame.sprite(&logo, x, y)
             .translate(-half, -half)
             .rotate_around((half, half), t)
             .draw();
        t += 0.01;
    }
}


extern crate lux;

use lux::prelude::*;
use lux::color;

fn main() {
    let mut lux = Window::new().unwrap();

    // A full sprite
    let sprite1 = lux.load_texture_file("./test/test.png").unwrap().into_sprite();

    // A sprite made by chopping off parts of the other one.
    let sprite2 = sprite1.sub_sprite((0, 0), (256 / 2, 255)).unwrap();

    while lux.is_open() {
        let mut frame = lux.cleared_frame(color::WHITE);
        let (x, y) = lux.mouse_pos();

        frame.sprite(&sprite1, 0.0, 0.0).draw();
        frame.sprite(&sprite2, x, y).draw();

        // Set a special size
        frame.sprite(&sprite1, x - 32.0, y-32.0).size(32.0, 32.0).draw();
    }
}

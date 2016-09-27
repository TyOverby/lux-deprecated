extern crate lux;

use lux::prelude::*;
use lux::color;

fn main() {
    let mut lux = Window::new_with_defaults().unwrap();

    // A full sprite
    let sprite1 = lux.load_texture_file("./test/test.png").unwrap().into_sprite();

    // A sprite made by chopping off parts of the other one.
    let sprite2 = sprite1.sub_sprite((0, 0), (256 / 2, 255)).unwrap();

    while lux.is_open() {
        let mut frame = lux.cleared_frame(color::WHITE);
        let (x, y) = lux.mouse_pos();

        frame.draw(Picture{
            x: 0.0, y: 0.0,
           sprite: Some(&sprite1),
            .. Default::default()
        }).unwrap();

        frame.draw(Picture {
            x: x, y: y,
            sprite: Some(&sprite2),
            .. Default::default()
        }).unwrap();

        // Set a special size
        frame.draw(Picture {
            x: x - 32.0, y: y-32.0,
            sprite: Some(&sprite1),
            size: Some((32.0, 32.0)),
            .. Default::default()
        }).unwrap();
    }
}

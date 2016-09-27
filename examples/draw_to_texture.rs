extern crate lux;

use lux::prelude::*;
use lux::color;

fn main() {
    let mut lux = Window::new_with_defaults().unwrap();

    let sprite = {
        let mut tex = Texture::empty(&lux, 256, 256).unwrap();
        {
            let mut tex = tex.as_drawable(&lux);
            tex.draw(Circle { x: 50.0, y: 50.0, diameter: 50.0, color: color::RED, .. Default::default() }).unwrap();
            tex.draw(Circle { x: 150.0, y: 50.0, diameter: 50.0, color: color::BLUE, .. Default::default() }).unwrap();
            tex.draw(Rectangle { x: 150.0, y: 50.0, w: 200.0, h: 25.0, color: color::BLUE, .. Default::default() }).unwrap();
        }
        tex.into_sprite()
    };

    while lux.is_open() {
        let mut frame = lux.cleared_frame(color::WHITE);
        let (x, y) = lux.mouse_pos();

        let picture = Picture { sprite: Some(&sprite), ..Default::default() };

        // Drawn at the origin
        frame.draw(picture).unwrap();

        // Drawn at the mouse
        frame.draw(Picture{x: x, y: y, ..picture}).unwrap();
    }
}

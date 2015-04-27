extern crate lux;
extern crate glium;
extern crate image;

use lux::prelude::*;

fn main() {
    let mut lux = Window::new().unwrap();

    let sprite = {
        let mut tex = Texture::empty(&lux, 256, 256);
        {
            let mut tex = tex.as_drawable_texture(&lux);
            tex.circle(50.0, 50.0, 50.0).set_color(rgb(255, 0, 0)).fill();
            tex.circle(150.0, 50.0, 50.0).set_color(rgb(255, 0, 0)).fill();
            tex.rect(50.0, 150.0, 200.0, 25.0).set_color(rgb(255, 0, 0)).fill();
        }
        tex.into_sprite()
    };

    while lux.is_open() {
        let mut frame = lux.cleared_frame(colors::WHITE);
        let (x, y) = lux.mouse_pos();

        frame.sprite(&sprite, 0.0, 0.0).draw();
        frame.sprite(&sprite, x, y).draw();

        // Set a special size
        frame.sprite(&sprite, x - 32.0, y - 32.0).size(32.0, 32.0).draw();

        frame.text("Hello World", 0.0, 0.0).draw();
    }
}

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
            tex.draw_pixel(5.0, 5.0, rgb(255, 0, 0));
            tex.square(5.0, 5.0, 50.0).border(5.0).fill_and_stroke();
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
        return;
    }
}

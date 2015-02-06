extern crate lux;
extern crate glium;
extern crate image;

use lux::*;
use std::path::Path;

fn main() {
    let mut lux = Window::new().unwrap();

    // A sprite made from an image
    let texture = lux.load_sprite(&Path::new("./test/minecraft_fixedwidth_font.png")).unwrap();
    let sheet   = texture.as_uniform_sprite_sheet(16, 16);

    while lux.is_open() {
        let mut frame = lux.cleared_frame(colors::BLACK);
        let (x, y) = lux.mouse_pos();

        let s1 = sheet.get(1, 4);

        frame.sprite(&texture, 0.0, 0.0).draw();
        frame.sprite(&s1, x, y).draw();
    }
}
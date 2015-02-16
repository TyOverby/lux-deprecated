#![feature(io, path, core)]

extern crate lux;
extern crate glium;
extern crate image;
extern crate freetype;

use lux::*;
use std::old_path::Path;
use std::old_io::File;

fn main() {
    let mut lux = Window::new().unwrap();

    let freetype = freetype::Library::init().unwrap();
    let font = Path::new("./resources/SourceCodePro-Regular.ttf");
    let mut face = freetype.new_face(&font, 0).unwrap();
    face.set_pixel_sizes(0, 48);

    let mut v = vec![];
    for i in 1u8 .. 255 {
        v.push(i as char);
    }

    let mut cache = FontCache::new(&mut lux).unwrap();

    /*
    let sprite_sheet = &cache.current.font_sheet;
    let letter_a = sprite_sheet.get(&'a');
    let whole = sprite_sheet.sprite.clone();
    */

    while lux.is_open() {
        let mut frame = lux.cleared_frame(colors::WHITE);
        cache.draw_onto(&mut frame, "Hello World", 0.0, 100.0);

//        frame.sprite(&whole, 0.0, 0.0).draw();
 //       frame.sprite(&letter_a, lux.mouse_x(), lux.mouse_y()).draw();
    }
}

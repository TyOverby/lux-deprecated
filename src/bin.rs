#![allow(unstable)]
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
    for _ in 0 .. 4 {
        for i in ('a' as u8) .. ('z' as u8) {
            v.push(i as char);
        }
        for i in ('A' as u8) .. ('Z' as u8) {
            v.push(i as char);
        }
    }

    let merged = merge_all(v.into_iter().map(|c| char_to_img(&mut face, c)));

    /*
    merged.save(File::create(&Path::new("foo.png")), image::ImageFormat::PNG);
    return;
    */

    let sprite = lux.sprite_from_image(merged);

    while lux.is_open() {
        let mut frame = lux.cleared_frame(colors::BLACK);
        let (x, y) = lux.mouse_pos();

        frame.sprite(&sprite, x, y).draw();
    }
}

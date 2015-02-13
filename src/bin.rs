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

    let cache = FontCache::new(&mut lux).unwrap();

    let ff = cache.current_face();

    /*
    let merged = merge_all(v.into_iter().map(|c| char_to_img(&mut face, c)));

    /*
    merged.save(File::create(&Path::new("foo.png")), image::ImageFormat::PNG);
    return;
    */

    let sprite = lux.sprite_from_image(merged);
    */

    while lux.is_open() {
        let mut frame = lux.cleared_frame(colors::BLUE);
        frame.sprite(&ff.get(&'a'), 0.0, 0.0).draw();
    }
}

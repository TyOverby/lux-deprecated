#![allow(unstable)]
extern crate lux;
extern crate glium;
extern crate image;
extern crate freetype;

use lux::*;
use std::path::Path;

fn glyph_to_vec(bf: &[u8], width: u32, height: u32) -> Vec<Vec<[f32; 4]>> {
    let mut v = vec![];
    for y in (0 .. height) {
        let mut v2 = vec![];
        for x in (0 .. width) {
            let v = bf[(y * width + x) as usize] as f32 / 255.0f32;
            v2.push([v, v, v, v]);
        }
        v.push(v2);
    }
    return v;
}

fn main() {
    let mut lux = Window::new().unwrap();

    let freetype = freetype::Library::init().unwrap();
    let font = Path::new("./resources/SourceCodePro-Medium.ttf");
    let mut face = freetype.new_face(font.as_str().unwrap(), 0).unwrap();
    face.set_pixel_sizes(0, 48);

    face.load_char('a' as u64, freetype::face::RENDER).unwrap();
    let g = face.glyph().bitmap();
    let vec = glyph_to_vec(g.buffer(), g.width() as u32, g.rows() as u32);

    let sprite = lux.sprite_from_pixels(vec);
    let (w, h) = (g.width(), g.rows());
    while lux.is_open() {
        let mut frame = lux.cleared_frame(colors::BLACK);
        let (x, y) = lux.mouse_pos();

        frame.draw_sprite(&sprite, (x as f32, y as f32), (w as f32, h as f32))
    }
}

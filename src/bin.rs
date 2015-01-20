#![allow(unstable)]
extern crate lux;
extern crate glium;
extern crate image;

use std::ops::Deref;

use lux::{
    LuxCanvas,
    PrimitiveCanvas,
    Transform,
    Interactive,
    Window,
    TexVertex,
    TrianglesList,
};

use lux::colors;

fn main() {
    let mut lux = Window::new().unwrap();

    //lux.assert_no_error();
    let img = image::open(&Path::new("test.png")).unwrap();
    let texture = lux.load_image(img);

    while lux.is_open() {
        let mut frame = lux.cleared_frame(colors::WHITE);
        let (x, y) = lux.mouse_pos();

        let tex_vs = vec![
            TexVertex {pos: [255.0, 0.0], tex_coords: [1.0, 0.0]},
            TexVertex {pos: [0.0, 0.0], tex_coords: [0.0, 0.0]},
            TexVertex {pos: [0.0, 255.0], tex_coords: [0.0, 1.0]},
            TexVertex {pos: [255.0, 255.0], tex_coords: [1.0, 1.0]},
        ];

        let idxs = vec![0u32, 1, 2, 0, 2, 3];
        frame.translate(x as f32, y as f32);

        frame.draw_tex_no_batch(TrianglesList, tex_vs, Some(idxs), None, texture.deref());
        ::std::io::timer::sleep(::std::time::Duration::milliseconds(17));
        frame.rect((100.0, 100.0), (50.0, 50.0)).fill();
    }
}

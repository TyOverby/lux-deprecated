extern crate lux;

use lux::prelude::*;

fn main() {
    let mut window = Window::new_with_defaults().unwrap();
    while window.is_open() {
        let mut frame = window.cleared_frame(rgb(0.9, 0.9, 0.9));

        // Colors are simply 4-element f32 arrays. [r, g, b, a]
        let _red = [1.0, 0.0, 0.0, 1.0];
        // There are some handy color creation functions though.
        let _red = rgb(1.0, 0.0, 0.0);
        let _red = rgb(255, 0, 0);
        let _red = hsv(0.0, 1.0, 1.0);
        // And with alpha values
        let _red = rgba(1.0, 0.0, 0.0, 1.0);
        let _red = rgba(255, 0, 0, 255);
        let _red = hsva(0.0, 1.0, 1.0, 1.0);
        // There are also a ton of pre-defined colors to choose from
        let red = lux::color::RED;


        frame.draw(Rectangle { x: 200.0, y: 20.0, w: 100.0, h: 300.0, color: red, .. Default::default() }).unwrap();
    }
}

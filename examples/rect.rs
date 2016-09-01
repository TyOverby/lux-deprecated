extern crate lux;
extern crate vecmath;

use lux::prelude::*;
use lux::color;

fn main() {
    let mut window = Window::new_with_defaults().unwrap();
    while window.is_open() {
        let mut frame = window.cleared_frame(rgb(0.9, 0.9, 0.9));
        // Default color (black)
        frame.draw(Rectangle { x: 10.0, y: 20.0, w: 150.0, h: 250.0, .. Default::default() }).unwrap();
        // Specify color
        frame.draw(Rectangle { x: 200.0, y: 20.0, w: 100.0, h: 300.0, color: rgb(1.0, 0.1, 0.1), .. Default::default() }).unwrap();
        // Specify the color and transformation matrix
        let mut matrix = vecmath::mat4_id();
        matrix.rotate_around((50.0, 50.0), 1.25);
        frame.draw(Rectangle { x: 0.0, y: 0.0, w: 100.0, h: 300.0, color: rgb(0.0, 1.0, 0.0), transform: Some(matrix)}).unwrap();
    }
}

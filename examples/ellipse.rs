extern crate lux;

use lux::prelude::*;

fn main() {
    let mut window = Window::new_with_defaults().unwrap();
    while window.is_open() {
        let mut frame = window.cleared_frame([0.9, 0.9, 0.9]);
        frame.draw(Circle { x: 20.0, y: 20.0, diameter: 45.0, .. Default::default()}).unwrap();

        // Manually set the number of segments to build the ellipse out of.
        frame.draw(Circle { x: 80.0, y: 80.0, diameter: 45.0, segments: Some(5), .. Default::default()}).unwrap();

        frame.draw(Ellipse { x: 20.0, y: 20.0, w: 50.0, h: 200.0, .. Default::default()}).unwrap();
    }
}

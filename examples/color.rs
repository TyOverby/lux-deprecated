extern crate lux;

use lux::prelude::*;

fn main() {
    let mut window = Window::new().unwrap();
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
        let _red = lux::color::RED;

        // You can set the color on the frame.
        // This is used as the default color for further drawing.
        frame.color(lux::color::BLUE);
        frame.square(20.0, 20.0, 100.0).fill();
        frame.square(0.0, 0.0, 10.0).fill();
        // Or set the color individually.
        frame.square(20.0, 20.0, 50.0).color(lux::color::PURPLE).fill();
        // When grouping colored objects together, you can use a color
        // temporarily, and it will automatically reset when done.
        frame.with_color([1.0, 0.0, 0.0, 0.5], |frame| {
            frame.circle(100.0, 100.0, 100.0).fill();
            frame.circle(150.0, 150.0, 100.0).fill();
        });
    }
}

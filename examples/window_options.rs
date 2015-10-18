extern crate lux;

use lux::window::WindowOptions;
use lux::interactive::Event;
use lux::prelude::*;
use lux::color;

const msg: &'static = "press space to increase the size of the window";

fn main() {
    let window = Window::new(WindowOptions {
        dimensions: (800, 500),
        title: "custom title".to_owned(),
        decorations: true,
        .. Default::default()
    });
    let mut window = window.unwrap();

    while window.is_open() {
        let mut frame = window.cleared_frame(color::WHITE);
        frame.text(msg, 0.0, 0.0).draw();
        for event in window.events() {
            if let Event::KeyPressed(_, Some(' '), _) = event {

                window.change_options(|opts| {
                    opts.dimensions.0 += 50;
                    opts.dimensions.1 += 50;
                }).unwrap();

            }
        }
    }
}

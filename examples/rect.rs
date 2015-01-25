extern crate lux;
use lux::*;

fn main() {
    let mut window = Window::new().unwrap();
    while window.is_open() {
        let mut frame = window.cleared_frame(rgb(0.9, 0.9, 0.9));
        // draw_rect((x, y), (width, height))
        frame.rect(10.0, 20.0, 15.0, 25.0).fill();
        frame.with_fill_color(colors::BLUE, |frame| {
            frame.rect(20.0, 10.0, 10.0, 10.0).fill();
            // TODO: when borders are better, add a border_rect example
        });
    }
}

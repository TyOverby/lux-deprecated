extern crate lovely;

use lovely::window::Window;
use lovely::Lovely;
use lovely::color;

fn main() {
    let mut window = Window::new().unwrap();
    loop {
        window.clear(color::consts::CYAN);
        window.draw_rect((0.0,0.0), (0.0, 0.0));
        window.render();
    }
}

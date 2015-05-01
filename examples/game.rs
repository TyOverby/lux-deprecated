extern crate lux;
use lux::prelude::*;
use lux::game::*;

const MOVEMENT_SPEED: f32 = 200.0;
const PLAYER_SIZE: f32 = 20.0;

struct MyGame {
    pos: (f32, f32),
    speed: (f32, f32)
}

impl MyGame {
    fn new() -> MyGame {
        MyGame {
            pos: (0.0, 0.0),
            speed: (10.0, 10.0)
        }
    }
}

impl Game for MyGame {
    fn prepare_window(&mut self, window: &mut Window) {
        window.preload_font("SourceCodePro", 10);
    }

    fn update(&mut self, dt: f32, window: &mut Window, _events: &mut EventIterator) {
        // position
        self.pos.0 += self.speed.0 * dt;
        self.pos.1 += self.speed.1 * dt;
        // Keep the player from moving off the edge
        self.pos.0 = clamp(0.0, self.pos.0, window.width() - PLAYER_SIZE);
        self.pos.1 = clamp(0.0, self.pos.1, window.height() - PLAYER_SIZE);
        // dampening
        self.speed.0 *= 1.0 - dt;
        self.speed.1 *= 1.0 - dt;

        // events
        // x
        if window.is_key_pressed('a') {
            self.speed.0 = -MOVEMENT_SPEED;
        } else if window.is_key_pressed('d') {
            self.speed.0 = MOVEMENT_SPEED;
        }
        // y
        if window.is_key_pressed('w') {
            self.speed.1 = -MOVEMENT_SPEED;
        } else if window.is_key_pressed('s') {
            self.speed.1 = MOVEMENT_SPEED;
        }
        // ::std::thread::sleep_ms(5);
    }

    fn render(&mut self, lag: f32, _window: &mut Window, frame: &mut Frame) {
        //let lag = 0.0;
        let (x, y) = self.pos;
        let (vx, vy) = self.speed;

        frame.text("Use the [w][a][s][d] keys to move around", 5.0, 5.0).draw().unwrap();
        frame.text("Hold the spacebar to see the debug fps viewer", 5.0, 25.0).draw().unwrap();
        frame.circle(x + vx * lag, y + vy * lag, PLAYER_SIZE).fill();
    }

    fn updates_per_s(&self) -> f64 { 120.0 }

    fn show_fps(&self, window: &Window) -> bool {
        window.is_key_pressed(' ')
    }
}

fn clamp(low: f32, value: f32, high: f32) -> f32 {
    if value < low {
        low
    } else if value > high {
        high
    } else {
        value
    }
}

fn main() {
    MyGame::new().run_until_end();
}

extern crate lux;
use lux::prelude::*;
use lux::game::*;

const DAMPENING: f32 = 0.99;
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
    fn update(&mut self, dt: f32, window: &mut Window, events: &mut EventIterator) {
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
        if window.is_key_pressed('h') {
            self.speed.0 = -MOVEMENT_SPEED;
        } else if window.is_key_pressed('l') {
            self.speed.0 = MOVEMENT_SPEED;
        }
        // y
        if window.is_key_pressed('k') {
            self.speed.1 = -MOVEMENT_SPEED;
        } else if window.is_key_pressed('j') {
            self.speed.1 = MOVEMENT_SPEED;
        }
    }

    fn render(&mut self, dt: f32, window: &mut Window, frame: &mut Frame) {
        let (x, y) = self.pos;
        frame.draw_text("Use the [hjkl] keys to move around", 3.5, 20.5);
        frame.circle(x, y, PLAYER_SIZE).fill();
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

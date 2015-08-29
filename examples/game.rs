extern crate lux;

use lux::prelude::*;
use lux::game::*;

const MOVEMENT_SPEED: f32 = 200.0;
const PLAYER_SIZE: f32 = 20.0;

struct MyGame {
    pos: (f32, f32),
    speed: (f32, f32),
    rotation: f32
}

impl Game for MyGame {
    fn update(&mut self, dt: f32, window: &mut Window, _events: &mut EventIterator) -> LuxResult<()> {
        // Clamps a value between low and high bounds
        fn clamp(low: f32, value: f32, high: f32) -> f32 {
            if value < low {
                low
            } else if value > high {
                high
            } else {
                value
            }
        }


        // Update position and rotation
        self.pos.0 += self.speed.0 * dt;
        self.pos.1 += self.speed.1 * dt;
        self.rotation += 1.0 * dt;

        // Keep the player from moving off the edge
        self.pos.0 = clamp(0.0, self.pos.0, window.width() - PLAYER_SIZE);
        self.pos.1 = clamp(0.0, self.pos.1, window.height() - PLAYER_SIZE);

        // Apply speed dampening
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

        Ok(())
    }

    fn render(&mut self, lag: f32, _window: &mut Window, frame: &mut Frame) -> LuxResult<()> {
        let (px, py) = self.pos;
        let (vx, vy) = self.speed;

        try!(frame.text("Use the [w][a][s][d] keys to move around.", 5.0, 5.0).size(20).draw());
        try!(frame.text("Hold the spacebar to see the debug fps viewer.", 5.0, 25.0).size(20).draw());

        frame.circle(px + vx * lag, py + vy * lag, PLAYER_SIZE)
             .rotate_around((PLAYER_SIZE / 2.0, PLAYER_SIZE / 2.0), self.rotation)
             .segments(6)
             .fill();

        Ok(())
    }

    fn updates_per_s(&self) -> f64 {
        120.0
    }

    fn show_fps(&self, window: &Window) -> bool {
        window.is_key_pressed(' ')
    }
}

fn main() {
    let game = MyGame {
        pos: (0.0, 0.0),
        speed: (10.0, 10.0),
        rotation: 0.0
    };

    game.run_until_end().unwrap();
}

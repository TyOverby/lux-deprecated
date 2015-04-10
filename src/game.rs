use super::prelude::*;
use clock_ticks;

pub trait Game {
    fn update(&mut self, dt: f32, window: &mut Window, events: &mut EventIterator);
    fn render(&mut self, window: &mut Window, frame: &mut Frame);

    fn clear_color(&self) -> Option<[f32; 4]> {Some([1.0, 1.0, 1.0, 1.0])}
    fn draw_fps(&self) -> Option<usize> { Some(100) }
    fn should_close(&self) -> bool { false }
    fn prepare_window(&mut self, _window: &mut Window) {}
    fn on_close(&mut self, _window: &mut Window) {}

    fn updates_per_s(&self) -> f64 { 60.0 }
    fn s_per_update(&self) -> f64 { 1.0 / self.updates_per_s() }

    fn run_until_end(self) where Self: Sized{
        let mut runner = GameRunner::new(self).unwrap();
        runner.run_until_end();
    }
}

pub struct GameRunner<G: Game> {
    pub window: Window,
    pub game: G
}

struct FrameTiming {
    updates: Vec<u64>,
    render: u64
}

fn time<F: FnOnce()>(f: F) -> u64 {
    let before = clock_ticks::precise_time_ns();
    f();
    let after = clock_ticks::precise_time_ns();
    after - before
}

impl <G: Game> GameRunner<G> {
    pub fn new(game: G) -> LuxResult<GameRunner<G>> {
        Ok(GameRunner {
            game: game,
            window: try!(Window::new())
        })
    }

    pub fn run_until_end(&mut self) {
        let mut previous = clock_ticks::precise_time_s();
        let mut lag = 0.0;

        while self.window.is_open() && !self.game.should_close() {
            let mut events = self.window.events();
            let mut frame = if let Some(c) = self.game.clear_color() {
                self.window.cleared_frame(c)
            } else {
                self.window.frame()
            };

            let current = clock_ticks::precise_time_s();
            let elapsed = current - previous;
            previous = current;
            lag += elapsed;

            let s_p_u = self.game.s_per_update();

            while lag >= s_p_u {
                let tu = time(|| self.game.update(s_p_u as f32, &mut self.window, &mut events));
                lag -= s_p_u;
            }

            let tr = time(|| self.game.render(&mut self.window, &mut frame));

            if !events.backing.is_empty() {
                self.window.restock_events(events);
            }
        }
    }
}

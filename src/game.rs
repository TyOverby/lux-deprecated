use super::prelude::*;
use time;

pub trait Game {
    fn update(&mut self, dt: f32, window: &mut Window, events: &mut EventIterator);
    fn render(&mut self, dt: f32, window: &mut Window, frame: &mut Frame);

    fn clear_color(&self) -> Option<[f32; 4]> {Some([1.0, 1.0, 1.0, 1.0])}
    fn draw_fps(&self) -> bool { true }
    fn should_close(&self) -> bool { false }
    fn prepare_window(&mut self, _window: &mut Window) {}
    fn on_close(&mut self, _window: &mut Window) {}

    fn run_until_end(self) where Self: Sized{
        let mut runner = GameRunner::new(self).unwrap();
        runner.run_until_end();
    }
}

pub struct GameRunner<G: Game> {
    pub window: Window,
    pub game: G
}

impl <G: Game> GameRunner<G> {
    pub fn new(game: G) -> LuxResult<GameRunner<G>> {
        Ok(GameRunner {
            game: game,
            window: try!(Window::new())
        })
    }

    pub fn run_until_end(&mut self) {
        let mut prev = time::precise_time_s();
        while self.window.is_open() && !self.game.should_close() {
            let mut events = self.window.events();
            let mut frame = if let Some(c) = self.game.clear_color() {
                self.window.cleared_frame(c)
            } else {
                self.window.frame()
            };

            let now = time::precise_time_s();
            let dt = now - prev;
            prev = now;

            self.game.update(dt as f32, &mut self.window, &mut events);
            self.game.render(dt as f32, &mut self.window, &mut frame);

            if !events.backing.is_empty() {
                self.window.restock_events(events);
            }
        }
    }
}

use super::prelude::*;
use std::collections::VecDeque;
use clock_ticks;

pub trait Game {
    fn update(&mut self, dt: f32, window: &mut Window, events: &mut EventIterator);
    fn render(&mut self, lag: f32, window: &mut Window, frame: &mut Frame);

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
    pub game: G,
    frame_timings: VecDeque<FrameTiming>
}

struct FrameTiming {
    update_durations: Vec<u64>,
    render_duration: u64,
    timestamp_start: u64,
    timestamp_end: u64
}

fn time<F: FnOnce()>(f: F) -> u64 {
    let before = clock_ticks::precise_time_ns();
    f();
    let after = clock_ticks::precise_time_ns();
    after - before
}

impl <G: Game> GameRunner<G> {
    pub fn new(game: G) -> LuxResult<GameRunner<G>> {
        let amnt = game.draw_fps().unwrap_or(10);
        Ok(GameRunner {
            game: game,
            window: try!(Window::new()),
            frame_timings: VecDeque::with_capacity(amnt + 1)
        })
    }

    pub fn run_until_end(&mut self) {
        let mut previous = clock_ticks::precise_time_s();
        let mut lag = 0.0;

        while self.window.is_open() && !self.game.should_close() {
            //
            // Preframe setup
            //
            let mut events = self.window.events();
            let mut frame = if let Some(c) = self.game.clear_color() {
                self.window.cleared_frame(c)
            } else {
                self.window.frame()
            };

            //
            // Core loop.
            //
            let current = clock_ticks::precise_time_s();
            let current_ns = clock_ticks::precise_time_ns();
            let elapsed = current - previous;
            previous = current;
            lag += elapsed;

            let s_p_u = self.game.s_per_update();

            /*println!("elapsed: {}
                      s_p_u  : {}", elapsed, s_p_u);
                      */

            let mut update_durations = vec![];
            while lag >= s_p_u {
                let tu = time(|| self.game.update(s_p_u as f32, &mut self.window, &mut events));
                update_durations.push(tu);
                lag -= s_p_u;
            }

            let tr = time(|| self.game.render(lag as f32, &mut self.window, &mut frame));

            //
            // Postframe cleanup and recording
            //
            if !events.backing.is_empty() {
                self.window.restock_events(events);
            }

            if let Some(max_timings) = self.game.draw_fps() {
                let now = clock_ticks::precise_time_ns();
                let timing = FrameTiming {
                    update_durations: update_durations,
                    render_duration: tr,
                    timestamp_start: current_ns,
                    timestamp_end: now
                };

                self.frame_timings.push_front(timing);
                self.frame_timings.truncate(max_timings);
                self.draw_timings(&mut frame);
            }
        }
    }

    fn calc_fps(&self) -> (u32, u32) {
        match (self.frame_timings.back(), self.frame_timings.front()) {
            (Some(oldest), Some(most_recent)) => {
                let time_elapsed = most_recent.timestamp_end -
                                   oldest.timestamp_start;
                let time_elapsed = time_elapsed as f64 / 1_000_000_000.0;

                let num_frames = self.frame_timings.len() as f64;

                let num_updates =
                    self.frame_timings
                        .iter()
                        .map(|timing| timing.update_durations.len())
                        .fold(0, |a, b| a + b) as f64;


                ((num_frames / time_elapsed).round() as u32,
                 (num_updates / time_elapsed).round() as u32)
            }
            _ => (0, 0)
        }
    }

    fn draw_timings(&self, frame: &mut Frame) {
        frame.draw_text(&format!("FPS: {:?}", self.calc_fps())[..], 100.5, 100.5);
    }
}

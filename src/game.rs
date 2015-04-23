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
    debug_drawing: u64,
    render_publish: u64,
    timestamp_start: u64,
    timestamp_end: u64,
}

fn time<R, F: FnOnce() -> R>(f: F) -> (u64, R) {
    let before = clock_ticks::precise_time_ns();
    let r = f();
    let after = clock_ticks::precise_time_ns();
    (after - before, r)
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
        let mut frame = Some(self.window.cleared_frame(rgb(255, 255, 255)));

        while self.window.is_open() && !self.game.should_close() {
            //
            // Preframe setup
            //
            let mut events = self.window.events();

            //
            // Core loop.
            //
            let current = clock_ticks::precise_time_s();
            let current_ns = clock_ticks::precise_time_ns();
            let elapsed = current - previous;
            previous = current;
            lag += elapsed;

            let s_p_u = self.game.s_per_update();

            let mut update_durations = vec![];
            while lag >= s_p_u {
                let (tu, _) = time(|| self.game.update(s_p_u as f32, &mut self.window, &mut events));
                update_durations.push(tu);
                lag -= s_p_u;
            }

            let (tr, _) = time(|| self.game.render(lag as f32, &mut self.window, frame.as_mut().unwrap()));

            let (t_timing, _) = time(|| self.draw_timings(frame.as_mut().unwrap()));

            let (tpublish, _) = time(|| {
                ::std::mem::drop(frame.take());
                frame = Some(if let Some(c) = self.game.clear_color() {
                            self.window.cleared_frame(c)
                        } else {
                            self.window.frame()
                        });
            });

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
                    render_publish: tpublish,
                    debug_drawing: t_timing,
                    timestamp_start: current_ns,
                    timestamp_end: now
                };

                self.frame_timings.push_front(timing);
                self.frame_timings.truncate(max_timings);
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
        fn draw_bars<B: Iterator<Item=(f32, [f32; 4])>, I: ExactSizeIterator<Item=B>>(frame: &mut Frame, bars: I) {
            let bar_count = bars.len();
            let bar_width = 1.0 / bar_count as f32;
            for (i, bar) in bars.enumerate() {
                let x = bar_width * i as f32;
                let mut y = 0.0;
                for (p, color) in bar {
                    frame.rect(x, y, bar_width, p).set_color(color).fill();
                    y += p;
                }
            }
        }

        fn percentage_time(span: u64) -> f32 {
            span as f32 / (16000000.0 )
        }

        const HEIGHT: f32 = 100.0;
        const WIDTH:  f32 = 161.0;
        let h = frame.height();
        frame.with_translate(WIDTH, h, |frame| {
        frame.with_scale(-WIDTH, -HEIGHT, |frame| {
            frame.rect(0.0, 0.0, 1.0, 1.0)
                 .set_color(rgba(1.0, 1.0, 0.0, 0.8))
                 .fill();

            draw_bars(frame, self.frame_timings.iter().map(|timing| {
                let update_colors = [rgb(0.0, 0.2, 0.9), rgb(0.2, 0.0, 0.9)];
                let mut v = vec![];
                v.extend(
                    timing.update_durations
                          .iter()
                          .enumerate()
                          .map(|(i, &t)| (percentage_time(t), update_colors[i % 2])));
                v.push((percentage_time(timing.render_duration), rgb(0.0, 0.9, 0.0)));
                v.push((percentage_time(timing.debug_drawing), rgb(0.9, 0.0, 0.0)));
                v.push((percentage_time(timing.render_publish), rgb(0.0, 0.5, 0.0)));
                v.into_iter()
            }));
        });
        });

        frame.rect(0.0, h - HEIGHT, WIDTH, 1.0).set_color(rgb(0, 0, 0)).fill();

        let (fps, ups) = self.calc_fps();
        frame.with_translate(WIDTH, h, |frame| {
        frame.with_rotation(-3.1415 / 2.0, |frame| {
            frame.text(format!("FPS {} UPS {}", fps, ups), 0.0, 0.0)
                 .size(12)
                 .draw().unwrap();
        });
        });
    }
}

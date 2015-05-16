use super::prelude::*;
use std::collections::VecDeque;
use clock_ticks;

const FRAMES_TO_TRACK: usize = 80;

/// A trait that represents basic game functionality.
///
/// The game is intended to be run with a `GameRunner`, but that is optional.
pub trait Game {
    /// The update portion of the game loop.
    ///
    /// The `dt` (delta-time) is in seconds and will always be the result
    /// of calling `self.s_per_update()`.
    ///
    /// Any events that are not consumed during this update cycle will remain
    /// available on the next update cycle.
    ///
    /// This function may be called multiple times in a row without render() being
    /// called depending on how the game loop operates and the value of
    /// `self.s_per_update()`.
    ///
    /// Returns a LuxResult that you can use to indicate if the update was successful or not.
    /// Non-Successful updates will terminate the game.
    fn update(&mut self, dt: f32, window: &mut Window, events: &mut EventIterator) -> LuxResult<()>;

    /// The render portion of the game loop.
    ///
    /// Lag is the amount of time (in seconds) that have accumulated between
    /// updates.  You should use this to ease object to their locations in order
    /// to keep smooth animations.
    ///
    /// Returns a LuxResult that you can use to indicate if the render was successful or not.
    /// Non-Successful updates will terminate the game.
    fn render(&mut self, lag: f32, window: &mut Window, frame: &mut Frame) -> LuxResult<()>;

    /// The color that is used to clear the screen before each frame.
    ///
    /// `None` means that the screen will not be cleared.
    ///
    /// Defaults to `Some(rgba(1.0, 1.0, 1.0, 1.0))` (solid white).
    fn clear_color(&self) -> Option<[f32; 4]> {Some([1.0, 1.0, 1.0, 1.0])}

    /// If running in a GameRunner, this function can be overridden to
    /// display a running FPS counter that shows how time is being spent
    /// in the game.
    ///
    /// Defaults to `false` (don't show fps).
    fn show_fps(&self, _window: &Window) -> bool { false }

    /// For custom game-closing logic this function can be overridden to
    /// conditionally return true.
    ///
    /// Defaults to using the value from `!window.was_open()`.
    fn should_close(&self, window: &Window) -> bool {
        !window.was_open()
    }

    /// Called once when the GameRunner is set up, this function can be
    /// overridden to set properties on the Window.
    ///
    /// Defaults to doing nothing.
    fn prepare_window(&mut self, _window: &mut Window) -> LuxResult<()> { Ok(())}

    /// Called once before terminating the window.
    ///
    /// Defaults to doing nothing.
    fn on_close(&mut self, _window: &mut Window) -> LuxResult<()> { Ok(()) }

    /// Returs the amount of updates you want to have run in one wall-clock
    /// second.
    ///
    /// Defaults to `60.0`.
    fn updates_per_s(&self) -> f64 { 60.0 }

    /// Returns the amount of (fractional) seconds that you want an individual
    /// update to take.
    ///
    /// Defaults to `1.0 / self.updates_per_s()`.
    ///
    /// Prefer changing the returning value of `updates_per_s` rather than
    /// changing this function.
    fn s_per_update(&self) -> f64 { 1.0 / self.updates_per_s() }

    /// Starts this game and runs it until the game is over.
    ///
    /// Defaults to wrapping this game in a `GameRunner` and calling
    /// `run_until_end` on that.
    fn run_until_end(self) -> LuxResult<()> where Self: Sized {
        let mut runner = try!(GameRunner::new(self));
        runner.run_until_end()
    }
}

/// A struct that wraps a `Game` and a `Window` and implementes a game loop.
pub struct GameRunner<G: Game> {
    /// The window being used to run the game.
    pub window: Window,
    /// The wrapped game.
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
    /// Constructs a new GameRunner wrapping a game.
    ///
    /// Attempts to build a window as well.
    pub fn new(game: G) -> LuxResult<GameRunner<G>> {
        Ok(GameRunner {
            game: game,
            window: try!(Window::new()),
            frame_timings: VecDeque::with_capacity(FRAMES_TO_TRACK + 1)
        })
    }

    /// Runs the game until the game is terminated.
    ///
    /// The game loop is fairly complicated but is explained as the final example
    /// in this blog post: http://gameprogrammingpatterns.com/game-loop.html
    ///
    /// ## Main features
    /// 1. All updates happen in discrete time steps.
    /// 2. Multiple updates can happen sequentially in order to handle lag and
    ///    get back on step.
    /// 3. Multiple renders can happen sequentially if the computer is too fast.
    ///    Renders will keep occuring until enough time has built up to trigger
    ///    another update.
    /// 4. The amount of "lag" time is passed in to the render function so that
    ///    the user can accomodate for lag.
    pub fn run_until_end(&mut self) -> LuxResult<()> {
        let mut previous = clock_ticks::precise_time_s();
        let mut lag = 0.0;
        let mut frame = Some(self.window.cleared_frame(rgb(255, 255, 255)));

        try!(self.game.prepare_window(&mut self.window));

        while self.game.should_close(&self.window) {
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
                let (tu, r) = time(|| self.game.update(s_p_u as f32, &mut self.window, &mut events));
                try!(r);
                update_durations.push(tu);
                lag -= s_p_u;
            }

            let (tr, r) = time(|| self.game.render(lag as f32, &mut self.window, frame.as_mut().unwrap()));
            try!(r);

            let (t_timing, _) = time(|| {
                if self.game.show_fps(&self.window) {
                    self.draw_timings(frame.as_mut().unwrap())
                }
            });

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
            if self.frame_timings.len() > FRAMES_TO_TRACK {
                self.frame_timings.pop_back();
            }
        }
        try!(self.game.on_close(&mut self.window));
        Ok(())
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
            span as f32 / (16666666.6 )
        }

        const HEIGHT: f32 = 100.0;
        const WIDTH:  f32 = 160.0;
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

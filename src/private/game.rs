#![allow(unused_imports, unused_variables)]

use super::glutin_window::{Window, Frame};
use super::interactive::EventIterator;
use super::color::{rgba, rgb};
use super::error::LuxResult;
use super::interactive::Interactive;
use super::canvas::{Canvas, Rectangle};
use super::raw::Transform;

use std::collections::VecDeque;
use std::mem::transmute;
use std::thread;
use clock_ticks;

use super::colors::WHITE;

const FRAMES_TO_TRACK: usize = 80;

/// A struct that stores closures which load assets and prepare the game for
/// running.
pub struct Loader<G> {
    async_tasks: Vec<(String, bool, Box<Fn() -> LuxResult<Box<Option<()>>> + Send + 'static>)>,
    apply_tasks: Vec<(String, Box<Fn(Box<Option<()>>, &mut Window, &mut G) -> LuxResult<()>>)>,
}

#[derive(Eq, PartialEq, Hash, Debug)]
pub enum LoadState {
    Idle,
    Loading,
    Applying,
    Finished
}

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
    fn clear_color(&self) -> Option<[f32; 4]> {
        Some([1.0, 1.0, 1.0, 1.0])
    }

    /// If running in a GameRunner, this function can be overridden to
    /// display a running FPS counter that shows how time is being spent
    /// in the game.
    ///
    /// Defaults to `false` (don't show fps).
    fn show_fps(&self, _window: &Window) -> bool {
        false
    }

    /// For custom game-closing logic this function can be overridden to
    /// conditionally return true.
    ///
    /// Defaults to using the value from `!window.was_open()`.
    fn should_close(&self, window: &Window) -> bool {
        !window.was_open()
    }

    /// Called once before terminating the window.
    ///
    /// Defaults to doing nothing.
    fn on_close(&mut self, _window: &mut Window) -> LuxResult<()> {
        Ok(())
    }

    /// Returs the amount of updates you want to have run in one wall-clock
    /// second.
    ///
    /// Defaults to `60.0`.
    fn updates_per_s(&self) -> f64 {
        60.0
    }

    /// Returns the amount of (fractional) seconds that you want an individual
    /// update to take.
    ///
    /// Defaults to `1.0 / self.updates_per_s()`.
    ///
    /// Prefer changing the returning value of `updates_per_s` rather than
    /// changing this function.
    fn s_per_update(&self) -> f64 {
        1.0 / self.updates_per_s()
    }

    /// Starts this game and runs it until the game is over.
    ///
    /// Defaults to wrapping this game in a `GameRunner` and calling
    /// `run_until_end` on that.
    fn run_until_end(self) -> LuxResult<()> where Self: Sized {
        let mut runner = try!(GameRunner::new(self));
        runner.run_until_end()
    }

    /// All asset loading and window preparation code can be done here.
    ///
    /// Loading and preparing assets can typically be done in parallel, but
    /// preparing the window and loading the textures into OpenGL should be
    /// done one at a time.  Because of this, the loader can take jobs that
    /// have an asyncronous component and jobs that are entirely synchronous.
    ///
    /// For more information, read the docs for `Loader`.
    ///
    /// Defaults to loading nothing.
    fn load(_loader: &mut Loader<Self>) where Self: Sized { }

    /// Draws the loading screen that shows progress when the game is loading.
    fn draw_loading_screen<'a, I>(status: I, progress: (usize, usize ), mut frame: Frame)
    where I: Iterator<Item=(&'a (usize, String), &'a LoadState)>{
        let mut buf = String::new();
        for (&(idx, ref name), state) in status {
            buf.push_str(&format!("{}. {}: {:?}\n", idx + 1, name, state));
        }

        // Draw progress Bar
        let (w, h) = frame.size();
        let percent = progress.0 as f32 / progress.1 as f32;
        let pos = percent * w;

        /*
        frame.text(&buf, 10.0, 0.0)
             .color(rgb(255, 0, 0))
             .size(30)
             .draw()
             .unwrap();

        frame.text(&format!("{}%", (percent * 100.0) as u32), pos - 30.0, h - 60.0)
             .color(rgb(255, 0, 0))
             .size(30)
             .draw()
             .unwrap();
         */

        frame.draw(Rectangle {
            x: 0.0, y:0.0,
            w: pos, h: h,
            color: rgb(255, 0, 0),
            .. Default::default()
        }).unwrap();

        frame.with_scissor(0, 0, pos as u32, h as u32, |frame| {
            /*
            frame.text(&buf, 10.0, 0.0)
                 .color(rgb(255, 255, 255))
                 .size(30)
                 .draw()
                 .unwrap();

            frame.text(&format!("{}%", (percent * 100.0) as u32), pos - 30.0, h - 60.0)
                 .color(rgb(255, 255, 255))
                 .size(30)
                 .draw()
                 .unwrap();
             */
        });

    }
}

impl <G> Loader<G> {
    fn new() -> Loader<G> {
        Loader {
            async_tasks: vec![],
            apply_tasks: vec![],
        }
    }

    /// Performs part of a job asynchronously and then applies the result
    /// of that loading to the game.
    ///
    /// The two closures are separate parts of the same loading job.
    ///
    /// * `load`: Does the bulk of the loading work.  Returns a `LuxResult` with a
    ///           return value.
    /// * `apply`: Takes the return value and applies it to the `Window` and `Game` object.
    ///
    /// The `load` closures are all run in separate threads, but the apply closures
    /// must run sequentially in the same thread in order to share the Game and
    /// Window objects.
    pub fn do_async<R, FL, FA, S: Into<String>>(&mut self, desc: S, load: FL, apply: FA)
    where FL: Fn() -> LuxResult<R> + Send + 'static,
          FA: Fn(R, &mut Window, &mut G) -> LuxResult<()> + 'static {

        let new_load: Box<Fn() -> LuxResult<Box<Option<()>>> + Send> = Box::new(move || {
            unsafe { transmute(load().map(|a| Box::new(Some(a)))) }
        });

        let new_apply: Box<Fn(Box<Option<()>>, &mut Window, &mut G) -> LuxResult<()>> = Box::new(move |r, w, g| {
            let mut res: Box<Option<R>> = unsafe { transmute(r) };
            apply(res.take().unwrap(), w, g)
        });

        let desc = desc.into();

        self.async_tasks.push((desc.clone(), true, new_load));
        self.apply_tasks.push((desc, new_apply));
    }

    /// Performs a job without an asynchronous component.
    ///
    /// The closure is executued on the main thread, and is called along with
    /// the `apply` closures from the `do_async` method.
    pub fn do_sync<F, S: Into<String>>(&mut self, desc: S, load: F)
    where F: Fn(&mut Window, &mut G) -> LuxResult<()> + 'static {
        self.async_tasks.push(("".into(), false, Box::new(|| Ok(Box::new(Some(()))))));

        let new_apply: Box<Fn(Box<Option<()>>, &mut Window, &mut G) -> LuxResult<()>> = Box::new(move |_, w, g| {
            load(w, g)
        });

        self.apply_tasks.push((desc.into(), new_apply));
    }
}

/// A struct that wraps a `Game` and a `Window` and implementes a game loop.
pub struct GameRunner<G: Game> {
    /// The window being used to run the game.
    pub window: Window,
    /// The wrapped game.
    pub game: G,
    frame_timings: VecDeque<FrameTiming>,

    previous: f64,
    lag: f64,
    // This is required because otherwise, we get weird time
    // reporting when creating and dropping frames.
    next_frame: Option<Frame>
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
            window: try!(Window::new_with_defaults()),
            frame_timings: VecDeque::with_capacity(FRAMES_TO_TRACK + 1),

            previous: 0.0,
            lag: 0.0,
            next_frame: None
        })
    }

    /// Moves the game forward one "step".
    ///
    /// A "step" has two main phases
    ///
    /// * Update Phase: The Games `update()` function is called any number of times (including 0).
    /// * Render Phase: The Games `render()` function is called exactly once.
    ///
    /// The game runner keeps track of lag manually, so `update()` might be called more than once
    /// per step.  If you are calling `step` manually and you stop the game (for example, to pause
    /// the game), you must call `reset_lag` so that `step` doesn't think that the game lagged for
    /// a very long time.
    pub fn step(&mut self) -> LuxResult<()> {
        let mut frame = self.next_frame.take().unwrap_or_else(|| self.window.cleared_frame(WHITE));

        //
        // Preframe setup
        //
        let mut events = self.window.events();

        //
        // Core loop.
        //
        let current = clock_ticks::precise_time_s();
        let current_ns = clock_ticks::precise_time_ns();
        let elapsed = current - self.previous;
        self.previous = current;
        self.lag += elapsed;

        let s_p_u = self.game.s_per_update();

        let mut update_durations = vec![];
        while self.lag >= s_p_u {
            let (tu, r) = time(|| self.game.update(s_p_u as f32, &mut self.window, &mut events));
            try!(r);
            update_durations.push(tu);
            self.lag -= s_p_u;
        }

        let (tr, r) = time(|| self.game.render(self.lag as f32, &mut self.window, &mut frame));
        try!(r);

        let (t_timing, _) = time(|| {
            if self.game.show_fps(&self.window) {
                self.draw_timings(&mut frame);
            }
        });

        let (tpublish, _) = time(|| {
            ::std::mem::drop(frame);
            let next_frame = self.window.cleared_frame(self.game.clear_color().unwrap_or(WHITE));
            self.next_frame = Some(next_frame);
        });

        //
        // Postframe cleanup and recording
        //
        if !events.is_empty() {
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

        Ok(())
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
        try!(self.do_load());
        self.previous = clock_ticks::precise_time_s();
        self.lag = 0.0;
        while !self.game.should_close(&self.window) {
            try!(self.step());
        }
        self.game.on_close(&mut self.window)
    }

    /// Resets the stored lag.
    ///
    /// This function should only be called when resuming calls to `step`
    /// after not calling it regularly.
    pub fn reset_lag(&mut self) {
        self.lag = 0.0;
        self.previous = clock_ticks::precise_time_s();
    }

    /// Execute the game loading closures that would be
    /// generated by `Game::load`.
    pub fn do_load(&mut self) -> LuxResult<()> {
        use std::sync::mpsc::channel;
        use std::collections::HashMap;

        let mut loader = Loader::new();

        G::load(&mut loader);

        let Loader{async_tasks, apply_tasks, ..} = loader;
        let mut load_state: HashMap<_, _> = apply_tasks.iter()
                                                       .enumerate()
                                                       .map(|(idx, &(ref name, _))| ((idx, name.clone()), LoadState::Idle))
                                                       .collect();
        let mut apply_tasks: HashMap<_, _> = apply_tasks.into_iter()
                                                        .enumerate()
                                                        .collect();
        let (sx, rx) = channel();

        let total_progress = apply_tasks.len() + async_tasks.iter()
                                                            .filter(|&&(_, b, _)| b)
                                                            .count();
        let mut current_progress = 0;

        for (idx, (name, real, async_t)) in async_tasks.into_iter().enumerate() {
            if real {
                let sx = sx.clone();
                let name_i = name.clone();
                thread::spawn(move || {
                    let _ = sx.send((idx, name_i, thread::spawn(move || async_t()).join()));
                });
                load_state.insert((idx, name), LoadState::Loading);
                G::draw_loading_screen(load_state.iter(),
                                       (current_progress, total_progress),
                                       self.window.cleared_frame(WHITE));
            } else {
                sx.send((idx, name.clone(), Ok(Ok(Box::new(None))))).unwrap();
            }
        }

        for (idx, name, res) in rx.iter().take(apply_tasks.len()) {
            let res = match res {
                Ok(r) => try!(r),
                Err(e) => panic!("The loading function '{}' crashed with message {:?}", name, e)
            };

            load_state.insert((idx, name.clone()), LoadState::Applying);
            current_progress += 1;
            G::draw_loading_screen(load_state.iter(),
                                   (current_progress, total_progress),
                                   self.window.cleared_frame(WHITE));

            let (_, apply_task) = apply_tasks.remove(&idx).unwrap();
            try!(apply_task(res, &mut self.window, &mut self.game));

            load_state.insert((idx, name), LoadState::Finished);
            current_progress += 1;
            G::draw_loading_screen(load_state.iter(),
                                   (current_progress, total_progress),
                                   self.window.cleared_frame(WHITE));
        }

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

    fn draw_timings(&self, frame: &mut Frame){
        fn draw_bars<B, I>(frame: &mut Frame, bars: I)
        where B: Iterator<Item=(f32, [f32; 4])>, I: ExactSizeIterator<Item=B>
        {
            let bar_count = bars.len();
            let bar_width = 1.0 / bar_count as f32;
            for (i, bar) in bars.enumerate() {
                let x = bar_width * i as f32;
                let mut y = 0.0;
                for (p, color) in bar {
                    frame.draw(Rectangle {
                        x: x, y: y,
                        w: bar_width, h: p,
                        color: color,
                        .. Default::default()
                    }).unwrap();
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
                frame.draw(Rectangle {
                    x: 0.0, y: 0.0,
                    w: 1.0, h: 1.0,
                    color: rgba(1.0, 1.0, 0.0, 0.8),
                    .. Default::default()
                }).unwrap();

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

        frame.draw(Rectangle{
            x: 0.0, y: h - HEIGHT,
            w: WIDTH, h: 1.0,
            color: rgb(0, 0, 0),
            .. Default::default()
        }).unwrap();

        let (fps, ups) = self.calc_fps();
        frame.with_translate(WIDTH, h, |frame| {
            frame.with_rotation(-3.1415 / 2.0, |frame| {
                /*
                frame.text(format!("FPS {} UPS {}", fps, ups), 0.0, 0.0)
                     .size(12)
                     .draw().unwrap();
                     */
            })
        });
    }
}

use std::collections::VecDeque;
use super::prelude::*;

trait Loadable<S> {
    fn step(&mut self, window: &mut Window, state: &mut S);
    fn describe(&self) -> &str;
}

pub struct GraphicalLoader<S> {
    window: Window,
    state: S,
    applications: VecDeque<Box<Loadable<S>>>,
    max: usize,
    cur_frame: Option<Frame>
}

impl <S, F: FnMut(&mut Window, &mut S), D: AsRef<str>> Loadable<S> for (D, F) {
    fn step(&mut self, window: &mut Window, state: &mut S) {
        let &mut (_, ref mut f) = self;
        f(window, state);
    }

    fn describe(&self) -> &str {
        let &(ref d, _) = self;
        d.as_ref()
    }
}

impl <S> GraphicalLoader<S> {
    pub fn new(window: Window, state: S) -> GraphicalLoader<S> {
        GraphicalLoader {
            window: window,
            state: state,
            applications: VecDeque::new(),
            max: 0,
            cur_frame: None
        }
    }

    pub fn load<D, F> (&mut self, desc: D, func: F)
        where D: AsRef<str> + 'static, F: FnMut(&mut Window, &mut S) + 'static {
        self.applications.push_back(Box::new((desc, func)) as
                                    Box<Loadable<S>>);
        self.max += 1;
    }

    fn before_step(&mut self, desc: &str) {
        let numer = self.max - self.applications.len() - 1;
        let text = format!("{} / {}: {}", numer, self.max, desc);
        let frame = self.cur_frame.as_mut().unwrap();

        let max = self.max;
        let (aw, ah) = frame.size();
        let text_height = (frame.height() / 2.0).ceil();

        frame.set_font("SourceCodePro", 20).unwrap();
        frame.with_color(rgb(0.9, 0.9, 0.9), |frame| {
            frame.rect(0.0, 0.0,
                       aw * (numer as f32 / max as f32), ah).fill();
            frame.set_color(rgb(0.0, 0.0, 0.0));
            frame.draw_text("LOADING", 0.0, text_height - 30.0).unwrap();
            frame.draw_text(&text[..], 0.0, text_height).unwrap();
        });
    }

    pub fn run(mut self) -> (Window, S) {
        while let Some(mut mutator) = self.applications.pop_front() {
            self.cur_frame = Some(self.window.cleared_frame([1.0, 1.0, 1.0, 1.0]));
            self.before_step(mutator.describe());
            self.cur_frame = None;

            mutator.step(&mut self.window, &mut self.state);
        }


        (self.window, self.state)
    }
}

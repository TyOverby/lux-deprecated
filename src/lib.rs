#![feature(phase)]
#![feature(unboxed_closures)]

#[phase(plugin)]
extern crate gfx_macros;
extern crate gfx;
extern crate render;
extern crate device;
extern crate glutin;
extern crate vecmath;

pub use window::gfx_integration::Vertex;

pub use render::{ ProgramError, ErrorVertex, ErrorFragment, ErrorLink };
pub use gfx::{ PrimitiveType, Point, Line, LineStrip,
               TriangleList, TriangleStrip, TriangleFan };

pub mod window;

pub trait Color {
    fn to_rgba(self) -> [f32, ..4];
}

#[deriving(Show)]
pub enum LuxError {
    WindowError(String),
    ShaderError(ProgramError)
}

pub type LuxResult<A> = Result<A, LuxError>;

pub trait Drawable {
    fn primitive(&self) -> PrimitiveType;
    fn vertices(&self) -> &Vec<Vertex>;
    fn texture(&self) -> Option<&()>;
    fn color(&self) -> Option<Color>;
}

pub trait LuxCanvas: LuxRaw {
    fn width(&self) -> i32;
    fn height(&self) -> i32;

    fn draw_rect(&mut self, pos: (f32, f32), size: (f32, f32));
    fn draw_border_rect(&mut self, pos: (f32, f32), size: (f32, f32), border_size: f32);

    fn draw_circle(&mut self, pos: (f32, f32), radius: f32);
    fn draw_border_circle(&mut self, pos: (f32, f32), radius: f32, border_size: f32);

    fn draw_elipse(&mut self, pos: (f32, f32), size: (f32, f32));
    fn draw_border_elipse(&mut self, pos: (f32, f32), size: (f32, f32), border_size: f32);

    fn draw_line(&mut self, start: (f32, f32), end: (f32, f32), line_size: f32);
    fn draw_lines(&mut self, positions: &[(f32, f32)], line_size: f32);
    fn draw_arc(&mut self, pos: (f32, f32), radius: f32, angle1: f32, angle2: f32);

    fn with_color<C: Color>(&mut self, color: C, f: |&mut Self| -> ());
    fn with_border_color<C: Color>(&mut self, color: C, f: |&mut Self| -> ());

    fn with_rotation(&mut self, rotation: f32, f: |&mut Self| -> ()) {
        self.push_matrix();
        self.rotate(rotation);
        f(self);
        self.pop_matrix();
    }
    fn with_translate(&mut self, dx: f32, dy: f32, f: |&mut Self| -> ()) {
        self.push_matrix();
        self.translate(dx, dy);
        f(self);
        self.pop_matrix();
    }
    fn with_scale(&mut self, scale_x: f32, scale_y: f32, f: |&mut Self| -> ()) {
        self.push_matrix();
        self.scale(scale_x, scale_y);
        f(self);
        self.pop_matrix();
    }
    fn with_shear(&mut self, sx: f32, sy: f32, f: |&mut Self| -> ()) {
        self.push_matrix();
        self.shear(sx, sy);
        f(self);
        self.pop_matrix();
    }

    fn draw<T: Drawable>(&mut self, figure: T);

    fn draw_text(&mut self, pos: (f32, f32), text: &str);
}

pub trait LuxWindow {
    fn is_open(&self) -> bool;
    fn title(&self) -> &str;
    fn set_title(&mut self, title: &str);
    fn set_size(&mut self, width: u32, height: u32);
    fn get_size(&self) -> (u32, u32);

    // Events
    fn is_focused(&self) -> bool;
    fn mouse_down(&self) -> bool;
    fn mouse_pos(&self) -> (i32, i32);
    fn mouse_x(&self) -> i32 {
        match self.mouse_pos() {
            (x, _) => x
        }
    }
    fn mouse_y(&self) -> i32 {
        match self.mouse_pos() {
            (_, y) => y
        }
    }
}

pub trait LuxRaw {
    fn current_matrix(&self) -> &[[f32, ..4], ..4];
    fn current_matrix_mut(&mut self) -> &mut [[f32, ..4], ..4];
    fn push_matrix(&mut self);
    fn pop_matrix(&mut self);
    fn with_matrix(&mut self, f: |&Self|) {
        self.push_matrix();
        f(self);
        self.pop_matrix();
    }
    fn apply_matrix(&mut self, matrix: [[f32, ..4], ..4]) {
        use vecmath::col_mat4_mul as multiply;
        let current = self.current_matrix_mut();
        *current = multiply(*current, matrix);
    }
    fn translate(&mut self, dx: f32, dy: f32) {
        let mut prod = vecmath::mat4_id();
        prod[3][0] = dx;
        prod[3][1] = dy;
        self.apply_matrix(prod);
    }
    fn scale(&mut self, sx: f32, sy: f32) {
        let mut prod = vecmath::mat4_id();
        prod[0][0] = sx;
        prod[1][1] = sy;
        self.apply_matrix(prod);
    }
    fn shear(&mut self, sx: f32, sy: f32) {
        let mut prod = vecmath::mat4_id();
        prod[1][0] = sx;
        prod[0][1] = sy;
        self.apply_matrix(prod);
    }
    fn rotate(&mut self, theta: f32) {
        let mut prod = vecmath::mat4_id();
        let (c, s) = (theta.cos(), theta.sin());
        prod[0][0] = c;
        prod[0][1] = s;
        prod[1][0] = -s;
        prod[1][1] = c;
        self.apply_matrix(prod);
    }
}

impl Color for [f32, ..4] {
    fn to_rgba(self) -> [f32, ..4] {
        self
    }
}

impl Color for [f32, ..3] {
    fn to_rgba(self) -> [f32, ..4] {
        match self {
            [r,g,b] => [r,g,b,1.0]
        }
    }
}

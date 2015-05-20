use std::rc::Rc;

use super::accessors::{Fetch, HasDisplay, HasSurface, HasDrawCache, DrawParamMod};
use super::gfx_integration::{ColorVertex, TexVertex};
use glium::index::PrimitiveType;
use super::color::Color;
use super::raw::Transform;
use super::gfx_integration;
use super::types::{Idx, Float};

use vecmath;
use glium;
use reuse_cache;


/// Signifies what state we are in with regards to drawing with stencils.
pub enum StencilState {
    /// We are currently drawing into the stencil buffer.
    DrawingStencil,
    /// We are currently drawing a shape that will be occluded by the stencil.
    DrawingWithStencil,
    /// We aren't doing anything with regards to stencils
    None
}

pub struct DrawParamModifier {
    pub scissor: Option<(u32, u32, u32, u32)>,
    pub stencil_state: StencilState
}

/// A cache for batching texture drawing commands.
///
/// This mechanism is used to reduce draw-calls in cases where
/// they could be more efficiently grouped together.
pub struct CachedColorDraw {
    /// The type of primitive that is being used to draw.
    pub typ: PrimitiveType,
    /// A cache of colored vertices.
    pub points: reuse_cache::Item<Vec<ColorVertex>>,
    /// A cache of indices indexing into the points cache.
    pub idxs: reuse_cache::Item<Vec<Idx>>,
}

/// A cache for batching texture drawing commands.
///
/// This mechanism is used to reduce draw-calls in cases where
/// they could be more efficiently grouped together.
pub struct CachedTexDraw {
    /// The type of primitive that is being used to draw.
    pub typ: PrimitiveType,
    /// A cache of colored vertices.
    pub points: reuse_cache::Item<Vec<TexVertex>>,
    /// The texture that is going to be bound for the draw call.
    pub texture: Rc<glium::texture::Texture2d>,
    /// A cache of indices indexing into the points cache.
    pub idxs: reuse_cache::Item<Vec<Idx>>,
    /// A color that will be multiplied against the values in the texture
    /// to give the texture color.
    pub color_mult: [Float; 4],
}

/// A Primitive canvas is a trait that is implemented by objects that
/// can have draw commands issued to them.
///
/// As the name implies, this is a lower-level API and you should probably
/// be using methods on the `Canvas` trait instead.
pub trait PrimitiveCanvas {
    /// Clears the canvas with a color.
    fn clear<C: Color>(&mut self, color: C);

    /// Draws the verteces to the canvas. This function uses caching to
    /// batch draw calls that are similar.
    ///
    /// typ: The primitive type used to draw the vertices.
    /// vs : A slice of vertices to be drawn.
    /// idxs: An optional list of indices that can be used to index into
    ///       the ColorVertex array.  Useful if you have many points that are
    ///       duplicates of each other.
    /// mat: An optional transformation matrix that would be applied to the
    ///      each point before drawing.
    fn draw_colored(&mut self,
                  typ: PrimitiveType,
                  vs: &[ColorVertex],
                  idxs: Option<&[Idx]>,
                  mat: Option<[[Float; 4]; 4]>);

    /// Draws colored vertices to the canvas with no thought given to the
    /// cached draw commands.
    ///
    /// This function is meant for internal use and shouldn't regularly show
    /// up in user code.  Instead, prefer `draw_colored` or `draw_colored_no_batch`
    fn draw_colored_now(&mut self,
                typ: PrimitiveType,
                points: &[ColorVertex],
                idxs: Option<&[Idx]>,
                base_mat: Option<[[Float; 4]; 4]>);

    /// Immediately draws colored vertices to the canvas.
    ///
    /// The vertex batch cache is cleared before drawing.
    /// These vertices are not cached.
    fn draw_colored_no_batch(&mut self,
                           typ: PrimitiveType,
                           vs: &[ColorVertex],
                           idxs: Option<&[Idx]>,
                           mat: Option<[[Float; 4]; 4]>);

    /// Same as `draw_colored` but for textured vertices.
    fn draw_tex(&mut self,
                typ: PrimitiveType,
                vs: &[TexVertex],
                idxs: Option<&[Idx]>,
                mat: Option<[[Float; 4]; 4]>,
                Rc<glium::texture::Texture2d>,
                color_mult: Option<[Float; 4]>);

    /// Same as `draw_colored_now` but for textured vertices.
    fn draw_textured_now(&mut self,
                typ: PrimitiveType,
                points: &[TexVertex],
                idxs: Option<&[Idx]>,
                base_mat: Option<[[Float; 4]; 4]>,
                texture: &glium::texture::Texture2d,
                color_mult: [Float; 4]);

    /// Same as `draw_colored_no_batch` but for textured vertices.
    fn draw_tex_no_batch(&mut self,
                         typ: PrimitiveType,
                         vs: &[TexVertex],
                         idxs: Option<&[Idx]>,
                         mat: Option<[[Float; 4]; 4]>,
                         &glium::texture::Texture2d,
                         color_mult: Option<[Float; 4]>);

    /// Flush all stored draw calls to the screen.
    ///
    /// This is an interal function that should not usually be called
    /// by the user of this library.
    fn flush_draw(&mut self);
}

fn draw_params<C: DrawParamMod>(c: &C) -> glium::DrawParameters<'static> {
        use glium::LinearBlendingFactor::*;
        let defaults: glium::DrawParameters = ::std::default::Default::default();

        glium::DrawParameters {
            depth_test: glium::DepthTest::Overwrite,
            blending_function: Some(glium::BlendingFunction::Addition{
                source: SourceAlpha,
                destination: OneMinusSourceAlpha
            }),
            backface_culling: glium::BackfaceCullingMode::CullingDisabled,
            multisampling: true,
            scissor: c.scissor().map(|a|
                glium::Rect{
                    left: a.0,
                    bottom: a.1,
                    width: a.2,
                    height: a.3
                }),
            ..defaults
        }
}

impl DrawParamModifier {
    pub fn new() -> DrawParamModifier {
        DrawParamModifier {
            scissor: None,
            stencil_state: StencilState::None
        }
    }
}

impl <T> PrimitiveCanvas for T where T: HasDisplay + HasSurface + HasDrawCache +
DrawParamMod + Transform + Fetch<Vec<Idx>> + Fetch<Vec<TexVertex>> +
Fetch<Vec<ColorVertex>>
{
    fn clear<C: Color>(&mut self, color: C) {
        use glium::Surface;
        let c = color.to_rgba();
        self.surface().clear_color(c[0], c[1], c[2], c[3]);
    }

    fn draw_colored_now(&mut self,
                typ: PrimitiveType,
                points: &[ColorVertex],
                idxs: Option<&[Idx]>,
                base_mat: Option<[[Float; 4]; 4]>) {
        use glium::{Surface, IndexBuffer};
        use glium::index::NoIndices;

        let vertex_buffer = glium::VertexBuffer::new(self.borrow_display(), points);

        let uniform = gfx_integration::ColorParams {
            matrix: base_mat.unwrap_or(vecmath::mat4_id())
        };

        let draw_params = draw_params(self as &T);

        match idxs {
            Some(idxs) => {
                let idx_buf = IndexBuffer::from_raw(self.borrow_display(), idxs, typ);
                let (frame, color_program) = self.surface_and_color_shader();
                frame.draw(&vertex_buffer, &idx_buf, &color_program, &uniform, &draw_params).unwrap();
            }
            None => {
                let (frame, color_program) = self.surface_and_color_shader();
                frame.draw(&vertex_buffer, &NoIndices(typ), &color_program, &uniform, &draw_params).unwrap();
            }
        }
    }

    fn draw_textured_now(&mut self,
                typ: PrimitiveType,
                points: &[TexVertex],
                idxs: Option<&[Idx]>,
                base_mat: Option<[[Float; 4]; 4]>,
                texture: &glium::texture::Texture2d,
                color_mult: [Float; 4]) {
        use glium::{Surface, IndexBuffer};
        use glium::index::NoIndices;

        let vertex_buffer = glium::VertexBuffer::new(self.borrow_display(), points);

        let uniform = gfx_integration::TexParams {
            matrix: base_mat.unwrap_or(vecmath::mat4_id()),
            tex: texture,
            color_mult: color_mult
        };

        let draw_params = draw_params(self);

        match idxs {
            Some(idxs) => {
                let idx_buf = IndexBuffer::from_raw(self.borrow_display(), idxs, typ);
                let (frame, tex_program) = self.surface_and_texture_shader();
                frame.draw(&vertex_buffer, &idx_buf, &tex_program, &uniform, &draw_params).unwrap();
            }
            None => {
                let (frame, tex_program) = self.surface_and_texture_shader();
                frame.draw(&vertex_buffer, &NoIndices(typ), &tex_program, &uniform, &draw_params).unwrap();
            }
        }
        // TODO: returrn error?
    }

    fn flush_draw(&mut self) {
        if let Some(CachedColorDraw{typ, points, idxs}) =
            self.color_draw_cache_mut().take() {
                self.draw_colored_now(typ, &points, Some(&idxs), None);
        }
        if let Some(CachedTexDraw{typ, points, texture, idxs, color_mult}) =
            self.tex_draw_cache_mut().take() {
                self.draw_textured_now(typ, &points, Some(&idxs), None, &*texture, color_mult);
        }
    }

    fn draw_colored_no_batch(&mut self,
                           n_typ: PrimitiveType,
                           n_points: &[ColorVertex],
                           idxs: Option<&[Idx]>,
                           transform: Option<[[Float; 4]; 4]>) {
        self.flush_draw();
        let transform = match transform {
            Some(t) => vecmath::col_mat4_mul(*self.current_matrix(), t),
            None => *self.current_matrix()
        };
        self.draw_colored_now(n_typ, n_points, idxs, Some(transform));
    }

    fn draw_tex_no_batch(&mut self,
                           n_typ: PrimitiveType,
                           n_points: &[TexVertex],
                           idxs: Option<&[Idx]>,
                           transform: Option<[[Float; 4]; 4]>,
                           texture: &glium::texture::Texture2d,
                           color_mult: Option<[Float; 4]>) {
        self.flush_draw();
        let transform = match transform {
            Some(t) => vecmath::col_mat4_mul(*self.current_matrix(), t),
            None => *self.current_matrix()
        };
        let color_mult = color_mult.unwrap_or([1.0, 1.0, 1.0, 1.0]);
        self.draw_textured_now(n_typ, n_points, idxs, Some(transform), texture, color_mult);
    }


    fn draw_tex(&mut self,
                  n_typ: PrimitiveType,
                  n_points: &[TexVertex],
                  idxs: Option<&[Idx]>,
                  transform: Option<[[Float; 4]; 4]>,
                  texture: Rc<glium::texture::Texture2d>,
                  color_mult: Option<[Float; 4]>) {
        use glium::index::PrimitiveType::{Points, LinesList, TrianglesList};
        use std::mem::transmute;

        if self.color_draw_cache().is_some() {
            self.flush_draw();
        }
        let color_mult = color_mult.unwrap_or([1.0, 1.0, 1.0, 1.0]);

        // Look at all this awful code for handling something that should
        // be dead simple!
        if self.tex_draw_cache().is_some() {
            let mut same_type;
            let mut coherant_group;
            let mut same_color_mult;
            let mut same_tex;
            {
                let draw_cache = self.tex_draw_cache().as_ref().unwrap();
                same_type = draw_cache.typ == n_typ;
                coherant_group = match n_typ {
                    Points | LinesList | TrianglesList => true,
                    _ => false
                };
                same_color_mult = draw_cache.color_mult == color_mult;

                let our_ptr: *mut () = unsafe {transmute(&*draw_cache.texture)};
                let otr_ptr: *mut () = unsafe {transmute(&*texture)};
                same_tex = our_ptr == otr_ptr;
            }

            if !same_type || !coherant_group || !same_color_mult || !same_tex {
                self.flush_draw();
                *self.tex_draw_cache_mut() = Some(CachedTexDraw {
                    typ: n_typ,
                    points: self.fetch(),
                    idxs: self.fetch(),
                    texture: texture,
                    color_mult: color_mult,
                });
            }
        } else {
            *self.tex_draw_cache_mut() = Some(CachedTexDraw {
                typ: n_typ,
                points: self.fetch(),
                idxs: self.fetch(),
                texture: texture,
                color_mult: color_mult
            });
        }

        if let Some(idxs) = idxs {
            assert!(idxs.len() % 3 == 0,
                "The length of the indexes array must be a multiple of three.");
        }

        let transform = transform.unwrap_or(vecmath::mat4_id());
        let mat = vecmath::col_mat4_mul(*self.current_matrix(), transform);
        let draw_cache = self.tex_draw_cache_mut().as_mut().unwrap();

        let already_in = draw_cache.points.len() as Idx;
        let adding = n_points.len() as Idx;

        // Perform the global transforms here
        draw_cache.points.extend(n_points.iter().map(|&point| {
            let mut point = point.clone();
            let res = vecmath::col_mat4_transform(
                mat,
                [point.pos[0], point.pos[1], 0.0, 1.0]);
            point.pos = [res[0], res[1]];
            point
        }));

        // TODO: test this
        // TODO: replace most of this with 'extend' and 'map'.

        match idxs {
            None => {
                for i in 0 .. adding {
                    draw_cache.idxs.push(already_in + i)
                }
            }
            Some(l_idxs) => {
                for &i in l_idxs.iter() {
                    draw_cache.idxs.push(already_in + i);
                }
            }
        }
    }

    fn draw_colored(&mut self,
                  n_typ: PrimitiveType,
                  n_points: &[ColorVertex],
                  idxs: Option<&[Idx]>,
                  transform: Option<[[Float; 4]; 4]>) {
        use glium::index::PrimitiveType::{Points, LinesList, TrianglesList};

        if self.tex_draw_cache().is_some() {
            self.flush_draw();
        }

        // Look at all this awful code for handling something that should
        // be dead simple!
        if self.color_draw_cache().is_some() {
            let same_type = self.color_draw_cache().as_ref().unwrap().typ == n_typ;
            let coherant_group = match n_typ {
                Points | LinesList | TrianglesList => true,
                _ => false
            };
            if !same_type || !coherant_group {
                self.flush_draw();
                *self.color_draw_cache_mut() = Some(CachedColorDraw {
                    typ: n_typ,
                    points: self.fetch(),
                    idxs: self.fetch()
                });
            }
        } else {
            *self.color_draw_cache_mut() = Some(CachedColorDraw {
                typ: n_typ,
                points: self.fetch(),
                idxs: self.fetch()

            });
        }

        if let Some(idxs) = idxs {
            assert!(idxs.len() % 3 == 0,
                "The length of the indexes array must be a multiple of three.");
        }

        let transform = transform.unwrap_or(vecmath::mat4_id());
        let mat = vecmath::col_mat4_mul(*self.current_matrix(), transform);
        let draw_cache = self.color_draw_cache_mut().as_mut().unwrap();

        let already_in = draw_cache.points.len() as Idx;
        let adding = n_points.len() as Idx;

        // Perform the global transforms here
        draw_cache.points.extend(n_points.iter().map(|&point| {
            let mut point = point.clone();
            let res = vecmath::col_mat4_transform(
                mat,
                [point.pos[0], point.pos[1], 0.0, 1.0]);
            point.pos = [res[0], res[1]];
            point
        }));

        // TODO: test this
        // TODO: replace most of this with 'extend' and 'map'.

        match idxs {
            None => {
                for i in 0 .. adding {
                    draw_cache.idxs.push(already_in + i)
                }
            }
            Some(l_idxs) => {
                for &i in l_idxs.iter() {
                    draw_cache.idxs.push(already_in + i);
                }
            }
        }
    }
}

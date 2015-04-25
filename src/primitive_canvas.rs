use std::rc::Rc;
use super::accessors::{
    HasDisplay,
    HasSurface,
    HasDrawCache
};

use super::prelude::{
    ColorVertex,
    TexVertex,
    PrimitiveType,
    Color,
    Points,
    Transform
};
use super::gfx_integration;

use vecmath;
use glium;

struct Indices<'a, T: 'a + glium::index::Index> {
    idxs: Option<&'a [T]>,
    prim: glium::index::PrimitiveType
}

impl <'a, T: 'a + glium::index::Index> Indices<'a, T> {
    fn new(prim: glium::index::PrimitiveType, idxs: Option<&'a [T]>) -> Indices<'a, T> {
        Indices {
            idxs: idxs,
            prim: prim
        }
    }
}

impl <'a, T: 'a + glium::index::Index> glium::index::ToIndicesSource for Indices<'a, T> {
    type Data = T;
    fn to_indices_source<'b>(&'b self) -> glium::index::IndicesSource<'b, T> {
        match self.idxs {
            Some(slice) => {
                glium::index::IndicesSource::Buffer {
                    pointer: slice,
                    primitives: self.prim,
                    offset: 0,
                    length: slice.len()
                }
            }
            None => {
                glium::index::IndicesSource::NoIndices {
                    primitives: self.prim
                }
            }
        }
    }
}

pub struct CachedColorDraw {
    pub typ: PrimitiveType,
    pub points: Vec<ColorVertex>,
    pub idxs: Vec<u32>,
}

pub struct CachedTexDraw {
    pub typ: PrimitiveType,
    pub points: Vec<TexVertex>,
    pub texture: Rc<glium::texture::Texture2d>,
    pub idxs: Vec<u32>,
    pub color_mult: [f32; 4],
}


/// A primitive canvas is a canvas that can be drawn to with only the
/// `draw_shape` function.
pub trait PrimitiveCanvas {
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
    fn draw_shape(&mut self,
                  typ: PrimitiveType,
                  vs: &[ColorVertex],
                  idxs: Option<&[u32]>,
                  mat: Option<[[f32; 4]; 4]>);

    fn draw_colored_now(&mut self,
                typ: PrimitiveType,
                points: &[ColorVertex],
                idxs: Option<&[u32]>,
                base_mat: Option<[[f32; 4]; 4]>);

    fn draw_textured_now(&mut self,
                typ: PrimitiveType,
                points: &[TexVertex],
                idxs: Option<&[u32]>,
                base_mat: Option<[[f32; 4]; 4]>,
                texture: &glium::texture::Texture2d,
                color_mult: [f32; 4]);

    /// Flush all stored draw calls to the screen.
    fn flush_draw(&mut self);

    fn draw_shape_no_batch(&mut self,
                           typ: PrimitiveType,
                           vs: &[ColorVertex],
                           idxs: Option<&[u32]>,
                           mat: Option<[[f32; 4]; 4]>);

    fn draw_tex(&mut self,
                typ: PrimitiveType,
                vs: &[TexVertex],
                idxs: Option<&[u32]>,
                mat: Option<[[f32; 4]; 4]>,
                Rc<glium::texture::Texture2d>,
                color_mult: Option<[f32; 4]>);

    fn draw_tex_no_batch(&mut self,
                         typ: PrimitiveType,
                         vs: &[TexVertex],
                         idxs: Option<&[u32]>,
                         mat: Option<[[f32; 4]; 4]>,
                         &glium::texture::Texture2d,
                         color_mult: Option<[f32; 4]>);
}

fn draw_params() -> glium::DrawParameters {
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
            ..defaults
        }
}

impl <T> PrimitiveCanvas for T where T: HasDisplay + HasSurface + HasDrawCache + Transform {
    fn clear<C: Color>(&mut self, color: C) {
        use glium::Surface;
        let c = color.to_rgba();
        self.surface().clear_color(c[0], c[1], c[2], c[3]);
    }

    fn draw_colored_now(&mut self,
                typ: PrimitiveType,
                points: &[ColorVertex],
                idxs: Option<&[u32]>,
                base_mat: Option<[[f32; 4]; 4]>) {
        use glium::index::*;
        use glium::index::PrimitiveType as Prim;
        use glium::Surface;

        let vertex_buffer = glium::VertexBuffer::new(self.borrow_display(), points);
        let (frame, color_program) = self.surface_and_color_shader();

        let uniform = gfx_integration::ColorParams {
            matrix: base_mat.unwrap_or(vecmath::mat4_id())
        };

        let draw_params = draw_params();

        let idx = Indices::new(typ, idxs);

        frame.draw(&vertex_buffer, &idx, &color_program, &uniform, &draw_params).unwrap();
    }

    fn draw_textured_now(&mut self,
                typ: PrimitiveType,
                points: &[TexVertex],
                idxs: Option<&[u32]>,
                base_mat: Option<[[f32; 4]; 4]>,
                texture: &glium::texture::Texture2d,
                color_mult: [f32; 4]) {
        use glium::index::*;
        use glium::index::PrimitiveType as Prim;
        use glium::Surface;

        let vertex_buffer = glium::VertexBuffer::new(self.borrow_display(), points);
        let (frame, tex_program) = self.surface_and_texture_shader();

        let uniform = gfx_integration::TexParams {
            matrix: base_mat.unwrap_or(vecmath::mat4_id()),
            texture: texture,
            color_mult: color_mult
        };

        let draw_params = draw_params();

        let idx = Indices::new(typ, idxs);

        frame.draw(&vertex_buffer, &idx, &tex_program, &uniform, &draw_params).unwrap();
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

    fn draw_shape_no_batch(&mut self,
                           n_typ: PrimitiveType,
                           n_points: &[ColorVertex],
                           idxs: Option<&[u32]>,
                           transform: Option<[[f32; 4]; 4]>) {
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
                           idxs: Option<&[u32]>,
                           transform: Option<[[f32; 4]; 4]>,
                           texture: &glium::texture::Texture2d,
                           color_mult: Option<[f32; 4]>) {
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
                  idxs: Option<&[u32]>,
                  transform: Option<[[f32; 4]; 4]>,
                  texture: Rc<glium::texture::Texture2d>,
                  color_mult: Option<[f32; 4]>) {
        use super::prelude::PrimitiveType::{Points, LinesList, TrianglesList};
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
                    points: Vec::with_capacity(1024),
                    idxs: Vec::with_capacity(1024),
                    texture: texture,
                    color_mult: color_mult,
                });
            }
        } else {
            *self.tex_draw_cache_mut() = Some(CachedTexDraw {
                typ: n_typ,
                points: Vec::with_capacity(1024),
                idxs: Vec::with_capacity(1024),
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

        let already_in = draw_cache.points.len() as u32;
        let adding = n_points.len() as u32;

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

    fn draw_shape(&mut self,
                  n_typ: PrimitiveType,
                  n_points: &[ColorVertex],
                  idxs: Option<&[u32]>,
                  transform: Option<[[f32; 4]; 4]>) {
        use super::prelude::PrimitiveType::{Points, LinesList, TrianglesList};

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
                    points: Vec::with_capacity(1024),
                    idxs: Vec::with_capacity(1024),
                });
            }
        } else {
            *self.color_draw_cache_mut() = Some(CachedColorDraw {
                typ: n_typ,
                points: Vec::with_capacity(1024),
                idxs: Vec::with_capacity(1024),

            });
        }

        if let Some(idxs) = idxs {
            assert!(idxs.len() % 3 == 0,
                "The length of the indexes array must be a multiple of three.");
        }

        let transform = transform.unwrap_or(vecmath::mat4_id());
        let mat = vecmath::col_mat4_mul(*self.current_matrix(), transform);
        let draw_cache = self.color_draw_cache_mut().as_mut().unwrap();

        let already_in = draw_cache.points.len() as u32;
        let adding = n_points.len() as u32;

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

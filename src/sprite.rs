use glium;
use image;

use std::rc::Rc;
use std::ops::Deref;
use std::collections::HashMap;
use std::borrow::Borrow;
use std::convert::AsRef;
use std::path::Path;

use std::cmp::Eq;
use std::hash::Hash;

use ::accessors::{HasDisplay, HasPrograms, HasSurface, HasDrawCache};
use ::prelude::{TexVertex, Figure, LuxCanvas, TrianglesList, Transform, Colored, Color};
use ::primitive_canvas::{CachedColorDraw, CachedTexDraw};

use vecmath;

use image::ImageError;

/// An owned texture on the hardware.
///
/// Only one Texture can be used at a time.
pub struct Texture {
    backing: glium::texture::Texture2d,
}

#[derive(Clone, Debug)]
pub struct Sprite {
    texture: Rc<glium::texture::Texture2d>,
    original_size: (u32, u32),

    size: (u32, u32),
    pos: (u32, u32),

    texture_size: (f32, f32),
    texture_pos: (f32, f32),
}

pub struct DrawableTexture<'a, D: 'a + HasDisplay + HasPrograms> {
    texture: glium::texture::TextureSurface<'a>,
    d: &'a D,
    matrix: [[f32; 4]; 4],

    color_draw_cache: Option<CachedColorDraw>,
    tex_draw_cache: Option<CachedTexDraw>,

    color: [f32; 4],
}

#[derive(Clone, Debug)]
pub struct UniformSpriteSheet {
    pub sprite: Sprite,
    divs: (u32, u32),
    indiv_size: (u32, u32),
}

#[derive(Clone, Debug)]
pub struct NonUniformSpriteSheet<K: Hash + Eq> {
    pub sprite: Sprite,
    pub mapping: HashMap<K, Sprite>
}

pub trait TextureLoader {
    fn load_texture_file<P: AsRef<Path> + ?Sized>(&mut self, path: &P) -> Result<Texture, ImageError>;

    fn texture_from_pixels(&mut self, Vec<Vec<[f32; 4]>>) -> Texture;
    fn texture_from_image(&mut self, img: image::DynamicImage) -> Texture;
}

impl <T> TextureLoader for T where T: HasDisplay {
    fn load_texture_file<P: AsRef<Path> + ?Sized>(&mut self, path: &P) -> Result<Texture, ImageError> {
        let img = try!(image::open(path)).flipv();
        let img = glium::texture::Texture2d::new(self.borrow_display(), img);
        Ok(Texture::new(img))
    }

    fn texture_from_pixels(&mut self, pixels: Vec<Vec<[f32; 4]>>) -> Texture {
        let pixels: Vec<Vec<(f32, f32, f32, f32)>> = unsafe {::std::mem::transmute(pixels)};
        Texture::new(glium::texture::Texture2d::new(self.borrow_display(), pixels))
    }

    fn texture_from_image(&mut self, img: image::DynamicImage) -> Texture {
        let img = img.flipv();
        let img = glium::texture::Texture2d::new(self.borrow_display(), img);
        Texture::new(img)
    }
}

impl Texture {
    pub fn empty<D: HasDisplay>(d: &D, width: u32, height: u32) -> Texture {
        use glium::Surface;
        let backing = glium::texture::Texture2d::empty(d.borrow_display(), width, height);
        {
            let mut s = backing.as_surface();
            s.clear_all((1.0, 1.0, 0.0, 1.0), 0.0, 0);
        }
        Texture {
            backing: backing
        }
    }

    fn new(texture: glium::texture::Texture2d) -> Texture {
        Texture {
            backing: texture
        }
    }

    pub fn into_sprite(self) -> Sprite {
        Sprite::new(Rc::new(self.backing))
    }

    pub fn as_drawable_texture<'a, D>(&'a mut self, d: &'a D) -> DrawableTexture<'a, D>
    where D: HasDisplay + HasPrograms {
        DrawableTexture::new(self.backing.as_surface(), d)
    }
}

impl <'a, D> DrawableTexture<'a, D>  where D: HasDisplay + HasPrograms {
    fn new(texture: glium::texture::TextureSurface<'a>, d: &'a D)
    -> DrawableTexture<'a, D> {
        DrawableTexture {
            texture: texture,
            d: d,
            matrix: vecmath::mat4_id(),
            color_draw_cache: None,
            tex_draw_cache: None,
            color: [0.0, 0.0, 0.0, 1.0],
        }
    }
}

impl <'a, D> Colored for DrawableTexture<'a, D> where D: HasDisplay + HasPrograms{
    fn color(&self) -> [f32; 4] {
        self.color
    }

    fn set_color<C: Color>(&mut self, color: C) -> &mut DrawableTexture<'a, D> {
        self.color = color.to_rgba();
        self
    }
}

impl <'a, D> Transform for DrawableTexture<'a, D> where D: HasDisplay + HasPrograms {
    fn current_matrix(&self) -> &[[f32; 4]; 4] {
        &self.matrix
    }
    fn current_matrix_mut(&mut self) -> &mut [[f32; 4]; 4] {
        &mut self.matrix
    }
}

impl <'a, D> HasDisplay for DrawableTexture<'a, D> where D: HasDisplay + HasPrograms {
    fn borrow_display(&self) -> &glium::Display {
        &self.d.borrow_display()
    }
}

impl <'a, D> HasSurface for DrawableTexture<'a, D> where D: HasDisplay + HasPrograms {
    type Out = glium::texture::TextureSurface<'a>;

    fn surface(&mut self) -> &mut Self::Out {
        &mut self.texture
    }

    fn surface_and_texture_shader(&mut self) -> (&mut Self::Out, &glium::Program) {
        (&mut self.texture, self.d.texture_shader())
    }
    fn surface_and_color_shader(&mut self) -> (&mut Self::Out, &glium::Program) {
        (&mut self.texture, self.d.color_shader())
    }
}

impl <'a, D> HasDrawCache for DrawableTexture<'a, D> where D: HasPrograms + HasDisplay {
    fn color_draw_cache(&self) -> &Option<CachedColorDraw> {
        &self.color_draw_cache
    }

    fn tex_draw_cache(&self) -> &Option<CachedTexDraw> {
        &self.tex_draw_cache
    }

    fn color_draw_cache_mut(&mut self) -> &mut Option<CachedColorDraw> {
        &mut self.color_draw_cache
    }

    fn tex_draw_cache_mut(&mut self) -> &mut Option<CachedTexDraw> {
        &mut self.tex_draw_cache
    }
}

impl <'a, D> LuxCanvas for DrawableTexture<'a, D> where D: HasDisplay + HasPrograms {
    fn size(&self) -> (f32, f32) {
        use glium::Surface;
        let (w, h) = self.texture.get_dimensions();
        (w as f32, h as f32)
    }
}

impl <'a, D> Drop for DrawableTexture<'a, D> where D: HasDisplay + HasPrograms {
    fn drop (&mut self) {
        use primitive_canvas::PrimitiveCanvas;
        println!("dropping!");
        self.flush_draw();
    }
}

impl Sprite {
    // TODO: hide from docs
    pub fn new(tex: Rc<glium::texture::Texture2d>) -> Sprite {
        use glium::Surface;
        let size = tex.as_surface().get_dimensions();
        Sprite {
            texture: tex,
            original_size: size,
            size: size,
            pos: (0, 0),

            texture_size: (1.0, 1.0),
            texture_pos: (0.0, 0.0)
        }
    }

    pub fn ideal_size(&self) -> (f32, f32) {
        let (w, h) = self.size;
        (w as f32, h as f32)
    }

    pub fn sub_sprite(&self, offset: (u32, u32), size: (u32, u32)) -> Option<Sprite> {
        if offset.0 + size.0 > self.size.0 { return None };
        if offset.1 + size.1 > self.size.1 { return None };

        let pos = (self.pos.0 + offset.0, self.pos.1 + offset.1);

        Some(Sprite {
            texture: self.texture.clone(),
            original_size: self.original_size,

            size: size,
            pos: pos,

            texture_size: (size.0 as f32 / self.original_size.0 as f32,
                           size.1 as f32 / self.original_size.1 as f32),
            texture_pos: (pos.0 as f32 / self.original_size.0 as f32,
                          pos.1 as f32 / self.original_size.1 as f32)
        })
    }

    pub fn original_sprite(&self) -> Sprite {
        Sprite {
            texture: self.texture.clone(),
            original_size: self.original_size,
            size: self.original_size,
            pos: (0, 0),
            texture_size: (1.0, 1.0),
            texture_pos: (0.0, 0.0)
        }
    }

    pub fn bounds(&self) -> [[f32; 2]; 4]{
        let top_left = [self.texture_pos.0,
                        self.texture_pos.1];
        let top_right = [self.texture_pos.0 + self.texture_size.0,
                         self.texture_pos.1];
        let bottom_left = [self.texture_pos.0,
                           self.texture_pos.1 + self.texture_size.1];
        let bottom_right= [self.texture_pos.0 + self.texture_size.0,
                           self.texture_pos.1 + self.texture_size.1];

        [top_left, top_right, bottom_left, bottom_right]
    }

    pub fn texture(&self) -> Rc<glium::texture::Texture2d> {
        self.texture.clone()
    }

    pub fn texture_ref(&self) -> &glium::texture::Texture2d {
        self.texture.deref()
    }

    pub fn zeroed_vertices(&self) -> (Vec<TexVertex>, Vec<u32>) {
        let bounds = self.bounds();

        let top_left = bounds[0];
        let top_right = bounds[1];
        let bottom_left = bounds[2];
        let bottom_right = bounds[3];

        (vec![
                TexVertex {pos: [1.0, 0.0], tex_coords: top_right},
                TexVertex {pos: [0.0, 0.0], tex_coords: top_left},
                TexVertex {pos: [0.0, 1.0], tex_coords: bottom_left},
                TexVertex {pos: [1.0, 1.0], tex_coords: bottom_right},
             ],
             vec![0u32, 1, 2, 0, 2, 3]
        )
    }

    pub fn as_uniform_sprite_sheet(&self, indiv_width: u32, indiv_height: u32)
    -> UniformSpriteSheet {
        UniformSpriteSheet::new(self.clone(), indiv_width, indiv_height)
    }

    pub fn as_nonuniform_sprite_sheet<T>(&self) -> NonUniformSpriteSheet<T>
    where T: Eq + Hash {
        NonUniformSpriteSheet::new(self.clone())
    }
}

impl Figure for Sprite {
    fn draw<C: LuxCanvas>(&self, canvas: &mut C) {
        let bounds = self.bounds();

        let top_left = bounds[0];
        let top_right = bounds[1];
        let bottom_left = bounds[2];
        let bottom_right = bounds[3];

        let tex_vs = vec![
            TexVertex {pos: [1.0, 0.0], tex_coords: top_right},
            TexVertex {pos: [0.0, 0.0], tex_coords: top_left},
            TexVertex {pos: [0.0, 1.0], tex_coords: bottom_left},
            TexVertex {pos: [1.0, 1.0], tex_coords: bottom_right},
        ];

        let idxs = [0u32, 1, 2, 0, 2, 3];

        canvas.draw_tex(TrianglesList,
                        &tex_vs[..],
                        Some(&idxs[..]),
                        None,
                        self.texture(),
                        None);
    }
}

/// A sprite sheet that that is laid out in a grid where every grid is the
/// same height and width.
impl UniformSpriteSheet {
    /// Creates a new sprite-sheet that is divided into a `div_x` by `div_y`
    /// grid.
    fn new(sprite: Sprite, div_x: u32, div_y: u32) -> UniformSpriteSheet {
        let indiv_width = sprite.size.0 / div_x;
        let indiv_height = sprite.size.1 / div_y;
        UniformSpriteSheet{
            sprite: sprite,
            divs: (div_x, div_y),
            indiv_size: (indiv_width, indiv_height)
        }
    }

    /// Returns the number of divisions made in the x and y direction.
    pub fn num_divs(&self) -> (u32, u32) {
        self.divs
    }

    /// Gets the sprite that is in the (x, y) position in the grid
    /// defined by this sprite sheet.
    ///
    /// ## Failure
    /// Fails if out of bounds.
    pub fn get(&self, x: u32, y: u32) -> Sprite {
        self.get_opt(x, y).unwrap()
    }

    /// Same as `get` but returns None instead of failing if
    /// the sprite is out of bounds.
    pub fn get_opt(&self, x: u32, y: u32) -> Option<Sprite> {
        let x_tex = x * self.indiv_size.0;
        let y_tex = y * self.indiv_size.1;

        self.sprite.sub_sprite((x_tex, y_tex), self.indiv_size)
    }
}

/// A non-uniform spritesheet is a sprite-sheet that is
/// indexable by arbitrary keys.
impl <K: Eq + Hash> NonUniformSpriteSheet<K> {
    /// Creates a new non-uniform spritesheet based off of this sprite.
    fn new(sprite: Sprite) -> NonUniformSpriteSheet<K> {
        NonUniformSpriteSheet {
            sprite: sprite,
            mapping: HashMap::new()
        }
    }

    /// Associates a key with a sprite location.
    pub fn associate(&mut self, key: K, pos: (u32, u32), size: (u32, u32)) {
        self.mapping.insert(key, self.sprite.sub_sprite(pos, size).unwrap());
    }

    /// Gets the sprite that is associated with a key.
    ///
    /// ## Failure
    /// Fails if the key doesn't associate to something yet.
    pub fn get<Q: ?Sized>(&self, key: &Q) -> Sprite
    where Q: Hash + Eq + ::std::fmt::Debug, K: Borrow<Q> {
        match self.get_opt(key) {
            Some(v) => v,
            None => panic!("No Key found for {:?}", key)
        }
    }

    /// Same as `get` but returns None instead of failing if the key
    /// doesn't associate to anything.
    pub fn get_opt<Q: ?Sized>(&self, key: &Q) -> Option<Sprite>
    where Q: Hash + Eq, K: Borrow<Q> {
        self.mapping.get(key).map(|a| a.clone())
    }
}

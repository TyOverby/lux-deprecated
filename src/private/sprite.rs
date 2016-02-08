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

use super::types::{Float, Idx};
use super::accessors::{
    HasDisplay,
    HasPrograms,
    HasSurface,
    HasDrawCache,
    Fetch,
    DrawParamMod
};
use super::error::{LuxError, LuxResult};
use super::gfx_integration::{TexVertex, ColorVertex};
use super::canvas::Canvas;
use super::raw::{Transform, Colored};
use super::color::Color;
use super::primitive_canvas::{CachedColorDraw, CachedTexDraw, DrawParamModifier};

use vecmath;
use poison_pool;

/// An owned texture on the hardware.
pub struct Texture {
    backing: glium::texture::Texture2d,
}


/// A ref-counted reference to a texture on the GPU.
///
/// This sprite can be cheaply cloned, resized, sliced, and drawn.
#[derive(Clone, Debug)]
pub struct Sprite {
    texture: Rc<glium::texture::Texture2d>,
    original_size: (u32, u32),

    size: (u32, u32),
    pos: (u32, u32),

    texture_size: (Float, Float),
    texture_pos: (Float, Float),
}

// TODO: add font rendering
/// A texture that can be drawn to like a regular canvas.
///
/// A DrawableTexture can be obtained by calling `as_drawable` on a `Texture`
/// object.
pub struct DrawableTexture<'a, D: 'a + HasDisplay + HasPrograms> {
    texture: glium::framebuffer::SimpleFrameBuffer<'a>,
    d: &'a D,

    matrix: [[Float; 4]; 4],
    color: [Float; 4],

    color_draw_cache: Option<CachedColorDraw>,
    tex_draw_cache: Option<CachedTexDraw>,
    draw_mod: DrawParamModifier
}

/// A uniform sprite sheet is a sprite sheet that is broken up into
/// a grid of equally sized sub-sprites.
///
/// A `UniformSpriteSheet` can be obtained by calling `as_uniform_sprite_sheet` on
/// a `Sprite` object.
#[derive(Clone, Debug)]
pub struct UniformSpriteSheet {
    /// The sprite that this sprite sheet indexes into
    pub sprite: Sprite,
    divs: (u32, u32),
    indiv_size: (u32, u32),
}

/// A uniform sprite sheet is a sprite sheet that has parts of the original
/// sprite broken up into chunks that are associated to a Key.
///
/// A `NonUniformSpriteSheet` can be obtained by calling
/// `as_non_uniform_sprite_sheet` on a `Sprite` object.
#[derive(Clone, Debug)]
pub struct NonUniformSpriteSheet<K: Hash + Eq> {
    /// The sprite that this sprite sheet indexes into
    pub sprite: Sprite,
    /// The mapping from a key to a subsprite
    pub mapping: HashMap<K, Sprite>
}

/// Implemented by any object that can be converted into a Sprite.
pub trait IntoSprite {
    /// Attempts to convert itself into a sprite.
    fn into_sprite<D: HasDisplay>(self, display: &D) -> LuxResult<Sprite>;
}

/// TextureLoader is implemented on any object that can load textures.
pub trait TextureLoader {
    /// Attempts to load a texture from a path.
    fn load_texture_file<P: AsRef<Path> + ?Sized>(&self, path: &P) -> Result<Texture, LuxError>;

    /// Attempts to load a texture from a `DynamicImage` from the `image` crate.
    fn texture_from_image(&self, img: image::DynamicImage) -> Result<Texture, LuxError>;
}

impl IntoSprite for Sprite {
    fn into_sprite<D: HasDisplay>(self, _display: &D) -> LuxResult<Sprite> {
        Ok(self)
    }
}

impl IntoSprite for image::DynamicImage {
    fn into_sprite<D: HasDisplay>(self, display: &D) -> LuxResult<Sprite> {
        use image::GenericImage;
        let img = glium::texture::RawImage2d::from_raw_rgba_reversed(self.raw_pixels(), self.dimensions());
        let img = try!(glium::texture::Texture2d::new(display.borrow_display(), img));
        let tex: Texture = Texture::new(img);
        Ok(tex.into_sprite())
    }
}

impl <'a> IntoSprite for &'a Path {
    fn into_sprite<D: HasDisplay>(self, display: &D) -> LuxResult<Sprite> {
        let img = try!(image::open(self)).flipv();
        img.into_sprite(display)
    }
}

impl <T> TextureLoader for T where T: HasDisplay {
    fn load_texture_file<P: AsRef<Path> + ?Sized>(&self, path: &P) -> Result<Texture, LuxError> {
        use image::GenericImage;
        let img = try!(image::open(path));
        let img = glium::texture::RawImage2d::from_raw_rgba_reversed(img.raw_pixels(), img.dimensions());
        let img = try!(glium::texture::Texture2d::new(self.borrow_display(), img));
        let tex = Texture::new(img);
        Ok(tex)
    }

    fn texture_from_image(&self, img: image::DynamicImage) -> Result<Texture, LuxError> {
        use image::GenericImage;
        let img = glium::texture::RawImage2d::from_raw_rgba_reversed(img.raw_pixels(), img.dimensions());
        let img = try!(glium::texture::Texture2d::new(self.borrow_display(), img));
        Ok(Texture::new(img))
    }
}

impl Texture {
    /// Creates an empty texture with a given width and height.
    ///
    /// Depending on the graphics card, the width and height might need
    /// to be powers of two.
    pub fn empty<D: HasDisplay>(d: &D, width: u32, height: u32) -> Result<Texture, LuxError> {
        use glium::Surface;
        let backing = try!(glium::texture::Texture2d::empty(d.borrow_display(), width, height));
        {
            let mut s = backing.as_surface();
            s.clear_depth(0.0);
            s.clear_stencil(0);
        }
        Ok(Texture {
            backing: backing
        })
    }

    fn new(texture: glium::texture::Texture2d) -> Texture {
        Texture {
            backing: texture
        }
    }

    /// Converts this texture into a `Sprite`.
    pub fn into_sprite(self) -> Sprite {
        Sprite::new(Rc::new(self.backing))
    }

    /// Returns a reference to this texture with a drawable context.
    pub fn as_drawable<'a, D>(&'a mut self, d: &'a D) -> DrawableTexture<'a, D>
    where D: HasDisplay + HasPrograms {
        DrawableTexture::new(self.backing.as_surface(), d)
    }
}

impl <'a, D> DrawableTexture<'a, D>  where D: HasDisplay + HasPrograms {
    fn new(texture: glium::framebuffer::SimpleFrameBuffer<'a>, d: &'a D)
    -> DrawableTexture<'a, D> {
        use glium::Surface;

        let (w, h) = texture.get_dimensions();
        let (w, h) = (w as Float, h as Float);
        let (sx, sy) = (2.0 / w, -2.0 / h);

        let mut basis = vecmath::mat4_id();
        basis[1][1] = -1.0;
        basis[3][0] = -1.0;
        basis[3][1] = 1.0;
        basis[0][0] = sx;
        basis[1][1] = sy;

        basis.scale(1.0, -1.0);
        basis.translate(0.0, -h);

        DrawableTexture {
            texture: texture,
            d: d,
            matrix: basis,
            color_draw_cache: None,
            tex_draw_cache: None,
            color: [0.0, 0.0, 0.0, 1.0],
            draw_mod: DrawParamModifier::new()
        }
    }
}

impl <'a, D> DrawParamMod for DrawableTexture<'a, D> where D: HasDisplay + HasPrograms {
    fn draw_param_mod(&self) -> &DrawParamModifier {
        &self.draw_mod
    }

    fn draw_param_mod_mut(&mut self) -> &mut DrawParamModifier {
        &mut self.draw_mod
    }
}

impl <'a, D> Colored for DrawableTexture<'a, D> where D: HasDisplay + HasPrograms{
    fn get_color(&self) -> [Float; 4] {
        self.color
    }

    fn color<C: Color>(&mut self, color: C) -> &mut DrawableTexture<'a, D> {
        self.color = color.to_rgba();
        self
    }
}

impl <'a, D> Transform for DrawableTexture<'a, D> where D: HasDisplay + HasPrograms {
    fn current_matrix(&self) -> &[[Float; 4]; 4] {
        &self.matrix
    }
    fn current_matrix_mut(&mut self) -> &mut [[Float; 4]; 4] {
        &mut self.matrix
    }
}

impl <'a, D> HasDisplay for DrawableTexture<'a, D> where D: HasDisplay + HasPrograms {
    fn borrow_display(&self) -> &glium::Display {
        &self.d.borrow_display()
    }
}

impl <'a, D> HasSurface for DrawableTexture<'a, D> where D: HasDisplay + HasPrograms {
    type Out = glium::framebuffer::SimpleFrameBuffer<'a>;

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

impl <'a, D> Fetch<Vec<Idx>> for DrawableTexture<'a, D> where D: HasPrograms + HasDisplay {
    fn fetch(&self) -> poison_pool::Item<Vec<Idx>> {
        poison_pool::Item::from_value(vec![])
    }
}

impl <'a, D> Fetch<Vec<TexVertex>> for DrawableTexture<'a, D> where D: HasPrograms + HasDisplay {
    fn fetch(&self) -> poison_pool::Item<Vec<TexVertex>> {
        poison_pool::Item::from_value(vec![])
    }
}

impl <'a, D> Fetch<Vec<ColorVertex>> for DrawableTexture<'a, D> where D: HasPrograms + HasDisplay {
    fn fetch(&self) -> poison_pool::Item<Vec<ColorVertex>> {
        poison_pool::Item::from_value(vec![])
    }
}

impl <'a, D> Canvas for DrawableTexture<'a, D> where D: HasDisplay + HasPrograms {
    fn size(&self) -> (Float, Float) {
        use glium::Surface;
        let (w, h) = self.texture.get_dimensions();
        (w as Float, h as Float)
    }
}

impl <'a, D> Drop for DrawableTexture<'a, D> where D: HasDisplay + HasPrograms {
    fn drop(&mut self) {
        use super::primitive_canvas::PrimitiveCanvas;
        self.flush_draw().unwrap();
    }
}

impl Sprite {
    fn new(tex: Rc<glium::texture::Texture2d>) -> Sprite {
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

    /// Returns the size of this sprite.
    pub fn size(&self) -> (f32, f32) {
        (self.size.0 as f32, self.size.1 as f32)
    }

    /// Returns the width of this sprite.
    pub fn width(&self) -> f32 {
        self.size().0
    }

    /// Returns the height of this sprite.
    pub fn height(&self) -> f32 {
        self.size().1
    }

    /// Returns the size of this sprite given the size of the image in pixels
    /// that this sprite was loaded from.
    pub fn ideal_size(&self) -> (Float, Float) {
        let (w, h) = self.size;
        (w as Float, h as Float)
    }

    /// Returns a new sprite located offset from this sprite with a specified size.
    pub fn sub_sprite(&self, offset: (u32, u32), size: (u32, u32)) -> Option<Sprite> {
        if offset.0 + size.0 > self.size.0 { return None };
        if offset.1 + size.1 > self.size.1 { return None };

        let pos = (self.pos.0 + offset.0, self.pos.1 + offset.1);

        Some(Sprite {
            texture: self.texture.clone(),
            original_size: self.original_size,

            size: size,
            pos: pos,

            texture_size: (size.0 as Float / self.original_size.0 as Float,
                           size.1 as Float / self.original_size.1 as Float),
            texture_pos: (pos.0 as Float / self.original_size.0 as Float,
                          pos.1 as Float / self.original_size.1 as Float)
        })
    }

    /// Returns a sprite that contains the entire texture that the sprite
    /// was loaded from.
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

    /// Returns a an array containing the positions of
    ///
    /// * The top left corner,
    /// * The top right corner,
    /// * The bottom left corner,
    /// * the bottom right corner
    ///
    /// all in order.
    ///
    /// These positions are positions in texture space of the texture
    /// that this sprite has been loaded from.
    pub fn bounds(&self) -> [[Float; 2]; 4]{
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

    /// Returns a clone of the reference counted texture that this
    /// sprite was loaded from.
    pub fn texture(&self) -> Rc<glium::texture::Texture2d> {
        self.texture.clone()
    }

    /// Returns a reference to the texture that this sprite wasa loaded from.
    pub fn texture_ref(&self) -> &glium::texture::Texture2d {
        self.texture.deref()
    }

    /// Returns a new uniform sprite sheet using this sprite as its base.
    pub fn as_uniform_sprite_sheet(&self, indiv_width: u32, indiv_height: u32)
    -> UniformSpriteSheet {
        UniformSpriteSheet::new(self.clone(), indiv_width, indiv_height)
    }

    /// Returns a new nonuniform sprite sheet using this sprite as its base.
    pub fn as_nonuniform_sprite_sheet<T>(&self) -> NonUniformSpriteSheet<T>
    where T: Eq + Hash {
        NonUniformSpriteSheet::new(self.clone())
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

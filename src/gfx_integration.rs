use glium::texture::Texture2d;
use glium::uniforms;

// Colored Vertex
#[derive(Copy, Debug, Clone)]
pub struct ColorVertex {
    pub pos: [f32; 2],
    pub color: [f32; 4],
}
implement_vertex!(ColorVertex, pos, color);

// Textured Vertex
#[derive(Copy, Debug, Clone)]
pub struct TexVertex {
    pub pos: [f32; 2],
    pub tex_coords: [f32; 2]
}
implement_vertex!(TexVertex, pos, tex_coords);

pub struct ColorParams {
    pub matrix: [[f32; 4]; 4],
}

impl uniforms::Uniforms for ColorParams {
    fn visit_values<F>(self, mut f: F) where F: FnMut(&str, &uniforms::UniformValue) {
        use glium::uniforms::IntoUniformValue;
        f("matrix", &self.matrix.into_uniform_value());
    }
}

pub struct TexParams<'a> {
    pub matrix: [[f32; 4]; 4],
    pub texture: &'a Texture2d,
    pub color_mult: [f32; 4]
}

impl <'a> uniforms::Uniforms for TexParams<'a> {
    fn visit_values<F>(self, mut f: F) where F: FnMut(&str, &uniforms::UniformValue) {
        use glium::uniforms::IntoUniformValue;
        f("matrix", &self.matrix.into_uniform_value());
        f("texture", &self.texture.into_uniform_value());
        f("color_mult", &self.color_mult.into_uniform_value());
    }
}

//
//
// BASIC
//
//

pub static COLOR_VERTEX_SRC: &'static str = r"
    #version 110

    uniform mat4 matrix;

    attribute vec2 pos;
    attribute vec4 color;

    varying vec4 v_color;

    void main() {
        gl_Position = matrix * vec4(pos, 0.0, 1.0);
        v_color = color;
    }
";

pub static COLOR_FRAGMENT_SRC: &'static str = r"
    #version 110

    varying vec4 v_color;

    void main() {
        gl_FragColor = v_color;
    }
";

pub static TEX_VERTEX_SRC: &'static str = r"
    #version 110

    uniform mat4 matrix;

    attribute vec2 pos;
    attribute vec2 tex_coords;

    varying vec2 v_tex_coords;
    void main() {
        gl_Position = matrix * vec4(pos, 0.0, 1.0);
        v_tex_coords = tex_coords;
    }
";

pub static TEX_FRAGMENT_SRC: &'static str = r"
    #version 110

    uniform sampler2D texture;
    uniform vec4 color_mult;
    varying vec2 v_tex_coords;

    void main() {
        vec4 t = texture2D(texture, v_tex_coords);
        gl_FragColor = vec4(t.r, t.g, t.b, t.a) * color_mult;
    }
";




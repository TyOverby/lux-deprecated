use glium::texture::CompressedTexture2d;

// Colored Vertex
#[derive(Copy, Show, Clone)]
#[vertex_format]
pub struct ColorVertex {
    pub pos: [f32; 2],
    pub color: [f32; 4],
}

#[uniforms]
pub struct ColorParams {
    pub matrix: [[f32; 4]; 4],
}

// Textured Vertex
#[derive(Copy, Show, Clone)]
#[vertex_format]
pub struct TexVertex {
    pub pos: [f32; 2],
    pub tex_coords: [f32; 2]
}

#[uniforms]
pub struct TexParams<'a> {
    pub matrix: [[f32; 4]; 4],
    pub texture: &'a CompressedTexture2d
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
    varying vec2 v_tex_coords;

    void main() {
        gl_FragColor = texture2D(texture, v_tex_coords);
    }
";




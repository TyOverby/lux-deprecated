use gfx::ShaderSource;

#[vertex_format]
pub struct Vertex {
    #[name = "a_Pos"]
    pub pos: [f32, ..2],
    #[name = "a_TexCoord"]
    pub tex: [f32, ..2]
}

#[shader_param(BasicBatch)]
pub struct Params {
    #[name = "u_Transform"]
    pub transform: [[f32, ..4], ..4],

    #[name = "t_Color"]
    pub color: [f32, ..4]
}

//
//
// BASIC
//
//

pub static VERTEX_SRC: ShaderSource<'static> = shaders! {
GLSL_120: b"
    #version 120
    attribute vec2 a_Pos;
    attribute vec2 a_TexCoord;
    varying vec2 v_TexCoord;
    uniform mat4 u_Transform;
    void main() {
        v_TexCoord = a_TexCoord;
        gl_Position = u_Transform * vec4(a_Pos, 0.0, 1.0);
    }
"
GLSL_150: b"
    #version 150 core
    in vec2 a_Pos;
    in vec2 a_TexCoord;
    out vec2 v_TexCoord;
    uniform mat4 u_Transform;
    void main() {
        v_TexCoord = a_TexCoord;
        gl_Position = u_Transform * vec4(a_Pos, 0.0, 1.0);
    }
"
};

pub static FRAGMENT_SRC: ShaderSource<'static> = shaders! {
GLSL_120: b"
    #version 120
    varying vec2 v_TexCoord;
    uniform vec4 t_Color;
    void main() {
        gl_FragColor = t_Color;
    }
"
GLSL_150: b"
    #version 150 core
    in vec2 v_TexCoord;
    out vec4 o_Color;
    uniform vec4 t_Color;
    void main() {
        o_Color = t_Color;
    }
"
};

//
//
// Circle Border
//
//

#[vertex_format]
pub struct EllipseBorderVertex {
    #[name = "a_Pos"]
    pub pos: [f32, ..2],
    #[name = "a_TexCoord"]
    pub tex: [f32, ..2],
    #[name = "a_IsOuter"]
    pub is_outer: f32
}

#[shader_param(EllipseBorderBatch)]
pub struct EllipseBorderParams {
    #[name = "u_Transform"]
    pub transform: [[f32, ..4], ..4],
    pub ratio: [f32, ..2],
    #[name = "u_Width"]
    pub width: f32,
    #[name = "t_Color"]
    pub color: [f32, ..4]
}

pub static ELLIPSE_BORDER_VERTEX_SRC: ShaderSource<'static> = shaders! {
GLSL_120: b"
    #version 120
    attribute vec2 a_Pos;
    attribute vec2 a_TexCoord;
    attribute float a_IsOuter;
    varying vec2 v_TexCoord;
    uniform mat4 u_Transform;
    uniform float u_Width;
    uniform vec2 ratio;
    void main() {
        v_TexCoord = a_TexCoord;

        vec4 this_t = vec4(a_Pos, 0.0, 1.0);

        vec2 normal = a_Pos;
        normal /= ratio;
        gl_Position = u_Transform * (this_t + vec4(a_IsOuter * normal * u_Width, 0, 0));
    }
"
GLSL_150: b"
    #version 150 core
    in vec2 a_Pos;
    in vec2 a_TexCoord;
    in float a_IsOuter;
    out vec2 v_TexCoord;
    uniform mat4 u_Transform;
    uniform float u_width;
    uniform vec2 ratio;
    void main() {
        v_TexCoord = a_TexCoord;

        vec4 this_t = vec4(a_Pos, 0.0, 1.0);
        vec2 mod = a_Pos * a_IsOuter;
        mod /= ratio;
        mod *= u_Width;
        gl_Position = u_Transform * (this_t + vec4(mod, 0, 0));
    }
"
};

pub static ELLIPSE_BORDER_FRAGMENT_SRC: ShaderSource<'static> = shaders! {
GLSL_120: b"
    #version 120
    varying vec2 v_TexCoord;
    uniform vec4 t_Color;
    void main() {
        gl_FragColor = t_Color;
    }
"
GLSL_150: b"
    #version 150 core
    in vec2 v_TexCoord;
    out vec4 o_Color;
    uniform vec4 t_Color;
    void main() {
        o_Color = t_Color;
    }
"
};

use glium::{self, Display};

pub fn gen_texture_shader(display: &Display) ->
Result<glium::Program,
       glium::ProgramCreationError> {
    program!(display,
        140 => {
            vertex: "
                #version 140
                uniform mat4 matrix;
                in vec2 pos;
                in vec2 tex_coords;
                out vec2 v_tex_coords;
                void main() {
                    gl_Position = matrix * vec4(pos, 0.0, 1.0);
                    v_tex_coords = tex_coords;
                }
            ",

            fragment: "
                #version 140
                uniform sampler2D tex;
                uniform vec4 color_mult;
                in vec2 v_tex_coords;
                out vec4 f_color;
                void main() {
                    f_color = texture(tex, v_tex_coords) * color_mult;
                }
            "
        },

        110 => {
            vertex: "
                #version 110
                uniform mat4 matrix;
                attribute vec2 pos;
                attribute vec2 tex_coords;
                varying vec2 v_tex_coords;
                void main() {
                    gl_Position = matrix * vec4(pos, 0.0, 1.0);
                    v_tex_coords = tex_coords;
                }
            ",

            fragment: "
                #version 110
                uniform sampler2D texture;
                uniform vec4 color_mult;
                varying vec2 v_tex_coords;
                void main() {
                    gl_FragColor = texture2D(texture, v_tex_coords) * color_mult;
                }
            ",
        },
    )
}

pub fn gen_color_shader(display: &Display) ->
Result<glium::Program,
       glium::ProgramCreationError> {
    program!(display,
        140 => {
            vertex: "
                #version 140
                uniform mat4 matrix;
                in vec2 pos;
                in vec4 color;
                out vec4 v_color;
                void main() {
                    gl_Position = vec4(pos, 0.0, 1.0) * matrix;
                    v_color = color;
                }
            ",

            fragment: "
                #version 140
                in vec4 v_color;
                out vec4 f_color;
                void main() {
                    f_color = v_color;
                }
            "
        },

        110 => {
            vertex: "
                #version 110
                uniform mat4 matrix;
                attribute vec2 pos;
                attribute vec4 color;
                varying vec4 v_color;
                void main() {
                    gl_Position = vec4(pos, 0.0, 1.0) * matrix;
                    v_color = color;
                }
            ",

            fragment: "
                #version 110
                varying vec4 v_color;
                void main() {
                    gl_FragColor = vec4(v_color);
                }
            ",
        }
    )
}


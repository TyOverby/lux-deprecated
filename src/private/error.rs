pub use image::ImageError;

use std::error::Error;
use std::io::Error as IoError;
use std::convert::From;
use std;

use glium;

/// A result returning either a value or a lux-generated error.
pub type LuxResult<A> = Result<A, LuxError>;

#[derive(Debug)]
pub enum ShaderError {
    Creation(glium::ProgramCreationError),
    Chooser(glium::program::ProgramChooserCreationError),
}

/// Any error that Lux might run into.
#[derive(Debug)]
pub enum LuxError {
    /// An error that can occur when creating a window.
    WindowError(String),
    /// An error that can occur when creating an opengl context.
    OpenGlError(String),
    /// An error related to image decoding.
    ImageError(ImageError),
    /// An error that can occur when compiling or linking shaders.
    ShaderError(ShaderError),
    /// An error that can occur when required I/O fails.
    IoError(IoError),
    /// An error that can occur when attempting to use a font that hasn't
    /// been loaded yet.
    FontNotLoaded(String),
    /// An error that was produced while submitting a draw call
    DrawError(glium::DrawError),
    /// An error occured while creating a texture
    TextureCreationError(glium::texture::TextureCreationError),
    /// An error creating an index buffer occured
    IndexBufferCreationError,
    /// An error creating an vertex buffer occured
    VertexBufferCreationError,
}

impl Error for LuxError {
    fn description(&self) -> &str {
        match self {
            &LuxError::WindowError(ref s) => &s[..],
            &LuxError::OpenGlError(ref s) => &s[..],
            &LuxError::ShaderError(ref e) => match e {
                &ShaderError::Creation(ref e) => e.description(),
                &ShaderError::Chooser(ref e) => e.description(),
            },
            &LuxError::IoError(ref ioe) => Error::description(ioe),
            &LuxError::FontNotLoaded(ref s) => &s[..],
            // TODO: implement this when glium/959 is finished.
            &LuxError::ImageError(ref e) => e.description(),
            &LuxError::DrawError(_) => "",
            &LuxError::TextureCreationError(_) => "",
            &LuxError::IndexBufferCreationError => "An index buffer could not be created",
            &LuxError::VertexBufferCreationError => "A vertex buffer could not be created",
        }
    }
}

impl From<glium::GliumCreationError<glium::glutin::CreationError>> for LuxError {
    fn from(e: glium::GliumCreationError<glium::glutin::CreationError>) -> LuxError {
        match e {
            glium::GliumCreationError::BackendCreationError(e) => {
                    LuxError::WindowError(String::from(e.description()))
            }
            glium::GliumCreationError::IncompatibleOpenGl(m) => {
                LuxError::OpenGlError(m)
            }
        }
    }
}

impl From<glium::program::ProgramCreationError> for LuxError {
    fn from(e: glium::program::ProgramCreationError) -> LuxError {
        LuxError::ShaderError(ShaderError::Creation(e))
    }
}

impl From<glium::program::ProgramChooserCreationError> for LuxError {
    fn from(e: glium::program::ProgramChooserCreationError) -> LuxError {
        LuxError::ShaderError(ShaderError::Chooser(e))
    }
}

impl From<glium::texture::TextureCreationError> for LuxError {
    fn from(e: glium::texture::TextureCreationError) -> LuxError {
        LuxError::TextureCreationError(e)
    }
}

impl From<IoError> for LuxError {
    fn from(ioe: IoError) -> LuxError {
        LuxError::IoError(ioe)
    }
}

impl From<glium::DrawError> for LuxError {
    fn from(e: glium::DrawError) -> LuxError {
        LuxError::DrawError(e)
    }
}

impl From<ImageError> for LuxError {
    fn from(e: ImageError) -> LuxError {
        LuxError::ImageError(e)
    }
}


impl From<glium::vertex::BufferCreationError> for LuxError {
    fn from(_: glium::vertex::BufferCreationError) -> LuxError {
        LuxError::VertexBufferCreationError
    }
}

impl From<glium::index::BufferCreationError> for LuxError {
    fn from(_: glium::index::BufferCreationError) -> LuxError {
        LuxError::IndexBufferCreationError
    }
}

impl std::fmt::Display for LuxError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
        match self {
            &LuxError::WindowError(ref s) => s.fmt(f),
            &LuxError::OpenGlError(ref s) => s.fmt(f),
            &LuxError::ShaderError(ref e) => match e {
                &ShaderError::Creation(ref e) => e.fmt(f),
                &ShaderError::Chooser(ref e) => e.fmt(f),
            },
            &LuxError::IoError(ref e) => e.fmt(f),
            &LuxError::FontNotLoaded(ref s) => s.fmt(f),
            &LuxError::DrawError(ref e) => e.fmt(f),
            &LuxError::ImageError(ref e) => e.fmt(f),
            &LuxError::TextureCreationError(ref e) => std::fmt::Debug::fmt(&e, f),
            &LuxError::IndexBufferCreationError => "An index buffer could not be created".fmt(f),
            &LuxError::VertexBufferCreationError => "A vertex buffer could not be created".fmt(f),
        }
    }
}



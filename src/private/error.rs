pub use image::ImageError;
pub use freetype::error::Error as FreetypeError;

use std::error::Error;
use std::io::Error as IoError;
use std::convert::From;
use std;

use glium;

/// A result returning either a value or a lux-generated error.
pub type LuxResult<A> = Result<A, LuxError>;

/// Any error that Lux might run into.
#[derive(Debug)]
pub enum LuxError {
    /// An error that can occur when creating a window.
    WindowError(String),
    /// An error that can occur when creating an opengl context.
    OpenGlError(String),
    /// An error that can occur when compiling or linking shaders.
    ShaderError(glium::ProgramCreationError),
    /// An error that comes from the freetype font system.
    FontError(FreetypeError, String),
    /// An error that can occur when required I/O fails.
    IoError(IoError),
    /// An error that can occur when attempting to use a font that hasn't
    /// been loaded yet.
    FontNotLoaded(String)
}

impl Error for LuxError {
    fn description(&self) -> &str {
        match self {
            &LuxError::WindowError(ref s) => &s[..],
            &LuxError::OpenGlError(ref s) => &s[..],
            &LuxError::ShaderError(ref e) => e.description(),
            &LuxError::FontError(_, ref s) => &s[..],
            &LuxError::IoError(ref ioe) => Error::description(ioe),
            &LuxError::FontNotLoaded(ref s) => &s[..],
        }
    }
}

impl From<FreetypeError> for LuxError {
    fn from(e: FreetypeError) -> LuxError {
        use std::fmt::Write;
        let mut bf = String::new();
        write!(&mut bf, "{}", e).unwrap();
        LuxError::FontError(e, bf)
    }
}

impl From<glium::ProgramCreationError> for LuxError {
    fn from(e: glium::ProgramCreationError) -> LuxError {
        LuxError::ShaderError(e)
    }
}

impl From<IoError> for LuxError {
    fn from(ioe: IoError) -> LuxError {
        LuxError::IoError(ioe)
    }
}

impl std::fmt::Display for LuxError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
        match self {
            &LuxError::WindowError(ref s) => s.fmt(f),
            &LuxError::OpenGlError(ref s) => s.fmt(f),
            &LuxError::ShaderError(ref e) => e.fmt(f),
            &LuxError::FontError(ref e, _) => e.fmt(f),
            &LuxError::IoError(ref e) => e.fmt(f),
            &LuxError::FontNotLoaded(ref s) => s.fmt(f),
        }
    }
}

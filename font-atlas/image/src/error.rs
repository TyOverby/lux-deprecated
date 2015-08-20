use ::std::convert::From;
use ::std::error::Error;

pub type DecodingResult<T> = Result<T, DecodingError>;
pub type EncodingResult<T> = Result<T, EncodingError>;

#[derive(Debug)]
pub enum DecodingError {
    ImageDecodingError(::image::ImageError),
    BincodeDecodingError(::bincode::DecodingError),
    IoError(::std::io::Error)
}

#[derive(Debug)]
pub enum EncodingError {
    ImageEncodingError(::image::ImageError),
    BincodeEncodingError(::bincode::EncodingError),
    IoError(::std::io::Error)
}

impl From<::image::ImageError> for DecodingError {
    fn from(e: ::image::ImageError) -> DecodingError {
        DecodingError::ImageDecodingError(e)
    }
}

impl From<::bincode::DecodingError> for DecodingError {
    fn from(e: ::bincode::DecodingError) -> DecodingError {
        DecodingError::BincodeDecodingError(e)
    }
}

impl From<::std::io::Error> for DecodingError {
    fn from(e: ::std::io::Error) -> DecodingError {
        DecodingError::IoError(e)
    }
}

impl From<::image::ImageError> for EncodingError {
    fn from(e: ::image::ImageError) -> EncodingError {
        EncodingError::ImageEncodingError(e)
    }
}

impl From<::bincode::EncodingError> for EncodingError {
    fn from(e: ::bincode::EncodingError) -> EncodingError {
        EncodingError::BincodeEncodingError(e)
    }
}

impl From<::std::io::Error> for EncodingError {
    fn from(e: ::std::io::Error) -> EncodingError {
        EncodingError::IoError(e)
    }
}

impl Error for DecodingError {
    fn description(&self) -> &str {
        match *self {
            DecodingError::ImageDecodingError(ref ie) => ie.description(),
            DecodingError::BincodeDecodingError(ref jde) => jde.description(),
            DecodingError::IoError(ref ioe) => ioe.description()
        }
    }
}

impl ::std::fmt::Display for DecodingError {
    fn fmt(&self, formatter: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        formatter.write_str(self.description())
    }
}

impl Error for EncodingError {
    fn description(&self) -> &str {
        match *self {
            EncodingError::ImageEncodingError(ref ie) => ie.description(),
            EncodingError::BincodeEncodingError(ref jde) => jde.description(),
            EncodingError::IoError(ref ioe) => ioe.description()
        }
    }
}

impl ::std::fmt::Display for EncodingError {
    fn fmt(&self, formatter: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        formatter.write_str(self.description())
    }
}

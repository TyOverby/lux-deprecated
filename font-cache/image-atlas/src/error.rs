use rustc_serialize::json;
use ::std::convert::From;

pub type DecodingResult<T> = Result<T, DecodingError>;
pub type EncodingResult<T> = Result<T, EncodingError>;

pub enum DecodingError {
    ImageDecodingError(::image::ImageError),
    JsonDecodingError(json::DecoderError),
    IoError(::std::io::Error)
}

pub enum EncodingError {
    ImageEncodingError(::image::ImageError),
    JsonEncodingError(json::EncoderError),
    IoError(::std::io::Error)
}

impl From<::image::ImageError> for DecodingError {
    fn from(e: ::image::ImageError) -> DecodingError {
        DecodingError::ImageDecodingError(e)
    }
}

impl From<json::DecoderError> for DecodingError {
    fn from(e: json::DecoderError) -> DecodingError {
        DecodingError::JsonDecodingError(e)
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

impl From<json::EncoderError> for EncodingError {
    fn from(e: json::EncoderError) -> EncodingError {
        EncodingError::JsonEncodingError(e)
    }
}

impl From<::std::io::Error> for EncodingError {
    fn from(e: ::std::io::Error) -> EncodingError {
        EncodingError::IoError(e)
    }
}

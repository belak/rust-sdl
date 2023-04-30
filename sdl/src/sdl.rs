use std::ffi::CStr;
use std::marker::PhantomPinned;

use crate::sys;
use crate::video;

#[derive(Debug)]
pub struct SDL {
    _pinned: std::marker::PhantomPinned,
}

impl Drop for SDL {
    fn drop(&mut self) {
        unsafe {
            sys::SDL_Quit();
        }
    }
}

pub fn init() -> Result<SDL> {
    if unsafe { sys::SDL_Init(0) } != 0 {
        Err(get_error())
    } else {
        Ok(SDL {
            _pinned: PhantomPinned,
        })
    }
}

impl SDL {
    pub fn video(&self) -> Result<VideoSubsystem> {
        VideoSubsystem::new(&self)
    }
}

#[derive(Debug)]
pub struct VideoSubsystem {
    _pinned: std::marker::PhantomPinned,
}

impl Drop for VideoSubsystem {
    fn drop(&mut self) {
        unsafe { sys::SDL_QuitSubSystem(sys::SDL_INIT_VIDEO) }
    }
}

impl VideoSubsystem {
    pub fn new(_sdl_context: &SDL) -> Result<VideoSubsystem> {
        if unsafe { sys::SDL_InitSubSystem(sys::SDL_INIT_VIDEO) } != 0 {
            Err(get_error())
        } else {
            Ok(VideoSubsystem {
                _pinned: PhantomPinned,
            })
        }
    }
}

pub type Result<T> = std::result::Result<T, Error>;

pub(crate) fn get_error() -> Error {
    ErrorRepr::Other(
        unsafe { CStr::from_ptr(sys::SDL_GetError()) }
            .to_string_lossy()
            .into_owned(),
    )
    .into()
}

#[derive(thiserror::Error, Debug)]
#[error(transparent)]
pub struct Error(#[from] ErrorRepr);

#[derive(thiserror::Error, Debug)]
enum ErrorRepr {
    #[error("error code: {0}")]
    ErrorCode(#[from] ErrorCode),
    #[error("unknown error: {0}")]
    Other(String),
}

#[derive(thiserror::Error, Debug)]
enum ErrorCode {
    #[error("out of memory")]
    NoMemError = sys::SDL_errorcode::SDL_ENOMEM as isize,
    #[error("error reading from datastream")]
    ReadError = sys::SDL_errorcode::SDL_EFREAD as isize,
    #[error("error writing to datastream")]
    WriteError = sys::SDL_errorcode::SDL_EFWRITE as isize,
    #[error("error seeking in datastream")]
    SeekError = sys::SDL_errorcode::SDL_EFSEEK as isize,
    #[error("unknown SDL error")]
    UnsupportedError = sys::SDL_errorcode::SDL_UNSUPPORTED as isize,
}

impl From<sys::SDL_errorcode> for ErrorCode {
    fn from(value: sys::SDL_errorcode) -> Self {
        match value {
            sys::SDL_errorcode::SDL_ENOMEM => ErrorCode::NoMemError,
            sys::SDL_errorcode::SDL_EFREAD => ErrorCode::ReadError,
            sys::SDL_errorcode::SDL_EFWRITE => ErrorCode::WriteError,
            sys::SDL_errorcode::SDL_EFSEEK => ErrorCode::SeekError,
            sys::SDL_errorcode::SDL_UNSUPPORTED => ErrorCode::UnsupportedError,
            _ => ErrorCode::UnsupportedError,
        }
    }
}

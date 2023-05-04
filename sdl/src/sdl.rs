use std::ffi::CStr;
use std::marker::PhantomPinned;

use crate::sys;

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

#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8,
}

impl Color {
    pub const fn rgb(r: u8, g: u8, b: u8) -> Color {
        Color { r, g, b, a: 0xff }
    }

    pub const fn rgba(r: u8, g: u8, b: u8, a: u8) -> Color {
        Color { r, g, b, a }
    }

    pub fn invert(self) -> Color {
        Color::rgba(255 - self.r, 255 - self.g, 255 - self.b, 255 - self.a)
    }

    pub const fn into_rgb(self) -> (u8, u8, u8) {
        (self.r, self.g, self.b)
    }

    pub const fn into_rgba(self) -> (u8, u8, u8, u8) {
        (self.r, self.g, self.b, self.a)
    }

    // Implemented manually and kept private, because reasons
    const fn raw(self) -> sys::SDL_Color {
        sys::SDL_Color {
            r: self.r,
            g: self.g,
            b: self.b,
            unused: self.a,
        }
    }

    pub const WHITE: Color = Color::rgba(255, 255, 255, 255);
    pub const BLACK: Color = Color::rgba(0, 0, 0, 255);
    pub const GRAY: Color = Color::rgba(128, 128, 128, 255);
    pub const GREY: Color = Color::GRAY;
    pub const RED: Color = Color::rgba(255, 0, 0, 255);
    pub const GREEN: Color = Color::rgba(0, 255, 0, 255);
    pub const BLUE: Color = Color::rgba(0, 0, 255, 255);
    pub const MAGENTA: Color = Color::rgba(255, 0, 255, 255);
    pub const YELLOW: Color = Color::rgba(255, 255, 0, 255);
    pub const CYAN: Color = Color::rgba(0, 255, 255, 255);
}

impl Into<sys::SDL_Color> for Color {
    fn into(self) -> sys::SDL_Color {
        self.raw()
    }
}

impl From<sys::SDL_Color> for Color {
    fn from(raw: sys::SDL_Color) -> Color {
        Color::rgba(raw.r, raw.g, raw.b, raw.unused)
    }
}

impl From<(u8, u8, u8)> for Color {
    fn from((r, g, b): (u8, u8, u8)) -> Color {
        Color::rgb(r, g, b)
    }
}

impl From<(u8, u8, u8, u8)> for Color {
    fn from((r, g, b, a): (u8, u8, u8, u8)) -> Color {
        Color::rgba(r, g, b, a)
    }
}

impl Into<u32> for Color {
    fn into(self) -> u32 {
        u32::from_be_bytes([self.r, self.g, self.b, self.a])
    }
}

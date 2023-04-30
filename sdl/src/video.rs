use std::ffi::c_char;
use std::ffi::c_int;
use std::ffi::CString;
use std::ffi::NulError;
use std::marker::PhantomPinned;

use sys::SDL_Flip;
use sys::SDL_FreeSurface;

use crate::get_error;
use crate::sdl;
use crate::sys;
use crate::VideoSubsystem;

#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8,
}

impl Color {
    #[inline]
    #[allow(non_snake_case)]
    pub const fn RGB(r: u8, g: u8, b: u8) -> Color {
        Color { r, g, b, a: 0xff }
    }

    #[inline]
    #[allow(non_snake_case)]
    pub const fn RGBA(r: u8, g: u8, b: u8, a: u8) -> Color {
        Color { r, g, b, a }
    }

    pub fn invert(self) -> Color {
        Color::RGBA(255 - self.r, 255 - self.g, 255 - self.b, 255 - self.a)
    }

    #[inline]
    pub const fn rgb(self) -> (u8, u8, u8) {
        (self.r, self.g, self.b)
    }

    #[inline]
    pub const fn rgba(self) -> (u8, u8, u8, u8) {
        (self.r, self.g, self.b, self.a)
    }

    // Implemented manually and kept private, because reasons
    #[inline]
    const fn raw(self) -> sys::SDL_Color {
        sys::SDL_Color {
            r: self.r,
            g: self.g,
            b: self.b,
            unused: self.a,
        }
    }

    pub const WHITE: Color = Color::RGBA(255, 255, 255, 255);
    pub const BLACK: Color = Color::RGBA(0, 0, 0, 255);
    pub const GRAY: Color = Color::RGBA(128, 128, 128, 255);
    pub const GREY: Color = Color::GRAY;
    pub const RED: Color = Color::RGBA(255, 0, 0, 255);
    pub const GREEN: Color = Color::RGBA(0, 255, 0, 255);
    pub const BLUE: Color = Color::RGBA(0, 0, 255, 255);
    pub const MAGENTA: Color = Color::RGBA(255, 0, 255, 255);
    pub const YELLOW: Color = Color::RGBA(255, 255, 0, 255);
    pub const CYAN: Color = Color::RGBA(0, 255, 255, 255);
}

impl Into<sys::SDL_Color> for Color {
    fn into(self) -> sys::SDL_Color {
        self.raw()
    }
}

impl From<sys::SDL_Color> for Color {
    fn from(raw: sys::SDL_Color) -> Color {
        Color::RGBA(raw.r, raw.g, raw.b, raw.unused)
    }
}

impl From<(u8, u8, u8)> for Color {
    fn from((r, g, b): (u8, u8, u8)) -> Color {
        Color::RGB(r, g, b)
    }
}

impl From<(u8, u8, u8, u8)> for Color {
    fn from((r, g, b, a): (u8, u8, u8, u8)) -> Color {
        Color::RGBA(r, g, b, a)
    }
}

#[derive(Debug)]
pub struct Surface {
    inner: *mut sys::SDL_Surface,
}

impl Surface {
    fn new(inner: *mut sys::SDL_Surface) -> Surface {
        Surface { inner }
    }

    pub fn raw(&self) -> *mut sys::SDL_Surface {
        self.inner
    }

    pub fn flip(&mut self) -> sdl::Result<()> {
        if unsafe { SDL_Flip(self.inner) } != 0 {
            Err(get_error())
        } else {
            Ok(())
        }
    }
}

impl Drop for Surface {
    fn drop(&mut self) {
        unsafe { SDL_FreeSurface(self.inner) }
    }
}

impl VideoSubsystem {
    pub fn window(&self, title: &str, width: u32, height: u32) -> WindowBuilder {
        WindowBuilder::new(self, title, width, height)
    }
}

#[derive(Debug)]
pub struct WindowBuilder {
    title: String,
    width: u32,
    height: u32,
    window_flags: u32,
    _marker: PhantomPinned,
}

impl WindowBuilder {
    /// Initializes a new `WindowBuilder`.
    pub fn new(_v: &VideoSubsystem, title: &str, width: u32, height: u32) -> WindowBuilder {
        WindowBuilder {
            title: title.to_owned(),
            width,
            height,
            window_flags: 0,
            _marker: PhantomPinned,
        }
    }

    /// Builds the window.
    pub fn build(&self) -> Result<Surface, WindowBuildError> {
        use self::WindowBuildError::*;
        let title = match CString::new(self.title.clone()) {
            Ok(t) => t,
            Err(err) => return Err(InvalidTitle(err)),
        };
        if self.width >= (1 << 31) {
            return Err(WidthOverflows(self.width));
        }
        if self.height >= (1 << 31) {
            return Err(HeightOverflows(self.width));
        }

        unsafe {
            let raw = sys::SDL_SetVideoMode(
                self.width as c_int,
                self.height as c_int,
                32,
                self.window_flags,
            );

            sys::SDL_WM_SetCaption(title.as_ptr() as *const c_char, std::ptr::null());

            if raw.is_null() {
                Err(sdl::get_error().into())
            } else {
                Ok(Surface::new(raw))
            }
        }
    }

    pub fn fullscreen(&mut self) -> &mut WindowBuilder {
        self.window_flags |= sys::SDL_WindowFlags::SDL_FULLSCREEN as u32;
        self
    }

    pub fn opengl(&mut self) -> &mut WindowBuilder {
        self.window_flags |= sys::SDL_WindowFlags::SDL_OPENGL as u32;
        self
    }

    pub fn borderless(&mut self) -> &mut WindowBuilder {
        self.window_flags |= sys::SDL_WindowFlags::SDL_NOFRAME as u32;
        self
    }

    /// Sets the window to be resizable.
    pub fn resizable(&mut self) -> &mut WindowBuilder {
        self.window_flags |= sys::SDL_WindowFlags::SDL_RESIZABLE as u32;
        self
    }
}

#[derive(thiserror::Error, Debug)]
pub enum WindowBuildError {
    #[error("window height overflow: {}", .0)]
    HeightOverflows(u32),
    #[error("window width overflow: {}", .0)]
    WidthOverflows(u32),
    #[error("invalid window title: {}", .0)]
    InvalidTitle(NulError),
    #[error("SDL error: {}", .0)]
    SdlError(#[from] sdl::Error),
}

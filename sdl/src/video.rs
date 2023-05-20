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

#[derive(Debug)]
pub struct Surface {
    inner: *mut sys::SDL_Surface,
}

impl Surface {
    pub(crate) fn new(inner: *mut sys::SDL_Surface) -> Surface {
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

    // TODO: set icon
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

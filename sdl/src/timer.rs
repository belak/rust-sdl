use std::marker::PhantomPinned;

use sys::SDL_InitSubSystem;

use crate::sys;
use crate::sdl;

#[derive(Debug)]
pub struct Subsystem {
    _pinned: std::marker::PhantomPinned,
}

impl Drop for Subsystem {
    fn drop(&mut self) {
        unsafe {
            sys::SDL_QuitSubSystem(sys::SDL_INIT_TIMER)
        }
    }
}

impl Subsystem {
    pub(crate) fn new() -> sdl::Result<Subsystem> {
        if unsafe { SDL_InitSubSystem(sys::SDL_INIT_TIMER) } != 0 {
            Err(sdl::get_error())
        } else {
            Ok(Subsystem {
                _pinned: PhantomPinned,
            })
        }
    }
}

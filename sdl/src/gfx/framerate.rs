use crate::sys::gfx::framerate;
use crate::sdl;

use libc::{size_t, c_void};
use std::mem;

/// Structure holding the state and timing information of the framerate controller.
pub struct FPSManager {
    raw: *mut framerate::FPSmanager,
}

impl FPSManager {
    /// Create the framerate manager.
    pub fn new() -> FPSManager {
        unsafe {
            let size = mem::size_of::<framerate::FPSmanager>() as size_t;
            let raw = libc::malloc(size) as *mut framerate::FPSmanager;
            framerate::SDL_initFramerate(raw);
            FPSManager { raw: raw }
        }
    }

    /// Set the target framerate.
    pub fn set_framerate(&mut self, rate: u32) -> sdl::Result<()> {
        let ret = unsafe { framerate::SDL_setFramerate(self.raw, rate) };
        match ret {
            0 => Ok(()),
            _ => Err(sdl::get_error())
        }
    }

    /// Return the current target framerate.
    pub fn get_framerate(&self) -> i32 {
        // will not get an error
        unsafe { framerate::SDL_getFramerate(self.raw) }
    }

    /// Return the current framecount.
    pub fn get_frame_count(&self) -> i32 {
        // will not get an error
        unsafe { framerate::SDL_getFramecount(self.raw) }
    }

    /// Delay execution to maintain a constant framerate and calculate fps.
    pub fn delay(&mut self) -> u32 {
        unsafe { framerate::SDL_framerateDelay(self.raw) }
    }
}

impl Drop for FPSManager {
    fn drop(&mut self) {
        unsafe { libc::free(self.raw as *mut c_void) }
    }
}

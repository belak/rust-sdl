pub use sdl_sys as sys;

mod sdl;
pub use crate::sdl::*;

// The 7 primary SDL subsystems
pub mod audio;
pub mod cdrom;
pub mod event;
pub mod joystick;
pub mod timer;
pub mod video;

#[cfg(feature = "gfx")]
pub mod gfx;

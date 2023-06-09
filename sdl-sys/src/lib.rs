#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(improper_ctypes)]
#![allow(deref_nullptr)]
#![no_std]

include!(concat!(env!("OUT_DIR"), "/sdl_bindings.rs"));

#[repr(u32)]
pub enum SDL_WindowFlags {
    SDL_SWSURFACE = crate::SDL_SWSURFACE,
    SDL_HWSURFACE = crate::SDL_HWSURFACE,
    SDL_ASYNCBLIT = crate::SDL_ASYNCBLIT,
    SDL_ANYFORMAT = crate::SDL_ANYFORMAT,
    SDL_HWPALETTE = crate::SDL_HWPALETTE,
    SDL_DOUBLEBUF = crate::SDL_DOUBLEBUF,
    SDL_FULLSCREEN = crate::SDL_FULLSCREEN,
    SDL_OPENGL = crate::SDL_OPENGL,
    SDL_OPENGLBLIT = crate::SDL_OPENGLBLIT,
    SDL_RESIZABLE = crate::SDL_RESIZABLE,
    SDL_NOFRAME = crate::SDL_NOFRAME,
}

#[cfg(feature = "mixer")]
pub mod mixer {
    include!(concat!(env!("OUT_DIR"), "/sdl_mixer_bindings.rs"));
}

#[cfg(feature = "image")]
pub mod image {
    include!(concat!(env!("OUT_DIR"), "/sdl_image_bindings.rs"));
}

#[cfg(feature = "ttf")]
pub mod ttf {
    include!(concat!(env!("OUT_DIR"), "/sdl_ttf_bindings.rs"));
}

#[cfg(feature = "gfx")]
pub mod gfx {
    pub mod framerate {
        include!(concat!(env!("OUT_DIR"), "/sdl_gfx_framerate_bindings.rs"));
    }

    pub mod imagefilter {
        include!(concat!(env!("OUT_DIR"), "/sdl_gfx_imagefilter_bindings.rs"));
    }

    pub mod primitives {
        include!(concat!(env!("OUT_DIR"), "/sdl_gfx_primitives_bindings.rs"));
    }

    pub mod rotozoom {
        include!(concat!(env!("OUT_DIR"), "/sdl_gfx_rotozoom_bindings.rs"));
    }
}

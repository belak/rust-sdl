#![allow(unused_imports, dead_code, unused_variables)]

use std::path::{Path, PathBuf};
use std::process::Command;
use std::{env, fs, io};

#[derive(Debug)]
struct NameOverrides;

// We use ParseCallbacks to override the names of some types/functions to make
// them match the same style as everything else.
impl bindgen::callbacks::ParseCallbacks for NameOverrides {
    fn item_name(&self, original_item_name: &str) -> Option<String> {
        match original_item_name {
            // Strange enum names
            "SDLKey" => Some("SDL_Key"),
            "SDLMod" => Some("SDL_KeyMod"),
            "CDstatus" => Some("SDL_CDStatus"),

            // Weird capitalizations
            "SDL_GLattr" => Some("SDL_GLAttr"),
            "SDL_audiostatus" => Some("SDL_AudioStatus"),
            "SDL_eventaction" => Some("SDL_EventAction"),

            // These are small fixes to type aliases - if you manually set up
            // the type alias here, it seems to skip the alias and just use the
            // type directly which looks way better in the documentation.
            "_TTF_Font" => Some("TTF_Font"),
            "_SDL_Joystick" => Some("SDL_Joystick"),

            // The trick doesn't always work though.
            //
            //   typedef struct _SDL_TimerID *SDL_TimerID;
            //
            // By default this generates the following rust:
            //
            //   pub struct _SDL_TimerID { _unused: [u8; 0] }
            //   pub type SDL_TimerID = *mut _SDL_TimerID;
            //
            // This is weird because it generates both a struct and a type which
            // show up in the docs when we really only need one.
            _ => None,
        }
        .map(|s| s.to_string())
    }
}

fn main() {
    let target = env::var("TARGET").expect("Cargo build scripts always have TARGET");
    let host = env::var("HOST").expect("Cargo build scripts always have HOST");

    let mut include_paths: Vec<String> = vec!["/opt/homebrew/include".to_string()];

    if let Ok(include_path) = env::var("SDL_INCLUDE_PATH") {
        include_paths.push(include_path);
    };

    let pkg_config_library = pkg_config::Config::new()
        .print_system_libs(false)
        .probe("sdl")
        .unwrap();
    for path in pkg_config_library.include_paths {
        include_paths.push(format!("{}", path.display()));
    }

    generate_bindings(target.as_str(), host.as_str(), include_paths.as_slice());
    println!("cargo:include={}", include_paths.join(":"));
    link_sdl();
}

fn create_bindgen_builder(target: &str, host: &str, headers_paths: &[String]) -> bindgen::Builder {
    let target_os = get_os_from_triple(target).unwrap();

    let mut bindings = bindgen::Builder::default()
        // enable no_std-friendly output by only using core definitions
        .use_core()
        .default_enum_style(bindgen::EnumVariation::Rust {
            non_exhaustive: false,
        })
        .ctypes_prefix("libc");

    // Set correct target triple for bindgen when cross-compiling
    if target != host {
        bindings = bindings.clang_arg("-target");
        bindings = bindings.clang_arg(target.clone());
    }

    for headers_path in headers_paths {
        bindings = bindings.clang_arg(format!("-I{}", headers_path));
    }

    // SDL is missing a default configuration for Linux, so we define one to
    // give us access to all possible functions.
    if target_os == "linux-gnu" {
        bindings = bindings.clang_arg("-DSDL_VIDEO_DRIVER_X11");
    }

    // There are a number of things which need to be blacklisted in all the
    // headers so we do it here to avoid repeating ourselves.
    bindings = bindings
        .parse_callbacks(Box::new(NameOverrides {}))
        .blacklist_type("FP_NAN")
        .blacklist_type("FP_INFINITE")
        .blacklist_type("FP_ZERO")
        .blacklist_type("FP_SUBNORMAL")
        .blacklist_type("FP_NORMAL")
        .blacklist_item("SDL_DUMMY_ENUM")
        .blacklist_item("SDL_dummy_.*")
        .blacklist_item("IOPOL.*");

    bindings
}

// headers_path is a list of additional directories for bindgen to search for
// SDL headers.
fn generate_bindings(target: &str, host: &str, headers_paths: &[String]) {
    let bindings = create_bindgen_builder(target, host, headers_paths)
        .header("wrapper.h")
        .whitelist_function("SDL_.*")
        .whitelist_type("SDL_.*")
        .whitelist_var("SDL_.*")
        //.opaque_type("^SDL_Joystick$")
        .disable_name_namespacing()
        .generate()
        .expect("Unable to generate bindings!");

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());

    bindings
        .write_to_file(out_path.join("sdl_bindings.rs"))
        .expect("Couldn't write bindings!");

    #[cfg(feature = "image")]
    {
        let image_bindings = create_bindgen_builder(target, host, headers_paths)
            .raw_line("use crate::*;")
            .header("wrapper_image.h")
            .whitelist_type("IMG.*")
            .whitelist_function("IMG.*")
            .whitelist_var("IMG.*")
            .blacklist_type("SDL_.*")
            .blacklist_type("_IO.*|FILE")
            .generate()
            .expect("Unable to generate image_bindings!");

        let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());

        image_bindings
            .write_to_file(out_path.join("sdl_image_bindings.rs"))
            .expect("Couldn't write image_bindings!");
    }

    #[cfg(feature = "ttf")]
    {
        let ttf_bindings = create_bindgen_builder(target, host, headers_paths)
            .raw_line("use crate::*;")
            .header("wrapper_ttf.h")
            .whitelist_type("TTF.*")
            .whitelist_function("TTF.*")
            .whitelist_var("TTF.*")
            .blacklist_type("SDL_.*")
            .blacklist_type("_IO.*|FILE")
            .generate()
            .expect("Unable to generate ttf_bindings!");

        let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());

        ttf_bindings
            .write_to_file(out_path.join("sdl_ttf_bindings.rs"))
            .expect("Couldn't write ttf_bindings!");
    }

    #[cfg(feature = "mixer")]
    {
        let mixer_bindings = create_bindgen_builder(target, host, headers_paths)
            .raw_line("use crate::*;")
            .header("wrapper_mixer.h")
            .whitelist_type("MIX.*")
            .whitelist_type("Mix.*")
            .whitelist_type("MUS.*")
            .whitelist_function("Mix.*")
            .whitelist_var("MIX.*")
            .whitelist_var("MUS.*")
            .blacklist_type("SDL_.*")
            .blacklist_type("_IO.*|FILE")
            .generate()
            .expect("Unable to generate mixer_bindings!");

        let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());

        mixer_bindings
            .write_to_file(out_path.join("sdl_mixer_bindings.rs"))
            .expect("Couldn't write mixer_bindings!");
    }

    #[cfg(feature = "gfx")]
    {
        let gfx_framerate_bindings = create_bindgen_builder(target, host, headers_paths)
            .header("wrapper_gfx_framerate.h")
            .whitelist_type("FPS.*")
            .whitelist_function("SDL_.*rame.*")
            .whitelist_var("FPS.*")
            .blacklist_type("_IO.*|FILE")
            .generate()
            .expect("Unable to generate gfx_framerate_bindings!");

        let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());

        gfx_framerate_bindings
            .write_to_file(out_path.join("sdl_gfx_framerate_bindings.rs"))
            .expect("Couldn't write gfx_framerate_bindings!");

        let gfx_primitives_bindings = create_bindgen_builder(target, host, headers_paths)
            .raw_line("use crate::*;")
            .header("wrapper_gfx_primitives.h")
            .blacklist_type("SDL_.*")
            .whitelist_function("pixel.*")
            .whitelist_function("rectangle.*")
            .whitelist_function("rounded.*")
            .whitelist_function("box.*")
            .whitelist_function(".*line(Color|RGBA).*")
            .whitelist_function("thick.*")
            .whitelist_function(".*circle.*")
            .whitelist_function("arc.*")
            .whitelist_function("filled.*")
            .whitelist_function(".*ellipse.*")
            .whitelist_function("pie.*")
            .whitelist_function(".*trigon.*")
            .whitelist_function(".*polygon.*")
            .whitelist_function("textured.*")
            .whitelist_function("bezier.*")
            .whitelist_function("character.*")
            .whitelist_function("string.*")
            .whitelist_function("gfx.*")
            .blacklist_type("_IO.*|FILE")
            .generate()
            .expect("Unable to generate gfx_primitives_bindings!");

        let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());

        gfx_primitives_bindings
            .write_to_file(out_path.join("sdl_gfx_primitives_bindings.rs"))
            .expect("Couldn't write gfx_primitives_bindings!");

        let gfx_imagefilter_bindings = create_bindgen_builder(target, host, headers_paths)
            //.raw_line("use crate::*;")
            .header("wrapper_gfx_imagefilter.h")
            .whitelist_function("SDL_image.*")
            .blacklist_type("_IO.*|FILE")
            .generate()
            .expect("Unable to generate gfx_imagefilter_bindings!");

        let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());

        gfx_imagefilter_bindings
            .write_to_file(out_path.join("sdl_gfx_imagefilter_bindings.rs"))
            .expect("Couldn't write gfx_imagefilter_bindings!");

        let gfx_rotozoom_bindings = create_bindgen_builder(target, host, headers_paths)
            .raw_line("use crate::*;")
            .header("wrapper_gfx_rotozoom.h")
            .blacklist_type("SDL_.*")
            .blacklist_type("_IO.*|FILE")
            .whitelist_function("rotozoom.*")
            .whitelist_function("zoom.*")
            .whitelist_function("shrink.*")
            .whitelist_function("rotate.*")
            .generate()
            .expect("Unable to generate gfx_rotozoom_bindings!");

        let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());

        gfx_rotozoom_bindings
            .write_to_file(out_path.join("sdl_gfx_rotozoom_bindings.rs"))
            .expect("Couldn't write gfx_rotozoom_bindings!");
    }
}

fn link_sdl() {
    println!("cargo:rustc-flags=-l SDL");

    #[cfg(feature = "mixer")]
    println!("cargo:rustc-flags=-l SDL_mixer");

    #[cfg(feature = "image")]
    println!("cargo:rustc-flags=-l SDL_image");

    #[cfg(feature = "ttf")]
    println!("cargo:rustc-flags=-l SDL_ttf");

    #[cfg(feature = "gfx")]
    println!("cargo:rustc-flags=-l SDL_gfx");
}

fn get_os_from_triple(triple: &str) -> Option<&str> {
    triple.splitn(3, "-").nth(2)
}

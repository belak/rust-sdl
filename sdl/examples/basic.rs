use std::time::Duration;

use sdl::{gfx::primitives::DrawRenderer, Color};

fn main() -> anyhow::Result<()> {
    println!("initializing sdl");

    let sdl_context = sdl::init()?;
    println!("SDL initialized");
    let video_subsystem = sdl_context.video()?;
    println!("video initialized");

    let mut window = video_subsystem
        .window("rust-sdl demo", 800, 600)
        .build()
        .unwrap();

    window.line(10, 10, 100, 100, Color::MAGENTA)?;
    window.circle(50, 50, 10, Color::RED)?;
    window.box_(150, 150, 200, 200, Color::RED)?;

    window.flip()?;

    ::std::thread::sleep(Duration::new(3, 1_000_000_000u32 / 60));

    println!("quiting");

    Ok(())
}

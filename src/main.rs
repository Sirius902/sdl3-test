use std::time::Duration;

use sdl3::event::Event;
use sdl3::keyboard::Keycode;
use sdl3::mouse::MouseButton;
use sdl3::pixels::Color;
use sdl3::video::FullscreenType;

pub fn main() -> Result<(), Box<dyn std::error::Error>> {
    let sdl_context = sdl3::init()?;
    let video_subsystem = sdl_context.video()?;

    let mut window = video_subsystem
        .window("sdl3-test", 800, 600)
        .position_centered()
        .build()?;

    let mut canvas = window.clone().into_canvas();

    canvas.set_draw_color(Color::BLACK);
    canvas.clear();

    let mut event_pump = sdl_context.event_pump()?;
    'running: loop {
        canvas.set_draw_color(Color::BLACK);
        canvas.clear();

        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => break 'running,
                Event::MouseButtonUp {
                    mouse_btn: MouseButton::Left,
                    ..
                } => {
                    let is_enter_fullscreen =
                        matches!(window.fullscreen_state(), FullscreenType::Off);

                    window.set_fullscreen(is_enter_fullscreen)?;
                    sdl_context.mouse().show_cursor(!is_enter_fullscreen);
                }
                _ => {}
            }
        }

        canvas.present();
        std::thread::sleep(Duration::from_secs(1) / 60);
    }

    Ok(())
}

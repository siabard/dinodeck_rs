use sdl2::event::Event;
use sdl2::image::{InitFlag, LoadTexture};
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::TimerSubsystem;

use std::path::Path;

fn main() -> Result<(), String> {
    let sdl_context = sdl2::init().expect("ERROR::MAIN::FAIL::SDL2_INIT");

    // video subsystem
    let video_subsystem = sdl_context
        .video()
        .expect("ERROR::MAIN::FAIL::SDL_CONTEXT.VIDEO");
    let _image_context = sdl2::image::init(InitFlag::PNG | InitFlag::JPG)
        .expect("ERROR::MAIN::FAIL::SDL2 IMAGE INIT");

    // timer subsystem
    let mut timer_subsystem: TimerSubsystem = sdl_context.timer().unwrap();

    // window creation
    let window = video_subsystem
        .window("DINODECK", 800, 600)
        .position_centered()
        .resizable()
        .build()
        .expect("ERROR::MAIN::FAIL::WINDOW CREATION");

    // building renderer
    let mut canvas = window
        .into_canvas()
        .build()
        .expect("ERROR::MAIN::FAIL::CANVAS CREATION");

    // texture_creator
    let texture_creator = canvas.texture_creator();

    // textures
    let mut textures: Vec<Rect> = vec![];

    // floor texture
    let world_texture = texture_creator
        .load_texture(Path::new("resources/town_tileset.png"))
        .unwrap();

    for y in (0..288).step_by(16) {
        for x in (0..176).step_by(16) {
            textures.push(Rect::new(x, y, 16, 16));
        }
    }

    // map
    let mut map: [[usize; 8]; 8] = [
        [165, 165, 165, 165, 172, 165, 165, 165],
        [165, 165, 165, 165, 172, 165, 165, 165],
        [165, 165, 165, 165, 172, 165, 165, 165],
        [165, 165, 165, 165, 172, 165, 165, 165],
        [165, 165, 165, 165, 165, 165, 165, 165],
        [165, 165, 165, 165, 165, 165, 165, 165],
        [165, 165, 165, 165, 165, 165, 165, 165],
        [165, 165, 165, 165, 165, 165, 165, 165],
    ];
    // event
    let mut event_pump = sdl_context
        .event_pump()
        .expect("ERROR::MAIN::FAIL::EVENT_PUMP");

    // delta time
    let mut dt: f64;
    let mut now: u32 = timer_subsystem.ticks();
    let mut last_time: u32 = 0;

    'running: loop {
        dt = (now - last_time) as f64 / 1000.0;
        last_time = now;

        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. } => break 'running,
                Event::KeyUp {
                    keycode: Some(k), ..
                } => match k {
                    Keycode::Escape => break 'running,
                    _ => {}
                },
                _ => {}
            }

            canvas.set_draw_color(Color::RGBA(255, 255, 255, 127));
            canvas.clear();

            for y in 0..8 {
                for x in 0..8 {
                    canvas
                        .copy_ex(
                            &world_texture,
                            Some(textures[map[y][x]]),
                            Some(Rect::new(x as i32 * 16, y as i32 * 16, 16, 16)),
                            0.0,
                            None,
                            false,
                            false,
                        )
                        .unwrap();
                }
            }

            canvas.present();
        }

        now = timer_subsystem.ticks();
    }

    Ok(())
}

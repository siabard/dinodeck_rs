use dinodeck_rs::states;
use sdl2::event::Event;
use sdl2::image::InitFlag;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::TimerSubsystem;

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

    // logical size
    canvas.set_logical_size(320, 240).unwrap();

    // texture_creator
    let texture_creator = canvas.texture_creator();

    // floor texture
    /*
        let world_texture = texture_creator
            .load_texture(Path::new("resources/town_tileset.png"))
            .unwrap();
    */
    // event
    let mut event_pump = sdl_context
        .event_pump()
        .expect("ERROR::MAIN::FAIL::EVENT_PUMP");

    // delta time
    let mut dt: f64;
    let mut now: u32 = timer_subsystem.ticks();
    let mut last_time: u32 = 0;

    // state
    let state = states::State::new(&texture_creator, "assets/tiled_base64_zlib.tmx");

    // states
    let mut states: Vec<Box<states::State>> = vec![];
    states.push(Box::new(state));

    'running: loop {
        dt = (now - last_time) as f64 / 1000.0;
        last_time = now;

        if states.len() == 0 {
            break 'running;
        }

        if let Some(hd_state) = states.last_mut() {
            for event in event_pump.poll_iter() {
                match event {
                    Event::Quit { .. } => break 'running,
                    Event::KeyUp {
                        keycode: Some(k), ..
                    } => match k {
                        Keycode::Escape => break 'running,
                        _ => {
                            hd_state.handle_key_event(&k);
                        }
                    },
                    _ => {}
                }

                hd_state.update(dt);

                canvas.set_draw_color(Color::RGBA(255, 255, 255, 127));
                canvas.clear();

                hd_state.render(&mut canvas);
                canvas.present();
            }
        }

        now = timer_subsystem.ticks();
    }

    Ok(())
}

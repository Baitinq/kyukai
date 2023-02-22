use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use std::time::Duration;

fn main() {
    let sdl_context = sdl2::init().expect("Couldn't initialize the SDL2 context!");
    let sdl_video = sdl_context
        .video()
        .expect("Couldn't get the SDL2 video subsystem!");

    ctrlc::set_handler(move || std::process::exit(1)).expect("Error setting Ctrl-C handler");

    let show_frequency = Duration::from_secs(5);
    loop {
        show_screensaver(&sdl_context, &sdl_video);

        ::std::thread::sleep(show_frequency);
    }
}

fn show_screensaver(sdl_context: &sdl2::Sdl, sdl_video: &sdl2::VideoSubsystem) {
    let window = sdl_video
        .window("Kyukai", 10, 10)
        .fullscreen()
        .build()
        .expect("Failed to build the SDL2 window!");

    let mut canvas = window
        .into_canvas()
        .build()
        .expect("Couldn't get the SDL2 canvas from the window!");

    canvas.set_draw_color(Color::RGB(255, 255, 255));
    canvas.clear();
    canvas.present();

    let mut sdl_event_pump = sdl_context
        .event_pump()
        .expect("Couldn't get the SDL2 event pump!");
    'running: loop {
        for event in sdl_event_pump.wait_iter() {
            match event {
                Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => {
                    break 'running;
                }
                Event::Quit { timestamp: _ } => std::process::exit(0),
                _ => {}
            }
        }
    }
}

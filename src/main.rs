extern crate sdl2;

mod events;

use events::Events;
use sdl2::pixels::Color;

fn main() {
    // Init sdl2
    let sdl_context = sdl2::init().unwrap();
    let video = sdl_context.video().unwrap();

    // Create Window
    let window = video.window("ArcadeRS Schump", 800, 600).position_centered().opengl().build().unwrap();
    let mut renderer = window.renderer().accelerated().build().unwrap();

    let mut events = Events::new(sdl_context.event_pump().unwrap());

    loop {
    events.pump();

        if events.quit || events.key_escape {
            break;
        }

        renderer.set_draw_color(Color::RGB(0,0,0));
        renderer.clear();
        renderer.present();
    }

}

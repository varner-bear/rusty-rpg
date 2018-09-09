extern crate sdl2;
use sdl2::event::Event;
fn main() {
/*specs = "0.12.1"*/
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();
    let window = video_subsystem.window("Rusty RPG",900,700)
                                .position_centered()
                                .resizable()
                                .build()
                                .unwrap();
    let mut event_pump = sdl_context.event_pump().unwrap();
    
    'main: loop {
        for event in event_pump.poll_iter(){
            match event {
                Event::Quit {..} => break 'main,
                _ => {},
            }
        
        }
    }
}

extern crate sdl3;

/* sdl3 */
use sdl3::pixels::Color;
use sdl3::event::Event;
use sdl3::keyboard::Keycode;
use std::time::Duration;

mod physics;

pub fn main() {
    /* create sdl context */
    let sdl_context = sdl3::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    /* create window */
    let window = video_subsystem.window("Motio", 800, 600)
        .position_centered()
        .build()
        .unwrap();

    /* create canvas */
    let mut canvas = window.into_canvas();
    canvas.set_draw_color(Color::RGB(0, 255, 255));
    canvas.clear();
    canvas.present();
    canvas.set_draw_color(Color::RGB(0, 0, 0));

    /* create vector of physics objects */
    let mut objects: Vec<physics::PhysicsObject> = Vec::new();
    let mut gravity_obj = physics::GravityObject::new(0, 0, 0.0, 0.0);
    objects.push(physics::PhysicsObject::GravityObject(gravity_obj));

    /* run loop, check events */
    let mut event_pump = sdl_context.event_pump().unwrap();
    'running: loop {
        canvas.clear();
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit {..} |
                Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                    break 'running
                },
                _ => {}
            }
        }
        /* update physics objects */
        let delta = 1.0 / 60.0;
        physics::update(&mut objects, delta);
        /* present canvas */
        canvas.present();
        /* idle */
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }
}


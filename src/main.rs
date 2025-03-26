extern crate sdl3;

/* sdl3 */
use sdl3::pixels::Color;
use sdl3::event::Event;
use sdl3::keyboard::Keycode;
use sdl3::Error;
use std::time::Duration;

mod objects;

fn main() -> Result<(), Error> {
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
    canvas.set_draw_color(Color::RGB(0, 0, 0));
    canvas.clear();
    canvas.present();

    /* create vectors of objects */
    let mut objects: Vec<&mut dyn objects::Object> = Vec::new();
    /* create square */
    let mut square_object = objects::Square::new(objects::Transform::new(0.0, 0.0, 0.0, 0.0, 1.0), 10.0);
    /* push to vector */
    objects.push(&mut square_object);

    /* run loop, check events */
    let mut event_pump = sdl_context.event_pump().unwrap();
    'running: loop {
        canvas.set_draw_color(Color::RGB(0, 0, 0));
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
        main_loop(&mut canvas, &mut objects);
        /* present canvas */
        canvas.present();
        /* idle */
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }
    Ok(())
}

/* objects must be mut in order to use iter_mut */
fn main_loop(canvas: &mut sdl3::render::Canvas<sdl3::video::Window>,
        objects: &mut Vec<&mut dyn objects::Object>)
        -> Result<(), Error> {
    /* get delta time */
    let delta = 1.0 / 60.0;
    /* iterate over objects */
    for object in objects.iter_mut() {
        object.physics_update(delta);
        object.draw(canvas)?;
    }
    Ok(())
}


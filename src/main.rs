extern crate sdl3;

/* sdl3 */
use sdl3::pixels::Color;
use sdl3::event::Event;
use sdl3::keyboard::Keycode;
use sdl3::render::FRect;
use sdl3::Error;
use std::time::Duration;

mod physics;

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

    /* create vector of physics objects */
    let mut objects: Vec<physics::PhysicsObject> = Vec::new();
    let gravity_obj = physics::GravityObject::new(0.0, 0.0, 0.0, 0.0);
    objects.push(physics::PhysicsObject::GravityObject(gravity_obj));

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
        /* update physics objects */
        let delta = 1.0 / 60.0;
        physics::update(&mut objects, delta);
        /* draw objects */
        draw(&mut canvas, &objects)?;
        /* present canvas */
        canvas.present();
        /* idle */
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }
    Ok(())
}


fn draw(canvas: &mut sdl3::render::Canvas<sdl3::video::Window>,
        objects: &Vec<physics::PhysicsObject>)
        -> Result<(), Error> {
    canvas.set_draw_color(Color::RGB(255, 0, 0));
    for object in objects.iter() {
        match object {
            /* apply update to gravity object */
            physics::PhysicsObject::GravityObject(object) => {
                canvas.draw_rect(FRect::new(object.x, object.y, 50.0, 50.0))?;
            }
        }
    }
    Ok(())
}




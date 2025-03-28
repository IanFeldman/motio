extern crate sdl3;

/* sdl3 */
use sdl3::pixels::Color;
use sdl3::event::Event;
use sdl3::image::LoadTexture;
use sdl3::keyboard::Keycode;
use sdl3::Error;
use sdl3::render::FRect;
use std::path::Path;
use std::time::Duration;

mod objects;

fn main() -> Result<(), Error>
{
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

    /* load square texture */
    let texture_creator = canvas.texture_creator();
    let path = Path::new("assets/square.png");
    let texture = texture_creator.load_texture(path)?;

    /* create vectors of objects */
    let mut objects: Vec<&mut objects::Object> = Vec::new();
    /* create square */
    let mut obj = objects::Object::new(
        objects::Transform::new(400.0, 300.0, 10.0, 10.0, 1.0),
        objects::Sprite::new(64.0, 64.0, texture),
        objects::ObjectType::Normal);
    /* push to vector */
    objects.push(&mut obj);

    /* run loop, check events */
    let mut event_pump = sdl_context.event_pump().unwrap();
    'running: loop
    {
        canvas.set_draw_color(Color::RGB(0, 0, 0));
        canvas.clear();
        for event in event_pump.poll_iter()
        {
            match event
            {
                Event::Quit {..} |
                Event::KeyDown { keycode: Some(Keycode::Escape), .. } =>
                {
                    break 'running
                },
                _ => {}
            }
        }
        main_loop(&mut canvas, &mut objects)?;
        /* present canvas */
        canvas.present();
        /* idle */
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }
    Ok(())
}

/* objects must be mut in order to use iter_mut */
fn main_loop(canvas: &mut sdl3::render::Canvas<sdl3::video::Window>,
    objects: &mut Vec<&mut objects::Object>) -> Result<(), Error>
{
    /* get delta time */
    let delta = 1.0 / 60.0;
    /* iterate over objects */
    for object in objects.iter_mut()
    {
        objects::update(object, delta);
        draw(canvas, object)?;
    }
    Ok(())
}

fn draw(canvas: &mut sdl3::render::Canvas<sdl3::video::Window>,
    object: &mut objects::Object) -> Result<(), Error>
{
    let pos_x = object.transform.x - object.sprite.width / 2.0;
    let pos_y = object.transform.y - object.sprite.height / 2.0;

    canvas.copy_ex(
        &object.sprite.texture,
        None,
        /* TODO: consider saving rect to struct to avoid this overhead */
        FRect::new(pos_x, pos_y, object.sprite.width, object.sprite.height),
        object.transform.theta as f64,
        None,
        false,
        false
    )?;
    Ok(())
}


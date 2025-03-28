extern crate sdl3;

/* sdl3 */
use sdl3::Error;
use sdl3::event::Event;
use sdl3::image::LoadTexture;
use sdl3::keyboard::{Keycode, Scancode};
use sdl3::pixels::Color;
use sdl3::render::FRect;
use std::path::Path;
use std::time::Duration;

mod objects;

pub struct Camera
{
    x: f32,
    y: f32,
    width: u32,
    height: u32,
    scale: f32,
    move_speed: f32,
    zoom_speed: f32,
}

impl Camera
{
    fn new(x: f32, y: f32, width: u32, height: u32, scale: f32,
        move_speed: f32, zoom_speed: f32) -> Self
    {
        Camera { x, y, width, height, scale, move_speed, zoom_speed }
    }
}

fn main() -> Result<(), Error>
{
    /* create camera */
    let mut camera = Camera::new(0.0, 0.0, 800, 600, 1.0, 200.0, 5.0);

    /* create sdl context */
    let sdl_context = sdl3::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    /* create window */
    let window = video_subsystem.window("Motio", camera.width, camera.height)
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
    let path = Path::new("assets/gear.png");
    let texture = texture_creator.load_texture(path)?;

    /* create vectors of objects */
    let mut objects: Vec<&mut objects::Object> = Vec::new();
    /* create square */
    let mut obj = objects::Object::new(
        objects::Transform::new(0.0, 0.0, 25.0, 10.0, 1.0),
        objects::Sprite::new(64.0, 64.0, texture),
        objects::ObjectType::Spring(5.0));
    /* push to vector */
    objects.push(&mut obj);

    /* run loop, check events */
    let mut event_pump = sdl_context.event_pump().unwrap();
    'running: loop
    {
        /* listen for quit event, mouse events */
        for event in event_pump.poll_iter()
        {
            match event
            {
                Event::Quit {..} |
                Event::KeyDown { keycode: Some(Keycode::Escape), .. } =>
                {
                    break 'running
                },
                Event::MouseWheel { y, .. } =>
                {
                    handle_mouse_wheel(y, &mut camera, 1.0 / 60.0);
                },
                _ => {}
            }
        }
        /* listen for keyboard presses */
        poll_key(&event_pump, &mut camera, 1.0 / 60.0);
        /* run main loop */
        main_loop(&mut canvas, &mut objects, &camera)?;
        /* idle */
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }
    Ok(())
}

/* poll keyboard for camera movement input */
fn poll_key(event_pump: &sdl3::EventPump, camera: &mut Camera, delta_time: f32)
{
    /* cam speed increases as scale increases for better feel */
    let keystates = event_pump.keyboard_state();
    if keystates.is_scancode_pressed(Scancode::W)
    {
        camera.y -= camera.move_speed * camera.scale * delta_time;
    }
    if keystates.is_scancode_pressed(Scancode::A)
    {
        camera.x -= camera.move_speed * camera.scale * delta_time;
    }
    if keystates.is_scancode_pressed(Scancode::S)
    {
        camera.y += camera.move_speed * camera.scale * delta_time;
    }
    if keystates.is_scancode_pressed(Scancode::D)
    {
        camera.x += camera.move_speed * camera.scale *delta_time;
    }
}

/* update camera scale based on mouse scroll wheel input */
fn handle_mouse_wheel(y: f32, camera:&mut Camera, delta_time: f32)
{
    if y > 0.0
    {
        camera.scale += camera.zoom_speed * delta_time;
    }
    else if y < 0.0
    {
        let scale = camera.scale - camera.zoom_speed * delta_time;
        if scale < 0.0
        {
            camera.scale = 0.0;
        }
        else
        {
            camera.scale = scale;
        }
    }
}

/* update and draw all objects */
/* objects must be mut in order to use iter_mut */
fn main_loop(canvas: &mut sdl3::render::Canvas<sdl3::video::Window>,
    objects: &mut Vec<&mut objects::Object>,
    camera: &Camera) -> Result<(), Error>
{
    /* clear canvas */
    canvas.set_draw_color(Color::RGB(0, 0, 0));
    canvas.clear();

    /* iterate over objects */
    for object in objects.iter_mut()
    {
        objects::update(object, 1.0 / 60.0);
        draw(canvas, object, camera)?;
    }

    /* present canvas */
    canvas.present();
    Ok(())
}

/* render object to screen */
fn draw(canvas: &mut sdl3::render::Canvas<sdl3::video::Window>,
    object: &mut objects::Object,
    camera: &Camera) -> Result<(), Error>
{
    /* apply scale to sprite dimensions */
    let sprite_width = object.sprite.width * camera.scale;
    let sprite_height = object.sprite.width * camera.scale;

    /* ensure sprite is drawn from center, not corner */
    let mut pos_x = object.transform.x - sprite_width / 2.0;
    let mut pos_y = object.transform.y - sprite_height / 2.0;

    /* apply camera position and size */
    pos_x = pos_x - camera.x + camera.width as f32 / 2.0;
    pos_y = pos_y - camera.y + camera.height as f32 / 2.0;

    /* draw */
    canvas.copy_ex(
        &object.sprite.texture,
        None,
        /* TODO: consider saving rect to struct to avoid this overhead */
        FRect::new(pos_x, pos_y, sprite_width, sprite_height),
        object.transform.theta as f64,
        None,
        false,
        false
    )?;
    Ok(())
}


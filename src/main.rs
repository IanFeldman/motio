extern crate sdl3;

/* sdl3 */
use sdl3::Error;
use sdl3::event::Event;
use sdl3::image::LoadTexture;
use sdl3::keyboard::{Keycode, Scancode};
use sdl3::mouse::MouseButton;
use sdl3::pixels::Color;
use sdl3::render::{Texture, ScaleMode};
use std::path::Path;
use std::time::Duration;

mod objects;
mod render;

fn main() -> Result<(), Error>
{
    /* create camera */
    let mut camera = render::Camera::new(0.0, 0.0, 800, 600, 1.0, 400.0, 5.0);

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

    /* load texture */
    let texture_creator = canvas.texture_creator();
    let mut textures: Vec<Texture> = Vec::new();
    let path = Path::new("assets/gear.png");
    let mut texture = texture_creator.load_texture(path)?;
    texture.set_scale_mode(ScaleMode::Nearest);
    textures.push(texture);

    /* create vector of objects */
    let mut objects: Vec<objects::Object> = Vec::new();
    create_gear(0.0, 0.0, 22.5, 100.0, objects::ObjectType::Spring(50.0), &mut objects);
    create_gear(60.0, 0.0, 0.0, 100.0, objects::ObjectType::Normal, &mut objects);

    /* run loop, check events */
    let mut event_pump = sdl_context.event_pump().unwrap();
    'running: loop
    {
        let delta_time = 1.0 / 60.0;
        /* listen for quit event, mouse events */
        for event in event_pump.poll_iter()
        {
            match event
            {
                /* quit actions */
                Event::Quit {..} |
                Event::KeyDown { keycode: Some(Keycode::Escape), .. } =>
                {
                    break 'running
                },
                /* scrolling */
                Event::MouseWheel { y, .. } =>
                {
                    handle_mouse_wheel(y, &mut camera, delta_time);
                },
                /* clicking */
                /* TODO: create mouse handler function */
                Event::MouseButtonDown { mouse_btn, x, y, .. } =>
                {
                    match mouse_btn
                    {
                        MouseButton::Left =>
                        {
                            let (x_world, y_world) =
                                render::screen_to_world(x, y, &camera);
                            create_gear(x_world, y_world, 22.5, 100.0,
                                objects::ObjectType::Normal, &mut objects);
                        },
                        MouseButton::Right =>
                        {
                            let (x_world, y_world) =
                                render::screen_to_world(x, y, &camera);
                            create_gear(x_world, y_world, 22.5, 100.0,
                                objects::ObjectType::Spring(50.0), &mut objects);
                        },
                        _ => (),
                    }
                },
                _ => {}
            }
        }
        /* listen for keyboard presses */
        poll_key(&event_pump, &mut camera, delta_time);
        /* run main loop */
        main_loop(&mut canvas, &textures, &mut objects, &camera, delta_time)?;
        /* idle */
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }
    Ok(())
}

/* poll keyboard for camera movement input */
fn poll_key(event_pump: &sdl3::EventPump, camera: &mut render::Camera, delta_time: f32)
{
    /* cam speed increases as scale increases for better feel */
    let keystates = event_pump.keyboard_state();
    if keystates.is_scancode_pressed(Scancode::W)
    {
        camera.y -= camera.move_speed * delta_time;
    }
    if keystates.is_scancode_pressed(Scancode::A)
    {
        camera.x -= camera.move_speed * delta_time;
    }
    if keystates.is_scancode_pressed(Scancode::S)
    {
        camera.y += camera.move_speed * delta_time;
    }
    if keystates.is_scancode_pressed(Scancode::D)
    {
        camera.x += camera.move_speed * delta_time;
    }
}

/* update camera scale based on mouse scroll wheel input */
fn handle_mouse_wheel(y: f32, camera:&mut render::Camera, delta_time: f32)
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
fn main_loop(canvas: &mut sdl3::render::Canvas<sdl3::video::Window>,
    textures: &Vec<Texture>,
    objects: &mut Vec<objects::Object>,
    camera: &render::Camera,
    delta_time: f32) -> Result<(), Error>
{
    /* clear canvas */
    canvas.set_draw_color(Color::RGB(0, 0, 0));
    canvas.clear();

    objects::update_all(objects, delta_time);
    for object in objects.iter_mut()
    {
        render::draw(canvas, textures, object, camera, true)?;
    }

    /* present canvas */
    canvas.present();
    Ok(())
}

/* create gear object and add it to list of all objects */
fn create_gear(x: f32, y: f32, theta: f32, mass: f32, object_type: objects::ObjectType,
    objects: &mut Vec<objects::Object>)
{
    let mut collider: Vec<objects::SphereCollider> = Vec::new();
    collider.push(objects::SphereCollider::new(0.0, 0.0, 32.0));
    let gear = objects::Object::new(
        objects::Transform::new(x, y, theta, 0.0, mass),
        objects::Sprite::new(64.0, 64.0, 0),
        collider,
        object_type);
    objects.push(gear);
}


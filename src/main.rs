extern crate sdl3;

/* sdl3 */
use sdl3::Error;
use sdl3::event::Event;
use sdl3::image::LoadTexture;
use sdl3::keyboard::{Keycode, Scancode};
use sdl3::pixels::Color;
use sdl3::render::{FRect, Texture, ScaleMode};
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
    let mut camera = Camera::new(0.0, 0.0, 800, 600, 1.0, 400.0, 5.0);

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
    let mut collider: Vec<objects::SphereCollider> = Vec::new();
    collider.push(objects::SphereCollider::new(0.0, 0.0, 32.0));
    /* create gear */
    let obj1 = objects::Object::new(
        objects::Transform::new(0.0, 0.0, 32.0, 0.0, 1.0),
        objects::Sprite::new(64.0, 64.0, 0),
        collider,
        objects::ObjectType::Spring(5.0));
    /* create gear */
    /* consider how to reuse collider */
    let mut collider: Vec<objects::SphereCollider> = Vec::new();
    collider.push(objects::SphereCollider::new(0.0, 0.0, 32.0));
    let obj2 = objects::Object::new(
        objects::Transform::new(0.0, 60.0, 180.0, 0.0, 1.0),
        objects::Sprite::new(64.0, 64.0, 0),
        collider,
        objects::ObjectType::Normal);
    /* create gear */
    let mut collider: Vec<objects::SphereCollider> = Vec::new();
    collider.push(objects::SphereCollider::new(0.0, 0.0, 32.0));
    let obj3 = objects::Object::new(
        objects::Transform::new(10.0, 120.0, 0.0, 0.0, 1.0),
        objects::Sprite::new(64.0, 64.0, 0),
        collider,
        objects::ObjectType::Normal);
    /* push to vector */
    objects.push(obj1);
    objects.push(obj2);
    objects.push(obj3);

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
                Event::Quit {..} |
                Event::KeyDown { keycode: Some(Keycode::Escape), .. } =>
                {
                    break 'running
                },
                Event::MouseWheel { y, .. } =>
                {
                    handle_mouse_wheel(y, &mut camera, delta_time);
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
fn poll_key(event_pump: &sdl3::EventPump, camera: &mut Camera, delta_time: f32)
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
fn main_loop(canvas: &mut sdl3::render::Canvas<sdl3::video::Window>,
    textures: &Vec<Texture>,
    objects: &mut Vec<objects::Object>,
    camera: &Camera,
    delta_time: f32) -> Result<(), Error>
{
    /* clear canvas */
    canvas.set_draw_color(Color::RGB(0, 0, 0));
    canvas.clear();

    objects::update_all(objects, delta_time);
    for object in objects.iter_mut()
    {
        draw(canvas, textures, object, camera, true)?;
    }

    /* present canvas */
    canvas.present();
    Ok(())
}

/* render object to screen */
fn draw(canvas: &mut sdl3::render::Canvas<sdl3::video::Window>,
    textures: &Vec<Texture>,
    object: &mut objects::Object,
    camera: &Camera,
    debug: bool) -> Result<(), Error>
{
    /* apply scale to sprite dimensions */
    let sprite_width = object.sprite.width * camera.scale;
    let sprite_height = object.sprite.height * camera.scale;

    /* apply scale to camera position */
    let cam_x = camera.x * camera.scale;
    let cam_y = camera.y * camera.scale;

    /* apply scale to object position */
    let obj_x = object.transform.x * camera.scale;
    let obj_y = object.transform.y * camera.scale;

    /* ensure sprite is drawn from center, not corner */
    let mut pos_x = obj_x - sprite_width / 2.0;
    let mut pos_y = obj_y - sprite_height / 2.0;

    /* apply camera position and size */
    pos_x = pos_x - cam_x + camera.width as f32 / 2.0;
    pos_y = pos_y - cam_y + camera.height as f32 / 2.0;

    /* draw */
    canvas.copy_ex(
        &textures[object.sprite.texture_idx as usize],
        None,
        /* TODO: consider saving rect to struct to avoid this overhead */
        FRect::new(pos_x, pos_y, sprite_width, sprite_height),
        object.transform.theta as f64,
        None,
        false,
        false
    )?;

    /* draw colliders */
    if debug
    {
        /* set draw color */
        canvas.set_draw_color(Color::RGB(0, 255, 0));

        /* get position of object center */
        pos_x = obj_x - cam_x + camera.width as f32 / 2.0;
        pos_y = obj_y - cam_y + camera.height as f32 / 2.0;

        /* iterate over sphere colliders */
        for sphere in object.collider.iter()
        {
            /* update radius with scale */
            let radius = sphere.r * camera.scale;

            /* rotate point */
            let (collider_x, collider_y) = objects::rotate_point(sphere.x, sphere.y,
                object.transform.theta);

            /* draw rectangle */
            canvas.draw_rect(
                FRect::new(pos_x + collider_x * camera.scale - radius,
                    pos_y + collider_y * camera.scale - radius,
                    radius * 2.0, radius * 2.0)
            )?;
        }
        /* reset draw color */
        canvas.set_draw_color(Color::RGB(0, 0, 0));
    }

    Ok(())
}


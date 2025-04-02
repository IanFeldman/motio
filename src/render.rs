use sdl3::Error;
use sdl3::pixels::Color;
use sdl3::render::{FRect, Texture, Canvas, FPoint};

use crate::objects;

pub struct Camera
{
    pub x: f32,
    pub y: f32,
    pub width: u32,
    pub height: u32,
    pub scale: f32,
    pub move_speed: f32,
    pub zoom_speed: f32,
}

impl Camera
{
    pub fn new(x: f32, y: f32, width: u32, height: u32, scale: f32,
        move_speed: f32, zoom_speed: f32) -> Self
    {
        Camera { x, y, width, height, scale, move_speed, zoom_speed }
    }
}

/* rotate point (x, y) theta degrees about the origin */
pub fn rotate_point(x: f32, y: f32, theta: f32) -> (f32, f32)
{
    let theta_rad = theta * std::f32::consts::PI / 180.0;
    let x2 = x * theta_rad.cos() - y * theta_rad.sin();
    let y2 = x * theta_rad.sin() + y * theta_rad.cos();
    (x2, y2)
}

/* convert (x, y) in screen space to world space */
pub fn screen_to_world(x: f32, y: f32, camera: &Camera) -> (f32, f32)
{
    let x_world = camera.x + (x - camera.width as f32 / 2.0) / camera.scale;
    let y_world = camera.y + (y - camera.height as f32 / 2.0) / camera.scale;
    (x_world, y_world)
}

/* convert (x, y) in world space to screen space */
pub fn world_to_screen(x: f32, y: f32, camera: &Camera) -> (f32, f32)
{
    let x_screen = (x - camera.x) * camera.scale + camera.width as f32 / 2.0;
    let y_screen = (y - camera.y) * camera.scale + camera.height as f32 / 2.0;
    (x_screen, y_screen)
}

/* draw a circle with num_points and (x, y) in screen space */
fn draw_circle(canvas: &mut Canvas<sdl3::video::Window>,
    x: f32, y: f32, r: f32, num_points: u32)
{
    let theta_increment = 360.0 / num_points as f32;
    let mut x_rot = r;
    let mut y_rot = 0.0;
    for _ in 0..num_points
    {
        (x_rot, y_rot) = rotate_point(x_rot, y_rot, theta_increment);
        let _ = canvas.draw_point(FPoint::new(x + x_rot, y + y_rot));
    }
}

/* render object to screen */
pub fn draw(canvas: &mut Canvas<sdl3::video::Window>,
    textures: &Vec<Texture>,
    object: &mut objects::Object,
    camera: &Camera,
    debug: bool) -> Result<(), Error>
{
    let (pos_x, pos_y) = world_to_screen(
        object.transform.x - object.sprite.width / 2.0,
        object.transform.y - object.sprite.height / 2.0,
        camera
    );

    let sprite_width = object.sprite.width * camera.scale;
    let sprite_height = object.sprite.height * camera.scale;

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
        let (mut pos_x, mut pos_y) = world_to_screen(
            object.transform.x,
            object.transform.y,
            camera
        );

        /* iterate over sphere colliders */
        for sphere in object.collider.iter()
        {
            /* update radius with scale */
            let radius = sphere.r * camera.scale;

            /* rotate point with object rotation */
            let (collider_x, collider_y) = rotate_point(
                sphere.x, sphere.y, object.transform.theta);

            /* add to position */
            pos_x += collider_x * camera.scale;
            pos_y += collider_y * camera.scale;

            draw_circle(canvas, pos_x, pos_y, radius, 360);
        }
        /* reset draw color */
        canvas.set_draw_color(Color::RGB(0, 0, 0));
    }

    Ok(())
}

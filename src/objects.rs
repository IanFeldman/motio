use sdl3::Error;
use sdl3::render::{FRect, Texture};

pub struct Sprite {
    pub width: f32,
    pub height: f32,
    pub texture: Texture
}

impl Sprite {
    pub fn new(width: f32, height: f32, texture: Texture) -> Self {
        Sprite { width, height, texture }
    }
}

/* Position, angle, and physics properties */
pub struct Transform {
    pub x: f32,
    pub y: f32,
    pub theta: f32,
    pub angular_velocity: f32,
    pub mass: f32,
}

/* Implement constructor and default physics update */
impl Transform {
    pub fn new(x: f32, y: f32, theta: f32, angular_velocity: f32, mass: f32) -> Self {
        Transform { x, y, theta, angular_velocity, mass }
    }
    pub fn update(&mut self, delta_time: f32) {
        /* apply angular velocity */
        self.theta += self.angular_velocity * delta_time;
    }
}

/* Object trait implements physics update and draw */
pub trait Object {
    fn physics_update(&mut self, delta_time: f32);
    fn draw(&self, canvas: &mut sdl3::render::Canvas<sdl3::video::Window>)
        -> Result<(), Error>;
}

/* Individual object types */
pub struct Square {
    pub transform: Transform,
    pub sprite: Sprite,
}

impl Square {
    pub fn new(transform: Transform, sprite: Sprite) -> Self {
        Square { transform, sprite }
    }
}

impl Object for Square {
    fn physics_update(&mut self, delta_time: f32) {
        /* square specific physics */
        self.transform.x += 10.0 * delta_time;
        self.transform.y += 10.0 * delta_time;
        /* transform update */
        self.transform.update(delta_time);
    }
    fn draw(&self, canvas: &mut sdl3::render::Canvas<sdl3::video::Window>)
        -> Result<(), Error> {
        canvas.copy(&self.sprite.texture, None, FRect::new(self.transform.x, self.transform.y, self.sprite.width, self.sprite.height))?;
        Ok(())
    }
}


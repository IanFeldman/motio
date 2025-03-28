use sdl3::render::Texture;

pub struct Sprite
{
    pub width: f32,
    pub height: f32,
    pub texture: Texture
}

impl Sprite
{
    pub fn new(width: f32, height: f32, texture: Texture) -> Self
    {
        Sprite { width, height, texture }
    }
}

/* Position, angle, and physics properties */
pub struct Transform
{
    pub x: f32,
    pub y: f32,
    pub theta: f32,
    pub angular_velocity: f32,
    pub mass: f32,
}

/* Implement constructor and default physics update */
impl Transform
{
    pub fn new(x: f32, y: f32, theta: f32, angular_velocity: f32, mass: f32) -> Self
    {
        Transform { x, y, theta, angular_velocity, mass }
    }
    pub fn update(&mut self, delta_time: f32)
    {
        /* apply angular velocity */
        self.theta += self.angular_velocity * delta_time;
    }
}

pub enum ObjectType
{
    Normal,
    Spring,
    Static
}

/* Object types */
pub struct Object
{
    pub transform: Transform,
    pub sprite: Sprite,
    pub object_type: ObjectType
    /* collider */
}

impl Object
{
    pub fn new(transform: Transform, sprite: Sprite, object_type: ObjectType) -> Self
    {
        Object { transform, sprite, object_type }
    }
}

pub fn update(object: &mut Object, delta_time: f32)
{
    /* check which object type, perform update */
    object.transform.update(delta_time);
}


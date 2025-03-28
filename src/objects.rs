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

/* position, angle, and physics properties */
pub struct Transform
{
    pub x: f32,
    pub y: f32,
    pub theta: f32,
    pub angular_velocity: f32,
    pub mass: f32,
}

/* implement constructor and default physics update */
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
        /* clamp rotation for both positive and negative */
        self.theta %= 360.0;
    }
}

pub enum ObjectType
{
    Normal,
    Spring(f32), /* spring constant */
    Static
}

/* object types */
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

/* update physics for object based on type */
pub fn update(object: &mut Object, delta_time: f32)
{
    match object.object_type
    {
        ObjectType::Normal =>
        {
            object.transform.update(delta_time);
        }
        ObjectType::Spring(k) =>
        {
            let torque = -k * object.transform.theta;
            let force = torque * (object.sprite.width / 2.0);
            let accel = force / object.transform.mass;
            object.transform.angular_velocity += accel * delta_time;
            object.transform.update(delta_time);
        }
        ObjectType::Static =>
        {
        }
    }
}


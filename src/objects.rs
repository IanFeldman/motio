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

/* texture information */
pub struct Sprite
{
    pub width: f32,
    pub height: f32,
    pub texture_idx: u32 /* index of texture in the master list */
}

/* implement sprite constructor */
impl Sprite
{
    pub fn new(width: f32, height: f32, texture_idx: u32) -> Self
    {
        Sprite { width, height, texture_idx }
    }
}

pub struct SphereCollider
{
    /* position relative to owner, and radius */
    pub x: f32,
    pub y: f32,
    pub r: f32
}

impl SphereCollider
{
    pub fn new(x: f32, y: f32, r: f32) -> Self
    {
        SphereCollider { x, y, r }
    }
}

/* object type indicating how object is affected by physics */
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
    pub collider: Vec<SphereCollider>,
    pub object_type: ObjectType
}

impl Object
{
    pub fn new(transform: Transform, sprite: Sprite,
        collider: Vec<SphereCollider>, object_type: ObjectType) -> Self
    {
        Object { transform, sprite, collider, object_type }
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


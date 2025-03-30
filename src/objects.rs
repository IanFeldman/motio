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
pub fn update_all(objects: &mut Vec<Object>, delta_time: f32)
{
    /* compute collisions */
    let len = objects.len();
    for i in 0..len
    {
        /* split objects */
        let (left, right) = objects.split_at_mut(i);
        /* edge case */
        if i == len - 1
        {
            for other in left.iter_mut()
            {
                detect_collision(&mut right[0], other);
            }
            break;
        }
        /* normal case */
        let (current, rest) = right.split_at_mut(1);
        for other in rest.iter_mut()
        {
            detect_collision(&mut current[0], other);
        }
        for other in left.iter_mut()
        {
            detect_collision(&mut current[0], other);
        }
    }

    for object in objects.iter_mut()
    {
        /* apply physics */
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
}

fn detect_collision(object1: &Object, object2: &Object) -> bool
{
    for collider1 in object1.collider.iter()
    {
        /* get absolute position of collider1 */
        let (mut x1, mut y1) = rotate_point(collider1.x, collider1.y,
            object1.transform.theta);
        x1 += object1.transform.x;
        y1 += object1.transform.y;

        for collider2 in object2.collider.iter()
        {
            /* get absolute position of collider2 */
            let (mut x2, mut y2) = rotate_point(collider2.x, collider2.y,
                object2.transform.theta);
            x2 += object2.transform.x;
            y2 += object2.transform.y;

            /* check collision */
            let dx = x2 - x1;
            let dy = y2 - y1;
            let distance = (dx * dx - dy * dy).sqrt();
            return distance < collider1.r + collider2.r
        }
    }
    false
}

pub fn rotate_point(x: f32, y: f32, theta: f32) -> (f32, f32)
{
    let theta_rad = theta * std::f32::consts::PI / 180.0;
    let x2 = x * theta_rad.cos() - y * theta_rad.sin();
    let y2 = x * theta_rad.sin() + y * theta_rad.cos();
    (x2, y2)
}


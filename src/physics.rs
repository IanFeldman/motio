pub struct GravityObject {
    pub x: i32,
    pub y: i32,
    pub v_x: f32,
    pub v_y: f32
}

impl GravityObject {
    pub fn new(x: i32, y: i32, v_x: f32, v_y: f32) -> Self {
        GravityObject{x, y, v_x, v_y}
    }
}

pub struct TorsionSpring {
    pub theta: f32
}

impl TorsionSpring {
    pub fn new(theta: f32) -> Self {
        TorsionSpring{theta}
    }
}

/* enum to hold different types of physics objects */
pub enum PhysicsObject {
    GravityObject(GravityObject),
    Spring(TorsionSpring),
}

/* update physics objects */
pub fn update(objects: &mut Vec<PhysicsObject>, delta_time: f32) {
    for object in objects.iter_mut() {
        match object {
            /* apply update to gravity object */
            PhysicsObject::GravityObject(object) => {
                object.v_y += 100.0 * delta_time;
                object.y += (object.v_y * delta_time) as i32;
                object.x += (object.v_x * delta_time) as i32;
                println!("Pos: ({}, {})", object.x, object.y);
            }
            /* apply update to spring */
            PhysicsObject::Spring(spring) => {
            }
        }
    }
}



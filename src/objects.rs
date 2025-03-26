/* Shared attributes across all objects */
pub struct Transform {
    pub x: f32,
    pub y: f32,
    pub theta: f32,
    pub angular_velocity: f32,
    pub mass: f32,
}

impl Transform {
    pub fn new(x: f32, y: f32, theta: f32, angular_velocity: f32, mass: f32) -> Self {
        Transform { x, y, theta, angular_velocity, mass }
    }
}

pub struct Square {
    pub transform: Transform,
    side_length: f32
}

impl Square {
    pub fn new(transform: Transform, side_length: f32) -> Self {
        Square { transform, side_length }
    }
}


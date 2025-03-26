use crate::objects;

pub trait Physics {
    fn update(&mut self, delta_time: f32);
}

impl Physics for objects::Transform {
    fn update(&mut self, delta_time: f32) {
        self.theta += self.angular_velocity * delta_time;
    }
}

impl Physics for objects::Square {
    fn update(&mut self, delta_time: f32) {
        /* testing */
        self.transform.x += 100.0 * delta_time;
        self.transform.update(delta_time);
    }
}


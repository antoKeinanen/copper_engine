use super::{EulerAngle, Vector3};

pub struct LookAtCoordinate {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl LookAtCoordinate {
    fn new(x: f32, y: f32, z: f32) -> LookAtCoordinate {
        LookAtCoordinate { x, y, z }
    }

    fn from_euler_angle(rotation: EulerAngle, offset: Vector3) -> LookAtCoordinate {
        
        todo!()
    }
}


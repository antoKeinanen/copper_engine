use std::f32::consts::PI;

use libm::{atan2f, copysignf};

pub struct EulerAngle {
    pub yaw: f32,
    pub pitch: f32,
    pub roll: f32,
}

impl EulerAngle {
    pub fn new(yaw: f32, pitch: f32, roll: f32) -> EulerAngle {
        EulerAngle { yaw, pitch, roll }
    }

    pub fn from_quaternion_separate(x: f32, y: f32, z: f32, w: f32) -> EulerAngle {
        //https://stackoverflow.com/a/70462919

        //roll
        let sint_cosp = 2.0 * (w * x + y * z);
        let cost_cosp = 1.0 - 2.0 * (x * x + y * y);
        let roll = atan2f(sint_cosp, cost_cosp);

        //pitch
        let sinp = 2.0 * (w * y - z * x);
        let pitch = if sinp.abs() >= 1.0 {
            copysignf(PI / 2.0, sinp)
        } else {
            sinp.asin()
        };

        //yaw
        let siny_cosp = 2.0 * (w * z + x * y);
        let cosy_cosp = 1.0 - 2.0 * (y * y + z * z);
        let yaw = atan2f(siny_cosp, cosy_cosp);

        EulerAngle { yaw, pitch, roll }
    }
}

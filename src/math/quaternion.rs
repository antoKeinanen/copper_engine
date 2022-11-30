pub struct Quaternion {
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub w: f32,
}

impl Quaternion {
    pub fn new(x: f32, y: f32, z: f32, w: f32) -> Quaternion {
        Quaternion { x, y, z, w }
    }

    pub fn from_euler_separate(yaw: f32, pitch: f32, roll: f32) -> Quaternion {
        //https://stackoverflow.com/a/70462919
        let cy = (yaw * 0.5).cos();
        let sy = (yaw * 0.5).sin();
        let cp = (pitch * 0.5).cos();
        let sp = (pitch * 0.5).sin();
        let cr = (roll * 0.5).cos();
        let sr = (roll * 0.5).sin();

        Quaternion {
            w: (cr * cp * cy + sr * sp * sy),
            x: (sr * cp * cy - cr * sp * sy),
            y: (cr * sp * cy + sr * cp * sy),
            z: (cr * cp * sy - sr * sp * cy),
        }
    }
}

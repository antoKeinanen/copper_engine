//! Camera is the main component responsible of rendering objects to the screen. Cameras are also the main audio listener of the scene.

use crate::math::{Matrix4x4, Vector3};

use super::scene::Scene;

/// #  Fields
/// - z_near: Distance of the near clipping plane on z-axis.
/// - z_far: Distance of the far clipping plane on z-axis.
/// - fov: Field of view of the camera in **radian**.
/// - position: Set of \[x, y, z] coordinates indicating the translation of the camera form the world origin (0, 0, 0).
/// - rotation: Set of \[x, y, z] f32. Indicates camera's rotation in **radian**.
/// - scale: Set of \[x, y, z] f32. Indicates the scale of the camera on each axis.
/// - up_vector: Vector that points up usually set to `[0, 1, 0]`.
/// - tick_update_func: `tick_update_func` is called every drawn frame. For more info about function call order refer to github wiki pages.
/// - on_awake: `on_awake` is called once after model loading is completed.
///
/// Usage of `::new()` is strongly recommended.

#[derive(Clone, Copy)]
pub struct Camera {
    pub z_near: f32,
    pub z_far: f32,
    pub fov: f32,
    pub position: Vector3,
    pub rotation: Vector3,
    pub up_vector: Vector3,
    pub right_vector: Vector3,
    pub tick_update_func: fn(&mut Scene),
    pub on_awake: fn(&mut Scene),
    pub window_size: [u32; 2],
    pub cam_dir: Vector3,
    pub local_up: Vector3,
    pub local_right: Vector3,
}

impl Camera {
    /// creates camera object
    ///
    /// # Examples
    /// ```
    /// use copper_engine::{blank_tick_update, blank_on_awake , structs::camera::Camera};
    ///
    /// let cam1 = Camera::new(
    ///     0.1,
    ///     1024.0,
    ///     PI / 3.0,
    ///     [0.0, 0.0, -10.0],
    ///     [0.0, 0.0, 1.0],
    ///     blank_tick_update,
    ///     blank_on_awake,
    /// );
    /// ```
    pub fn new(
        near: f32,
        far: f32,
        fov: f32,
        position: Vector3,
        rotation: Vector3,
        tick_update_func: fn(&mut Scene),
        on_awake: fn(&mut Scene),
    ) -> Self {
        Self {
            z_near: near,
            z_far: far,
            fov: fov,
            position: position,
            rotation: rotation,
            up_vector: Vector3::new(0.0, 1.0, 0.0),
            right_vector: Vector3::new(1.0, 0.0, 0.0),
            tick_update_func: tick_update_func,
            on_awake: on_awake,
            window_size: [0, 0],
            cam_dir: Vector3::new(0.0, 0.0, 0.0),
            local_up: Vector3::new(0.0, 0.0, 0.0),
            local_right: Vector3::new(0.0, 0.0, 0.0),
        }
    }

    pub fn look_at(&mut self) -> Matrix4x4 {
        let position: glm::Vector3<f32> = glm::Vector3 {
            x: self.position.x,
            y: self.position.y,
            z: self.position.z,
        };

        let target = glm::Vector3 {
            x: self.rotation.x,
            y: self.rotation.y,
            z: self.rotation.z,
        };
        let up = glm::Vector3 {
            x: self.up_vector.x,
            y: self.up_vector.y,
            z: self.up_vector.z,
        };

        // let cam_dir = (position - target).normalize();
        // let cam_right = cam_dir.cross_product(up).normalize();
        // let cam_up = cam_right.cross_product(cam_dir);

        // self.cam_dir = cam_dir;
        // self.local_right = cam_right;
        // self.local_up = cam_up;

        // let rotation_mat = Matrix4x4::new(
        //     [cam_right.x, cam_right.y, cam_right.z, 0.0],
        //     [cam_up.x, cam_up.y, cam_up.z, 0.0],
        //     [cam_dir.x, cam_dir.y, cam_dir.z, 0.0],
        //     [
        //         -cam_dir.dot(position),
        //         -cam_right.dot(position),
        //         cam_up.dot(position),
        //         1.0,
        //     ],
        // );

        let look_mat = *glm::ext::look_at(position, target, up).as_array();

        let mut view_matrix = Matrix4x4::empty();

        for i in 0..4 {
            for j in 0..4 {
                view_matrix.matrix[i][j] = look_mat[i][j];
            }
        }

        view_matrix
    }
}

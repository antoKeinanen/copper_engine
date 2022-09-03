//! Camera is the main component responsible of rendering objects to the screen.

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
    pub position: [f32; 3],
    pub rotation: [f32; 3],
    pub up_vector: [f32; 3],
    pub tick_update_func: fn(&mut Scene),
    pub on_awake: fn(&mut Scene),
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
        position: [f32; 3],
        rotation: [f32; 3],
        tick_update_func: fn(&mut Scene),
        on_awake: fn(&mut Scene),
    ) -> Self {
        Self {
            z_near: near,
            z_far: far,
            fov: fov,
            position: position,
            rotation: rotation,
            up_vector: [0.0, 1.0, 0.0],
            tick_update_func: tick_update_func,
            on_awake: on_awake,
        }
    }
}
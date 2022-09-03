//! Scene is collection of everything copper engine needs to function. 

use crate::AudioSource;

use super::{object::Object, input_manager::InputManager, camera::Camera};

/// # Fields
/// - game_objects: List of currently loaded game objects.
/// - audio_sources: List of all enabled audio sources.
/// - input_manager: Main input manager.
/// - main_camera: Main camera.
/// - delta_time: Time between last frame release and this instant in seconds.
/// - time_since_start: Time since the program was started in seconds. Updated at start of every loop cycle.
/// 
/// Usage of `::new()` is strongly recommended!
pub struct Scene {
    pub game_objects: Vec<Object>,
    pub audio_sources: Vec<AudioSource>,
    pub input_manager: InputManager,
    pub main_camera: Camera,
    pub delta_time: f32,
    pub time_since_start: f32,
}

impl Scene {
    /// creates Scene
    /// 
    /// # Examples
    /// 
    /// ```
    /// // Create objects, audio sources, input manager, and main camera first!
    /// let scene: Scene = Scene::new(vec![model], vec![audio_source], input_manager, main_camera);
    /// ```

    pub fn new(objects: Vec<Object>, audio_sources:Vec<AudioSource>, input_manager: InputManager, main_camera: Camera) -> Self {
        Self {
            game_objects: objects,
            audio_sources: audio_sources,
            input_manager: input_manager,
            main_camera: main_camera,
            delta_time: 0.0,
            time_since_start: 0.0,
        }
    }
}
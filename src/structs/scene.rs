//! Scene is collection of everything copper engine needs to function.

use crate::audio::AudioSource;
use crate::object::GameObject;

use super::{camera::Camera, input_manager::InputManager};

/// # Fields
/// - game_objects: List of currently loaded game objects.
/// - audio_sources: List of all audio sources in scene.

/// - input_manager: Main input manager.
/// - main_camera: Main camera.
/// - delta_time: Time between last frame release and this instant in seconds.
/// - time_since_start: Time since the program was started in seconds. Updated at start of every loop cycle.
///
/// Usage of `::new()` is strongly recommended!
pub struct Scene {
    pub game_objects: Vec<GameObject>,
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
    /// // Create objects, both local and global audio sources, input manager, and main camera first!
    /// let scene: Scene = Scene::new(vec![model], vec![local_audio_source], vec![local_audio_source], input_manager, main_camera);
    /// ```
    pub fn new(
        objects: Vec<GameObject>,
        audio_sources: Vec<AudioSource>,
        input_manager: InputManager,
        main_camera: Camera,
    ) -> Self {
        Self {
            game_objects: objects,
            audio_sources,
            input_manager,
            main_camera,
            delta_time: 0.0,
            time_since_start: 0.0,
        }
    }
}

//! Audio source can be used to play audio in scene relative to the camera.

use soloud::{audio, AudioExt, LoadExt, Wav};
use std::path::Path;

/// **Supported formats: wav, mp3, ogg, and flac.**
/// # Fields
/// - sound: Instance of `slound::audio::Wav`.
/// - volume: Volume of the sound where `1.0` is normal.
/// - triggered: Bool indicating if the audio will be plaid this cycle.
/// - position: Set of \[x, y, z] coordinates indicating the translation of the sound form the world origin (0, 0, 0).
/// - amplifier: Multiplies the distance to give clearer effects.
/// 
/// Usage of `::new()` is strongly recommended!
pub struct AudioSourceLocal {
    pub sound: Wav,
    pub volume: f32,
    pub triggered: bool,
    pub position: [f32; 3],
    pub amplifier: f32,
}

impl AudioSourceLocal {
    /// Supported formats: wav, mp3, ogg, and flac.
    /// # Errors
    /// - i/o error
    /// - Null value error in the read memory
    /// - Internal soloud error
    /// - Unknown error
    ///
    /// # Examples
    /// ```
    /// let audio_source = AudioSourceLocal::new("path/to/music.mp3", 1.0, true, [10.0, 0.0, 0.0], 50.0);
    /// ```
    pub fn new(
        path: &str,
        volume: f32,
        play_on_awake: bool,
        position: [f32; 3],
        amplifier: f32,
    ) -> Self {
        let mut sound = audio::Wav::default();
        sound.load(&Path::new(path)).expect("Failed to load audio!");

        Self {
            sound,
            volume,
            triggered: play_on_awake,
            position,
            amplifier,
        }
    }
}

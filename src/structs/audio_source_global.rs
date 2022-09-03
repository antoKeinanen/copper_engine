//! Audio source can be used to play audio in the scene.

use soloud::{audio, AudioExt, LoadExt, Wav};
use std::path::Path;

/// **Supported formats: wav, mp3, ogg, and flac.**
/// # Fields
/// - sound: Instance of `slound::audio::Wav`.
/// - volume: Volume of the sound where `1.0` is normal.
/// - triggered: Bool indicating if the audio will be plaid this cycle.
/// Usage of `::new()` is strongly recommended!
pub struct AudioSourceGlobal {
    pub sound: Wav,
    pub volume: f32,
    pub triggered: bool,
}

impl AudioSourceGlobal {
    /// Supported formats: wav, mp3, ogg, and flac.
    /// # Errors
    /// - i/o error
    /// - Null value error in the read memory
    /// - Internal soloud error
    /// - Unknown error
    ///
    /// # Examples
    /// ```
    /// let audio_source = AudioSourceGlobal::new("path/to/music.mp3", 1.0, true);
    /// ```
    pub fn new(path: &str, volume: f32, play_on_awake: bool) -> Self {
        let mut sound = audio::Wav::default();
        sound.load(&Path::new(path)).expect("Failed to load audio!");

        Self {
            sound,
            volume,
            triggered: play_on_awake,
        }
    }
}

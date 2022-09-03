use std::path::Path;

use soloud::{audio, AudioExt, LoadExt, Wav};


pub struct AudioSource {
    pub sound: Wav,
    pub volume: f32,
    pub triggered: bool,
}

impl AudioSource {
    pub fn new(path: &str, volume: f32, play_on_awake: bool) -> Self {
        let mut sound = audio::Wav::default();
        sound.load(&Path::new(path)).unwrap();

        Self {
            sound,
            volume,
            triggered: play_on_awake,
        }
    }
}

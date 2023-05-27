#![allow(unused)]

use padme_core::AudioSpeaker;

pub struct AudioPlayer;

impl AudioPlayer {
    pub fn new() -> Self {
        Self
    }

    pub fn pause(&mut self) {}

    pub fn play(&mut self) {}

    pub fn set_volume(&mut self, volume: f32) {}
}

impl AudioSpeaker for AudioPlayer {
    fn set_samples(&mut self, left: f32, right: f32) {}
}

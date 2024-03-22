use std::collections::HashMap;

use sfml::{
    audio::{Sound, SoundBuffer, SoundSource, SoundStatus},
    SfBox,
};

use crate::util::assets;

pub struct Sounds<'s> {
    playing: Vec<Sound<'s>>,
}

impl<'s> Sounds<'s> {
    pub fn new() -> Self {
        Self {
            playing: Vec::new(),
        }
    }

    pub fn play(&mut self, sound_buffer: &'s SfBox<SoundBuffer>) {
        let mut sound = Sound::with_buffer(sound_buffer);
        sound.set_volume(5.5);
        sound.play();
        self.playing.push(sound);
    }

    pub fn update(&mut self) {
        self.playing
            .retain(|sound| sound.status() == SoundStatus::PLAYING);
    }
}

#[derive(Hash, Eq, PartialEq, Copy, Clone)]
pub enum SoundType {
    Bounce,
}

#[derive(Default)]
pub struct SoundList<'s>(HashMap<SoundType, &'s SfBox<SoundBuffer>>);

impl<'s> SoundList<'s> {
    pub fn new() -> Self {
        Self(HashMap::new())
    }

    pub fn preload(&mut self) {
        self.0.insert(
            SoundType::Bounce,
            Box::leak(Box::new(
                SoundBuffer::from_file(assets!("bounce.wav")).unwrap(),
            )),
        );
    }

    #[allow(dead_code)]
    pub fn load(&mut self, soundtype: SoundType, filename: &str) {
        self.0.insert(
            soundtype,
            Box::leak(Box::new(SoundBuffer::from_file(filename).unwrap())),
        );
    }

    pub fn get(&self, soundtype: SoundType) -> &'s SfBox<SoundBuffer> {
        self.0.get(&soundtype).unwrap()
    }
}

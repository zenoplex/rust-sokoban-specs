use ggez::audio::SoundSource;
use ggez::{audio, Context};
use specs::{World, WorldExt};
use std::collections::HashMap;

#[derive(Default)]
pub struct AudioStore {
    pub sounds: HashMap<String, audio::Source>,
}

impl AudioStore {
    pub fn play(&mut self, sound: &str) {
        let _ = self
            .sounds
            .get_mut(sound)
            .expect("Sound error")
            .play_detached();
    }
}

pub fn initialize_sounds(world: &mut World, context: &mut Context) {
    let mut audio_store = world.write_resource::<AudioStore>();
    let sounds = ["correct", "incorrect", "wall"];

    for sound in sounds.iter() {
        let name = sound.to_string();
        let path = format!("/sounds/{}.wav", name);
        let source = audio::Source::new(context, path).expect("Sound load error");

        audio_store.sounds.insert(name, source);
    }
}

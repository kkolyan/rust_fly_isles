use macroquad::audio::{play_sound, play_sound_once, set_sound_volume, stop_sound};
use crate::common::resource::Resource;
use crate::{f32, PlaySoundParams};
use crate::common::unsorted::IndexRange;
use crate::model::def::GameSound;
use crate::model::state::{AudioManager, LoopedSoundState};

pub trait PlaySound: StopSound {
    fn play_once(&self, manager: &AudioManager);
    fn play_looped(&self, manager: &mut AudioManager);
    fn set_volume(&self, volume: f32, manager: &mut AudioManager);
}

pub trait StopSound {
    fn stop(&self, manager: &mut AudioManager);
}

pub trait SoundList {
    fn play_looped(&self, index: usize, manager: &mut AudioManager);
}

impl StopSound for Vec<Option<Resource<GameSound>>> {
    fn stop(&self, manager: &mut AudioManager) {
        for sound in self {
            sound.stop(manager);
        }
    }
}

impl StopSound for Option<Resource<GameSound>> {
    fn stop(&self, manager: &mut AudioManager) {
        if let Some(sound) = self {
            stop_sound(sound.sound);
            for i in manager.looped.indices().rev() {
                let state = manager.looped.get(i).unwrap();
                if state.def.sound == sound.sound {
                    manager.looped.remove(i);
                }
            }
        }
    }
}

impl SoundList for Vec<Option<Resource<GameSound>>> {
    fn play_looped(&self, index: usize, manager: &mut AudioManager) {
        for (i, sound) in self.iter().enumerate() {
            if i != index {
                sound.stop(manager);
            }
        }
        for (i, sound) in self.iter().enumerate() {
            if i == index {
                sound.play_looped(manager);
            }
        }
    }
}

impl PlaySound for Option<Resource<GameSound>> {
    fn play_once(&self, manager: &AudioManager) {
        if let Some(sound) = self {
            if let Some(throttling_sec) = sound.throttling_sec {
                let mut cool_down_sec = manager.cool_down_sec.borrow_mut();
                let remaining = cool_down_sec.get(sound).copied().unwrap_or(0.0);
                if remaining > 0.0 {
                    return;
                }
                cool_down_sec.insert(sound.clone(), throttling_sec);
            }
            play_sound(sound.sound, PlaySoundParams { looped: false, volume: sound.base_volume * manager.volume.get() });
        }
    }

    fn play_looped(&self, manager: &mut AudioManager) {
        if let Some(sound) = self {
            play_sound(sound.sound, PlaySoundParams { looped: true, volume: sound.base_volume * manager.volume.get() });
            manager.looped.push(LoopedSoundState {
                def: sound.clone(),
                instance_volume: 1.0,
            });
        }
    }

    fn set_volume(&self, volume: f32, manager: &mut AudioManager) {
        if let Some(sound) = self {
            for state in &mut manager.looped {
                if state.def.sound == sound.sound {
                    state.instance_volume = volume;
                }
            }

            set_sound_volume(sound.sound, volume * sound.base_volume * manager.volume.get());
        }
    }
}

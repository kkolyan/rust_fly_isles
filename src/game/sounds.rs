use macroquad::audio::set_sound_volume;
use crate::common::contract::Get;
use crate::common::sound::{PlaySound, SoundList, StopSound};
use crate::GameState;

pub fn on_start(state: &mut GameState) {
    if let Some(plane) = state.player.plane {
        if let Some(plane) = state.planes.get(&plane) {
            // plane.def.engine_sound.play_looped(plane.gear, &mut state.subsystems.audio);
        }
    }
}

pub fn on_pause(state: &mut GameState, pause: bool) {
    if let Some(plane) = state.player.plane {
        if let Some(plane) = state.planes.get(&plane) {
            if pause {
                plane.def.engine_sound.stop(&mut state.subsystems.audio);
            } else {
                plane.def.engine_sound.play_looped(plane.gear, &mut state.subsystems.audio);
            }
        }
    }
}

pub fn on_volume_change(state: &mut GameState) {
    for sound in &state.subsystems.audio.looped {
        set_sound_volume(sound.def.sound, sound.def.base_volume * sound.instance_volume * state.subsystems.audio.volume.get());
    }
}
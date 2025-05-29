use std::collections::HashMap;
use std::ops::{Mul, Not, Sub};
use macroquad::input::{is_key_down, is_key_released, is_mouse_button_down, is_mouse_button_pressed, mouse_position, mouse_wheel};
use std::process::exit;
use chrono::Utc;
use macroquad::audio::set_sound_volume;
use macroquad::prelude::{is_key_pressed, is_mouse_button_released, screen_height, screen_width};
use macroquad::prelude::Vec2;
use macroquad::math::clamp;
use macroquad::texture::get_screen_data;
use macroquad::time::get_frame_time;
use crate::{AppState, GameState, KeyCode, MenuState, MouseButton, start};
use crate::common::angle::AsRadians;
use crate::common::camera::ViewPort;
use crate::common::contract::{Get, GetMut};
use crate::common::sound::{PlaySound, SoundList};
use crate::common::unsorted::{IndexRange, ToAngle};
use crate::game::{control_guard, sounds};
use crate::game_viewport::create_viewport;
use crate::KeyCode::{A, D, M, S, W};
use crate::model::state::Durable::Destroyed;
use crate::model::state::{Ammo, CannonState, DeviceSpec, DeviceState, EquipmentBinding, EquipmentWindow, JournalWindow, ShopWindow, UiWindow, WeaponOwner};
use crate::resources::constants::{DEV, SPECTATOR_SPEED};

static NUMBER_KEYS: &[KeyCode] = &[
    KeyCode::Key0,
    KeyCode::Key1,
    KeyCode::Key2,
    KeyCode::Key3,
    KeyCode::Key4,
    KeyCode::Key5,
    KeyCode::Key6,
    KeyCode::Key7,
    KeyCode::Key8,
    KeyCode::Key9,
];

pub fn process_input(mut app: AppState) -> AppState {
    match app {
        AppState::Title { menu } => {
            AppState::Title { menu }
        }
        AppState::Game { mut game } => {
            process_game_input(&mut game);
            if is_key_pressed(KeyCode::Escape) && game.player.windows.pop_back().is_none() {
                AppState::GameMenu { game, menu: MenuState {} }
            } else {
                AppState::Game { game }
            }
        }
        AppState::GameMenu { game, menu, } => {
            if is_key_pressed(KeyCode::Escape) {
                AppState::Game { game }
            } else {
                AppState::GameMenu { game, menu }
            }
        }
        AppState::Intro => { AppState::Intro }
    }
}

fn process_game_input(state: &mut GameState) {
    if is_key_pressed(KeyCode::Enter) {
        state.unpause_one_frame = true;
    }
    if DEV && is_key_pressed(KeyCode::G) {
        state.player.god = !state.player.god;
    }
    if is_key_pressed(KeyCode::M) {
        let audio = &mut state.subsystems.audio;
        if audio.volume.get() > 0.0 {
            audio.volume.set(0.0);
        } else {
            audio.volume.set(1.0);
        }
        sounds::on_volume_change(state);
    }
    if is_key_pressed(KeyCode::Space) {
        if state.player.plane.and_then(|it| state.planes.get(&it)).is_none() {
            state.player.death_count += 1;
            // state.player.resources.clear();
            for (_, device) in state.player.equipment.iter_mut() {
                match &mut device.spec {
                    DeviceSpec::Weapon(w) => { w.trigger = false; }
                    DeviceSpec::Booster(w) => { w.trigger = false; }
                }
            }
            start::spawn_player_plane(state);
        } else if DEV {
            state.paused = !state.paused;
        }
    }
    if is_key_pressed(KeyCode::F11) {
        let image = get_screen_data();
        let fname = Utc::now().format("%Y%m%d_%H%M%S%f.png").to_string();
        image.export_png(fname.as_str());
    }

    let view_port = &create_viewport(state);
    control_guard::process_input(state, view_port);
    if let Some(plane_id) = state.player.plane {
        if let Some(plane) = state.planes.get_mut(&plane_id) {
            let (_, wheel_y) = mouse_wheel();
            let mut gear = plane.gear as i32;
            if wheel_y > 0.001 { gear += 1; }
            if wheel_y < -0.001 { gear -= 1; }
            let available_gears = plane.def.gears.iter().filter(|it| it.tech_level <= state.player.thrust_tech_level).count();
            plane.gear = clamp(gear, 0, available_gears as i32) as usize;

            if is_key_pressed(KeyCode::K) {
                plane.durable = Destroyed(WeaponOwner::Plane(plane_id));
            }

            for (_, device) in state.player.equipment.iter_mut() {
                let trigger = match &mut device.spec {
                    DeviceSpec::Weapon(v) => &mut v.trigger,
                    DeviceSpec::Booster(v) => &mut v.trigger,
                };
                if !state.player.windows.is_empty() {
                    *trigger = false;
                } else {
                    trait BindingButton {
                        fn is_pressed(&self) -> bool;
                        fn is_released(&self) -> bool;
                    }

                    impl BindingButton for MouseButton {
                        fn is_pressed(&self) -> bool { is_mouse_button_pressed(*self) }
                        fn is_released(&self) -> bool { is_mouse_button_released(*self) }
                    }

                    impl BindingButton for KeyCode {
                        fn is_pressed(&self) -> bool { is_key_pressed(*self) }
                        fn is_released(&self) -> bool { is_key_released(*self) }
                    }

                    if let Some(binding) = device.binding {
                        let h: &dyn BindingButton = match binding {
                            EquipmentBinding::Primary => &MouseButton::Left,
                            EquipmentBinding::Secondary => &MouseButton::Right,
                            EquipmentBinding::_1 => &KeyCode::Key1,
                            EquipmentBinding::_2 => &KeyCode::Key2,
                            EquipmentBinding::_3 => &KeyCode::Key3,
                            EquipmentBinding::_4 => &KeyCode::Key4,
                            EquipmentBinding::_5 => &KeyCode::Key5,
                        };
                        if h.is_pressed() {
                            *trigger = true;
                        }
                        if h.is_released() {
                            *trigger = false;
                        }
                    }
                }
            }

            let screen_size = Vec2::new(
                screen_width(),
                screen_height(),
            );
            if DEV && state.player.god {
                let mut movement = Vec2::ZERO;
                if is_key_down(W) { movement.y -= 1.0; }
                if is_key_down(S) { movement.y += 1.0; }
                if is_key_down(A) { movement.x -= 1.0; }
                if is_key_down(D) { movement.x += 1.0; }
                if let Some(movement) = movement.try_normalize() {
                    let velocity = movement * SPECTATOR_SPEED;
                    plane.trans.velocity = velocity;
                    plane.trans.pos += velocity * get_frame_time();
                } else {
                    plane.trans.velocity = Vec2::ZERO;
                }
                if state.player.steering {
                    plane.rot.angle = Vec2::from(mouse_position()).sub(screen_size.mul(0.5)).to_angle();
                }
                plane.rot.ang_velocity_rad = 0.0;
            } else {
                if state.player.steering {
                    let pointer = Vec2::from(mouse_position());
                    let dir = (pointer - screen_size * 0.5);
                    plane.desired_rot = dir.y.atan2(dir.x).as_radians();
                } else {
                    plane.desired_rot = plane.rot.angle;
                }
            }

            if DEV && is_key_pressed(KeyCode::H) {
                let closest_mob = state.mobs.iter_mut()
                    .map(|(_, mob)| (mob.anchor.get_pos_rel().get_abs(&state.isles) - plane.trans.pos, mob))
                    .min_by_key(|(dir, mob)| dir.length() as i32);
                if let Some((dir, mob)) = closest_mob {
                    mob.base.debug = !mob.base.debug;
                }
            }

            if DEV && is_key_pressed(KeyCode::J) {
                for (_, mob) in state.mobs.iter_mut() {
                    mob.base.debug = false;
                    mob.base.debug_once = false;
                }
                let closest_mob = state.mobs.iter_mut()
                    .map(|(_, mob)| (mob.anchor.get_pos_rel().get_abs(&state.isles) - plane.trans.pos, mob))
                    .min_by_key(|(dir, mob)| dir.length() as i32);
                if let Some((dir, mob)) = closest_mob {
                    mob.base.debug_once = true;
                }
            }

            if is_key_pressed(KeyCode::F1) {
                state.player.windows.clear();
                state.player.windows.push_back(UiWindow::Help);
                state.player.steering = false;
            }
            if is_key_pressed(KeyCode::F2) {
                state.player.windows.clear();
                state.player.windows.push_back(UiWindow::Improvements);
                state.player.steering = false;
            }
            if is_key_pressed(KeyCode::F3) {
                state.player.windows.clear();
                state.player.windows.push_back(UiWindow::Equipment(EquipmentWindow { selected_item: None }));
                state.player.steering = false;
            }
            if is_key_pressed(KeyCode::F4) {
                state.player.windows.clear();
                state.player.windows.push_back(UiWindow::Journal(JournalWindow { page: state.journal.len() - 1 }));
                state.player.steering = false;
            }
            if DEV && is_key_pressed(KeyCode::C) {
                state.show_colliders = !state.show_colliders;
            }
        }
    }
}

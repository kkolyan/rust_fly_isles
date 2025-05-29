use std::cell::Cell;
use std::f32::consts::PI;
use macroquad::math::{clamp, Rect};
use crate::{GameState, PlaneState, Vec2};
use crate::common::camera::ViewPort;
use crate::common::contract::{Get, GetMut, InsertSimple};
use crate::common::frame::FrameCtx;
use crate::common::sprite_clip;
use crate::common::unsorted::gen_range;
use crate::game::{bots, cannon, plane, particles, mobs, durable, control_guard, loot, isles, rpg, progression};
use crate::model::def::{Obtainable, DeviceSlot, Collider, CollisionCircle};
use crate::model::state::{ManualBuffState, CannonState, DamageTarget, DeviceState, Durable, GameCommand, WindowsAction, MobState, ManualBuffAmmo, LootState};

pub fn update_game_state(state: &mut GameState, dt: &FrameCtx, vp: &ViewPort) {
    state.metrics.clear();

    progression::update(state);

    for (sound, rem_sec) in state.subsystems.audio.cool_down_sec.get_mut() {
        if *rem_sec > 0.0 {
            *rem_sec -= dt.dt;
        }
    }

    fn delete_at_end<T>(source: &mut Vec<T>, i: usize) { source.remove(i); }

    // smoke processed before planes (spawning new smoke instances) to avoid first frame skip
    sprite_clip::update(dt.dt, delete_at_end, &mut state.particles.fixed, |it| Some(&mut it.clip));
    sprite_clip::update(dt.dt, delete_at_end, &mut state.particles.moving, |it| Some(&mut it.clip));
    sprite_clip::update(dt.dt, sprite_clip::do_nothing, &mut state.projectiles, |it| it.exhaust_clip.as_mut());

    for state in &mut state.particles.moving {
        state.pos += state.velocity * dt.dt;
    }

    isles::update(state, dt);

    mobs::update(state, dt);

    loot::update(state, dt);

    bots::update_bots(state);

    plane::update_planes(state, dt);

    rpg::update(state);

    cannon::update_projectiles(state, dt, vp);

    while let Some(event) = state.commands.pop_front() {
        match event {
            GameCommand::Damage { target, amount, source } => {
                match target {
                    DamageTarget::Mob(target) => {
                        if let Some(target) = state.mobs.get_mut(&target) {
                            target.base.durable.accept_damage(amount, source);
                        }
                    }
                    DamageTarget::Plane(target) => {
                        if let Some(target) = state.planes.get_mut(&target) {
                            target.durable.accept_damage(amount, source);
                        }
                    }
                }
            }
            GameCommand::FireCannon { bal, rot, owner, cannon, initial_angle } => {
                cannon::fire(
                    &state.player,
                    &bal,
                    &rot,
                    owner,
                    &cannon.barrel,
                    initial_angle,
                    &mut state.projectiles,
                    &cannon,
                    &state.subsystems,
                    &state.planes,
                    &state.isles,
                    &state.mobs,
                    &state.reachable_mobs,
                    &mut state.commands,
                );
            }
            GameCommand::NewRay(ray) => state.rays.push(ray),
            GameCommand::Drop(loot, pos) => {
                state.loot.insert_simple(LootState {
                    def: loot.clone(),
                    pos,
                })
            }
        }
    }

    durable::update(state, dt);

    control_guard::update(state, dt);

    if let Some(plane_id) = state.player.plane {
        if let Some(plane) = &mut state.planes.get_mut(&plane_id) {
            state.player.camera_pos = plane.trans.pos;
        }
    }
}

pub fn update_command_queue(state: &mut GameState) {
    while let Some(action) = state.ui_commands.pop_front() {
        match action {
            WindowsAction::Buy(item) => {
                state.player.windows.pop_back();
                if let Some(player) = state.player.plane.and_then(|it| state.planes.get_mut(&it)) {
                    match &item.item {
                        Obtainable::Weapon { weapon, ammo, slot } => {
                            match slot {
                                DeviceSlot::Primary => {
                                    player.primary = DeviceState::weapon(CannonState::new(&player.def.arms.primary, &weapon, *ammo))
                                }
                                DeviceSlot::Secondary => {
                                    player.secondary = Some(DeviceState::weapon(CannonState::new(&player.def.arms.secondary, &weapon, *ammo)))
                                }
                            }
                        }
                        Obtainable::HP { amount, .. } => {
                            match &mut player.durable {
                                Durable::Good { hp, .. } => {
                                    *hp = state.player.hp_max.min(*hp + amount);
                                }
                                Durable::Destroyed(_) => {}
                            };
                        }
                        Obtainable::Consumable { def, reserve_sec, slot } => {
                            match slot {
                                DeviceSlot::Primary => {
                                    player.primary = DeviceState::booster(ManualBuffState::new(def.clone(), ManualBuffAmmo::Hard { reserve_sec: *reserve_sec }))
                                }
                                DeviceSlot::Secondary => {
                                    player.secondary = Some(DeviceState::booster(ManualBuffState::new(def.clone(), ManualBuffAmmo::Hard { reserve_sec: *reserve_sec })))
                                }
                            }
                        }
                        Obtainable::Passive { def } => {
                            player.passive_buff = Some(def.clone());
                        }
                        Obtainable::PassiveReset { .. } => {
                            player.passive_buff = None;
                        }
                    }
                    for (res, price) in &item.price {
                        if let Some(value) = state.player.resources.get_mut(res) {
                            *value = value.saturating_sub(*price);
                        }
                    }
                }
            }
        }
    }
}
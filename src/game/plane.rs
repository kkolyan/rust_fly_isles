use std::borrow::Borrow;
use std::collections::HashMap;
use std::f32::consts::PI;
use std::ops::{Deref, Div, Mul};
use std::rc::Rc;

use macroquad::prelude::{Mat2, Vec2};
use macroquad::math::{clamp, Rect};
use macroquad::prelude::WHITE;

use crate::{Game, GameState, PlaneId, PlaneState, PlayerState};
use crate::common::{camera, physics, sprite};
use crate::common::angle::{Angle, AsRadians};
use crate::common::resource::Resource;
use crate::common::camera::ViewPort;
use crate::common::contract::{Get, Insert, InsertSimple};
use crate::common::curve::Lerp;
use crate::common::frame::FrameCtx;
use crate::common::metrics::Metrics;
use crate::common::pool::Pool;
use crate::common::sound::{PlaySound, SoundList, StopSound};
use crate::common::sprite::draw_sprite;
use crate::common::unsorted::WithMut;
use crate::game::{cannon, durable, particles, rpg};
use crate::model::def::{Buff, BuffSpec, Explosion, Location, Plane, Sprite, Stabilization, TrailSource, TransientBallisticBody};
use crate::model::state::{Ammo, CannonState, DeviceSpec, DeviceState, Durable, FixedSpriteClipState, GameCommand, ManualBuffAmmo, ManualBuffState, ParticleEmitterState, ParticlesState, RotState, RpgState, TransState, WeaponOwner};
use crate::model::state::Durable::{Destroyed, Good};
use crate::resources::constants::{FULL_THROTTLE_ENERGY_PER_SECOND, INITIAL_ENERGY, NOMINAL_SPEED, PLANE_THRUST_NOMINAL, SECONDS_TO_RESTORE_FULL_ENERGY};
use crate::resources::objects::objects;

pub fn allocate_plane<I: Insert<PlaneId, PlaneState>>(pool: &mut I, plane: Resource<Plane>, pos: Vec2, rot: Angle, game: &Resource<Game>, player: &PlayerState) -> PlaneId {
    let mut state = PlaneState {
        trans: TransState {
            pos,
            velocity: rot.to_vec2_norm() * NOMINAL_SPEED,
        },
        durable: Durable::new(player.hp_max),
        rot: RotState {
            angle: rot,
            ang_velocity_rad: 0.0,
        },
        desired_rot: rot,
        gear: plane.default_gear,
        def: plane.clone(),
        trail: plane.trail.as_ref().map(|trail_pod| ParticleEmitterState {
            def: trail_pod.clone(),
            emission_queue: 0.0,
        }),
        primary: DeviceState::weapon(CannonState::new(&plane.arms.primary, &plane.arms.primary_default, Ammo::Infinite)),
        secondary: None,
        active_buffs: vec![],
        passive_buff: None,
        energy: player.energy_max,
        effective_gear: 0,
        passive_buffs: vec![],
        effective_gear_prev: None,
    };
    pool.insert(state)
}

impl PlaneState {
    fn is_plane_flip_y(angle: Angle) -> bool {
        angle.to_rad() > PI * 0.5 || angle.to_rad() < -PI * 0.5
    }
}

pub fn update_planes(state: &mut GameState, dt: &FrameCtx) {
    let mut disposal_queue = Vec::new();
    for (id, plane) in &mut state.planes.iter_mut() {
        if let Some(lava_damage) = &state.location.lava_damage_by_height_per_sec_norm {
            let damage_per_sec = lava_damage.lerp(plane.trans.pos.y / state.location.size.y);
            if damage_per_sec > 0.0 {
                plane.durable.accept_damage(damage_per_sec * dt.dt, WeaponOwner::Environment);
            }
        }
        if plane.energy < state.player.energy_max {
            let regen_rate = state.player.energy_max / SECONDS_TO_RESTORE_FULL_ENERGY;
            plane.energy = f32::min(plane.energy + regen_rate * dt.dt, state.player.energy_max);
        }
        match plane.durable {
            Durable::Good { .. } => {}
            Durable::Destroyed(_) => {
                let is_player = state.player.plane.map(|it| *id == it).unwrap_or(false);
                if is_player && state.player.god {
                    plane.durable = Durable::new(state.player.hp_max);
                } else {
                    disposal_queue.push(*id);
                }
                if is_player {
                    plane.def.death_sound.play_once(&state.subsystems.audio);
                    plane.def.engine_sound.stop(&mut state.subsystems.audio);
                }
                continue;
            }
        }

        plane.active_buffs.clear();
        if let Some(buff) = &plane.passive_buff {
            plane.active_buffs.push(buff.clone());
        }

        for buff in plane.active_buffs.iter()
            .map(|it| &it.spec)
            .chain(plane.passive_buffs.iter().map(|it| it.deref()))
        {
            match buff {
                BuffSpec::ThrustAddendum { .. } => {}
                BuffSpec::ThrustMultiplier { .. } => {}
                BuffSpec::SteerBooster { .. } => {}
                BuffSpec::Repair { hp_per_sec } => {}
                BuffSpec::Nitro { .. } => {}
            }
        }
        let is_player = state.player.plane.map(|it| it == *id).unwrap_or(false);
        let metrics = state.metrics.clone();
        metrics.set_enabled(is_player);

        if is_player && plane.effective_gear_prev != Some(plane.effective_gear) {
            plane.def.engine_sound.play_looped(plane.effective_gear, &mut state.subsystems.audio);
            plane.effective_gear_prev = Some(plane.effective_gear);
        }

        for (device_id, device) in state.player.equipment.iter_mut() {
            match &mut device.spec {
                DeviceSpec::Weapon(weapon) => {
                    update_plane_cannon(dt, id, weapon, &mut state.commands, &plane.trans, &plane.rot, &mut plane.energy);
                    if let Ammo::Finite(0) = weapon.ammo {
                        plane.primary = DeviceState::weapon(CannonState::new(&plane.def.arms.primary, &plane.def.arms.primary_default, Ammo::Infinite));
                    }
                }
                DeviceSpec::Booster(booster) => {
                    update_booster(booster, dt, &mut plane.active_buffs, &mut plane.energy);

                    match booster.reserve {
                        ManualBuffAmmo::Hard { reserve_sec } => {
                            if reserve_sec <= 0.0 {
                                plane.primary = DeviceState::weapon(CannonState::new(&plane.def.arms.primary, &plane.def.arms.primary_default, Ammo::Infinite));
                            }
                        }
                        ManualBuffAmmo::Energy { .. } => {}
                    }
                }
            }
        }

        let mut thrust = 0.0;
        plane.effective_gear = 0;
        for i in (0..(plane.gear + 1)).rev() {
            if let Some(gear) = plane.def.gears.get(i - 1) {
                if gear.tech_level > state.player.thrust_tech_level {
                    continue;
                }
                let energy_drain = gear.energy_per_sec * dt.dt;
                if plane.energy >= energy_drain {
                    thrust = gear.thrust;
                    plane.energy -= energy_drain;
                    plane.effective_gear = i;
                    break;
                }
            }
        }

        if is_player && state.player.god {
            continue;
        }

        for buff in plane.active_buffs.iter()
            .map(|it| &it.spec)
            .chain(plane.passive_buffs.iter().map(|it| it.deref()))
        {
            match buff {
                BuffSpec::ThrustAddendum { .. } => {}
                BuffSpec::Nitro { .. } => {}
                BuffSpec::ThrustMultiplier { .. } => {}
                BuffSpec::SteerBooster { .. } => {}
                BuffSpec::Repair { hp_per_sec } => {
                    match &mut plane.durable {
                        Durable::Good { hp, .. } => {
                            *hp = state.player.hp_max.min(*hp + hp_per_sec * dt.dt);
                        }
                        Destroyed(_) => {}
                    }
                }
            }
        }

        let iterations = 1;
        for _ in 0..iterations {
            let step_dt = dt.dt / iterations as f32;


            let metrics_argument = &metrics;
            let dt = step_dt;
            let mut acceleration = thrust;
            let mut steer_stabilization = plane.def.stabilization.steer.clone();
            for buff in plane.active_buffs.iter()
                .map(|it| &it.spec)
                .chain(plane.passive_buffs.iter().map(|it| it.deref()))
            {
                match buff {
                    BuffSpec::ThrustAddendum { extra_acceleration } => {
                        acceleration += extra_acceleration;
                    }
                    BuffSpec::SteerBooster { stabilization, .. } => {
                        steer_stabilization = stabilization.clone();
                    }
                    BuffSpec::ThrustMultiplier { acceleration_multiplier, .. } => {
                        acceleration *= acceleration_multiplier;
                    }
                    BuffSpec::Repair { .. } => {}
                    BuffSpec::Nitro { acceleration_by_speed, top_speed, .. } => {
                        acceleration = acceleration_by_speed.lerp(
                            plane.trans.velocity.length().div(top_speed)
                        )
                    }
                }
            }
            physics::apply_steering(&mut plane.trans, &mut plane.rot, &steer_stabilization, plane.desired_rot, &metrics, step_dt);
            physics::apply_thrust(metrics_argument, dt, acceleration, &plane.rot, &mut plane.trans);
            physics::apply_gravity(&mut plane.trans, &metrics, step_dt);
            physics::apply_drag(&mut plane.trans, &metrics, step_dt, dt);
            physics::apply_slide(&metrics, step_dt, dt, &mut plane.trans, &plane.rot, &plane.def.stabilization.clone().slide);
            physics::apply_velocity(&mut plane.trans, &state.location, step_dt);
            physics::apply_rotation(&mut plane.rot, step_dt);

            if let Some(trail) = &mut plane.trail {
                let mut smoke_factor = thrust / PLANE_THRUST_NOMINAL;
                for buff in plane.active_buffs.iter()
                    .map(|it| &it.spec)
                    .chain(plane.passive_buffs.iter().map(|it| it.deref()))
                {
                    match &buff {
                        BuffSpec::ThrustAddendum { extra_acceleration } => smoke_factor += extra_acceleration / PLANE_THRUST_NOMINAL,
                        BuffSpec::SteerBooster { smoke_factor_abs, .. } => smoke_factor += *smoke_factor_abs,
                        BuffSpec::ThrustMultiplier { acceleration_multiplier, smoke_factor_rel } => smoke_factor += smoke_factor * smoke_factor_rel,
                        BuffSpec::Repair { .. } => {}
                        BuffSpec::Nitro { smoke_factor: sf, .. } => { smoke_factor += *sf }
                    }
                }
                particles::update_particles(
                    trail,
                    &plane.trans,
                    &plane.rot,
                    step_dt,
                    &mut state.particles,
                    smoke_factor,
                );
            }
        }
        metrics.set_enabled(false);
    }
    for id in disposal_queue {
        if let Some(plane) = state.planes.remove(id) {
            if let Some(explosion) = &plane.def.explosion {
                particles::emit_explosion(explosion, &plane.trans, &mut state.particles, &state.subsystems);
            }
        }
    }
}

fn update_booster(booster: &mut ManualBuffState, dt: &FrameCtx, active_booster: &mut Vec<Resource<Buff>>, energy: &mut f32) {
    if booster.trigger {
        match &mut booster.reserve {
            ManualBuffAmmo::Hard { reserve_sec } => {
                *reserve_sec -= dt.dt;
                active_booster.push(booster.def.clone());
            }
            ManualBuffAmmo::Energy { energy_per_second } => {
                let required = *energy_per_second * dt.dt;
                if *energy >= required {
                    *energy -= required;
                    active_booster.push(booster.def.clone());
                }
            }
        }
    }
}

fn update_plane_cannon(dt: &FrameCtx, id: &PlaneId, cannon: &mut CannonState, commands: &mut impl InsertSimple<GameCommand>, trans: &TransState, state: &RotState, energy: &mut f32) {
    let mut pod = trans.clone();
    let offset = pod_offset(cannon, state);
    pod.pos += offset;
    cannon::update_cannon(
        cannon,
        &pod,
        &state,
        WeaponOwner::Plane(*id),
        dt,
        commands,
        energy,
    );
}

pub fn pod_offset(cannon: &CannonState, rot: &RotState) -> Vec2 {
    let pod_offset = cannon.pod.offset
        * Vec2::new(1.0, if PlaneState::is_plane_flip_y(rot.angle) { -1.0 } else { 1.0 });
    let offset = Mat2::from_angle(rot.angle.to_rad()) * pod_offset;
    offset
}


#[test]
fn test1() {
    for degrees_10 in (-1800..1801).step_by(225) {
        let degrees = degrees_10 as f32 / 10.0;
        let radians = degrees / 180.0 * PI;
        println!("{:.0}: {:.03}", degrees, radians.cos() * radians.sin());
    }
}

#[test]
fn test2() {
    println!("{}", Vec2::new(0.0, 1.0).perp());
}

#[test]
fn test3() {
    let angle = PI / 4.0;
    let dir = angle.as_radians().to_vec2_norm();
    assert_eq!(angle, Vec2::X.angle_between(dir));
}

pub fn draw_planes(state: &GameState, view_port: &ViewPort) {
    for (_, plane) in state.planes.iter() {
        view_port.port(plane.trans.pos, 1.0, |ported| {
            draw_sprite(&plane.def.sprite, ported.screen_pos, |it| {
                it.screen_scale = ported.screen_scale;
                it.angle = plane.rot.angle;
                it.flip_y = PlaneState::is_plane_flip_y(plane.rot.angle);
                it.material = durable::pain_option(state, &plane.durable);
            });
        });
    }
}

pub fn find_near_planes(state: &GameState) -> Vec<(Vec2, f32)> {
    let mut distances: Vec<(Vec2, f32)> = state.bots.iter()
        .filter_map(|it| state.planes.get(&it.plane))
        .map(|it| {
            let dir = it.trans.pos - state.player.camera_pos;
            (dir, dir.length())
        })
        .collect();
    distances.sort_by_key(|it| it.1 as i32);
    distances
}

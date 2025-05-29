use std::f32::consts::PI;
use std::rc::Rc;

use macroquad::prelude::{Mat2, Vec2};
use macroquad::math::Rect;
use macroquad::prelude::WHITE;
use macroquad::rand::ChooseRandom;

use crate::common::angle::{Angle, AsRadians};
use crate::common::sound::PlaySound;
use crate::common::sprite;
use crate::common::unsorted::gen_range;
use crate::GameState;
use crate::lifecycle::draw::Stats;
use crate::model::def::{Explosion, ParticleEmission, SpriteClip, TrailSource};
use crate::model::state::{AudioManager, FixedSpriteClipState, MovingSpriteClipState, ParticleEmitterState, ParticlesState, RotState, SpriteClipState, SubSystems, TransState};

pub fn emit_particles(
    emission: &ParticleEmission,
    pos: Vec2,
    velocity: Option<Vec2>,
    particles: &mut ParticlesState,
) {
    let spread_dir = Angle::random().to_vec2_norm();
    let spread_distance = f32::powi(gen_range(0.0..1.0), 1) * emission.spread_distance;
    let clip = match emission.clip_variants.choose() {
        None => return,
        Some(clip) => clip,
    };
    let sprite = SpriteClipState {
        clip: clip.clone(),
        frame: 0.0,
        rate: emission.rate.random(),
        delay: emission.delay.random(),
    };
    if let Some(velocity) = velocity {
        particles.moving.push(MovingSpriteClipState {
            clip: sprite,
            pos: pos + spread_dir * spread_distance,
            scale: emission.scale.random(),
            velocity,
            rot: gen_range(0.0..PI * 2.0).as_radians(),
        });
    } else {
        particles.fixed.push(FixedSpriteClipState {
            clip: sprite,
            pos: pos + spread_dir * spread_distance,
            scale: emission.scale.random(),
            rot: gen_range(0.0..PI * 2.0).as_radians(),
        });
    }
}

pub fn update_particles(state: &mut ParticleEmitterState, bal: &TransState, rot: &RotState, step_dt: f32, particles: &mut ParticlesState, rate_mod: f32) {
    let emitter = &state.def.emitter;
    state.emission_queue += step_dt * emitter.spawn_rate * rate_mod;

    while state.emission_queue > 0.0 {
        state.emission_queue -= 1.0;

        emit_particles(&emitter.emission, bal.pos + Mat2::from_angle(rot.angle.to_rad()) * state.def.offset, None, particles);
    }
}

pub fn emit_explosion(explosion: &Explosion, source: &TransState, particles: &mut ParticlesState, settings: &SubSystems) {
    explosion.sound.play_once(&settings.audio);
    for emission in &explosion.particles {
        for _ in 0..emission.count.random() {
            let velocity = source.velocity + Angle::random().to_vec2_norm() * emission.off_center_speed.random();
            emit_particles(&emission.emission, source.pos,
                           Some(velocity * emission.speed_factor), particles)
        }
    }
}

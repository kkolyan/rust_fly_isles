use std::f32::consts::PI;
use std::ops::{Add, Mul};
use std::rc::Rc;

use crate::{GameState, info, Vec2};
use crate::common::angle::Angle;
use crate::common::camera::ViewPort;
use crate::common::contract::{GetMut, Insert, InsertSimple};
use crate::common::frame::FrameCtx;
use crate::common::physics;
use crate::common::sprite::draw_sprite;
use crate::common::unsorted::gen_range;
use crate::game::{generator_001, generator_002, mobs};
use crate::model::def::{LocationContent, Mob, MobAnimation, MobKind, MobSpawn};
use crate::model::state::{FixedSpriteClipState, IsleState, MobMission, MobPhase, RelativePos, TransState};

pub fn init_isles(state: &mut GameState) {
    match &state.location.clone().content {
        LocationContent::Generator001(generator) => generator_001::generate(state, generator),
        LocationContent::Generator002(generator) => generator_002::generate(state, generator),
    }
    info!("mobs: {}, isles: {}", state.mobs.len(), state.isles.len())
}

pub fn draw(state: &GameState, vp: &ViewPort) {
    for (_, isle) in state.isles.iter() {
        draw_isle(&isle, vp);
    }
}

pub fn draw_isle(isle: &IsleState, vp: &ViewPort) {
    vp.port(isle.trans.pos, isle.def.scale, |ported| {
        draw_sprite(&isle.def.sprite, ported.screen_pos, |it| it.screen_scale = ported.screen_scale);
    });
}

pub fn update(state: &mut GameState, dt: &FrameCtx) {
    for (_, isle) in state.isles.iter_mut() {
        isle.course_seconds_remaining -= dt.dt;
        if isle.course_seconds_remaining < 0.0 {
            isle.course_change_interval_last = isle.def.course_change_interval_seconds.random();
            isle.course_seconds_remaining = isle.course_change_interval_last;
            isle.course = Angle::random().to_vec2_norm();
        }
        let phase = isle.course_seconds_remaining / isle.course_change_interval_last;
        isle.trans.velocity = isle.course * isle.def.drift_speed * (phase.clamp(0.0, 1.0) * 2.0 * PI - PI / 2.0).sin().mul(0.5).add(0.5);
        physics::apply_velocity(&mut isle.trans, &state.location, dt.dt);
    }
}
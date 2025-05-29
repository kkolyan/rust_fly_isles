use macroquad::prelude::{f32, WHITE};
use macroquad::color::Color;
use crate::common::curve::Curve;
use crate::f64;

pub const RES_K : f32 = 1.2;
pub const SCALE_SPEED: f32 = 1.4;
pub const K: f32 = 4.0;

pub const SPEED_ABS_MAX: f32 = 2500.0;
pub const GRAVITY: f32 = 500.0 / K;
pub const PLANE_THRUST_NOMINAL: f32 = 400.0 / K;
pub const MISSILE_THRUST: f32 = 8000.0 / K;
pub const DRAG: f32 = 0.0005 / K;
pub const SLIDE: f32 = 0.15;
pub const DT_MAX: f32 = 0.016;
pub const DT_MIN: f32 = 0.008;
// used for evaluation curves based on speed
pub const NOMINAL_SPEED: f32 = 700.0;

// some phases should be escaped in animation end handler. this timeout is just for safety
// if animation end handler not invoked due to bug
pub const ANIM_END_PHASE_TIMEOUT_SEC: f32 = 3.0;

pub const LOGIC_RESOLUTION: (f32, f32) = (1024.0 * SCALE_SPEED * RES_K, 720.0 * SCALE_SPEED * RES_K);

// pub const TINT_DUPLICATE: Color = GRAY;
pub const TINT_DUPLICATE: Color = WHITE;

pub const SPECTATOR_SPEED: f32 = 2700.0;

pub const PAIN_SECONDS: f32 = 0.03;

pub const FLYING_SWING_PERIOD: f32 = 1.0;
pub const FLYING_SWING_ACCELERATION: f32 = 40.0;

pub const INITIAL_ENERGY: f32 = 100.0;

pub const SECONDS_TO_RESTORE_FULL_ENERGY: f32 = 13.5;

pub const INITIAL_ENERGY_RESTORE_PER_SEC: f32 = INITIAL_ENERGY / SECONDS_TO_RESTORE_FULL_ENERGY;

pub const INITIAL_HP: f32 = 100.0;

pub const FULL_THROTTLE_ENERGY_PER_SECOND: f32 = 15.0;

pub const DEV: bool = false;

pub fn standard_lava_damage_per_sec_norm() -> Curve<f32> {
    Curve::from_function(|y| {
        let limit = 0.90;
        f32::clamp((y - limit) / (1.0 - limit), 0.0, 1.0).powi(2) * 300.0
    })
}

pub fn next_level_xp(current_level: u16) -> u32 {
    match current_level {
        0 => 0,
        1 => 1000,
        _ => (next_level_xp(current_level - 1) as f32 + 1000.0 * 1.1f32.powi(current_level as i32)) as u32
    }
}

pub const XP_MUL: f32 = 1.0;

pub const MAX_FPS: f64 = 60.0;

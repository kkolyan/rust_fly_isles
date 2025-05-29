use macroquad::math::clamp;
use macroquad::prelude::Vec2;
use std::f32::consts::PI;
use std::ops::{Div, Mul};

use crate::common::angle::{Angle, AsRadians};
use crate::common::metrics::Metrics;
use crate::model::def::{Location, SlideStabilization, Stabilization, SteerStabilization};
use crate::model::state::{RotState, TransState};
use crate::Plane;
use crate::resources::constants::{DRAG, GRAVITY, NOMINAL_SPEED, SPEED_ABS_MAX};
use crate::resources::objects::objects;

pub fn apply_rotation(body: &mut RotState, dt: f32) {
    let rot_vel = body.ang_velocity_rad;
    let d_rad = (rot_vel * dt);
    let angle = d_rad.as_radians();
    body.angle += angle;
    body.angle = body.angle.normalize();
}

pub fn apply_velocity(body: &mut TransState, location: &Location, dt: f32) {
    body.pos += body.velocity * dt;

    while body.pos.x < 0.0 {
        body.pos.x += location.size.x;
    }
    while body.pos.x > location.size.x {
        body.pos.x -= location.size.x;
    }
    body.pos.y = clamp(body.pos.y, 0.0, location.size.y);
}

pub fn apply_gravity(plane: &mut TransState, metrics: &Metrics, dt: f32)  {
    let dv = Vec2::new(0.0, GRAVITY * dt);
    metrics.record("dV/gravity", dv);
    plane.velocity += dv;
}

pub fn apply_drag(plane: &mut TransState, metrics: &Metrics, dt: f32, dt_total: f32) {
    let speed = plane.velocity.length();
    let drag = DRAG * speed.powi(2) * dt;
    let dv = if drag > speed {
        -plane.velocity / dt_total * dt
    } else {
        plane.velocity.try_normalize().unwrap_or(Vec2::ZERO) * (-drag)
    };
    metrics.record("dV/drag", dv);
    plane.velocity += dv;
}

pub fn apply_thrust(metrics: &Metrics, dt: f32, acceleration: f32, rot: &RotState, pos: &mut TransState) {
    let thrust_dir = rot.angle.to_vec2_norm();
    let thrust_amount = acceleration * dt;
    let dv = thrust_dir * thrust_amount;
    metrics.record("dV/thrust", dv);
    pos.velocity += dv;
    if pos.velocity.length() > SPEED_ABS_MAX {
        pos.velocity = pos.velocity.normalize() * SPEED_ABS_MAX;
    }
}

pub fn apply_slide(metrics: &Metrics, dt: f32, dt_total: f32, trans: &mut TransState, rot: &RotState, def: &SlideStabilization) {
    let dv = if let Some(move_dir) = trans.velocity.try_normalize() {
        let plane_dir = rot.angle.to_vec2_norm();
        let plane_normal = -plane_dir.perp();

        let attack_angle = plane_dir.angle_between(move_dir);
        let slide_amount = (attack_angle.cos().signum() * attack_angle.sin()) * plane_dir.dot(move_dir).signum();
        let slide_by_speed = def.slide_by_speed.lerp(trans.velocity.length() / NOMINAL_SPEED);
        let by_attack = def.slide_by_attack.lerp(attack_angle.abs() / PI);

        metrics.record("dV/slide/by_attack", by_attack);

        let slide = plane_normal * trans.velocity.length() * slide_amount * slide_by_speed * by_attack;

        slide / dt_total * dt
    } else {
        Vec2::ZERO
    };
    metrics.record("dV/slide", dv);
    trans.velocity += dv;
}

pub fn apply_steering(body: &mut TransState, rot: &mut RotState, def: &SteerStabilization, desired_rot: Angle, rec: &Metrics, dt: f32) {
    let by_speed = def.steering_by_speed.lerp(body.velocity.length() / NOMINAL_SPEED);
    let by_attack = def.steering_by_attack.lerp(body.velocity.angle_between(rot.angle.to_vec2_norm()).abs() / PI);
    let max_ang_accel = def.max_angular_acceleration * by_attack * by_speed;

    let desired_angle_rad = rot.angle.to_vec2_norm().angle_between(desired_rot.to_vec2_norm());

    rec.record("desired_angle", desired_angle_rad.as_radians());

    // ускорение, требующееся для поворота на заданный угол за заданное время с учетом текущей угловой скорости (но без учета конечной)
    // s = v0 * t + a * t^2 / 2
    // s - v0 * t = a * t^2 / 2
    // (s - v0 * t) = a * t^2
    // (s - v0 * t) / t^2 = a
    // a = (s - v0 * t) / t^2

    // подобранная поправка, чтобы не было болтанки, вызываемой тем, что желаемая конечная скорость (нулевая) не учитывая в рассчете
    let fixed_angle_rad = desired_angle_rad.div(20.0);

    let desired_acceleration = (fixed_angle_rad - rot.ang_velocity_rad * dt) / dt.powi(2);
    let limited_accel = desired_acceleration.abs().min(max_ang_accel).mul(desired_acceleration.signum());

    rot.ang_velocity_rad += limited_accel * dt;
}

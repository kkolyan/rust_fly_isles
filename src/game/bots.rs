use macroquad::prelude::Vec2;
use std::f32::consts::PI;
use crate::common::angle::AsRadians;
use crate::common::contract::GetMut;
use crate::game::plane;
use crate::GameState;
use crate::model::state::BotState;
use crate::rand::gen_range;
use crate::resources::constants::NOMINAL_SPEED;

pub fn init_bots(state: &mut GameState) {
    for i in 0..state.location.bots.len() {
        let bot = &state.location.bots[i];
        let direction_x = if gen_range(0, 1) == 0 { -1.0 } else { 1.0 };
        let pos = Vec2::new(
            gen_range(0.0, state.location.size.x),
            bot.height_normalized.lerp(gen_range(0.0, 1.0)) * state.location.size.y,
        );
        let plane = plane::allocate_plane(
            &mut state.planes,
            bot.plane.clone(),
            pos,
            (PI * 0.5 - direction_x * PI * 0.5).as_radians().normalize(),
            &state.def,
            &state.player
        );
        state.bots.push(BotState {
            plane,
            direction_x,
        });
    }
}

pub fn update_bots(state: &mut GameState) {
    for bot in &mut state.bots {
        if let Some(plane) = state.planes.get_mut(&bot.plane) {
            let plane = plane;
            if plane.trans.pos.x + bot.direction_x * 10.0 > state.location.size.x {
                plane.rot.angle = PI.as_radians();
                plane.desired_rot = PI.as_radians();
                bot.direction_x = -1.0;
                plane.trans.velocity = plane.rot.angle.to_vec2_norm() * NOMINAL_SPEED;
            }
            if plane.trans.pos.x + bot.direction_x * 10.0 < 0.0 {
                plane.rot.angle = 0f32.as_radians();
                plane.desired_rot = 0f32.as_radians();
                bot.direction_x = 1.0;
                plane.trans.velocity = plane.rot.angle.to_vec2_norm() * NOMINAL_SPEED;
            }
        }
    }
}

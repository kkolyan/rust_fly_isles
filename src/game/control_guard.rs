use macroquad::input::mouse_position;
use macroquad::prelude::{BLUE, Color, RED};
use macroquad::shapes::draw_circle;
use crate::common::camera::ViewPort;
use crate::draw::Stats;
use crate::{DrawState, GameState, Mat2, Vec2};
use crate::common::contract::Get;
use crate::common::frame::FrameCtx;

const GUARD_DISTANCE: f32 = 200.0;
const GUARD_RADIUS: f32 = 80.0;

const GUARD_COLOR: Color = Color::new(0.90, 0.90, 0.90, 0.70);

pub fn draw(state: &GameState, stats: &Stats, draw_state: &DrawState, vp: &ViewPort) {
    if should_show_guard(state) {
        if let Some(guard_pos) = get_guard_pos(state) {
            vp.port(guard_pos, 1.0, |ported| {
                let circle_pos = ported.screen_pos;
                draw_circle(circle_pos.x, circle_pos.y, ported.screen_scale * GUARD_RADIUS, GUARD_COLOR);
            });
        }
    }
}

pub fn update(state: &mut GameState, dt: &FrameCtx) {}

pub fn process_input(state: &mut GameState, vp: &ViewPort) {
    if should_show_guard(state) {
        if let Some(guard_pos) = get_guard_pos(state) {
            let mpw = vp.screen_to_world(Vec2::from(mouse_position()), None);
            if (guard_pos - mpw).length() < GUARD_RADIUS {
                state.player.steering = true;
            }
        }
    }
}

fn should_show_guard(state: &GameState) -> bool {
    !state.player.steering && state.player.windows.is_empty()
}

fn get_guard_pos(state: &GameState) -> Option<Vec2> {
    state.player.plane
        .and_then(|it| state.planes.get(&it))
        .map(|it| it.trans.pos + Mat2::from_angle(it.rot.angle.to_rad()) * Vec2::new(GUARD_DISTANCE, 0.0))
}
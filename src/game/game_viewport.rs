use macroquad::prelude::{screen_height, screen_width};
use macroquad::math::Rect;
use crate::common::camera::ViewPort;
use crate::{GameState, Vec2};
use crate::common::unsorted::RectExtOps;
use crate::resources::constants::LOGIC_RESOLUTION;

const BOUNDS_MARGIN: f32 = 1.0;

pub fn create_viewport(state: &GameState) -> ViewPort {
    let (sw, sh) = LOGIC_RESOLUTION;

    let screen_size = Vec2::new(
        screen_width(),
        screen_height(),
    );

    let view_scale = (screen_size.x / sw).max(screen_size.y / sh);

    let bounds = Rect::new(
        screen_size.x * -BOUNDS_MARGIN,
        screen_size.y * -BOUNDS_MARGIN,
        screen_size.x * (1.0 + BOUNDS_MARGIN * 2.0),
        screen_size.y * (1.0 + BOUNDS_MARGIN * 2.0),
    );

    let loc_size = state.location.size;

    let left_pass = bounds.overlaps(&Rect::new(-loc_size.x, 0.0, loc_size.x, loc_size.y)
        .offset(-state.player.camera_pos)
        .scale_with_pos(view_scale)
        .offset(screen_size * 0.5)
    );

    let right_pass = bounds.overlaps(&Rect::new(loc_size.x, 0.0, loc_size.x, loc_size.y)
        .offset(-state.player.camera_pos)
        .scale_with_pos(view_scale)
        .offset(screen_size * 0.5)
    );

    let view_port = ViewPort {
        camera_pos: state.player.camera_pos,
        screen_size,
        view_scale,
        bounds,
        loc_size,
        left_pass,
        right_pass,
    };
    view_port
}

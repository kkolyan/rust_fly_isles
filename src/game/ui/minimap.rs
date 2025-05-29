use macroquad::prelude::{draw_circle, draw_line, draw_rectangle};
use std::collections::HashMap;
use macroquad::color::RED;
use crate::common::camera::ViewPort;
use crate::{GameState, Vec2};
use crate::common::unsorted::{ColorOps, ToColor};
use crate::model::state::{Durable, MobMission};

pub fn draw_minimap(state: &GameState, view_port: &ViewPort) {
    let minimap_screen_size = Vec2::new(
        view_port.screen_size.x * 0.1,
        view_port.screen_size.x * 0.1 * state.location.size.y / state.location.size.x,
    );

    let margin = Vec2::new(1.0, 1.0) * 8.0;

    let to_screen = |pos: Vec2| {
        view_port.screen_size - minimap_screen_size - margin + pos / state.location.size * minimap_screen_size
    };

    let frame_0 = to_screen(Vec2::ZERO);
    // draw_rectangle(frame_0.x, frame_0.y, state.location.size.x, state.location.size.y, "#CCC".to_color().with_alpha(0.5));
    draw_rectangle(
        frame_0.x,
        frame_0.y,
        minimap_screen_size.x,
        minimap_screen_size.y,
        "#000".to_color().with_alpha(1.0),
    );

    let mut guards_by_isle = HashMap::new();
    for (_, mob) in state.mobs.iter() {
        if let Durable::Good { .. } = mob.base.durable {
            match mob.base.mission {
                MobMission::IsleGuard(isle_id) => {
                    let entry = guards_by_isle.entry(isle_id).or_insert(0u32);
                    *entry += 1;
                }
            }
        }
    }

    for (isle_id, isle) in state.isles.iter() {
        if isle.guard_count_threshold == 0 {
            continue;
        }
        let screen_pos = to_screen(isle.trans.pos);
        let guards = *guards_by_isle.get(isle_id).unwrap_or(&0);
        let color = match guards {
            0 => "#00BA00".to_color(),
            v if v < isle.guard_count_threshold => "#FFB200".to_color(),
            _ => RED,
        };

        if let Some(rank) = isle.guard_rank {
            let r = rank as f32 * 2.0 - 1.0;
            draw_circle(screen_pos.x, screen_pos.y, r, color);
        }
    }
    let frame_1 = to_screen(state.location.size);

    let cam_screen_pos = to_screen(state.player.camera_pos);
    draw_line(
        frame_0.x,
        cam_screen_pos.y,
        frame_1.x,
        cam_screen_pos.y,
        1.0,
        "#FFF".to_color().with_alpha(0.5),
    );
    draw_line(
        cam_screen_pos.x,
        frame_0.y,
        cam_screen_pos.x,
        frame_1.y,
        1.0,
        "#FFF".to_color().with_alpha(0.5),
    );
    // draw_circle(cam_screen_pos.x, cam_screen_pos.y, 1.0, "#00F".to_color());
}

use macroquad::color::{BLACK, Color, DARKBLUE, RED};
use macroquad::prelude::{draw_text, measure_text};
use rust_macroquad_ui::primitives::node::node;
use rust_macroquad_ui::primitives::{horizontal_content, single_content, vertical_content};
use rust_macroquad_ui::basic_composites::stretch::stretch_vertical;
use rust_macroquad_ui::basic_composites::margin::margin;
use crate::resources::constants::LOGIC_RESOLUTION;
use crate::{DrawState, GameState, PlaneState, Vec2};
use crate::common::angle::AsRadians;
use crate::common::camera::ViewPort;
use crate::common::contract::Get;
use crate::common::unsorted::ColorOps;
use crate::draw::Stats;
use crate::game::{plane, ui};
use crate::game::ui::{minimap, modal_windows, new_hud, weapons_panel};
use crate::model::def::GameResource;
use crate::model::state::{Durable, UiEvent};

const STYLE_B: f32 = 2.0;
pub const STYLE: &[(Color, f32, f32)] = &[
    (BLACK, 0.0, 0.0),
    (BLACK, 0.0, STYLE_B),
    (BLACK, STYLE_B, 0.0),
    (RED, STYLE_B, STYLE_B),
];

pub fn draw_text_shadowed(text: &str, pos: Vec2, font_size: f32, color: Color, shadow_color: Color) {
    let pattern = [
        // (-1.0, 0.0, background_color),
        // (0.0, -1.0, background_color),
        (-1.0, -1.0, color),
        // (0.0, 1.0, background_color),
        // (1.0, 0.0, background_color),
        (0.0, 0.0, shadow_color),
    ];
    for (dx, dy, color) in pattern {
        draw_text(text, pos.x + dx, pos.y + dy, font_size, color);
    }
}

pub fn draw_hud(state: &GameState, view_port: &ViewPort, plane: &&PlaneState) {
    let mut row_y = 16.0;
    let font = 24.0 * view_port.view_scale;
    let background_color = BLACK.rgb_mul(1.0);
    let pattern = [
        // (-1.0, 0.0, background_color),
        // (0.0, -1.0, background_color),
        (-1.0, -1.0, background_color),
        // (0.0, 1.0, background_color),
        // (1.0, 0.0, background_color),
        (0.0, 0.0, DARKBLUE.rgb_mul(0.5)),
    ];
    let mut draw_var = |text: String| {
        row_y += 24.0 * view_port.view_scale;
        for (dx, dy, color) in pattern {
            draw_text(
                text.as_str(),
                16.0 * view_port.view_scale + dx,
                row_y + dy,
                font,
                color,
            );
        }
    };
    draw_var("".to_owned());
    draw_var("".to_owned());
    draw_var(match plane.durable {
        Durable::Good { hp, .. } => format!("HP: {}", hp),
        Durable::Destroyed(by) => format!("Destroyed by {:?}", by),
    });
    draw_var("".to_owned());
    draw_var(format!("pos: {}", plane.trans.pos));
    draw_var(format!("rot.: {}", plane.rot.angle));
    draw_var("".to_owned());
    draw_var(format!(
        "velocity: {:.3} ({})",
        plane.trans.velocity.length(),
        plane.trans.velocity
    ));
    draw_var(format!(
        "rot. velocity: {:.3}/s",
        plane.rot.ang_velocity_rad.as_radians()
    ));
    draw_var("".to_owned());
    draw_var(format!(
        "attack: {}",
        plane
            .trans
            .velocity
            .angle_between(plane.rot.angle.to_vec2_norm())
            .abs()
            .as_radians()
    ));
    draw_var("".to_owned());
    // draw_var(format!("desired_rot: {}", plane.desired_rot));
    draw_var(format!("throttle: {} / {}", plane.gear, plane.def.gears.iter().filter(|it| it.tech_level <= state.player.thrust_tech_level).count()));
    draw_var("".to_owned());
    draw_var("".to_owned());
    draw_var("".to_owned());


    for name in state.metrics.metrics_order.borrow().iter() {
        if let Some(value) = state.metrics.metrics.borrow().get(name) {
            // draw_var(format!("{}: {}", name, value));
        }
    }

    for (i, (dir, distance)) in plane::find_near_planes(state).iter().enumerate() {
        if i > 10 {
            // draw_var("...".to_owned());
            break;
        }
        // draw_var(format!("Bot {}: {} ({})", i, distance, dir));
    }

    {
        let mut y = 30.0;
        let mut draw_var = |text: &str| {
            draw_text_center_x(
                text,
                y * view_port.view_scale,
                54.0,
                STYLE,
                view_port.view_scale,
            );
            y += 50.0;
        };
        if state.player.windows.is_empty() {
            draw_var("F1 for help");
        }
    }

    {
        let mut y = 150.0;
        let mut draw_var = |text: &str| {
            draw_text_center_x(
                text,
                y * view_port.view_scale,
                36.0,
                STYLE,
                view_port.view_scale,
            );
            y += 30.0;
        };
    }
}

pub fn draw_text_center_x(
    text: &str,
    y: f32,
    font_size: f32,
    colors: &[(Color, f32, f32)],
    view_scale: f32,
) {
    let text_size = measure_text(text, None, font_size as u16, 1.0);
    for (color, ox, oy) in colors {
        draw_text(
            text,
            LOGIC_RESOLUTION.0 * 0.5 * view_scale - text_size.width * 0.5 + ox,
            y + oy,
            font_size,
            *color,
        );
    }
}

pub fn info(pos: Vec2, text: &str, draw_state: &DrawState, view_port: &ViewPort) {
    draw_text_shadowed(
        text,
        Vec2::new(
            pos.x * view_port.view_scale,
            pos.y + 24.0 * view_port.view_scale,
        ),
        24.0 * view_port.view_scale,
        DARKBLUE.rgb_mul(0.5),
        BLACK,
    );
}

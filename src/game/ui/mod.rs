pub mod minimap;
pub mod legacy_hud;
pub mod new_hud;
pub mod shop_window;
mod res_indicator;
mod modal_windows;
mod weapons_panel;
mod improvements_window;
mod equipment_window;
mod main_menu;
mod journal_window;

use crate::common::angle::AsRadians;
use crate::common::contract::{Get, GetMut, InsertSimple};
use crate::common::resource::Resource;
use crate::common::unsorted::{ColorOps, ToColor};
use crate::model::state::{AppStateEvent, UiEvent};
use crate::{AppState, DrawState, Game, GameState, MenuState, MouseButton, start, ui};
use macroquad::color::{RED, WHITE};
use macroquad::prelude::BLACK;
use macroquad::prelude::Vec4Swizzles;
use rust_macroquad_ui::basic_composites::background::background;
use rust_macroquad_ui::basic_composites::label::label;
use rust_macroquad_ui::basic_composites::stretch::stretch_horizontal;
use rust_macroquad_ui::primitives::node::{node, Node};
use rust_macroquad_ui::primitives::text::TextStyle;
use rust_macroquad_ui::primitives::{horizontal_content, single_content};
use std::collections::VecDeque;
use std::fmt::Debug;
use std::ops::{Deref, Mul, Not};
use std::process::exit;
use macroquad::input::{get_last_key_pressed, is_key_pressed};
use macroquad::math::vec2;
use macroquad::miniquad::KeyCode;
use macroquad::time::get_time;
use macroquad::window::screen_width;
use rust_macroquad_ui::basic_composites::margin::margin;
use rust_macroquad_ui::basic_composites::no_stretch::{no_stretch, NoStretchMode};
use rust_macroquad_ui::basic_composites::node_factories::{height_node, margin_node, stretch_around_node, vertical_node};
use rust_macroquad_ui::basic_composites::stretch::StretchSide::{StretchHorizontal, StretchVertical};
use rust_macroquad_ui::common::to_vec::ToVec;
use rust_macroquad_ui::primitives::border::border;
use rust_macroquad_ui::primitives::conditional::conditional;
use rust_macroquad_ui::primitives::mouse::{on_click, on_hover};
use rust_macroquad_ui::UILayer;
use legacy_hud::STYLE;
use crate::game::ui::new_hud::{HudStyle, PanelStyle};
use crate::game_viewport::create_viewport;
use crate::resources::constants::LOGIC_RESOLUTION;

pub fn do_ui(mut state: AppState, draw_state: &DrawState, def: &Resource<Game>) -> AppState {
    let mut events = VecDeque::new();

    match &mut state {
        AppState::Title { menu } => {
            main_menu::main_menu(&mut events, def, true);
        }
        AppState::Game { game } => {
            draw_game_state(game, draw_state);
        }
        AppState::GameMenu { game, menu } => {
            draw_game_state(game, draw_state);
            main_menu::main_menu(&mut events, def, false);
        }
        AppState::Intro => {
            let mut text = vec![
                "Long-long ago the Great Cataclysm passed. No one knows",
                "for sure what happened, but what we have is the fiery",
                "ocean of lava with floating islands above.",
                "",
                "Good for survivors, a lot of ancient artifacts left.",
                "These wonderful echoes of lost civilization allow us",
                "to do wonderful things. Ancients called that Technologies.",
                "The most interesting pieces rest at the Forbidden Isles - ",
                "patrolled by powerful ancient guard creatures.",
                "",
                "Stalkers - are the ones who dare to unveil these secrets.",
                "At the risk of their lives."
            ];

            if get_last_key_pressed().is_some() {
                return AppState::Title { menu: MenuState {} }
            }

            let node: Node<()> = stretch_around_node([StretchHorizontal, StretchVertical], vertical_node(text
                .iter()
                .map(|it| label(*it, ui::intro_style()))
                .to_vec()));

            let mut layer = UILayer::new(1.0, node);
            layer.update();
            layer.draw();
        }
    }
    while let Some(event) = events.pop_front() {
        match event {
            AppStateEvent::NewGame { location } => {
                let game = start::new_game(def, &location);
                return AppState::Game { game };
            }
            AppStateEvent::Quit => {
                exit(0);
            }
            AppStateEvent::Hover(_) => {}
            AppStateEvent::Intro => {
                return AppState::Intro;
            }
        }
    }
    state
}

fn draw_game_state(state: &mut GameState, draw_state: &DrawState) {
    let style = HudStyle {
        hud_panels: PanelStyle { margin: 3.0 },
        window_panels: PanelStyle { margin: 8.0 },
    };

    let view_port = &create_viewport(state);

    minimap::draw_minimap(state, view_port);

    legacy_hud::info(vec2(16.0, 16.0), format!("Version: {}", include_str!("../../../version.txt")).as_str(), draw_state, view_port);
    legacy_hud::info(vec2(16.0, 48.0), format!("FPS: {}", draw_state.fps_counter.smooth).as_str(), draw_state, view_port);

    if let Some(plane_id) = state.player.plane {
        if let Some(plane) = state.planes.get(&plane_id) {
            // legacy_hud::draw_hud(state, view_port, &plane);
        }
    }

    let player_plane = state.player.plane.and_then(|it| state.planes.get(&it));
    if player_plane.is_none() {
        legacy_hud::draw_text_center_x(
            "Press space to respawn",
            200.0 * view_port.view_scale,
            54.0,
            STYLE,
            view_port.view_scale,
        );
    }

    if let Some(player_plane) = player_plane {
        new_hud::draw_hud_new(state, player_plane, style);
    }
    modal_windows::draw_modal_windows(state, view_port, style);
}


pub fn panel<T: Clone + Debug + 'static>(content: Node<T>, style: PanelStyle) -> Node<T> {
    margin_node(style.margin, node()
        .set(single_content(node()
            .pad(background("#000000ee".to_color()))
            .pad(margin(style.margin))
            .set(single_content(content))))
        .set(border(1.0, BLACK)))
        .pad(no_stretch(NoStretchMode::Both))
}

pub fn resources_panel(state: &GameState, style: HudStyle) -> Node<UiEvent> {
    let resources = &state.player.resources;
    if let Some(resources) = res_indicator::resources_indicator(resources, 1.0, true) {
        panel(
            resources,
            style.hud_panels,
        )
    } else {
        height_node(0.0)
    }
}

pub fn text_style() -> TextStyle {
    TextStyle {
        font_size: 30.0,
        color: WHITE,
        shadow: Some(vec![
            (vec2(0.0, -2.0), BLACK),
            (vec2(-2.0, 0.0), BLACK),
            (vec2(-2.0, -2.0), BLACK),
        ]),
    }
}

pub fn intro_style() -> TextStyle {
    TextStyle {
        font_size: 40.0,
        color: WHITE,
        shadow: Some(vec![
            (vec2(0.0, -2.0), BLACK),
            (vec2(-2.0, 0.0), BLACK),
            (vec2(-2.0, -2.0), BLACK),
        ]),
    }
}

pub fn hud_text() -> TextStyle {
    TextStyle {
        font_size: 24.0,
        color: WHITE,
        shadow: Some(vec![
            (vec2(0.0, -2.0), BLACK),
            (vec2(-2.0, 0.0), BLACK),
            (vec2(-2.0, -2.0), BLACK),
        ]),
    }
}

pub fn hud_weapon_style() -> TextStyle {
    TextStyle {
        font_size: 16.0,
        color: WHITE,
        shadow: Some(vec![
            (vec2(0.0, -2.0), BLACK),
            (vec2(-2.0, 0.0), BLACK),
            (vec2(-2.0, -2.0), BLACK),
        ]),
    }
}

pub fn hud_objectives_style() -> TextStyle {
    TextStyle {
        font_size: 24.0,
        color: RED,
        shadow: Some(vec![
            (vec2(0.0, -2.0), BLACK),
            (vec2(-2.0, 0.0), BLACK),
            (vec2(-2.0, -2.0), BLACK),
        ]),
    }
}

pub fn hint_style() -> TextStyle {
    TextStyle {
        font_size: 24.0,
        color: "#FFF".to_color(),
        shadow: Some(vec![
            (vec2(0.0, -2.0), BLACK),
            (vec2(-2.0, 0.0), BLACK),
            (vec2(-2.0, -2.0), BLACK),
        ]),
    }
}

pub fn hotkey_style(active: bool) -> TextStyle {
    TextStyle {
        font_size: 24.0,
        color: if active { "#CCC".to_color() } else { "#777".to_color() },
        shadow: Some(vec![
            (vec2(0.0, -2.0), BLACK),
            (vec2(-2.0, 0.0), BLACK),
            (vec2(-2.0, -2.0), BLACK),
        ]),
    }
}

pub fn header_style() -> TextStyle {
    TextStyle {
        font_size: 40.0,
        color: "#DDC".to_color(),
        shadow: Some(vec![
            (vec2(0.0, -2.0), BLACK),
            (vec2(-2.0, 0.0), BLACK),
            (vec2(-2.0, -2.0), BLACK),
        ]),
    }
}

const WINDOW_HEADER_SPACING: f32 = 16.0;

fn button<S: Into<String>, Event: Clone + Debug + 'static + Eq, Hover: FnOnce(&Event) -> Event>(text: S, click: Event, hover: Hover) -> Node<Event> {
    let hover = hover(&click);
    node()
        .set(on_click(MouseButton::Left, click.clone()))
        .set(on_hover(hover.clone()))
        .set(conditional((
            None,
            [(hover, Some(border(1.0, WHITE)))]
        )))
        .set(horizontal_content([
            stretch_horizontal(),
            margin_node((8.0, 8.0), label(text, header_style())),
            stretch_horizontal(),
        ]))
}

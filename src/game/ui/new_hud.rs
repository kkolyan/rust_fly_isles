use macroquad::color::{Color, DARKBLUE, DARKGREEN, GRAY, ORANGE};
use rust_macroquad_ui::basic_composites::label::label;
use rust_macroquad_ui::basic_composites::margin::margin;
use rust_macroquad_ui::basic_composites::no_stretch::{no_stretch, NoStretchMode};
use rust_macroquad_ui::basic_composites::no_stretch::NoStretchMode::Both;
use rust_macroquad_ui::basic_composites::node_factories::{height_node, horizontal_node, margin_node, stretch_around_node, vertical_node};
use rust_macroquad_ui::basic_composites::stretch::{stretch_horizontal, stretch_vertical};
use rust_macroquad_ui::basic_composites::stretch::StretchSide::StretchHorizontal;
use rust_macroquad_ui::common::to_vec::ToVec;
use rust_macroquad_ui::primitives::{color_fill, height, horizontal_content, layers, single_content, vertical_content, width};
use rust_macroquad_ui::primitives::node::{node, Node};
use rust_macroquad_ui::UILayer;

use crate::{GameState, PlaneState, PlayerState};
use crate::common::angle::AsRadians;
use crate::common::curve::Curve;
use crate::common::unsorted::{ColorOps, ToColor};
use crate::game::ui;
use crate::game::ui::{hud_text, panel, weapons_panel};
use crate::model::def::GameProgressCtx;
use crate::model::state::{Durable, UiEvent};
use crate::resources::constants::{next_level_xp};

const INDICATOR_WIDTH: f32 = 100.0;

#[derive(Clone, Copy, Debug)]
pub struct HudStyle {
    pub hud_panels: PanelStyle,
    pub window_panels: PanelStyle,
}

#[derive(Clone, Copy, Debug)]
pub struct PanelStyle {
    pub margin: f32,
}

pub fn draw_hud_new(state: &GameState, plane: &PlaneState, hud_style: HudStyle) {
    let left_bottom = node::<UiEvent>()
        .set(horizontal_content(vec![
            node()
                .set(vertical_content(
                    vec![
                        stretch_vertical(),
                        node()
                            .set(single_content(
                                node()
                                    .set(vertical_content(vec![
                                        ui::resources_panel(state, hud_style),
                                        info("Help: F1", hud_style),
                                        stats(plane, hud_style, &state.player),
                                        weapons_panel::weapons_panel(state, plane, hud_style),
                                    ]))
                            )),
                    ],
                )),
        ]));
    let right_top = horizontal_node([
        stretch_horizontal(),
        vertical_node([
            margin_node(16.0, objectives(state, &hud_style)),
            stretch_vertical(),
        ]),
    ]);
    let mut layer = UILayer::new(1.0, node()
        .name("HUD")
        .set(layers([left_bottom, right_top])));
    layer.update();
    layer.draw();
}

fn objectives(state: &GameState, style: &HudStyle) -> Node<UiEvent> {
    let node = vertical_node(state.location.progression.iter()
        .filter(|it| it.objective.is_some())
        .filter(|it| (it.display_condition.0)(&GameProgressCtx { player: &state.player, progression: &state.progression, loot: &state.loot }))
        .filter_map(|it| it.objective)
        .map(|it| label(it, ui::hud_objectives_style()))
        .to_vec());

    // panel(
    //     margin_node(style.hud_panels.margin, node),
    //     style.hud_panels,
    // )
    node
}

fn stats(plane: &PlaneState, hud_style: HudStyle, player: &PlayerState) -> Node<UiEvent> {
    let attack_angle = plane.trans
        .velocity
        .angle_between(plane.rot.angle.to_vec2_norm())
        .abs()
        .as_radians();
    let hp = match plane.durable {
        Durable::Good { hp, .. } => hp,
        Durable::Destroyed(_) => 0.0,
    };
    let velocity = plane.trans.velocity.length();
    panel(node()
              .set(vertical_content(vec![
                  info_raw(format!("Fails: {}", player.death_count), hud_style),
                  indicator(hp, player.hp_max, hud_style.hud_panels, format!("{:.0}/{:.0}", hp, player.hp_max), "#C00".to_color()),
                  indicator(plane.energy, player.energy_max, hud_style.hud_panels, format!("{:.0}/{:.0}", plane.energy, player.energy_max), "#00C".to_color()),
                  level_indicator(player, hud_style),
                  stretch_around_node(
                      [StretchHorizontal],
                      label(format!("{:.0} m/s", velocity), hud_text()).pad(margin(hud_style.hud_panels.margin)),
                  ),
                  stretch_around_node(
                      [StretchHorizontal],
                      label(format!("{:.0}° AOA", attack_angle.to_deg()), hud_text()).pad(margin(hud_style.hud_panels.margin)),
                  ),
                  throttle(plane, &player, hud_style),
              ]))
              .pad(no_stretch(NoStretchMode::Both)),
          hud_style.hud_panels,
    )
}

fn level_indicator(plane: &PlayerState, hud_style: HudStyle) -> Node<UiEvent> {
    let color = "#FFC62B".to_color();
    let next_level = next_level_xp(plane.rpg.level);
    let current_level = next_level_xp(plane.rpg.level - 1);
    indicator(plane.rpg.xp - current_level, next_level - current_level, hud_style.hud_panels, format!("Lev.{}", plane.rpg.level), color)
}

pub fn info<S: Into<String>>(text: S, style: HudStyle) -> Node<UiEvent> {
    let target = info_raw(text, style);
    panel(
        margin_node(style.hud_panels.margin, target),
        style.hud_panels,
    )
}

fn info_raw<S: Into<String>>(text: S, style: HudStyle) -> Node<UiEvent> {
    let w = INDICATOR_WIDTH;
    let text_style = hud_text();
    let mut items = vec![
        node()
            .set(layers([
                node()
                    .set(width(w))
                    .set(height(text_style.font_size)),
                horizontal_node([
                    stretch_horizontal(),
                    label(text, text_style).pad(margin(style.hud_panels.margin)),
                    stretch_horizontal(),
                ]),
            ]))
            .pad(no_stretch(Both))
    ];

    let target = horizontal_node(items);
    target
}

fn indicator<V: Into<f64>, S: Into<String>>(current: V, max: V, style: PanelStyle, text: S, color: Color) -> Node<UiEvent> {
    let current = current.into();
    let max = max.into();
    let rel_value = f64::clamp(current / max, 0.0, 1.0);
    let w = INDICATOR_WIDTH;
    let text_style = hud_text();
    let mut items = vec![
        node()
            .set(layers([
                node()
                    .set(color_fill("#222".to_color()))
                    .set(width(w))
                    .set(height(text_style.font_size)),
                horizontal_node([
                    node()
                        .set(color_fill(color))
                        .set(width(w * rel_value as f32))
                        .set(height(text_style.font_size)),
                    stretch_horizontal(),
                ]),
                horizontal_node([
                    stretch_horizontal(),
                    label(text, text_style).pad(margin(style.margin)),
                    stretch_horizontal(),
                ]),
            ]))
            .pad(no_stretch(Both))
    ];

    margin_node(style.margin, horizontal_node(items))
}

fn energy(plane: &PlaneState, player: &PlayerState, style: HudStyle) -> Node<UiEvent> {
    let curve: Curve<Color> = Curve::new([DARKGREEN, ORANGE]);
    let rel_value = plane.energy / player.energy_max;
    let w = 100.0;
    let mut items = vec![
        node()
            .set(layers([
                node()
                    .set(color_fill(DARKBLUE))
                    .set(width(w))
                    .set(height(32.0)),
                horizontal_node([
                    node()
                        .set(color_fill(curve.lerp(rel_value)))
                        .set(width(w * rel_value))
                        .set(height(32.0)),
                    stretch_horizontal()
                ]),
            ]))
            .pad(no_stretch(Both))
    ];

    margin_node(style.hud_panels.margin, horizontal_node(items))
}

fn throttle(plane: &PlaneState, player: &PlayerState, hud_style: HudStyle) -> Node<UiEvent> {
    let curve: Curve<Color> = Curve::new([DARKGREEN, ORANGE]);
    let mut items = vec![];
    let total_w = INDICATOR_WIDTH;
    let gears_available = plane.def.gears.iter()
        .filter(|it| it.tech_level <= player.thrust_tech_level)
        .count();
    for gear in 0..gears_available {
        let color = if gear < plane.gear {
            let color = curve.lerp(gear as f32 / (gears_available as f32 - 1.0));
            if gear < plane.effective_gear {
                color
            } else {
                color.rgb_mul(0.25)
            }
        } else {
            "#447".to_color()
        };
        let w = total_w / gears_available as f32;
        items.push(node()
            .set(width(w))
            .set(height(30.0))
            .set(color_fill(color)))
    }
    margin_node(hud_style.hud_panels.margin, horizontal_node(items))
}
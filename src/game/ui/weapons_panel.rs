use futures::StreamExt;
use rust_macroquad_ui::primitives::node::{Node, node};
use rust_macroquad_ui::basic_composites::label::label;
use rust_macroquad_ui::basic_composites::margin::margin;
use rust_macroquad_ui::basic_composites::node_factories::{margin_node, vertical_node};
use rust_macroquad_ui::common::to_vec::ToVec;
use rust_macroquad_ui::primitives::{height, vertical_content, width};
use crate::{GameState, PlaneState};
use crate::game::ui;
use crate::game::ui::new_hud::HudStyle;
use crate::game::ui::panel;
use crate::model::state::{Ammo, DeviceSpec, DeviceState, EquipmentBinding, ManualBuffAmmo, UiEvent};

fn device(device: &DeviceState, binding: EquipmentBinding) -> Node<UiEvent> {
    let style = ui::hud_weapon_style();
    match &device.spec {
        DeviceSpec::Weapon(weapon) => {
            label(weapon.def.title, style)
        }
        DeviceSpec::Booster(booster) => {
            label(booster.def.title, style)
        }
    }
}

pub fn weapons_panel(state: &GameState, plane: &PlaneState, hud_style: HudStyle) -> Node<UiEvent> {
    if true {
        return node()
            .set(height(0.0))
            .set(width(0.0));
    }
    let mut children = vec![];
    let style = ui::text_style();
    for (_, d) in state.player.equipment.iter() {
        if let Some(binding) = d.binding {
            children.push(device(d, binding));
        }
    }
    panel(
        vertical_node(children.into_iter()
            .map(|it| margin_node(hud_style.hud_panels.margin, it))
            .to_vec()),
        hud_style.hud_panels,
    )
}

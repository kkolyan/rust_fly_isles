use rust_macroquad_ui::primitives::node::{Node, node};
use rust_macroquad_ui::basic_composites::label::label;
use rust_macroquad_ui::basic_composites::margin::margin;
use rust_macroquad_ui::primitives::{color_fill, height, horizontal_content, single_content, vertical_content, width};
use rust_macroquad_ui::primitives::mouse::{on_click, on_hover};
use rust_macroquad_ui::primitives::conditional::conditional;
use macroquad::color::{BLANK, DARKBLUE, YELLOW};
use rust_macroquad_ui::basic_composites::no_stretch::{no_stretch, NoStretchMode};
use rust_macroquad_ui::basic_composites::stretch::{stretch_around, stretch_horizontal};
use std::collections::HashMap;
use rust_macroquad_ui::basic_composites::node_factories::{height_node, horizontal_node, no_stretch_node, stretch_around_node, vertical_node};
use rust_macroquad_ui::basic_composites::stretch::StretchSide::{StretchBottom, StretchLeft, StretchRight, StretchTop};
use crate::common::unsorted::ColorOps;
use crate::game::ui;
use crate::game::ui::{res_indicator, WINDOW_HEADER_SPACING};
use crate::model::def::{DeviceSlot, GameResource, Obtainable};
use crate::model::state::{Ammo, ShopWindow, UiEvent};
use crate::MouseButton::Left;
use crate::PlayerState;

pub fn is_enough_resources(available: &HashMap<GameResource, u32>, required: &HashMap<GameResource, u32>) -> bool {
    required
        .iter()
        .all(|(res, price)| available.get(res).unwrap_or(&0) >= price)
}

pub fn shop_window(shop: &ShopWindow, player: &PlayerState) -> Node<UiEvent> {
    let mut items = vec![];
    for (i, item) in shop.items.iter().enumerate() {
        let caption = match &item.item {
            Obtainable::Weapon { weapon, ammo, slot } => {
                let slot = match slot {
                    DeviceSlot::Primary => "Primary",
                    DeviceSlot::Secondary => "Secondary",
                };
                match ammo {
                    Ammo::Infinite => {
                        format!("{}: {} (unlim)", slot, weapon.title)
                    }
                    Ammo::Finite(ammo) => {
                        format!("{}: {} (x{})", slot, weapon.title, ammo)
                    }
                    Ammo::Energy { energy_per_shot } => {
                        format!("{}: {} ({} energy/shot)", slot, weapon.title, energy_per_shot)
                    }
                }
            }
            Obtainable::HP { title, .. } => {
                format!("Repair: {}", title)
            }
            Obtainable::Consumable { def, reserve_sec: reserve, slot } => {
                let slot = match slot {
                    DeviceSlot::Primary => "Primary",
                    DeviceSlot::Secondary => "Secondary",
                };
                format!("{}: {} ({:.0}s)", slot, def.title, reserve)
            }
            Obtainable::Passive { def } => {
                format!("Passive: {}", def.title)
            }
            Obtainable::PassiveReset { title } => {
                "Passive: None".to_owned()
            }
        };

        let enough_resources = is_enough_resources(&player.resources, &item.price);

        let mut style = ui::text_style();
        if !enough_resources {
            style.color = style.color.with_alpha(0.5);
        }
        let mut label = label(caption, style)
            .pad(margin((0.0, 8.0)));
        if enough_resources {
            label = node()
                .set(single_content(label))
                .set(on_hover(UiEvent::HoverTopItem(i)))
                .set(on_click(Left, UiEvent::ClickShopItem(item.clone())));
        }
        let label = horizontal_node([
            label,
            stretch_around_node(
                [StretchLeft, StretchTop, StretchBottom],
                res_indicator::resources_indicator(&item.price, 0.7, true)
                    .unwrap_or_else(|| height_node(0.0)),
            ),
        ]);

        let label = node()
            .set(conditional((
                Some(color_fill(BLANK)),
                [
                    (UiEvent::HoverTopItem(i), Some(color_fill(DARKBLUE)))
                ]
            )))
            .set(single_content(label));

        let label = node()
            .pad(no_stretch(NoStretchMode::Both))
            .set(conditional((
                Some(color_fill(BLANK)),
                [
                    (UiEvent::HoverTopItem(i), Some(color_fill(DARKBLUE)))
                ]
            )))
            .set(single_content(label));
        items.push(label);
    }
    vertical_node(items)
}

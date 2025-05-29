use std::ptr::eq;
use futures::AsyncReadExt;
use macroquad::color::{DARKBLUE, WHITE};
use rust_macroquad_ui::basic_composites::background::background;
use rust_macroquad_ui::basic_composites::label::label;
use rust_macroquad_ui::basic_composites::margin::margin;
use rust_macroquad_ui::basic_composites::node_factories::{background_node, height_node, horizontal_node, margin_node, vertical_node};
use rust_macroquad_ui::basic_composites::stretch::stretch_horizontal;
use rust_macroquad_ui::common::to_vec::ToVec;
use rust_macroquad_ui::primitives::border::border;
use rust_macroquad_ui::primitives::mouse::{on_click, on_hover};
use rust_macroquad_ui::primitives::node::{Node, node};
use rust_macroquad_ui::primitives::{color_fill, single_content, vertical_content};
use rust_macroquad_ui::primitives::conditional::conditional;
use crate::common::contract::Get;
use crate::common::resource::Resource;
use crate::game::ui;
use crate::{GameState, MouseButton};
use crate::common::unsorted::ToColor;
use crate::model::def::{ImprovementSpec, PlaneWeapon};
use crate::model::state::{DeviceSpec, DeviceState, EquipmentBinding, EquipmentWindow, UiEvent};

pub(crate) fn show_window(state: &GameState, window: &EquipmentWindow) -> Node<UiEvent> {
    vertical_node([
        equipment_panel(state, window),
        height_node(16.0),
        binding_panel(state, window)
    ])
}

fn binding_panel(state: &GameState, window: &EquipmentWindow) -> Node<UiEvent> {
    let mut items = vec![];

    for binding in [Option::<EquipmentBinding>::None]
        .into_iter()
        .chain(enum_iterator::all::<EquipmentBinding>().map(Some))
    {
        let text = binding_title(&binding);
        let button = node();
        let button = if window.selected_item.is_some() {
            button.set(conditional((
                Some(color_fill("#E0AB26".to_color())),
                [(UiEvent::EquipmentClickBinding(binding).to_hover(), Some(color_fill(DARKBLUE)))],
            )))
        } else {
            button.set(color_fill("#444".to_color()))
        };
        let button = margin_node(
            (4.0, 4.0),
            button
                .set(single_content(
                    margin_node((8.0, 4.0), label(text, ui::hotkey_style(window.selected_item.is_some())))
                ))
                .set(on_click(MouseButton::Left, UiEvent::EquipmentClickBinding(binding)))
                .set(on_hover(UiEvent::EquipmentClickBinding(binding).to_hover())),
        );
        items.push(button);
    }

    margin_node((4.0, 4.0), horizontal_node(items))
}

fn equipment_panel(state: &GameState, window: &EquipmentWindow) -> Node<UiEvent> {
    let mut items = vec![];

    let mut equipment = state.player.equipment.iter().to_vec();
    equipment.sort_by_key(|(_, device)| device.order);
    for (id, device) in equipment.iter().copied() {
        let title = match &device.spec {
            DeviceSpec::Weapon(state) => { state.def.title }
            DeviceSpec::Booster(state) => { state.def.title }
        };
        let binding = binding_title(&device.binding);
        let binding_cell = margin_node((8.0, 2.0), label(binding, ui::text_style()));
        let binding_cell = if window.selected_item == Some(*id) {
            background_node("#CCC".to_color(), binding_cell)
        } else {
            binding_cell
        };
        let row = margin_node((12.0, 4.0), horizontal_node([
            label(title, ui::text_style()),
            stretch_horizontal(),
            binding_cell,
        ]));
        items.push(row
            .set(conditional((
                Some(border(0.0, WHITE)),
                [(UiEvent::EquipmentClickItem(*id).to_hover(), Some(border(1.0, WHITE)))]
            )))
            .set(on_click(MouseButton::Left, UiEvent::EquipmentClickItem(*id)))
            .set(on_hover(UiEvent::EquipmentClickItem(*id).to_hover())));
    }

    vertical_node(items)
}

fn binding_title(binding: &Option<EquipmentBinding>) -> &'static str {
    binding
        .map(|it| match it {
            EquipmentBinding::Primary => { "L.Mouse" }
            EquipmentBinding::Secondary => { "R.Mouse" }
            EquipmentBinding::_1 => { "1" }
            EquipmentBinding::_2 => { "2" }
            EquipmentBinding::_3 => { "3" }
            EquipmentBinding::_4 => { "4" }
            EquipmentBinding::_5 => { "5" }
        })
        .unwrap_or("-")
}


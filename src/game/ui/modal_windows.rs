use std::ops::{Add, AddAssign, Deref};
use rust_macroquad_ui::basic_composites::label::label;
use rust_macroquad_ui::basic_composites::no_stretch::NoStretchMode;
use rust_macroquad_ui::basic_composites::node_factories::{height_node, margin_node, no_stretch_node, stretch_around_node, vertical_node};
use rust_macroquad_ui::primitives::node::{node, Node};
use rust_macroquad_ui::primitives::{horizontal_content, vertical_content};
use rust_macroquad_ui::basic_composites::stretch::{stretch_around, stretch_horizontal, stretch_vertical};
use rust_macroquad_ui::basic_composites::stretch::StretchSide::{StretchHorizontal, StretchLeft, StretchRight};
use rust_macroquad_ui::common::to_vec::ToVec;
use rust_macroquad_ui::UILayer;
use crate::common::camera::ViewPort;
use crate::common::contract::InsertSimple;
use crate::game::rpg;
use crate::game::ui::{header_style, panel, shop_window, improvements_window, text_style, WINDOW_HEADER_SPACING, equipment_window};
use crate::game::ui::new_hud::HudStyle;
use crate::GameState;
use crate::model::def::GameProgressCtx;
use crate::model::state::{EquipmentWindow, JournalWindow, UiEvent, UiWindow, WindowsAction};
use crate::ui::journal_window;

pub fn window_content_line(text: &str) -> Node<UiEvent> {
    margin_node(
        (0.0, 4.0),
        stretch_around_node([], label(text, text_style())),
    )
}

pub fn draw_modal_windows(state: &mut GameState, view_port: &ViewPort, style: HudStyle) {
    if let Some(window) = &mut state.player.windows.back() {
        let (title, window) = match window {
            UiWindow::Shop(shop) => ("Flying Workshop", shop_window::shop_window(shop, &state.player)),
            UiWindow::Improvements => { ("Improvements", improvements_window::show_window(state)) }
            UiWindow::Help => {
                ("Help", vertical_node([
                    window_content_line("F2: Improvements Shop"),
                    window_content_line("F3: Equipment Customization"),
                    window_content_line("F4: Journal"),
                    window_content_line("Mouse wheel: throttle control"),
                    window_content_line("Left click: primary weapon activation"),
                    window_content_line("Right click: secondary equipment activation"),
                    window_content_line("Move mouse to a circle to regain control after pause"),
                    window_content_line("M: toggle sound"),
                    window_content_line("Esc: hide these stupid window"),
                ]))
            }
            UiWindow::Equipment(window) => { ("Equipment Customization (Bindings)", equipment_window::show_window(state, window)) }
            UiWindow::Journal(window) => { ("Journal", journal_window::show_window(state, window)) }
        };
        let window = panel(
            margin_node(
                8.0,
                no_stretch_node(
                    NoStretchMode::Both,
                    vertical_node([
                        label(title, header_style())
                            .pad(stretch_around([StretchLeft, StretchRight])),
                        height_node(WINDOW_HEADER_SPACING),
                        window,
                    ]),
                ),
            ),
            style.window_panels,
        );
        let node = node()
            .name("window")
            .set(vertical_content(vec![
                stretch_vertical(),
                node().set(horizontal_content(vec![
                    stretch_horizontal(),
                    window,
                    stretch_horizontal(),
                ])),
                stretch_vertical(),
            ]));
        let mut layer = UILayer::new(1.0, node);
        layer.update();
        layer.draw();

        for event in layer.get_events() {
            match event {
                UiEvent::HoverTopItem(_) => {}
                UiEvent::ClickShopItem(item) => {
                    if shop_window::is_enough_resources(&state.player.resources, &item.price) {
                        state.ui_commands.insert_simple(WindowsAction::Buy(item.clone()));
                    }
                }
                UiEvent::ClickSkillsItem(skill_id) => {
                    rpg::click_skill(state, skill_id);
                }
                UiEvent::Hover(_) => {}
                UiEvent::EquipmentClickItem(item) => {
                    if let Some(UiWindow::Equipment(window)) = state.player.windows.back_mut() {
                        window.selected_item = Some(*item);
                    }
                }
                UiEvent::EquipmentClickBinding(binding) => {
                    if let Some(UiWindow::Equipment(EquipmentWindow { selected_item: Some(selected_item) })) = state.player.windows.back().cloned() {
                        rpg::bind_equipment(state, &selected_item, binding);
                    }
                }
                UiEvent::JournalAbs { page } => {
                    if let Some(UiWindow::Journal(window)) = state.player.windows.back_mut() {
                        window.page = *page;
                    }
                }
            }
        }
    }
}

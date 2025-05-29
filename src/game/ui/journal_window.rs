use rust_macroquad_ui::basic_composites::label::label;
use rust_macroquad_ui::basic_composites::no_stretch::{no_stretch, NoStretchMode};
use rust_macroquad_ui::basic_composites::node_factories::{horizontal_node, margin_node, vertical_node, width_node};
use rust_macroquad_ui::basic_composites::stretch::{stretch_horizontal, stretch_vertical};
use rust_macroquad_ui::common::to_vec::ToVec;
use rust_macroquad_ui::primitives::{height, horizontal_content, single_content, width};
use rust_macroquad_ui::primitives::mouse::on_click;
use rust_macroquad_ui::primitives::node::{Node, node};
use crate::{GameState, ui};
use crate::model::state::{JournalWindow, UiEvent};
use crate::ui::modal_windows::window_content_line;

pub fn show_window(state: &GameState, window: &JournalWindow) -> Node<UiEvent> {
    let lines = state.journal.get(window.page as usize)
        .map(|page| &page.lines);
    node()
        .set(width(800.0))
        .set(height(400.0))
        .set(single_content(
            margin_node((8.0, 0.0),
                        vertical_node([
                            lines
                                .map(|it| vertical_node(it.iter()
                                    .map(|line| window_content_line(line))
                                    .to_vec())
                                )
                                .unwrap_or_else(|| width_node(0.0)),
                            stretch_vertical(),
                            horizontal_node([
                                horizontal_node(state.journal.iter()
                                    .enumerate()
                                    .map(|(i, _)| ui::button(format!("{}", i + 1), UiEvent::JournalAbs { page: i }, UiEvent::to_hover))
                                    .to_vec()),
                            ]),
                        ]),
            )
        ))
}
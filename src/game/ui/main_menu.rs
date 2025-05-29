use std::collections::VecDeque;
use rust_macroquad_ui::basic_composites::node_factories::{stretch_around_node, vertical_node};
use rust_macroquad_ui::basic_composites::stretch::StretchSide::{StretchHorizontal, StretchVertical};
use rust_macroquad_ui::common::to_vec::ToVec;
use rust_macroquad_ui::primitives::layers;
use rust_macroquad_ui::primitives::node::{Node, node};
use rust_macroquad_ui::UILayer;

use crate::{Game, ui};
use crate::common::resource::Resource;
use crate::model::state::AppStateEvent;
use crate::ui::new_hud::PanelStyle;
use crate::ui::panel;

pub fn main_menu(events: &mut VecDeque<AppStateEvent>, def: &Resource<Game>, allow_show_intro: bool) {
    let mut items = vec![];
    items.push(("Start Training Flight", AppStateEvent::NewGame { location: def.training.clone() }));
    items.push(("Start Combat Mission", AppStateEvent::NewGame { location: def.combat.clone() }));
    if allow_show_intro {
        items.push(("Intro", AppStateEvent::Intro));
    }
    items.push(("Quit", AppStateEvent::Quit));
    generic_menu(events, items);
}

pub fn generic_menu(events: &mut VecDeque<AppStateEvent>, options: Vec<(&str, AppStateEvent)>) {
    let style = PanelStyle { margin: 16.0 };
    let node: Node<AppStateEvent> = node()
        .set(layers([
            stretch_around_node([StretchHorizontal, StretchVertical], vertical_node([
                panel(vertical_node(options.into_iter()
                    .map(|(text, event)| ui::button(text, event, AppStateEvent::hover))
                    .to_vec()), style),
            ])),
        ]));
    let mut layer = UILayer::new(1.0, node);
    layer.update();
    for event in layer.get_events() {
        events.push_back(event.clone());
    }
    layer.draw();
}
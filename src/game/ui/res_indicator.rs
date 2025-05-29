use std::collections::HashMap;
use rust_macroquad_ui::primitives::node::{Node, node};
use rust_macroquad_ui::basic_composites::margin::margin;
use rust_macroquad_ui::primitives::horizontal_content;
use rust_macroquad_ui::basic_composites::icon::{icon, IconSize};
use macroquad::prelude::Texture2D;
use macroquad::color::WHITE;
use rust_macroquad_ui::basic_composites::label::label;
use rust_macroquad_ui::basic_composites::no_stretch::{no_stretch, NoStretchMode};
use rust_macroquad_ui::basic_composites::node_factories::{height_node, stretch_around_node, width_node};
use rust_macroquad_ui::basic_composites::stretch::StretchSide::StretchVertical;
use crate::game::ui;
use crate::model::def::GameResource;
use crate::model::state::UiEvent;
use crate::resources::sprites::loot::{image_resource_a, image_resource_b, image_resource_c};
use crate::Vec2;

pub fn resources_indicator(resources: &HashMap<GameResource, u32>, scale: f32, skip_zero: bool) -> Option<Node<UiEvent>> {
    let mut items = vec![];
    for res in enum_iterator::all::<GameResource>() {
        let bytes = match res {
            GameResource::A => image_resource_a(),
            GameResource::B => image_resource_b(),
            GameResource::C => image_resource_c(),
        };
        let value = resources.get(&res).copied().unwrap_or(0);
        if skip_zero && value == 0 {
            break;
        }
        items.push(node().name("resource")
            .pad(margin((4.0, 4.0)))
            .pad(no_stretch(NoStretchMode::Both))
            .set(horizontal_content(vec![
                stretch_around_node([StretchVertical], icon(
                    Texture2D::from_file_with_format(bytes, None),
                    WHITE,
                    IconSize::Fixed(30.0 * scale, 30.0 * scale),
                )),
                match value {
                    1 => width_node(0.0),
                    _ => stretch_around_node([StretchVertical], label(
                        format!("x{}", value).as_str(),
                        ui::text_style(),
                    ))
                },
            ])));
    }
    if items.is_empty() {
        return None;
    }
    Some(node()
        .name("resources")
        .set(horizontal_content(items)))
}

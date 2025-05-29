use macroquad::color::{BLACK, DARKBLUE, WHITE, YELLOW};
use macroquad::input::MouseButton;
use rust_macroquad_ui::basic_composites::label::label;
use rust_macroquad_ui::basic_composites::margin::margin;
use rust_macroquad_ui::basic_composites::node_factories::{horizontal_node, margin_node, vertical_node};
use rust_macroquad_ui::basic_composites::stretch::stretch_horizontal;
use rust_macroquad_ui::common::to_vec::ToVec;
use rust_macroquad_ui::primitives::{color_fill, height, horizontal_content, single_content, vertical_content, width, width_stretch};
use rust_macroquad_ui::primitives::border::border;
use rust_macroquad_ui::primitives::conditional::conditional;
use rust_macroquad_ui::primitives::mouse::{on_click, on_hover};
use rust_macroquad_ui::primitives::node::{Node, node};
use crate::{Game, GameState, PlaneState, PlayerState};
use crate::common::contract::Get;
use crate::common::unsorted::ToColor;
use crate::game::{rpg, ui};
use crate::game::ui::{hint_style, text_style};
use crate::model::def::{Improvement, ImprovementCategory, ImprovementId, ImprovementLevel, ImprovementTitle};
use crate::model::state::UiEvent;

pub(crate) fn show_window(state: &GameState) -> Node<UiEvent> {
    let mut items = vec![];
    items.push(improvements_grid(state, &state.player));
    items.push(hint_panel(state));
    items.push(label(format!("Available points: {}", state.player.rpg.skill_points), ui::text_style()));
    node()
        .name("improvements window content")
        .set(vertical_content(items))
}

fn improvements_grid(state: &GameState, player: &PlayerState) -> Node<UiEvent> {
    let mut columns = vec![];
    let mut button_id = 0;
    for category in enum_iterator::all::<ImprovementCategory>() {
        let mut skills = state.def.rpg.iter()
            .filter(|(id, skill)| skill.category == category)
            .to_vec();
        skills.sort_by_key(|(id, _)| **id);
        columns.push(improvements_category_column(player, &mut button_id, category, skills));
    }
    horizontal_node(columns)
}

fn improvements_category_column(player: &PlayerState, button_id: &mut usize, category: ImprovementCategory, mut skills: Vec<(&ImprovementId, &Improvement)>) -> Node<UiEvent> {
    let items = skills.iter().copied()
        .map(|(id, skill)| improvement_cell(player, button_id, id, skill))
        .to_vec();
    vertical_node(vec![
        label(match category {
            ImprovementCategory::Passives => { "Passives" }
            ImprovementCategory::Weapons => { "Weapons" }
            ImprovementCategory::Utility => { "Utility" }
        }, ui::text_style()),
        node()
            .name("improvements column")
            .set(vertical_content(items))
            .pad(margin(16.0)),
    ])
}

fn improvement_cell(player: &PlayerState, button_id: &mut usize, id: &ImprovementId, skill: &Improvement) -> Node<UiEvent> {
    let level = player.rpg.skills.get(id).copied().unwrap_or(0);
    let next_level = skill.levels.get(usize::from(level));
    let items = [
        label(format!("{}/{}", level, skill.levels.len()), ui::text_style()),
        stretch_horizontal(),
        match next_level {
            None => label("", ui::text_style()),
            Some(next_level) => {
                let enough_points = player.rpg.skill_points >= next_level.points;
                node()
                    .set(on_hover(UiEvent::HoverTopItem(*button_id)))
                    .set(on_click(MouseButton::Left, UiEvent::ClickSkillsItem(*id)))
                    .set(conditional((
                        Some(color_fill(if enough_points { "#FFC32B".to_color() } else { "#333".to_color() })),
                        [
                            (UiEvent::HoverTopItem(*button_id), Some(color_fill(
                                if enough_points { DARKBLUE } else { "#CCC".to_color() }
                            )))
                        ]
                    )))
                    .set(single_content(margin_node((4.0, 0.0), label(format!("Invest {} points", next_level.points), ui::text_style()))))
            }
        }
    ];
    *button_id += 1;
    let items = [
        label(rpg::improvement_title(skill), ui::text_style()),
        node()
            .set(width(300.0))
            .set(horizontal_content(items)),
    ];
    node()
        .set(on_hover(UiEvent::ClickSkillsItem(*id).to_hover()))
        .set(conditional((
            None,
            [(UiEvent::ClickSkillsItem(*id).to_hover(), Some(border(1.0, WHITE)))]
        )))
        .set(single_content(margin_node(16.0, vertical_node(items))))
}

fn hint_panel(state: &GameState) -> Node<UiEvent> {
    node()
        .name("hint panel")
        .pad(margin((16.0, 0.0)))
        // .set(color_fill(BLACK))
        .set(single_content(
            node()
                .name("hint box")
                .pad(margin((8.0, 0.0)))
                .set(width_stretch())
                .set(height(64.0))
                .set(conditional((
                    Some(vertical_content([
                        label("", hint_style()),
                        label("", hint_style())
                    ])),
                    state.def.rpg.iter()
                        .map(|(skill_id, skill)| {
                            (
                                UiEvent::ClickSkillsItem(*skill_id).to_hover(),
                                Some(vertical_content(skill.description
                                    .map(|it| label(it, hint_style()))))
                            )
                        })
                        .to_vec()
                ))))
        )
}
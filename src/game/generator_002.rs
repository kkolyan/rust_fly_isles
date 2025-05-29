use std::collections::HashSet;
use std::ops::Not;
use macroquad::logging::info;
use macroquad::prelude::Vec2;
use macroquad::rand::{ChooseRandom};
use crate::common::contract::{GetMut, Insert};
use crate::common::curve::Curve;
use crate::common::resource::Resource;
use crate::common::unsorted::gen_range;
use crate::game::mobs;
use crate::GameState;
use crate::model::def::{Isle, Mob, MobKind, MobRank};
use crate::model::state::{IsleState, MobMission, RelativePos, TransState};
use crate::resources::constants::DEV;

#[derive(Debug)]
pub struct LocationGenerator002 {
    pub layers: Vec<Layer>,
}

#[derive(Debug)]
pub struct Layer {
    pub name: &'static str,
    pub mobs: Vec<MobConfig>,
    pub mob_per_isle: Curve<f32>,
    pub isles_per_archipelago: Curve<f32>,
    pub isles: Vec<Resource<Isle>>,
    pub height_normal: Curve<f32>,
    pub x_normal: Curve<f32>,
    pub rank_minimap_penalty: u32,
}

#[derive(Debug)]
pub struct MobConfig {
    pub mob: Resource<Mob>,
    pub count: u32,
}

const ISLE_SIZE: (f32, f32) = (300.0, 200.0);

pub fn generate(state: &mut GameState, generator: &LocationGenerator002) {
    for layer in &generator.layers {
        info!("generating layer {}", layer.name);
        let mut mobs: Vec<Resource<Mob>> = layer.mobs.iter()
            .flat_map(|it| (0..it.count).map(|_| it.mob.clone()))
            .collect();
        mobs.shuffle();
        let mut isles = vec![];
        {
            let mut remaining_mobs = mobs.as_slice();
            while remaining_mobs.is_empty().not() {
                let n = layer.mob_per_isle.random().round() as usize;
                let (rem, isle) = remaining_mobs.split_at(remaining_mobs.len().saturating_sub(n));
                remaining_mobs = rem;
                isles.push(isle);
            }
        }
        let mut archipelagos = vec![];
        {
            let mut remaining_isles = isles.as_slice();
            while remaining_isles.is_empty().not() {
                let n = layer.isles_per_archipelago.random().round() as usize;
                let (rem, archipelago) = remaining_isles.split_at(remaining_isles.len().saturating_sub(n));
                remaining_isles = rem;
                archipelagos.push(archipelago);
            }
        }
        place_objects(state, archipelagos, &layer);
    }
}

fn place_objects(state: &mut GameState, archipelagos: Vec<&[&[Resource<Mob>]]>, layer: &Layer) {
    let mut archipelago_positions = vec![];
    let mut isle_positions = vec![];
    for isles in archipelagos {
        let group_size = Vec2::from(ISLE_SIZE) * (isles.len() as f32).sqrt() * 1.5;
        if let Some(group_pos) = generate_archipelago_pos(state, &layer.height_normal, &layer.x_normal, group_size, &mut archipelago_positions) {
            for mobs in isles {
                if let Some(isle_pos) = generate_isle_pos(state, group_pos, group_size, &mut isle_positions) {
                    let isle_id = state.isles.insert(IsleState {
                        order: isle_pos.y as i32,
                        def: layer.isles.choose().unwrap().clone(),
                        trans: TransState {
                            pos: isle_pos,
                            velocity: Vec2::ZERO,
                        },
                        course_change_interval_last: 0.0,
                        course_seconds_remaining: 0.0,
                        course: Vec2::ONE,
                        guard_count_threshold: 0,
                        guard_rank: Default::default(),
                    });
                    let isle = state.isles.get_mut(&isle_id).unwrap();
                    let mut ranks = HashSet::new();
                    for mob in mobs.iter() {
                        let pos = match mob.kind {
                            MobKind::Walker => RelativePos::Isle(isle_id, Vec2::new(
                                gen_range(isle.def.bounds.clone()),
                                0.0,
                            )),
                            MobKind::Flyer => RelativePos::Global(isle.trans.pos)
                        };
                        mobs::spawn_mob(
                            &mut state.mobs,
                            &mob,
                            pos,
                            MobMission::IsleGuard(isle_id),
                            &mut state.gids,
                        );
                        isle.guard_count_threshold += 1;
                        ranks.insert(mob.rank);
                    }
                    if ranks.is_empty().not() {
                        isle.guard_rank = Some((ranks.iter().max().unwrap() - ranks.iter().min().unwrap()).saturating_sub(layer.rank_minimap_penalty) + 1);
                    }
                }
            }
        }
    }
}

fn generate_archipelago_pos(state: &GameState, height_normal: &Curve<f32>, x_normal: &Curve<f32>, size: Vec2, archipelagos: &mut Vec<Vec2>) -> Option<Vec2> {
    let max_attempts = 300;
    for _ in 0..max_attempts {
        let pos = Vec2::new(
            x_normal.random() * state.location.size.x,
            height_normal.random() * state.location.size.y,
        );
        let has_clashes = archipelagos.iter()
            .any(|it| {
                let distance = (*it - pos).abs();
                let tolerance = 1.2;
                distance.x < size.x * tolerance && distance.y < size.y * tolerance
            });
        if !has_clashes {
            archipelagos.push(pos);
            return Some(pos);
        }
    }

    if DEV {
        panic!("failed to find archipelago pos after {} attempts", max_attempts);
    }
    None
}

fn generate_isle_pos(state: &GameState, group_pos: Vec2, group_size: Vec2, isle_positions: &mut Vec<Vec2>) -> Option<Vec2> {
    let max_attempts = 100;
    for _ in 0..max_attempts {
        let pos = group_pos + Vec2::new(
            gen_range(-0.5..0.5) * group_size.x,
            gen_range(-0.5..0.5) * group_size.y,
        );
        let isles_nearby = isle_positions.iter().any(|it| {
            let distance = (*it - pos).abs();
            let tolerance = 0.7;
            distance.x < ISLE_SIZE.0 * tolerance && distance.y < ISLE_SIZE.1 * tolerance
        });
        if !isles_nearby {
            isle_positions.push(pos);
            return Some(pos);
        }
    }
    if DEV {
        panic!("failed to find isle position after {} attempts", max_attempts);
    }
    None
}

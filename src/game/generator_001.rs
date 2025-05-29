use std::collections::HashSet;
use std::ops::Not;
use crate::game::{isles, mobs};
use crate::{f32, GameState, Vec2};
use crate::common::contract::{GetMut, Insert};
use crate::common::curve::Curve;
use crate::common::unsorted::gen_range;
use crate::model::def::{IsleSpawn, MobKind, MobRank};
use crate::model::state::{IsleState, MobMission, RelativePos, TransState};

pub fn generate(state: &mut GameState, generator: &LocationGenerator001) {
    for archipelago in &generator.isles {
        let mut archipelago_positions = vec![];
        for _ in 0..archipelago.count {
            let group_pos = generate_archipelago_pos(state, archipelago, &mut archipelago_positions);
            let mut isle_positions = vec![];
            for spawn in &archipelago.isles {
                for _ in 0..spawn.count.random() {
                    let isle_pos = generate_isle_pos(state, archipelago, group_pos, &mut isle_positions);
                    let isle_id = state.isles.insert(IsleState {
                        order: isle_pos.y as i32,
                        def: spawn.isle.clone(),
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

                    for mob_spawn in &spawn.mobs {
                        for _ in 0..mob_spawn.count.random() {
                            let pos = match mob_spawn.mob.kind {
                                MobKind::Walker => RelativePos::Isle(isle_id, Vec2::new(
                                    gen_range(spawn.isle.bounds.clone()),
                                    0.0,
                                )),
                                MobKind::Flyer => RelativePos::Global(isle.trans.pos)
                            };
                            mobs::spawn_mob(
                                &mut state.mobs,
                                &mob_spawn.mob,
                                pos,
                                MobMission::IsleGuard(isle_id),
                                &mut state.gids,
                            );
                            isle.guard_count_threshold += 1;
                            ranks.insert(mob_spawn.mob.rank);
                        }
                    }
                    if ranks.is_empty().not() {
                        isle.guard_rank = Some((ranks.iter().max().unwrap() - ranks.iter().min().unwrap()));
                    }
                }
            }
        }
    }
}

fn generate_archipelago_pos(state: &GameState, archipelago: &ArchipelagoSpawn, archipelagos: &mut Vec<Vec2>) -> Vec2 {
    let max_attempts = 100;
    for _ in 0..max_attempts {
        let pos = Vec2::new(
            gen_range(0.0..state.location.size.x),
            archipelago.height_normal.random() * state.location.size.y,
        );
        let has_clashes = archipelagos.iter()
            .any(|it| {
                let distance = (*it - pos).abs();
                distance.x < archipelago.size.x * 1.5 && distance.y < archipelago.size.y * 1.5
            });
        if !has_clashes {
            archipelagos.push(pos);
            return pos;
        }
    }
    panic!("failed to find archipelago pos after {} attempts", max_attempts)
}

fn generate_isle_pos(state: &GameState, archipelago: &ArchipelagoSpawn, group_pos: Vec2, isle_positions: &mut Vec<Vec2>) -> Vec2 {
    let max_attempts = 100;
    let isle_size = Vec2::new(300.0, 200.0);
    for _ in 0..max_attempts {
        let pos = group_pos + Vec2::new(
            gen_range(-0.5..0.5) * archipelago.size.x,
            gen_range(-0.5..0.5) * archipelago.size.y,
        );
        let isles_nearby = isle_positions.iter().any(|it| {
            let distance = (*it - pos).abs();
            distance.x < isle_size.x && distance.y < isle_size.y
        });
        if !isles_nearby {
            isle_positions.push(pos);
            return pos;
        }
    }
    panic!("failed to find isle position after {} attempts", max_attempts)
}

#[derive(Debug)]
pub struct ArchipelagoSpawn {
    pub isles: Vec<IsleSpawn>,
    pub height_normal: Curve<f32>,
    pub size: Vec2,
    pub count: u32,
}

#[derive(Debug)]
pub struct LocationGenerator001 {
    pub isles: Vec<ArchipelagoSpawn>,
}

use macroquad::prelude::{Vec2, YELLOW};
use rust_macroquad_ui::common::to_vec::ToVec;

use crate::{FutureExt, PlayerState, ResourceGet, ResourceManager};
use crate::common::curve::Curve;
use crate::common::pool::Pool;
use crate::common::resource::ResourceLoad;
use crate::common::resource::Resource;
use crate::common::unsorted::ToColor;
use crate::model::def::{Cannon, CannonPodProps, CollisionCircle, Loot, ProgressPredicate, ProgressPredicateFn, Mob, MobAttack, MobCharge, MobKind, MobLootChance, MobRank, ProgressFlag, GameResource, Burst, Item};
use crate::model::def::GameResource::{A, B, C};
use crate::model::def::MobAttackPattern::{Distant, Melee};
use crate::model::state::{LootId, LootState};
use crate::resources::materials::huer::create_huer_material;
use crate::resources::objects::arms::{cannon_gatling_robot, launcher_jagger_homing, launcher_jagger_homing_fast};
use crate::resources::objects::attacks::{attack_plasma, attack_railgun, attack_railgun2};
use crate::resources::objects::loot::{loot_A, loot_B, loot_C};
use crate::resources::sounds::{sound_death_drone, sound_death_robot, sound_death_wasp, sound_hit_001};
use crate::resources::sprites::mob_drone_001_clip_set::mob_drone_001_clip_set;
use crate::resources::sprites::robot_001_clip_set::robot_001_clip_set;
use crate::resources::sprites::wasp_001_clip_set::wasp_001_clip_set;

const RANK_STYLE: [(Option<&'static str>, MobRank); 3] = [(None, (1)), (Some("#07F"), (2)), (Some("#F77"), (3))];

fn is_present_resource(player: &PlayerState, loot: &Pool<LootId, LootState>, res: GameResource) -> bool {
    player.resources.get(&res).copied().unwrap_or_default() > 0
        ||
        loot.iter()
            .any(|(_, loot)| loot.def.content.iter()
                .any(|it| match it {
                    Item::Resource { resource, count } => { *resource == res && *count > 0 }
                }))
}

const floppy_not_found: ProgressPredicateFn = |game| !is_present_resource(game.player, game.loot, A);
const floppy_found: ProgressPredicateFn = |game| is_present_resource(game.player, game.loot, A);
const disk_not_found: ProgressPredicateFn = |game| !is_present_resource(game.player, game.loot, B);
const disk_found: ProgressPredicateFn = |game| is_present_resource(game.player, game.loot, B);
const sd_not_found: ProgressPredicateFn = |game| !is_present_resource(game.player, game.loot, C);
const sd_found: ProgressPredicateFn = |game| is_present_resource(game.player, game.loot, C);

pub const mob_drone__001: ResourceLoad<Mob> = |rm| drone(&rm, RANK_STYLE[0], 1.0, 10.0, 150, &attack_plasma, vec![
    (floppy_found, 0.05, loot_A.get(&rm)),
]);
pub const mob_drone__002: ResourceLoad<Mob> = |rm| drone(&rm, RANK_STYLE[1], 1.1, 30.0, 2000, &attack_railgun, vec![
    (disk_not_found, 1.0, loot_B.get(&rm)),
    (disk_found, 0.2, loot_B.get(&rm)),
]);
pub const mob_drone__003: ResourceLoad<Mob> = |rm| drone(&rm, RANK_STYLE[2], 1.2, 80.0, 20000, &attack_railgun2, vec![
    (sd_not_found, 1.0, loot_C.get(&rm)),
    (sd_found, 0.1, loot_C.get(&rm)),
]);

pub const mob_wasp___001: ResourceLoad<Mob> = |rm| wasp_(&rm, RANK_STYLE[0], 0.7, 10.0, 1.0, 200, 10.0, vec![
    (floppy_found, 0.1, loot_A.get(&rm)),
]);
pub const mob_wasp___002: ResourceLoad<Mob> = |rm| wasp_(&rm, RANK_STYLE[1], 1.1, 30.0, 1.4, 3000, 20.0, vec![
    (disk_not_found, 1.0, loot_B.get(&rm)),
    (disk_found, 0.2, loot_B.get(&rm)),
]);
pub const mob_wasp___003: ResourceLoad<Mob> = |rm| wasp_(&rm, RANK_STYLE[2], 1.7, 50.0, 2.0, 15000, 30.0, vec![
    (sd_not_found, 1.0, loot_C.get(&rm)),
    (sd_found, 0.1, loot_C.get(&rm)),
]);

pub const mob_jagger_001: ResourceLoad<Mob> = |rm| robot(&rm, RANK_STYLE[0], 1.0, 60.0, 300, 1, &launcher_jagger_homing, vec![
    (floppy_not_found, 1.0, loot_A.get(&rm)),
    (floppy_found, 0.1, loot_A.get(&rm)),
]);
pub const mob_jagger_002: ResourceLoad<Mob> = |rm| robot(&rm, RANK_STYLE[1], 1.1, 80.0, 4000, 7, &cannon_gatling_robot, vec![
    (disk_not_found, 1.0, loot_B.get(&rm)),
    (disk_found, 0.2, loot_B.get(&rm)),
]);
pub const mob_jagger_003: ResourceLoad<Mob> = |rm| robot(&rm, RANK_STYLE[2], 1.2, 120.0, 12000, 3, &launcher_jagger_homing_fast, vec![
    (sd_not_found, 1.0, loot_C.get(&rm)),
    (sd_found, 0.1, loot_C.get(&rm)),
]);

fn robot(rm: &ResourceManager, rank: (Option<&'static str>, MobRank), scale: f32, hp: f32, xp_reward: u32, burst: u16, cannon: &'static ResourceLoad<Cannon>, loot: Vec<(ProgressPredicateFn, f32, Resource<Loot>)>) -> Mob {
    let center = Vec2::new(0.0, -100.0);

    let cannon = cannon.get(&rm);
    Mob {
        rank: rank.1,
        sprite_set: robot_001_clip_set.get(&rm),
        scale,
        move_speed: 120.0,
        move_seconds: Curve::new([0.1, 0.5]),
        idle_seconds: Curve::new([0.1, 1.0]),
        collider_unscaled: CollisionCircle { center, radius: 100.0 },
        hp,
        pod: CannonPodProps { offset: center },
        attacks: vec![
            Resource::detached(MobAttack {
                trigger_range: 150.0,
                hold_sec: 0.1,
                cooldown_sec: Curve::new([1.0]),
                pattern: Melee {
                    connect_range: 150.0,
                    damage: Curve::new([80.0, 120.0]),
                },
                charge: None,
                burst: None,
                late_aim: false,
                hold_effect: None,
            }),
            if burst > 1 {
                Resource::detached(MobAttack {
                    trigger_range: 700.0,
                    hold_sec: 0.0,
                    cooldown_sec: Curve::new([1.5]),
                    pattern: Distant {
                        cannon: cannon.clone(),
                    },
                    charge: None,
                    burst: Some(Resource::detached(Burst { rounds_in_row: burst, cannon })),
                    late_aim: true,
                    hold_effect: None,
                })
            } else {
                Resource::detached(MobAttack {
                    trigger_range: 500.0,
                    hold_sec: 0.1,
                    cooldown_sec: Curve::new([1.0]),
                    pattern: Distant {
                        cannon,
                    },
                    charge: None,
                    burst: None,
                    late_aim: false,
                    hold_effect: None,
                })
            }
            ,
        ],
        flier_aggro_distance: 0.0,
        flier_chase_step: 0.0,
        material: rank.0.map(|color| Resource::detached(create_huer_material(&rm, YELLOW, color.to_color()))),
        loot_chances: convert_loot(loot),
        kind: MobKind::Walker,
        xp_reward,
        death_sound: Some(sound_death_robot.get(&rm)),
        pain_sound: Some(sound_hit_001.get(&rm)),
        burst: Some(burst),
        attack_chance: 1.0,
    }
}

fn convert_loot(loot: Vec<(ProgressPredicateFn, f32, Resource<Loot>)>) -> Vec<MobLootChance> {
    loot.iter()
        .cloned()
        .map(|(rule, probability, loot)| MobLootChance {
            probability,
            loot,
            rule: ProgressPredicate(rule),
        })
        .to_vec()
}

fn drone(rm: &ResourceManager, rank: (Option<&'static str>, MobRank), scale: f32, hp: f32, xp_reward: u32, attack: &'static ResourceLoad<MobAttack>, loot: Vec<(ProgressPredicateFn, f32, Resource<Loot>)>) -> Mob {
    let center = Vec2::new(0.0, -40.0);
    let tint = rank.0;
    Mob {
        rank: rank.1,
        sprite_set: mob_drone_001_clip_set.get(&rm),
        scale,
        move_speed: 120.0,
        move_seconds: Curve::new([0.1, 0.5]),
        idle_seconds: Curve::new([0.6, 1.6]),
        collider_unscaled: CollisionCircle { center, radius: 40.0 },
        hp,
        pod: CannonPodProps {
            offset: (center + Vec2::new(12.0, -17.0)) * scale
        },
        attacks: vec![
            attack.get(&rm),
        ],
        flier_aggro_distance: 0.0,
        flier_chase_step: 0.0,
        material: tint.map(|color| Resource::detached(create_huer_material(&rm, YELLOW, color.to_color()))),
        loot_chances: convert_loot(loot),
        kind: MobKind::Walker,
        xp_reward,
        death_sound: Some(sound_death_drone.get(&rm)),
        pain_sound: Some(sound_hit_001.get(&rm)),
        burst: None,
        attack_chance: 1.0,
    }
}

fn wasp_(rm: &ResourceManager, rank: (Option<&'static str>, MobRank), scale: f32, hp: f32, move_speed: f32, xp_reward: u32, damage: f32, loot: Vec<(ProgressPredicateFn, f32, Resource<Loot>)>) -> Mob {
    let center = Vec2::new(0.0, 0.0);
    Mob {
        rank: rank.1,
        sprite_set: wasp_001_clip_set.get(&rm),
        scale,
        move_speed: 500.0 * move_speed,
        move_seconds: Curve::new([0.1, 0.5]),
        idle_seconds: Curve::new([0.1, 0.3]),
        collider_unscaled: CollisionCircle { center, radius: 40.0 },
        hp,
        pod: CannonPodProps { offset: center },
        attacks: vec![
            Resource::detached(MobAttack {
                trigger_range: 500.0,
                hold_sec: 0.2,
                cooldown_sec: Curve::new([0.1, 0.4]),
                pattern: Melee {
                    connect_range: 50.0,
                    damage: Curve::new([0.5 * damage, damage]),
                },
                charge: Some(MobCharge {
                    velocity: 1000.0 * move_speed,
                    duration_sec: 0.4,
                }),
                burst: None,
                late_aim: true,
                hold_effect: None,
            }),
        ],
        flier_aggro_distance: 1000.0,
        flier_chase_step: 0.0,
        material: rank.0.map(|color| Resource::detached(create_huer_material(&rm, YELLOW, color.to_color()))),
        loot_chances: convert_loot(loot),
        kind: MobKind::Flyer,
        xp_reward,
        death_sound: Some(sound_death_wasp.get(&rm)),
        pain_sound: Some(sound_hit_001.get(&rm)),
        burst: None,
        attack_chance: 0.5,
    }
}

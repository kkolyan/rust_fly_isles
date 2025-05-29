use std::ops::Not;
use macroquad::prelude::Vec2;
use rust_macroquad_ui::common::to_vec::ToVec;
use crate::common::curve::{Curve};
use crate::common::curve::Point::{Transition, Value};
use crate::common::resource::ResourceLoad;
use crate::common::unsorted::ToColor;
use crate::game::generator_002::{Layer, LocationGenerator002, MobConfig};
use crate::model::def::{BackgroundObject, GameResource, JournalEntry, Location, LocationContent, PlaneWeapon, ProgressFlag, ProgressPredicate, ProgressRule, Sky};
use crate::model::state::DeviceOrder;
use crate::ResourceGet;
use crate::resources::constants::{SCALE_SPEED, standard_lava_damage_per_sec_norm};
use crate::resources::materials::fog::fog_material;
use crate::resources::objects::arms::cannon_default;
use crate::resources::objects::isles::{isle_001, isle_slow};
use crate::resources::objects::locations::{DEV_SCALE, location003_objectives, tutorial_objectives};
use crate::resources::objects::mobs::{mob_jagger_001, mob_drone__002, mob_wasp___002, mob_drone__001, mob_wasp___001, mob_jagger_002, mob_drone__003, mob_wasp___003, mob_jagger_003};
use crate::resources::sprites::clouds::{cloud1_sprite, cloud2_sprite, cloud3_sprite};
use crate::resources::sprites::isles::isle1_sprite;

pub const location003: ResourceLoad<Location> = |rm| {
    Location {
        sky: Sky {
            color_by_height: Curve::new_ext(&[
                Value("#000033".to_color()),
                Transition(6),
                Value("#079BFF".to_color()),
                Transition(6),
                Value("#14B2FF".to_color()),
                Value("#FF8800".to_color()),
                Value("#FF3300".to_color()),
                Value("#FF3300".to_color()),
            ]),
        },
        size: Vec2::new(10000.0, 20000.0) * SCALE_SPEED * DEV_SCALE,
        start_pos_norm: Vec2::new(0.10, 0.7),
        background_objects: vec![
            BackgroundObject {
                z: Curve::new_ext(&[
                    Value(1.0),
                    // Value(4.0),
                    // Transition(3),
                    Value(4.0),
                ]),
                size: Curve::new([1.0]),
                height_normal: Some(Curve::new_ext(&[
                    Value(0.3),
                    Value(0.6),
                    Transition(8),
                    Value(0.8),
                ])),
                sprite: vec![
                    cloud1_sprite.get(&rm),
                    cloud2_sprite.get(&rm),
                    cloud3_sprite.get(&rm),
                ],
                count: Curve::new([300]),
                material: None,
            },
            BackgroundObject {
                z: Curve::new([2.0, 4.0]),
                size: Curve::new([0.5]),
                height_normal: Some(Curve::new_ext(&[
                    Value(0.3),
                    Value(0.6),
                    Transition(8),
                    Value(0.8),
                ])),
                sprite: vec![
                    isle1_sprite.get(&rm),
                    // isle2_sprite.asset(&rm),
                    // isle3_sprite.asset(&rm),
                ],
                count: Curve::new([30]),
                material: Some(fog_material.get(&rm)),
            },
        ],
        bots: vec![],
        content: LocationContent::Generator002(LocationGenerator002 {
            layers: vec![
                Layer {
                    name: "sixth biome - gods",
                    mobs: vec![
                        MobConfig { mob: mob_wasp___001.get(&rm), count: 10 },
                        MobConfig { mob: mob_drone__002.get(&rm), count: 12 },
                        MobConfig { mob: mob_wasp___002.get(&rm), count: 8 },
                        MobConfig { mob: mob_jagger_002.get(&rm), count: 8 },
                        MobConfig { mob: mob_drone__003.get(&rm), count: 4 },
                        MobConfig { mob: mob_wasp___003.get(&rm), count: 6 },
                        MobConfig { mob: mob_jagger_003.get(&rm), count: 5 },
                    ],
                    mob_per_isle: Curve::new([3.0, 6.0]),
                    isles_per_archipelago: Curve::new([2.0, 3.0]),
                    isles: vec![isle_slow.get(&rm)],
                    height_normal: Curve::new([0.10, 0.25]),
                    x_normal: Curve::new([0.5, 0.9]),
                    rank_minimap_penalty: 2,
                },
                Layer {
                    name: "fifth biome - super mix",
                    mobs: vec![
                        MobConfig { mob: mob_wasp___001.get(&rm), count: 30 },
                        MobConfig { mob: mob_drone__002.get(&rm), count: 25 },
                        MobConfig { mob: mob_wasp___002.get(&rm), count: 14 },
                        MobConfig { mob: mob_jagger_002.get(&rm), count: 15 },
                        MobConfig { mob: mob_drone__003.get(&rm), count: 4 },
                        MobConfig { mob: mob_wasp___003.get(&rm), count: 6 },
                        MobConfig { mob: mob_jagger_003.get(&rm), count: 5 },
                    ],
                    mob_per_isle: Curve::new([3.0, 6.0]),
                    isles_per_archipelago: Curve::new([2.0, 3.0]),
                    isles: vec![isle_slow.get(&rm)],
                    height_normal: Curve::new([0.30, 0.35]),
                    x_normal: Curve::new([0.1, 0.7]),
                    rank_minimap_penalty: 2,
                },
                Layer {
                    name: "fourth biome - wasps",
                    mobs: vec![
                        MobConfig { mob: mob_drone__001.get(&rm), count: 5 },
                        MobConfig { mob: mob_wasp___001.get(&rm), count: 40 },
                        MobConfig { mob: mob_jagger_001.get(&rm), count: 5 },
                        MobConfig { mob: mob_drone__002.get(&rm), count: 5 },
                        MobConfig { mob: mob_wasp___002.get(&rm), count: 20 },
                    ],
                    mob_per_isle: Curve::new([1.0, 6.0]),
                    isles_per_archipelago: Curve::new([1.0, 3.0]),
                    isles: vec![isle_slow.get(&rm)],
                    height_normal: Curve::new([0.38, 0.42]),
                    x_normal: Curve::new([0.0, 1.0]),
                    rank_minimap_penalty: 1,
                },
                Layer {
                    name: "third biome - just tough mix. no unique boss",
                    mobs: vec![
                        MobConfig { mob: mob_drone__001.get(&rm), count: 31 },
                        MobConfig { mob: mob_jagger_001.get(&rm), count: 8 },
                        MobConfig { mob: mob_wasp___001.get(&rm), count: 12 },
                        MobConfig { mob: mob_jagger_002.get(&rm), count: 3 },
                        MobConfig { mob: mob_drone__002.get(&rm), count: 3 },
                    ],
                    mob_per_isle: Curve::new([2.0, 3.0]),
                    isles_per_archipelago: Curve::new([2.0, 4.0]),
                    isles: vec![isle_slow.get(&rm)],
                    height_normal: Curve::new([0.53, 0.57]),
                    x_normal: Curve::new([0.5, 0.9]),
                    rank_minimap_penalty: 1,
                },
                Layer {
                    name: "second biome boss",
                    mobs: vec![
                        MobConfig { mob: mob_drone__001.get(&rm), count: 6 },
                        // MobConfig { mob: mob_drone__002.get(&rm), count: 1 },
                        MobConfig { mob: mob_jagger_002.get(&rm), count: 2 },
                    ],
                    mob_per_isle: Curve::new([3.0, 3.0]),
                    isles_per_archipelago: Curve::new([5.0]),
                    isles: vec![isle_slow.get(&rm)],
                    height_normal: Curve::new([0.6, 0.62]),
                    x_normal: Curve::new([0.85, 0.88]),
                    rank_minimap_penalty: 0,
                },
                Layer {
                    name: "second biome",
                    mobs: vec![
                        MobConfig { mob: mob_drone__001.get(&rm), count: 60 },
                        MobConfig { mob: mob_jagger_001.get(&rm), count: 20 },
                        MobConfig { mob: mob_wasp___001.get(&rm), count: 8 },
                    ],
                    mob_per_isle: Curve::new([2.0, 3.0]),
                    isles_per_archipelago: Curve::new([2.0, 4.0]),
                    isles: vec![isle_slow.get(&rm)],
                    height_normal: Curve::new([0.6, 0.8]),
                    x_normal: Curve::new([0.65, 0.95]),
                    rank_minimap_penalty: 0,
                },
                Layer {
                    name: "default biome boss",
                    mobs: vec![
                        MobConfig { mob: mob_drone__001.get(&rm), count: 14 },
                        MobConfig { mob: mob_jagger_001.get(&rm), count: 1 },
                    ],
                    mob_per_isle: Curve::new([2.0, 3.0]),
                    isles_per_archipelago: Curve::new([7.0]),
                    isles: vec![isle_slow.get(&rm)],
                    height_normal: Curve::new([0.67]),
                    x_normal: Curve::new([0.35]),
                    rank_minimap_penalty: 0,
                },
                Layer {
                    name: "default biome - to get used with controls",
                    mobs: vec![
                        MobConfig { mob: mob_drone__001.get(&rm), count: 50 },
                    ],
                    mob_per_isle: Curve::new([2.0, 4.0]),
                    isles_per_archipelago: Curve::new([1.0, 1.0, 1.0, 1.0, 2.0, 2.0, 3.0]),
                    isles: vec![isle_slow.get(&rm)],
                    height_normal: Curve::new([0.67, 0.8]),
                    x_normal: Curve::new([0.05, 0.4]),
                    rank_minimap_penalty: 0,
                },
                Layer {
                    name: "bottom - a little bit of easy enemies just for a fill",
                    mobs: vec![
                        MobConfig { mob: mob_drone__001.get(&rm), count: 30 },
                    ],
                    mob_per_isle: Curve::new([2.0, 4.0]),
                    isles_per_archipelago: Curve::new([1.0, 1.0]),
                    isles: vec![isle_slow.get(&rm)],
                    height_normal: Curve::new([0.8, 0.9]),
                    x_normal: Curve::new([0.0, 1.0]),
                    rank_minimap_penalty: 0,
                },
            ]
        }),
        lava_damage_by_height_per_sec_norm: Some(standard_lava_damage_per_sec_norm()),
        progression: [
            tutorial_objectives::objectives(),
            location003_objectives::objectives(),
        ].into_iter().flatten().to_vec(),
        journal: vec![],
        default_weapon: Some(PlaneWeapon { spec: cannon_default.get(&rm), energy_per_shot: 1.5, order: DeviceOrder(-1) }),
    }
};
use std::rc::Rc;
use macroquad::prelude::Rect;
use crate::common::curve::{Curve};
use crate::common::curve::Point::{Transition, Value};
use crate::common::resource::{ResourceGet, ResourceLoad, ResourceManagerRc};
use crate::common::unsorted::ToColor;
use crate::model::def::{BackgroundObject, Bot, Isle, IsleSpawn, Location, LocationContent, Mob, MobSpawn, Sky, Sprite};
use crate::resources::constants::{SCALE_SPEED, standard_lava_damage_per_sec_norm};
use crate::resources::materials::fog::fog_material;
use crate::resources::objects::objects::plane001;
use crate::{FutureExt, ResourceManager, Vec2};
use crate::common::resource::Resource;
use crate::game::generator_001::{ArchipelagoSpawn, LocationGenerator001};
use crate::resources::objects::isles::{isle_001, isle_slow};
use crate::resources::objects::locations;
use crate::resources::objects::locations::DEV_SCALE;
use crate::resources::objects::mobs::{mob_jagger_001, mob_jagger_002, mob_jagger_003, mob_drone__001, mob_drone__002, mob_drone__003, mob_wasp___001, mob_wasp___002, mob_wasp___003};
use crate::resources::sprites::clouds::{cloud1_sprite, cloud2_sprite, cloud3_sprite};
use crate::resources::sprites::isles::isle1_sprite;
use crate::resources::sprites::robot_001_clip_set::robot_001_clip_set;

pub const location001: ResourceLoad<Location> = |rm| {
    let bot_pane = plane001.get(&rm);
    let bots = (0..locations::dev_scale(500))
        .map(|_| {
            Bot {
                plane: bot_pane.clone(),
                height_normalized: Curve::new([0.2, 0.8]),
            }
        })
        .collect();

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
        size: Vec2::new(10000.0, 30000.0) * SCALE_SPEED * DEV_SCALE,
        start_pos_norm: Vec2::new(0.5, 0.65),
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
                count: Curve::new([1000]),
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
                count: Curve::new([80]),
                material: Some(fog_material.get(&rm)),
            },
        ],
        bots,
        content: LocationContent::Generator001(LocationGenerator001 {
            isles: vec![
                ArchipelagoSpawn {
                    height_normal: Curve::new_ext(&[
                        Value(0.3),
                        Value(0.6),
                        Transition(8),
                        Value(0.8),
                    ]),
                    count: locations::dev_scale_usize(120),
                    isles: vec![
                        IsleSpawn {
                            isle: isle_001.get(&rm),
                            mobs: vec![
                                MobSpawn { mob: mob_jagger_001.get(&rm), count: Curve::new([0, 0, 0, 0, 0, 0, 1]) },
                                MobSpawn { mob: mob_jagger_002.get(&rm), count: Curve::new([0, 0, 0, 0, 0, 0, 1]) },
                                MobSpawn { mob: mob_jagger_003.get(&rm), count: Curve::new([0, 0, 0, 0, 0, 0, 1]) },
                                MobSpawn { mob: mob_drone__001.get(&rm), count: Curve::new([0, 1]) },
                                MobSpawn { mob: mob_drone__002.get(&rm), count: Curve::new([0, 1]) },
                                MobSpawn { mob: mob_drone__003.get(&rm), count: Curve::new([0, 1]) },
                                MobSpawn { mob: mob_wasp___001.get(&rm), count: Curve::new([0, 0, 0, 0, 1]) },
                                MobSpawn { mob: mob_wasp___002.get(&rm), count: Curve::new([0, 0, 0, 0, 1]) },
                                MobSpawn { mob: mob_wasp___003.get(&rm), count: Curve::new([0, 0, 0, 0, 1]) },
                            ],
                            count: Curve::new([1]),
                        }
                    ],
                    size: Vec2::ZERO,
                }
            ],
        }),
        lava_damage_by_height_per_sec_norm: Some(standard_lava_damage_per_sec_norm()),
        progression: vec![],
        journal: vec![],
        default_weapon: None
    }
};

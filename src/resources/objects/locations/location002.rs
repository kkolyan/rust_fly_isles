use crate::common::curve::{Curve};
use crate::common::curve::Point::{Transition, Value};
use crate::common::resource::ResourceLoad;
use crate::common::unsorted::ToColor;
use crate::game::generator_001::{ArchipelagoSpawn, LocationGenerator001};
use crate::model::def::{BackgroundObject, IsleSpawn, Location, LocationContent, Mob, MobSpawn, Sky};
use crate::resources::constants::{SCALE_SPEED, standard_lava_damage_per_sec_norm};
use crate::resources::materials::fog::fog_material;
use crate::resources::objects::locations::location001;
use crate::resources::objects::locations::DEV_SCALE;
use crate::resources::objects::mobs::{mob_jagger_001, mob_jagger_002, mob_jagger_003, mob_drone__001, mob_drone__002, mob_drone__003};
use crate::resources::sprites::clouds::{cloud1_sprite, cloud2_sprite, cloud3_sprite};
use crate::resources::sprites::isles::isle1_sprite;
use crate::{ResourceGet, ResourceManager, Vec2};
use crate::resources::objects::isles::isle_slow;
use crate::resources::objects::locations;

pub const location002: ResourceLoad<Location> = |rm| {
    let height_normal = Curve::new_ext(&[
        Value(0.3),
        Value(0.6),
        Transition(8),
        Value(0.8),
    ]);
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
        size: Vec2::new(30000.0, 30000.0) * SCALE_SPEED * DEV_SCALE,
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
                height_normal: Some(height_normal.clone()),
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
                height_normal: Some(height_normal.clone()),
                sprite: vec![
                    isle1_sprite.get(&rm),
                    // isle2_sprite.asset(&rm),
                    // isle3_sprite.asset(&rm),
                ],
                count: Curve::new([80]),
                material: Some(fog_material.get(&rm)),
            },
        ],
        bots: vec![],
        content: LocationContent::Generator001(LocationGenerator001 {
            isles: vec![
                ArchipelagoSpawn {
                    height_normal: height_normal.clone(),
                    isles: vec![
                        isle(&rm, vec![(mob_drone__001, &[0, 2]), (mob_drone__002, &[0, 0, 1])], &[3, 4]),
                        isle(&rm, vec![(mob_drone__001, &[1, 2]), (mob_jagger_001, &[1])], &[1]),
                    ],
                    size: Vec2::new(1200.0, 800.0),
                    count: locations::dev_scale_usize(20),
                },
                ArchipelagoSpawn {
                    height_normal: height_normal.clone(),
                    isles: vec![
                        isle(&rm, vec![(mob_drone__001, &[1, 2])], &[3, 6]),
                        isle(&rm, vec![(mob_drone__001, &[0, 2]), (mob_drone__002, &[0, 0, 1])], &[3, 4]),
                        isle(&rm, vec![(mob_drone__001, &[1, 2]), (mob_drone__002, &[0, 0, 1]), (mob_jagger_001, &[1])], &[2, 3]),
                        isle(&rm, vec![(mob_drone__001, &[1, 2]), (mob_jagger_002, &[1])], &[1]),
                    ],
                    size: Vec2::new(3800.0, 2800.0),
                    count: locations::dev_scale_usize(10),
                },
                ArchipelagoSpawn {
                    height_normal: height_normal.clone(),
                    isles: vec![
                        isle(&rm, vec![(mob_drone__001, &[1, 2])], &[3, 6]),
                        isle(&rm, vec![(mob_drone__001, &[0, 2]), (mob_drone__002, &[0, 0, 1])], &[3, 4]),
                        isle(&rm, vec![(mob_drone__001, &[1, 2]), (mob_drone__002, &[0, 0, 1]), (mob_jagger_001, &[1])], &[2, 3]),
                        isle(&rm, vec![(mob_drone__001, &[1, 2]), (mob_drone__002, &[0, 1]), (mob_drone__003, &[0, 1])], &[1]),
                    ],
                    size: Vec2::new(4000.0, 3800.0),
                    count: locations::dev_scale_usize(10),
                },
                ArchipelagoSpawn {
                    height_normal: height_normal.clone(),
                    isles: vec![
                        isle(&rm, vec![(mob_drone__001, &[1, 2])], &[3, 6]),
                        isle(&rm, vec![(mob_drone__001, &[1, 2]), (mob_drone__002, &[1])], &[2, 3]),
                        isle(&rm, vec![(mob_drone__001, &[1, 2]), (mob_jagger_002, &[1])], &[2, 3]),
                        isle(&rm, vec![(mob_drone__002, &[1, 2]), (mob_jagger_003, &[1])], &[1]),
                    ],
                    size: Vec2::new(8200.0, 5200.0),
                    count: locations::dev_scale_usize(3),
                },
            ],
        }),
        lava_damage_by_height_per_sec_norm: Some(standard_lava_damage_per_sec_norm()),
        progression: vec![],
        journal: vec![],
        default_weapon: None
    }
};

pub fn isle(rm: &ResourceManager, mobs: Vec<(ResourceLoad<Mob>, &[u32])>, count: &[u32]) -> IsleSpawn {
    IsleSpawn {
        isle: isle_slow.get(rm),
        mobs: mobs.iter()
            .map(|(mob, count)| MobSpawn { mob: mob.get(rm), count: Curve::Manual { points: Vec::from(*count) } })
            .collect(),
        count: Curve::Manual { points: Vec::from(count) },
    }
}

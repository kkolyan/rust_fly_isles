use macroquad::prelude::Vec2;
use crate::common::curve::{Curve};
use crate::common::curve::Point::{Transition, Value};
use crate::common::resource::ResourceLoad;
use crate::common::unsorted::ToColor;
use crate::game::generator_002::{Layer, LocationGenerator002, MobConfig};
use crate::model::def::{BackgroundObject, Location, LocationContent, ProgressPredicate, ProgressRule, Sky};
use crate::model::def::ProgressFlag::BriefingShown;
use crate::ResourceGet;
use crate::resources::constants::{SCALE_SPEED, standard_lava_damage_per_sec_norm};
use crate::resources::materials::fog::fog_material;
use crate::resources::objects::isles::{isle_001, isle_slow};
use crate::resources::objects::locations::DEV_SCALE;
use crate::resources::objects::mobs::{mob_jagger_001, mob_drone__002, mob_wasp___002, mob_drone__001, mob_wasp___001, mob_jagger_002, mob_drone__003, mob_wasp___003, mob_jagger_003};
use crate::resources::sprites::clouds::{cloud1_sprite, cloud2_sprite, cloud3_sprite};
use crate::resources::sprites::isles::isle1_sprite;

pub const location003_training: ResourceLoad<Location> = |rm| {
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
            layers: vec![]
        }),
        lava_damage_by_height_per_sec_norm: Some(standard_lava_damage_per_sec_norm()),
        progression: vec![
            ProgressRule {
                objective: None,
                journal_entry: Some(vec![
                    "Hey stalker. This area is safe as mother's embrace. You can",
                    "try controlling your vehicle here.",
                    "Use your right hand manipulator for steering and wheel for",
                    "switching gears - that's easy, just don't throw the clutch.",
                    "Gray circle in front of you is for safety - to prevent",
                    "oversteering after you read journals.",
                    "Catch the air flow by the wings or you'll end",
                    "up at the chilling bottom of the Arctic Ocean. Train to fly",
                    "with engine off - it willp hel a lot in combat",
                    "And don't spend to much time here - this place is boring."
                ]),
                display_condition: ProgressPredicate(|game| !game.progression.flags.contains(&BriefingShown)),
                complete_condition: ProgressPredicate(|game| true),
                output_flags: vec![BriefingShown]
            },
            ProgressRule {
                objective: Some("- get used with a plane controls and proceed to the combat mission"),
                journal_entry: None,
                display_condition: ProgressPredicate(|game| true),
                complete_condition: ProgressPredicate(|game| false),
                output_flags: vec![]
            },
        ],
        journal: vec![],
        default_weapon: None
    }
};
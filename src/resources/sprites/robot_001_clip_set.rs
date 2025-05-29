use macroquad::prelude::Vec2;
use crate::common::resource::{ResourceLoad, ResourceLoadAsync};
use crate::lifecycle::loading;
use crate::model::data::{GenericSplitSpriteClipData, MobSpriteSetData};
use crate::model::data::SpriteMod::{ OriginNorm, Scale};
use crate::model::def::{CollisionCircle, CollisionCircleNorm, MobAnimation, MobSpriteSet};
use crate::model::def::OnClipEnd::{Clamp, Repeat};

pub const robot_001_clip_set: ResourceLoadAsync<MobSpriteSet> = |rm| loading::mob_clips(rm, MobSpriteSetData {
    sprite_mod: vec![
        Scale(0.4),
        OriginNorm(Vec2::new(0.4, 1.0 - 0.125)),
    ],
    clips: |anim| match anim {
        MobAnimation::Idle => GenericSplitSpriteClipData::MultiFile {
            sprite_mod: vec![],
            rate: 12.0,
            data: vec![
                include_bytes!("../../../art/pzuh/Steampunk/Robot/Idle (1).png"),
                include_bytes!("../../../art/pzuh/Steampunk/Robot/Idle (2).png"),
                include_bytes!("../../../art/pzuh/Steampunk/Robot/Idle (3).png"),
                include_bytes!("../../../art/pzuh/Steampunk/Robot/Idle (4).png"),
                include_bytes!("../../../art/pzuh/Steampunk/Robot/Idle (5).png"),
                include_bytes!("../../../art/pzuh/Steampunk/Robot/Idle (6).png"),
                include_bytes!("../../../art/pzuh/Steampunk/Robot/Idle (7).png"),
                include_bytes!("../../../art/pzuh/Steampunk/Robot/Idle (8).png"),
                include_bytes!("../../../art/pzuh/Steampunk/Robot/Idle (9).png"),
                include_bytes!("../../../art/pzuh/Steampunk/Robot/Idle (10).png"),
            ],
            on_end: Repeat,
        },
        MobAnimation::Move => GenericSplitSpriteClipData::MultiFile {
            sprite_mod: vec![],
            rate: 30.0,
            data: vec![
                include_bytes!("../../../art/pzuh/Steampunk/Robot/Walk (1).png"),
                include_bytes!("../../../art/pzuh/Steampunk/Robot/Walk (2).png"),
                include_bytes!("../../../art/pzuh/Steampunk/Robot/Walk (3).png"),
                include_bytes!("../../../art/pzuh/Steampunk/Robot/Walk (4).png"),
                include_bytes!("../../../art/pzuh/Steampunk/Robot/Walk (5).png"),
                include_bytes!("../../../art/pzuh/Steampunk/Robot/Walk (6).png"),
                include_bytes!("../../../art/pzuh/Steampunk/Robot/Walk (7).png"),
                include_bytes!("../../../art/pzuh/Steampunk/Robot/Walk (8).png"),
                include_bytes!("../../../art/pzuh/Steampunk/Robot/Walk (9).png"),
                include_bytes!("../../../art/pzuh/Steampunk/Robot/Walk (10).png"),
                include_bytes!("../../../art/pzuh/Steampunk/Robot/Walk (11).png"),
                include_bytes!("../../../art/pzuh/Steampunk/Robot/Walk (12).png"),
                include_bytes!("../../../art/pzuh/Steampunk/Robot/Walk (13).png"),
                include_bytes!("../../../art/pzuh/Steampunk/Robot/Walk (14).png"),
                include_bytes!("../../../art/pzuh/Steampunk/Robot/Walk (15).png"),
            ],
            on_end: Repeat,
        },
        MobAnimation::Die => GenericSplitSpriteClipData::MultiFile {
            sprite_mod: vec![],
            rate: 15.0,
            data: vec![
                include_bytes!("../../../art/pzuh/Steampunk/Robot/Dead (1).png"),
                include_bytes!("../../../art/pzuh/Steampunk/Robot/Dead (2).png"),
                include_bytes!("../../../art/pzuh/Steampunk/Robot/Dead (3).png"),
                include_bytes!("../../../art/pzuh/Steampunk/Robot/Dead (4).png"),
                include_bytes!("../../../art/pzuh/Steampunk/Robot/Dead (5).png"),
                include_bytes!("../../../art/pzuh/Steampunk/Robot/Dead (6).png"),
                include_bytes!("../../../art/pzuh/Steampunk/Robot/Dead (7).png"),
                include_bytes!("../../../art/pzuh/Steampunk/Robot/Dead (8).png"),
            ],
            on_end: Clamp,
        },
        MobAnimation::Dead => GenericSplitSpriteClipData::MultiFile {
            sprite_mod: vec![],
            rate: 0.0,
            data: vec![
                include_bytes!("../../../art/pzuh/Steampunk/Robot/Dead (8).png"),
            ],
            on_end: Repeat,
        },
        MobAnimation::AttackWindup => GenericSplitSpriteClipData::MultiFile {
            rate: 30.0,
            sprite_mod: vec![],
            data: vec![
                include_bytes!("../../../art/pzuh/Steampunk/Robot/Attack (1).png"),
                include_bytes!("../../../art/pzuh/Steampunk/Robot/Attack (2).png"),
                include_bytes!("../../../art/pzuh/Steampunk/Robot/Attack (3).png"),
                include_bytes!("../../../art/pzuh/Steampunk/Robot/Attack (4).png"),
            ],
            on_end: Clamp,
        },
        MobAnimation::AttackHold => GenericSplitSpriteClipData::MultiFile {
            rate: 30.0,
            sprite_mod: vec![],
            data: vec![
                include_bytes!("../../../art/pzuh/Steampunk/Robot/Attack (5).png"),
            ],
            on_end: Clamp,
        },
        MobAnimation::AttackDeliver => GenericSplitSpriteClipData::MultiFile {
            rate: 30.0,
            sprite_mod: vec![],
            data: vec![
                include_bytes!("../../../art/pzuh/Steampunk/Robot/Attack (6).png"),
                include_bytes!("../../../art/pzuh/Steampunk/Robot/Attack (7).png"),
                include_bytes!("../../../art/pzuh/Steampunk/Robot/Attack (8).png"),
            ],
            on_end: Clamp,
        },
        MobAnimation::Burst => GenericSplitSpriteClipData::MultiFile {
            rate: 30.0,
            sprite_mod: vec![],
            data: vec![
                include_bytes!("../../../art/pzuh/Steampunk/Robot/Attack (7).png"),
            ],
            on_end: Repeat,
        },
        MobAnimation::AttackFinish => GenericSplitSpriteClipData::MultiFile {
            rate: 30.0,
            sprite_mod: vec![],
            data: vec![
                include_bytes!("../../../art/pzuh/Steampunk/Robot/Attack (9).png"),
                include_bytes!("../../../art/pzuh/Steampunk/Robot/Attack (10).png"),
                include_bytes!("../../../art/pzuh/Steampunk/Robot/Attack (11).png"),
                include_bytes!("../../../art/pzuh/Steampunk/Robot/Attack (12).png"),
                include_bytes!("../../../art/pzuh/Steampunk/Robot/Attack (13).png"),
                include_bytes!("../../../art/pzuh/Steampunk/Robot/Attack (14).png"),
                include_bytes!("../../../art/pzuh/Steampunk/Robot/Attack (15).png"),
            ],
            on_end: Clamp,
        },
    },
});

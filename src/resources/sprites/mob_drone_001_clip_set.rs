use macroquad::prelude::Vec2;
use crate::common::resource::{ResourceLoad, ResourceLoadAsync};
use crate::lifecycle::loading;
use crate::model::data::{GenericSplitSpriteClipData, MobSpriteSetData};
use crate::model::data::SpriteMod::{OriginNorm, Scale};
use crate::model::def::{MobAnimation, MobSpriteSet};
use crate::model::def::OnClipEnd::{Clamp, Repeat};

pub const mob_drone_001_clip_set: ResourceLoadAsync<MobSpriteSet> = |rm| loading::mob_clips(rm, MobSpriteSetData {
    sprite_mod: vec![
        Scale(0.4),
        OriginNorm(Vec2::new(0.5, 1.0 - 0.125)),
    ],
    clips: |anim| match anim {
        MobAnimation::Idle => GenericSplitSpriteClipData::MultiFile {
            sprite_mod: vec![],
            rate: 12.0,
            data: vec![
                include_bytes!("../../../art/pzuh/Steampunk/Drone2/Idle (1).png"),
                include_bytes!("../../../art/pzuh/Steampunk/Drone2/Idle (2).png"),
                include_bytes!("../../../art/pzuh/Steampunk/Drone2/Idle (3).png"),
                include_bytes!("../../../art/pzuh/Steampunk/Drone2/Idle (4).png"),
                include_bytes!("../../../art/pzuh/Steampunk/Drone2/Idle (5).png"),
                include_bytes!("../../../art/pzuh/Steampunk/Drone2/Idle (6).png"),
                include_bytes!("../../../art/pzuh/Steampunk/Drone2/Idle (7).png"),
                include_bytes!("../../../art/pzuh/Steampunk/Drone2/Idle (8).png"),
                include_bytes!("../../../art/pzuh/Steampunk/Drone2/Idle (9).png"),
                include_bytes!("../../../art/pzuh/Steampunk/Drone2/Idle (10).png"),
            ],
            on_end: Repeat,
        },
        MobAnimation::Move => GenericSplitSpriteClipData::MultiFile {
            sprite_mod: vec![],
            rate: 30.0,
            data: vec![
                include_bytes!("../../../art/pzuh/Steampunk/Drone2/Walk (1).png"),
                include_bytes!("../../../art/pzuh/Steampunk/Drone2/Walk (2).png"),
                include_bytes!("../../../art/pzuh/Steampunk/Drone2/Walk (3).png"),
                include_bytes!("../../../art/pzuh/Steampunk/Drone2/Walk (4).png"),
                include_bytes!("../../../art/pzuh/Steampunk/Drone2/Walk (5).png"),
                include_bytes!("../../../art/pzuh/Steampunk/Drone2/Walk (6).png"),
                include_bytes!("../../../art/pzuh/Steampunk/Drone2/Walk (7).png"),
                include_bytes!("../../../art/pzuh/Steampunk/Drone2/Walk (8).png"),
                include_bytes!("../../../art/pzuh/Steampunk/Drone2/Walk (9).png"),
                include_bytes!("../../../art/pzuh/Steampunk/Drone2/Walk (10).png"),
            ],
            on_end: Repeat,
        },
        MobAnimation::Die => GenericSplitSpriteClipData::MultiFile {
            sprite_mod: vec![],
            rate: 15.0,
            data: vec![
                include_bytes!("../../../art/pzuh/Steampunk/Drone2/Dead (1).png"),
                include_bytes!("../../../art/pzuh/Steampunk/Drone2/Dead (2).png"),
                include_bytes!("../../../art/pzuh/Steampunk/Drone2/Dead (3).png"),
                include_bytes!("../../../art/pzuh/Steampunk/Drone2/Dead (4).png"),
                include_bytes!("../../../art/pzuh/Steampunk/Drone2/Dead (5).png"),
                include_bytes!("../../../art/pzuh/Steampunk/Drone2/Dead (6).png"),
                include_bytes!("../../../art/pzuh/Steampunk/Drone2/Dead (7).png"),
                include_bytes!("../../../art/pzuh/Steampunk/Drone2/Dead (8).png"),
                include_bytes!("../../../art/pzuh/Steampunk/Drone2/Dead (9).png"),
                include_bytes!("../../../art/pzuh/Steampunk/Drone2/Dead (10).png"),
            ],
            on_end: Clamp,
        },
        MobAnimation::Dead => GenericSplitSpriteClipData::MultiFile {
            sprite_mod: vec![],
            rate: 0.0,
            data: vec![
                include_bytes!("../../../art/pzuh/Steampunk/Drone2/Dead (10).png"),
            ],
            on_end: Repeat,
        },
        MobAnimation::AttackWindup => GenericSplitSpriteClipData::MultiFile {
            rate: 30.0,
            sprite_mod: vec![],
            data: vec![
                include_bytes!("../../../art/pzuh/Steampunk/Drone2/Idle (1).png"),
            ],
            on_end: Clamp,
        },
        MobAnimation::AttackHold => GenericSplitSpriteClipData::MultiFile {
            rate: 30.0,
            sprite_mod: vec![],
            data: vec![
                include_bytes!("../../../art/pzuh/Steampunk/Drone2/Idle (1).png"),
            ],
            on_end: Clamp,
        },
        MobAnimation::AttackDeliver => GenericSplitSpriteClipData::MultiFile {
            rate: 30.0,
            sprite_mod: vec![],
            data: vec![],
            on_end: Clamp,
        },
        MobAnimation::Burst =>  GenericSplitSpriteClipData::MultiFile {
            rate: 30.0,
            sprite_mod: vec![],
            data: vec![],
            on_end: Clamp,
        },
        MobAnimation::AttackFinish => GenericSplitSpriteClipData::MultiFile {
            rate: 30.0,
            sprite_mod: vec![],
            data: vec![],
            on_end: Clamp,
        },
    },
});

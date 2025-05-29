use macroquad::prelude::Vec2;
use crate::common::resource::{ResourceLoad, ResourceLoadAsync};
use crate::common::sprite::SheetRegion;
use crate::lifecycle::loading;
use crate::model::data::{GenericSplitSpriteClipData, MobSpriteSetData};
use crate::model::data::SpriteMod::{OriginNorm, Scale};
use crate::model::def::{CollisionCircle, CollisionCircleNorm, MobAnimation, MobSpriteSet};
use crate::model::def::OnClipEnd::{Clamp, Repeat};

fn wasp_flying_bytes() -> &'static [u8] { include_bytes!("../../../art/gamedeveloperstudio/__wasp-flying_fly_776px_by_867px_per_frame.png") }
fn wasp_sting_bytes() ->  &'static [u8] { include_bytes!("../../../art/gamedeveloperstudio/__wasp-flying_sting_776px_by_867px_per_frame.png") }
fn wasp_corpse_bytes() -> &'static [u8] { include_bytes!("../../../art/gamedeveloperstudio/__wasp-flying_corpse.png") }

pub const wasp_001_clip_set: ResourceLoadAsync<MobSpriteSet> = |rm| loading::mob_clips(rm, MobSpriteSetData {
    sprite_mod: vec![
        Scale(0.4),
        OriginNorm(Vec2::new(0.5, 0.5)),
    ],
    clips: |anim| match anim {
        MobAnimation::Idle => GenericSplitSpriteClipData::Sheet {
            sprite_mod: vec![],
            rate: 12.0,
            data: wasp_flying_bytes(),
            sheet_size: (4, 4),
            on_end: Repeat,
            region: SheetRegion::All,
        },
        MobAnimation::Move => GenericSplitSpriteClipData::Sheet {
            sprite_mod: vec![],
            rate: 30.0,
            data: wasp_flying_bytes(),
            sheet_size: (4, 4),
            on_end: Repeat,
            region: SheetRegion::All,
        },
        MobAnimation::Die => GenericSplitSpriteClipData::Sheet {
            sprite_mod: vec![],
            rate: 15.0,
            data: wasp_corpse_bytes(),
            sheet_size: (1, 1),
            on_end: Clamp,
            region: SheetRegion::All,
        },
        MobAnimation::Dead => GenericSplitSpriteClipData::Sheet {
            sprite_mod: vec![],
            rate: 0.0,
            data: wasp_corpse_bytes(),
            sheet_size: (1, 1),
            on_end: Repeat,
            region: SheetRegion::All,
        },
        MobAnimation::AttackWindup => GenericSplitSpriteClipData::Sheet {
            rate: 30.0,
            sprite_mod: vec![],
            data: wasp_flying_bytes(),
            sheet_size: (4, 4),
            on_end: Clamp,
            region: SheetRegion::Cells(vec![
                (3, 0), (2, 0), (1, 0), (0, 0),
                (3, 1), (2, 1), (1, 1), (0, 1),
                (1, 2), (0, 2),
            ]),
        },
        MobAnimation::AttackHold => GenericSplitSpriteClipData::Sheet {
            rate: 30.0,
            sprite_mod: vec![],
            data: wasp_flying_bytes(),
            sheet_size: (4, 4),
            on_end: Clamp,
            region: SheetRegion::Cells(vec![
                (1, 2),
            ]),
        },
        MobAnimation::AttackDeliver => GenericSplitSpriteClipData::Sheet {
            rate: 30.0,
            sprite_mod: vec![],
            data: wasp_flying_bytes(),
            sheet_size: (4, 4),
            on_end: Clamp,
            region: SheetRegion::Cells(vec![
                (1, 2),
            ]),
        },
        MobAnimation::Burst => GenericSplitSpriteClipData::Sheet {
            rate: 30.0,
            sprite_mod: vec![],
            data: wasp_sting_bytes(),
            sheet_size: (4, 4),
            on_end: Clamp,
            region: SheetRegion::Cells(vec![
                (1, 2),
            ]),
        },
        MobAnimation::AttackFinish => GenericSplitSpriteClipData::Sheet {
            rate: 30.0,
            sprite_mod: vec![],
            data: wasp_sting_bytes(),
            sheet_size: (4, 4),
            on_end: Clamp,
            region: SheetRegion::Cells(vec![
                (1, 2),
                (3, 3), (2, 3), (1, 3), (0, 3),
            ]),
        },
    },
});

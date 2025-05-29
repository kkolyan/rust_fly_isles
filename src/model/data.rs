use std::fmt::{Debug, Formatter};
use macroquad::prelude::Vec2;
use crate::common::angle::Angle;
use crate::common::curve::Curve;
use crate::common::sprite::SheetRegion;
use crate::model::def::{CollisionCircle, CollisionCircleNorm, MobAnimation, OnClipEnd, Sprite};

#[derive(Debug, Clone)]
pub struct SplitSpriteClipData {
    pub data: GenericSplitSpriteClipData,
    pub scale: f32,
    pub alpha: Option<Curve<f32>>,
}

#[derive(Debug, Clone)]
pub enum GenericSplitSpriteClipData {
    MultiFile {
        rate: f32,
        sprite_mod: Vec<SpriteMod>,
        data: Vec<&'static [u8]>,
        on_end: OnClipEnd,
    },
    Sheet {
        rate: f32,
        sprite_mod: Vec<SpriteMod>,
        data: &'static [u8],
        sheet_size: (usize, usize),
        on_end: OnClipEnd,
        region: SheetRegion,
    },
}

pub struct GenericMultiFileSpriteClipData {}

#[derive(Debug, Clone)]
pub struct MobSpriteSetData {
    pub sprite_mod: Vec<SpriteMod>,
    pub clips: fn(MobAnimation) -> GenericSplitSpriteClipData,
}

#[derive(Debug, Clone)]
pub enum SpriteMod {
    Scale(f32),

    Rot(Angle),

    OriginNorm(Vec2),
}

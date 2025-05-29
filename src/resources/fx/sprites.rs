use std::f32::consts::PI;
use std::ops::Mul;
use crate::common::curve::{Curve, Point};
use crate::common::curve::Point::{Transition, Value};
use crate::common::resource::ResourceLoad;
use crate::common::sprite::{load_sprites_from_sheet, SheetRegion};
use crate::common::unsorted::ToColor;
use crate::FutureExt;
use crate::model::def::OnClipEnd::Repeat;
use crate::model::def::SpriteClip;
use crate::model::def::SpriteClipMod::{ColorMult, Scale};

pub const explosion3_clip: ResourceLoad<SpriteClip> = |rm| SpriteClip {
    frames: load_sprites_from_sheet(
        todo!("../../art/sinestesia-2d-explosions-animations/explosion 3.png"),
        (8, 8),
        SheetRegion::All,
        |it| it.scale = 0.5,
    ),
    rate: 120.0,
    mods: vec![],
    on_end: Repeat,
};

pub const explosion3a_clip: ResourceLoad<SpriteClip> = |rm| SpriteClip {
    frames: load_sprites_from_sheet(
        todo!("../../art/sinestesia-free-2d-explosion-animations-2/3.png"),
        (8, 8),
        SheetRegion::All,
        |it| it.scale = 0.5,
    ),
    rate: 120.0,
    mods: vec![],
    on_end: Repeat,
};

pub const smoke_clip_001: ResourceLoad<SpriteClip> = |rm| SpriteClip {
    frames: load_sprites_from_sheet(
        include_bytes!("../../../art/my/smoke_001.png"),
        (1, 1),
        SheetRegion::All,
        |it| {},
    ),
    rate: 2.5,
    mods: vec![
        Scale(Curve::from_function(|it| (1.0 - it))),
        ColorMult("#CCC".to_color()),
    ],
    on_end: Repeat,
};

pub const smoke_clip_001_dark: ResourceLoad<SpriteClip> = |rm| SpriteClip {
    frames: load_sprites_from_sheet(
        include_bytes!("../../../art/my/smoke_001.png"),
        (1, 1),
        SheetRegion::All,
        |it| {},
    ),
    rate: 4.5,
    mods: vec![
        Scale(Curve::new_ext(&[
            Value(0.0),
            Value(1.0),
            Transition(4),
            Value(0.0),
        ])),
        ColorMult("#777".to_color()),
    ],
    on_end: Repeat,
};

pub const smoke_clip_plane_explosion: ResourceLoad<SpriteClip> = |rm| SpriteClip {
    frames: load_sprites_from_sheet(
        include_bytes!("../../../art/my/smoke_001.png"),
        (1, 1),
        SheetRegion::All,
        |it| {},
    ),
    rate: 3.5,
    mods: vec![
        Scale(Curve::new_ext(&[
            Value(1.0),
            Transition(8),
            Value(0.0),
        ])),
        ColorMult("#777".to_color()),
    ],
    on_end: Repeat,
};

pub const cloud_swirling_clip: ResourceLoad<SpriteClip> = |rm| SpriteClip {
    frames: load_sprites_from_sheet(
        todo!("../../art/smoke-aura/Smoke15Frames.png"),
        (4, 4),
        SheetRegion::Range((0, 2)..(3, 3)),
        |it| {},
    ),
    rate: 20.0,
    mods: vec![],
    on_end: Repeat,
};

#![allow(non_upper_case_globals)]

use std::collections::HashMap;
use std::rc::Rc;
use futures::{FutureExt, StreamExt, TryFutureExt};
use futures::future::{join_all, LocalBoxFuture};
use crate::common::curve::{Curve};
use crate::common::enum_maps::new_enum_map_async;
use crate::common::resource::{ResourceLoad, ResourceLoadAsync, ResourceManagerRc};
use crate::common::sprite;
use crate::common::sprite::{load_sprites_from_sheet, SheetRegion};
use crate::common::sprite_clip::cell_range;
use crate::lifecycle::loading;
use crate::model::data::{GenericSplitSpriteClipData, MobSpriteSetData, SplitSpriteClipData, SpriteMod};
use crate::model::data::SpriteMod::{OriginNorm, Scale};
use crate::model::def::{MobAnimation, MobSpriteSet, Sprite};
use crate::model::def::SpriteClip;
use crate::{ResourceManager, Vec2};

pub const plane_sprite: ResourceLoadAsync<Sprite> = |rm| {
    sprite::load_sprite(include_bytes!("../../../art/free_plane/Plane/Fly (1).png"), vec![Scale(0.12)]).boxed_local()
};
const bullet_sprite_bytes: &[u8] = include_bytes!("../../../art/my/bullet_001.png");

pub const bullet_sprite: ResourceLoadAsync<Sprite> = |rm| {
    sprite::load_sprite(bullet_sprite_bytes, vec![Scale(0.1)]).boxed_local()
};

pub const bullet_sprite_big: ResourceLoadAsync<Sprite> = |rm| {
    sprite::load_sprite(bullet_sprite_bytes, vec![Scale(0.15)]).boxed_local()
};

pub const missile_sprite: ResourceLoadAsync<Sprite> = |rm| sprite::load_sprite(
    include_bytes!("../../../art/my/missile_001.png"),
    vec![
        Scale(0.08),
        OriginNorm(Vec2::new(0.1, 0.5)),
    ]).boxed_local();

pub const missile_sprite_yellow: ResourceLoadAsync<Sprite> = |rm| sprite::load_sprite(
    include_bytes!("../../../art/my/missile_001_yellow.png"),
    vec![
        Scale(0.08),
        OriginNorm(Vec2::new(0.1, 0.5)),
    ]).boxed_local();

pub const missile_sprite_blue: ResourceLoadAsync<Sprite> = |rm| sprite::load_sprite(
    include_bytes!("../../../art/my/missile_001_blue.png"),
    vec![
        Scale(0.08),
        OriginNorm(Vec2::new(0.1, 0.5)),
    ]).boxed_local();

pub const sprite_plasma_001: ResourceLoadAsync<Sprite> = |rm|
    sprite::load_sprite(include_bytes!("../../../art/my/plasma_001.png"), vec![Scale(0.12)]).boxed_local();

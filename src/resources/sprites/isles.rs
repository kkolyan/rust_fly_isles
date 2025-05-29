use macroquad::prelude::Vec2;
use crate::common::resource::{ResourceLoad, ResourceLoadAsync};
use crate::common::sprite;
use crate::FutureExt;
use crate::model::data::SpriteMod::OriginNorm;
use crate::model::def::Sprite;

pub const isle1_sprite: ResourceLoadAsync<Sprite> = |rm| sprite::load_sprite(
    include_bytes!("../../../art/sky_level_creation/premade_islands/premade_island_01.png"),
    vec![
        OriginNorm(Vec2::new(0.55, 0.49))
    ],
).boxed_local();

pub const isle2_sprite: ResourceLoadAsync<Sprite> = |rm| {
    sprite::load_sprite(include_bytes!("../../../art/sky_level_creation/premade_islands/premade_island_02.png"), vec![]).boxed_local()
};

pub const isle3_sprite: ResourceLoadAsync<Sprite> = |rm| {
    sprite::load_sprite(include_bytes!("../../../art/sky_level_creation/premade_islands/premade_island_03.png"), vec![]).boxed_local()
};

pub const isle_empy_1_sprite: ResourceLoadAsync<Sprite> = |rm| sprite::load_sprite(
    include_bytes!("../../../art/sky_level_creation/blank_islands/island_4.png"),
    vec![
        OriginNorm(Vec2::new(0.5, 0.15))
    ],
).boxed_local();

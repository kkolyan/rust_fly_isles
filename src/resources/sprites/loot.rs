use futures::FutureExt;
use crate::common::resource::ResourceLoadAsync;
use crate::common::sprite;
use crate::model::data::SpriteMod;
use crate::model::def::Sprite;
use crate::Vec2;

pub fn image_resource_a() -> &'static [u8]  { include_bytes!("../../../art/my/resource_a.png") }
pub fn image_resource_b() -> &'static [u8]  { include_bytes!("../../../art/my/resource_b.png") }
pub fn image_resource_c() -> &'static [u8]  { include_bytes!("../../../art/my/resource_c.png") }

pub const sprite_resource_a: ResourceLoadAsync<Sprite> = |rm| {
    sprite::load_sprite(
        image_resource_a(),
        vec![
            SpriteMod::Scale(0.25),
            SpriteMod::OriginNorm(Vec2::new(0.5, 1.0))
        ],
    ).boxed_local()
};

pub const sprite_resource_b: ResourceLoadAsync<Sprite> = |rm| {
    sprite::load_sprite(
        image_resource_b(),
        vec![
            SpriteMod::Scale(0.25),
            SpriteMod::OriginNorm(Vec2::new(0.5, 1.0))
        ],
    ).boxed_local()
};

pub const sprite_resource_c: ResourceLoadAsync<Sprite> = |rm| {
    sprite::load_sprite(
        image_resource_c(),
        vec![
            SpriteMod::Scale(0.25),
            SpriteMod::OriginNorm(Vec2::new(0.5, 1.0))
        ],
    ).boxed_local()
};

use crate::common::resource::{ResourceLoad, ResourceLoadAsync};
use crate::common::sprite;
use crate::FutureExt;
use crate::model::def::Sprite;

pub const cloud1_sprite: ResourceLoadAsync<Sprite> = |rm| {
    sprite::load_sprite(include_bytes!("../../../art/2d-clouds-pack/cloud1.PNG"), vec![]).boxed_local()
};

pub const cloud2_sprite: ResourceLoadAsync<Sprite> = |rm| {
    sprite::load_sprite(include_bytes!("../../../art/2d-clouds-pack/cloud2.PNG"), vec![]).boxed_local()
};

pub const cloud3_sprite: ResourceLoadAsync<Sprite> = |rm| {
    sprite::load_sprite(include_bytes!("../../../art/2d-clouds-pack/cloud3.PNG"), vec![]).boxed_local()
};

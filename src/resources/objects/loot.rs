use macroquad::prelude::{GREEN, Vec2, YELLOW};
use crate::common::resource::{Resource, ResourceLoad, ResourceLoadAsync};
use crate::model::def::{CollisionCircle, GameResource, Item, Loot, Sprite};
use crate::{ResourceGet, ResourceManager};
use crate::common::unsorted::ToColor;
use crate::resources::materials::huer::create_huer_material;
use crate::resources::sounds::sound_pick_001;
use crate::resources::sprites::loot::{sprite_resource_a, sprite_resource_b, sprite_resource_c};

const RANK_STYLE: [Option<&'static str>; 3] = [Some("#005C63"), Some("#000000"), Some("#964CFF")];

pub const loot_A: ResourceLoad<Loot> = |rm| create_loot(rm, RANK_STYLE[0], &sprite_resource_a, GameResource::A, 1);
pub const loot_B: ResourceLoad<Loot> = |rm| create_loot(rm, RANK_STYLE[1], &sprite_resource_b, GameResource::B, 1);
pub const loot_C: ResourceLoad<Loot> = |rm| create_loot(rm, RANK_STYLE[2], &sprite_resource_c, GameResource::C, 1);

fn create_loot(rm: ResourceManager, tint: Option<&'static str>, sprite: &'static ResourceLoadAsync<Sprite>, resource: GameResource, count: u32) -> Loot {
    Loot {
        sprite: sprite.get(&rm),
        content: vec![
            Item::Resource {
                count,
                resource
            }
        ],
        collider_unscaled: CollisionCircle {
            center: Vec2::new(0.0, -24.0),
            radius: 24.0
        },
        pick_sound: Some(sound_pick_001.get(&rm)),
    }
}
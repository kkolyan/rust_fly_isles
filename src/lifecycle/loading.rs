use std::collections::HashMap;
use std::rc::Rc;
use futures::future::LocalBoxFuture;
use crate::common::enum_maps::new_enum_map_async;
use crate::common::resource::{ResourceLoad, ResourceManagerRc};
use crate::common::sprite::{apply_sprite_mods, load_sprites_from_files, load_sprites_from_sheet};
use crate::{FutureExt, ResourceManager};
use crate::common::resource::Resource;
use crate::common::curve::Curve;
use crate::model::data::{GenericSplitSpriteClipData, MobSpriteSetData, SplitSpriteClipData, SpriteMod};
use crate::model::data::SpriteMod::Scale;
use crate::model::def::{MobAnimation, MobSpriteSet, OnClipEnd, Sprite, SpriteClip};
use crate::model::def::OnClipEnd::Repeat;
use crate::model::def::SpriteClipMod::Alpha;

pub fn clip_split(rm: ResourceManager, data: SplitSpriteClipData) -> LocalBoxFuture<'static, SpriteClip> {
    generic_clip_split(rm, data.data, vec![Scale(data.scale)], data.alpha)
}

pub fn mob_clips(rm: ResourceManager, data: MobSpriteSetData) -> LocalBoxFuture<'static, MobSpriteSet> {
    async move {
        let clips = new_enum_map_async(move |anim: MobAnimation| {
            let clip_by_animation = data.clips;
            let generic_data = clip_by_animation(anim);
            let rm = rm.clone();
            let sprite_mod = data.sprite_mod.clone();
            async move {
                Resource::detached(generic_clip_split(rm, generic_data, sprite_mod, None).await)
            }.boxed_local()
        }).await;

        MobSpriteSet {
            clips,
        }
    }.boxed_local()
}

fn generic_clip_split(
    rm: ResourceManager,
    data: GenericSplitSpriteClipData,
    sprite_mods: Vec<SpriteMod>,
    alpha: Option<Curve<f32>>,
) -> LocalBoxFuture<'static, SpriteClip> {
    async move {
        match data {
            GenericSplitSpriteClipData::MultiFile {
                data,
                sprite_mod,
                rate,
                on_end
            } => SpriteClip {
                frames: load_sprites_from_files(
                    data,
                    [sprite_mods, sprite_mod].concat(),
                ).await,
                rate: rate,
                mods: alpha.map_or_else(Vec::new, |it| vec![Alpha(it)]),
                on_end: on_end,
            },
            GenericSplitSpriteClipData::Sheet {
                data,
                sheet_size,
                sprite_mod,
                rate,
                on_end,
                region
            } => SpriteClip {
                frames: load_sprites_from_sheet(
                    data,
                    sheet_size,
                    region,
                    |it| {
                        apply_sprite_mods(it, [sprite_mods.clone(), sprite_mod.clone()].concat());
                    },
                ),
                rate,
                mods: alpha.map_or_else(Vec::new, |it| vec![Alpha(it)]),
                on_end,
            }
        }
    }.boxed_local()
}

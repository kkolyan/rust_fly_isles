use std::ops::Range;
use std::rc::Rc;

use macroquad::color::WHITE;
use macroquad::math::Rect;
use macroquad::prelude::{Vec2, Vec4Swizzles};
use SpriteDrawingOption::{Empty, FlipY, Material, Rot, Scale};

use crate::common::{camera, sprite};
use crate::common::angle::Angle;
use crate::common::resource::Resource;
use crate::common::camera::ViewPort;
use crate::common::contract::{Get, GetMut};
use crate::common::pool::{Pool, PoolKey};
use crate::common::sprite::draw_sprite;
use crate::common::sprite_clip::SpriteDrawingOption::FlipX;
use crate::common::unsorted::{IndexRange, ToColor};
use crate::draw::Stats;
use crate::model::def::{MaterialInstance, OnClipEnd, SpriteClip, SpriteClipMod};
use crate::model::state::{FixedSpriteClipState, SpriteClipState};

pub fn do_nothing<T>(_: &mut Vec<T>, _: usize) {}

pub(crate) fn update<T, F>(dt: f32, mut on_clip_end: F, source: &mut Vec<T>, extractor: fn(&mut T) -> Option<&mut SpriteClipState>)
    where F: FnMut(&mut Vec<T>, usize)
{
    for i in source.indices().rev() {
        if let Some(state) = extractor(&mut source[i]) {
            if state.delay > 0.0 {
                state.delay -= dt;
            }
            if state.delay <= 0.0 {
                state.frame += (state.clip.rate * state.rate) as f64 * dt as f64;

                let next_frame = (state.frame + dt as f64).floor() as usize;
                let last_frame = next_frame >= state.clip.frames.len();

                if state.frame.floor() as usize >= state.clip.frames.len() {
                    match state.clip.on_end {
                        OnClipEnd::Clamp => {
                            state.frame = state.clip.frames.len() as f64 - 1.0;
                        },
                        OnClipEnd::Repeat => {
                            let len = state.clip.frames.len() as f64;
                            state.frame = (state.frame / len).fract() * len;
                        }
                    }
                }
                if last_frame {
                    on_clip_end(source, i);
                }
            }
        }
    }
}

pub(crate) fn update_pool<K:PoolKey, T, F>(dt: f32, mut on_clip_end: F, source: &mut Pool<K, T>, extractor: fn(&mut T) -> Option<&mut SpriteClipState>)
    where F: FnMut(&mut Pool<K, T>, K)
{
    let keys: Vec<K> = source.iter().map(|(k, _)| k).copied().collect();
    for key in keys {
        if let Some(state) = extractor(source.get_mut(&key).unwrap()) {
            if state.delay > 0.0 {
                state.delay -= dt;
            }
            if state.delay <= 0.0 {
                state.frame += (state.clip.rate * state.rate) as f64 * dt as f64;

                let next_frame = (state.frame + dt as f64).floor() as usize;
                let last_frame = next_frame >= state.clip.frames.len();

                if state.frame.floor() as usize >= state.clip.frames.len() {
                    match state.clip.on_end {
                        OnClipEnd::Clamp => {
                            state.frame = state.clip.frames.len() as f64 - 1.0;
                        },
                        OnClipEnd::Repeat => {
                            let len = state.clip.frames.len() as f64;
                            state.frame = (state.frame / len).fract() * len;
                        }
                    }
                }
                if last_frame {
                    on_clip_end(source, key);
                }
            }
        }
    }
}

pub struct SpriteDrawingItem<'a, const N: usize> {
    pub pos: Vec2,
    pub clip: &'a SpriteClipState,
    pub options: [SpriteDrawingOption; N],
}

pub enum SpriteDrawingOption {
    Empty,
    FlipX(bool),
    FlipY(bool),
    Rot(Angle),
    Scale(f32),
    Material(Resource<MaterialInstance>),
}

pub struct SpriteDrawer<'a> {
    pub stats: &'a mut Stats,
    pub view_port: &'a ViewPort,
}

impl<'a> SpriteDrawer<'a> {
    pub fn draw<T, const N: usize, F: Fn(&T) -> Option<SpriteDrawingItem<N>>>(&'a self, source: &Vec<T>, mapper: F) {
        for item in source {
            if let Some(SpriteDrawingItem { clip, pos, options }) = mapper(item) {
                if clip.delay > 0.0 {
                    continue;
                }
                self.view_port.port(pos, 1.0, |ported| {
                    draw_sprite(&clip.clip.frames[calc_index(clip)], ported.screen_pos, |it| {
                        it.screen_scale = ported.screen_scale;
                        for option in &options {
                            match option {
                                FlipX(flip) => it.flip_x = *flip,
                                FlipY(flip) => it.flip_y = *flip,
                                Scale(scale) => it.screen_scale *= scale,
                                Material(mat) => it.material = Some(mat.clone()),
                                Rot(angle) => it.angle = *angle,
                                Empty => {}
                            }
                        }
                        for clip_mod in &clip.clip.mods {
                            match clip_mod {
                                SpriteClipMod::Empty => {}
                                SpriteClipMod::Alpha(curve) => {
                                    let mut c = WHITE;
                                    c.a = curve.lerp(clip.frame as f32 / clip.clip.frames.len() as f32);
                                    it.tint = c;
                                }
                                SpriteClipMod::Scale(curve) => {
                                    it.screen_scale *= curve.lerp(clip.frame as f32 / clip.clip.frames.len() as f32);
                                }
                                SpriteClipMod::ColorMult(c) => {
                                    it.tint = (it.tint.to_vec() * c.to_vec()).to_color();
                                }
                            }
                        }
                    });
                });
            }
        }
    }
}

fn calc_index(clip: &SpriteClipState) -> usize {
    clip.frame.floor() as usize
}

pub fn cell_range(range: Range<(usize, usize)>) -> Vec<(usize, usize)> {
    let mut results = vec![];
    for col in range.start.1..range.end.1 {
        for row in range.start.0..range.end.0 {
            results.push((row, col));
        }
    }
    results
}

impl SpriteClipState {
    pub fn new(clip: &Resource<SpriteClip>) -> Self {
        SpriteClipState {
            clip: clip.clone(),
            frame: 0.0,
            rate: 1.0,
            delay: 0.0,
        }
    }
}

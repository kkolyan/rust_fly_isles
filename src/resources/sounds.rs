use std::rc::Rc;
use futures::future::LocalBoxFuture;
use futures::FutureExt;
use macroquad::audio::{load_sound_from_bytes, Sound};
use crate::common::resource::{ResourceLoad, ResourceLoadAsync};
use crate::model::def::GameSound;
use crate::ResourceManager;

async fn load_sound_with_volume(base_volume: f32, throttling_sec: Option<f32>, data: &[u8]) -> GameSound {
    GameSound {
        sound: load_sound_from_bytes(data).await.unwrap(),
        base_volume,
        throttling_sec,
    }
}

pub const engine_001_sound: &[Option<ResourceLoadAsync<GameSound>>] = &[
    Some(|rm| sound(rm, 0.4, None, include_bytes!("../../audio/engine_001_d11.ogg.ogg"))),
    Some(|rm| sound(rm, 0.4, None, include_bytes!("../../audio/engine_001_d2.ogg.ogg"))),
    Some(|rm| sound(rm, 0.4, None, include_bytes!("../../audio/engine_001.ogg.ogg"))),
    Some(|rm| sound(rm, 0.4, None, include_bytes!("../../audio/engine_001_u1.ogg.ogg"))),
    Some(|rm| sound(rm, 0.4, None, include_bytes!("../../audio/engine_001_u2.ogg.ogg"))),
    Some(|rm| sound(rm, 0.4, None, include_bytes!("../../audio/engine_001_u3.ogg.ogg"))),
    Some(|rm| sound(rm, 0.4, None, include_bytes!("../../audio/engine_001_u4.ogg.ogg"))),
    Some(|rm| sound(rm, 0.4, None, include_bytes!("../../audio/engine_001_u5.ogg.ogg"))),
];

pub const sound_missile_001: ResourceLoadAsync<GameSound> = |rm| sound(rm, 1.0, None, include_bytes!("../../audio/missile_001.ogg.ogg"));
pub const sound_explosion_001: ResourceLoadAsync<GameSound> = |rm| sound(rm, 1.0, None, include_bytes!("../../audio/explosion_001.ogg.ogg"));
pub const sound_cannon_001: ResourceLoadAsync<GameSound> = |rm| sound(rm, 1.0, None, include_bytes!("../../audio/cannon_001.ogg.ogg"));
pub const sound_cannon_002: ResourceLoadAsync<GameSound> = |rm| sound(rm, 1.0, None, include_bytes!("../../audio/cannon_002.ogg"));
pub const sound_hit_001: ResourceLoadAsync<GameSound> = |rm| sound(rm, 1.0, None, include_bytes!("../../audio/hit_001.ogg.ogg"));
pub const sound_death_001: ResourceLoadAsync<GameSound> = |rm| sound(rm, 1.0, None, include_bytes!("../../audio/death_001.ogg.ogg"));
pub const sound_pick_001: ResourceLoadAsync<GameSound> = |rm| sound(rm, 1.0, None, include_bytes!("../../audio/joy_001.ogg"));
pub const sound_plasma: ResourceLoadAsync<GameSound> = |rm| sound(rm, 1.0, None, include_bytes!("../../audio/plasma_001.ogg"));
pub const sound_rail: ResourceLoadAsync<GameSound> = |rm| sound(rm, 1.0, None, include_bytes!("../../audio/rail_001.ogg"));
pub const sound_levelup: ResourceLoadAsync<GameSound> = |rm| sound(rm, 0.8, None, include_bytes!("../../audio/levelup.ogg"));
pub const sound_skillup: ResourceLoadAsync<GameSound> = |rm| sound(rm, 0.5, None, include_bytes!("../../audio/skillup.ogg"));
pub const sound_death_robot: ResourceLoadAsync<GameSound> = |rm| sound(rm, 4.5, None, include_bytes!("../../audio/death_mech_001.ogg"));
pub const sound_death_wasp: ResourceLoadAsync<GameSound> = |rm| sound(rm, 1.2, None, include_bytes!("../../audio/death_wasm_001.ogg"));
pub const sound_death_drone: ResourceLoadAsync<GameSound> = |rm| sound(rm, 1.0, None, include_bytes!("../../audio/death_mini_robo.ogg"));
pub const sound_pain: ResourceLoadAsync<GameSound> = |rm| sound(rm, 0.5, Some(0.2), include_bytes!("../../audio/pain_001.ogg"));

fn sound(rm: ResourceManager, volume: f32, throttling_sec: Option<f32>, bytes: &[u8]) -> LocalBoxFuture<GameSound> {
    async move {
        load_sound_with_volume(volume, throttling_sec, bytes).await
    }.boxed_local()
}
use std::cmp::Ordering;
use std::collections::{HashMap, HashSet};
use std::fmt::{Debug, Formatter};
use std::ops::Range;
use std::rc::Rc;

use enum_iterator::Sequence;
use macroquad::audio::Sound;
use macroquad::color::Color;
use macroquad::material::Material;
use macroquad::prelude::{Rect, Texture2D};
use macroquad::prelude::{f32, Vec2};
use crate::common::angle::Angle;
use crate::common::resource::Resource;

use crate::common::curve::Curve;
use crate::common::pool::{Pool, PoolKey};
use crate::game::generator_001::LocationGenerator001;
use crate::game::generator_002::LocationGenerator002;
use crate::{GameState, PlayerState};
use crate::model::state::{Ammo, DeviceOrder, GameProgression, LimitedCannon, LootId, LootState, WeaponSelector};

#[derive(Debug)]
pub struct Sprite {
    pub texture: Texture2D,
    pub origin_normalized: Vec2,
    pub size: Vec2,
    pub scale: f32,
    pub angle: Angle,
    pub region: SpriteRegion,
    pub collision_circle_normalized: Option<(Vec2, f32)>,
}

#[derive(Debug)]
pub struct SpriteRegion {
    pub x0: f32,
    pub y0: f32,
    pub x1: f32,
    pub y1: f32,
}

#[derive(Debug)]
pub struct Plane {
    pub sprite: Resource<Sprite>,
    pub hp: f32,
    pub gears: Vec<Gear>,
    pub default_gear: usize,
    pub trail: Option<Resource<TrailSource>>,
    pub stabilization: Resource<Stabilization>,
    pub explosion: Option<Resource<Explosion>>,
    pub arms: PlaneArms,
    pub collision_radius: f32,
    pub engine_sound: Vec<Option<Resource<GameSound>>>,
    pub death_sound: Option<Resource<GameSound>>,
    pub pain_sound: Option<Resource<GameSound>>,
}

#[derive(Debug)]
pub struct Gear {
    pub thrust: f32,
    pub energy_per_sec: f32,
    pub tech_level: u32,
}

#[derive(Debug)]
pub struct PlaneArms {
    pub primary_default: Resource<Cannon>,
    pub primary: Resource<CannonPodProps>,
    pub secondary: Resource<CannonPodProps>,
}

#[derive(Debug)]
pub struct Stabilization {
    pub slide: SlideStabilization,
    pub steer: SteerStabilization,
}

#[derive(Debug)]
pub struct SlideStabilization {
    // normalized by nominal_speed
    pub slide_by_speed: Curve<f32>,
    // normalized by PI/2
    pub slide_by_attack: Curve<f32>,
}

#[derive(Debug, Clone)]
pub struct SteerStabilization {
    // normalized by nominal_speed
    pub steering_by_speed: Curve<f32>,
    // normalized by PI
    pub steering_by_attack: Curve<f32>,
    pub max_angular_acceleration: f32,
}

#[derive(Debug)]
pub struct TrailSource {
    pub offset: Vec2,
    pub emitter: Resource<ParticleEmitter>,
}

#[derive(Debug)]
pub struct CannonPod {
    pub props: Resource<CannonPodProps>,
    pub cannon: Option<Resource<Cannon>>,
}

#[derive(Debug)]
pub struct CannonPodProps {
    pub offset: Vec2,
}

#[derive(Debug)]
pub struct Cannon {
    pub title: &'static str,
    pub rate: f32,
    pub barrel: CannonBarrel,
    pub spread_degrees: Curve<f32>,
    pub sound: Option<Resource<GameSound>>,
}

#[derive(Debug)]
pub struct Buff {
    pub title: &'static str,
    pub spec: BuffSpec,
}

#[derive(Debug)]
pub enum BuffSpec {
    ThrustAddendum {
        extra_acceleration: f32,
    },
    Nitro {
        smoke_factor: f32,
        top_speed: f32,
        acceleration_by_speed: Curve<f32>,
    },
    ThrustMultiplier {
        acceleration_multiplier: f32,
        smoke_factor_rel: f32,
    },
    SteerBooster {
        stabilization: SteerStabilization,
        smoke_factor_abs: f32,
    },
    Repair {
        hp_per_sec: f32,
    },
}

#[derive(Debug)]
pub enum BonusSpec {
    ThrustTechInc,
    Power {
        extra_energy: f32,
    },
    Armor {
        extra_hp: f32,
    },
}

#[derive(Debug)]
pub enum CannonBarrel {
    Projectile(Resource<Projectile>),
    HitScan(Resource<HitScan>),
}

#[derive(Debug)]
pub struct HitScan {
    pub action: HitScanAction,
    pub look: HitScanLook,
}

#[derive(Debug)]
pub struct HitScanAction {
    pub damage: Curve<f32>,
    pub range: f32,
    pub collider_thickness: f32,
}

#[derive(Debug)]
pub enum HitScanLook {
    None,
    Ray(Resource<HitScanRay>),
}

#[derive(Debug)]
pub struct HitScanRay {
    pub width: Curve<f32>,
    pub color: Curve<Color>,
    pub duration_sec: f32,
}

#[derive(Debug)]
pub struct Projectile {
    pub body: TransientBallisticBody,
    pub collision_radius: f32,
    pub splash_damage: Option<SplashDamage>,
    pub damage: Curve<f32>,
    pub explosion: Option<ExplosionSource>,
    pub acceleration: Option<f32>,
    pub stabilization: Option<Resource<Stabilization>>,
    pub trail: Option<Resource<TrailSource>>,
    pub exhaust_clip: Option<Resource<SpriteClip>>,
    pub hit_sound: Option<Resource<GameSound>>,
    pub rotation: ProjectileRot,
    pub pulsation: Option<ProjectilePulsation>,
    pub mods: Vec<ProjectileMod>,
}

#[derive(Debug)]
pub enum ProjectileMod {
    Homing
}

#[derive(Debug)]
pub struct SplashDamage {
    pub damage: Curve<f32>,
    pub radius: f32,
    pub damage_factor_by_distance_norm: Curve<f32>,
}

#[derive(Debug)]
pub struct ProjectilePulsation {
    pub scale: Curve<f32>,
}

#[derive(Debug)]
pub enum ProjectileRot {
    InitialVelocity,
    Spinning { degrees_per_second: f32 },
}

#[derive(Debug)]
pub struct ExplosionSource {
    pub offset: Vec2,
    pub explosion: Resource<Explosion>,
}

#[derive(Debug)]
pub struct Bot {
    pub plane: Resource<Plane>,
    pub height_normalized: Curve<f32>,
}

#[derive(Debug)]
pub struct BackgroundObject {
    pub size: Curve<f32>,
    pub height_normal: Option<Curve<f32>>,
    pub sprite: Vec<Resource<Sprite>>,
    pub z: Curve<f32>,
    pub count: Curve<i32>,
    pub material: Option<Resource<MaterialInstance>>,
}

#[derive(Debug)]
pub struct IsleSpawn {
    pub isle: Resource<Isle>,
    pub mobs: Vec<MobSpawn>,
    pub count: Curve<u32>,
}

#[derive(Debug)]
pub struct Isle {
    pub scale: f32,
    pub sprite: Resource<Sprite>,
    pub bounds: Range<f32>,
    pub course_change_interval_seconds: Curve<f32>,
    pub drift_speed: f32,
}

#[derive(Debug)]
pub struct MobSpawn {
    pub mob: Resource<Mob>,
    pub count: Curve<u32>,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, Sequence)]
pub enum MobAnimation {
    Idle,
    Move,
    // till the possibility to pause animation to confuse enemy
    AttackWindup,
    // the highest position of animation
    AttackHold,
    // path from the highest position to the active frame
    AttackDeliver,
    // frame when mob holds deliver and shoots multiple rounds in row
    Burst,
    // path after active frame
    AttackFinish,
    Die,
    Dead,
}

#[derive(Debug)]
pub struct MobSpriteSet {
    pub clips: HashMap<MobAnimation, Resource<SpriteClip>>,
}

#[derive(Debug)]
pub struct Mob {
    pub rank: MobRank,
    pub sprite_set: Resource<MobSpriteSet>,
    pub scale: f32,
    pub move_speed: f32,
    pub move_seconds: Curve<f32>,
    pub idle_seconds: Curve<f32>,
    pub collider_unscaled: CollisionCircle,
    pub hp: f32,
    pub attacks: Vec<Resource<MobAttack>>,
    pub pod: CannonPodProps,
    pub flier_aggro_distance: f32,
    pub flier_chase_step: f32,
    pub material: Option<Resource<MaterialInstance>>,
    pub loot_chances: Vec<MobLootChance>,
    pub kind: MobKind,
    pub xp_reward: u32,
    pub death_sound: Option<Resource<GameSound>>,
    pub pain_sound: Option<Resource<GameSound>>,
    pub burst: Option<u16>,
    pub attack_chance: f32,
}

pub type MobRank = u32;

#[derive(Debug)]
pub enum MobKind {
    Walker,
    Flyer,
}

#[derive(Debug, PartialEq, Eq, Copy, Clone, Hash)]
pub enum ProgressFlag {
    DeathTutorialShown,
    LevelUpTutorialShown,
    EquipmentTutorialShown,
    BriefingShown,
    FloppiesFound,
    DisksFound,
    SdCardsFound,
}

#[derive(Debug)]
pub struct MobLootChance {
    pub probability: f32,
    pub loot: Resource<Loot>,
    pub rule: ProgressPredicate,
}

#[derive(Debug)]
pub struct MobLootOption {
    pub weight: f32,
    pub cnt: u32,
    pub loot: Option<Resource<Loot>>,
}

#[derive(Debug)]
pub struct MobAttack {
    pub trigger_range: f32,
    pub hold_sec: f32,
    pub cooldown_sec: Curve<f32>,
    pub pattern: MobAttackPattern,
    pub charge: Option<MobCharge>,
    pub burst: Option<Resource<Burst>>,
    pub late_aim: bool,
    pub hold_effect: Option<HoldEffect>,
}

#[derive(Debug)]
pub struct Burst {
    pub rounds_in_row: u16,
    pub cannon: Resource<Cannon>,
}

#[derive(Debug)]
pub enum HoldEffect {
    Circle(Resource<CircleEffect>),
}

#[derive(Debug)]
pub struct CircleEffect {
    pub radius: Curve<f32>,
    pub color: Curve<Color>,
}

#[derive(Debug)]
pub struct MobCharge {
    pub velocity: f32,
    pub duration_sec: f32,
}

#[derive(Debug)]
pub enum MobAttackPattern {
    Melee { connect_range: f32, damage: Curve<f32> },
    Distant { cannon: Resource<Cannon> },
}

#[derive(Debug)]
pub struct ParticleEmitter {
    pub emission: ParticleEmission,
    pub spawn_rate: f32,
}

#[derive(Debug)]
pub struct ParticleEmission {
    pub clip_variants: Vec<Resource<SpriteClip>>,
    pub scale: Curve<f32>,
    pub spread_distance: f32,
    pub rate: Curve<f32>,
    pub delay: Curve<f32, >,
}

#[derive(Debug)]
pub struct Explosion {
    pub particles: Vec<ExplosionParticleEmission>,
    pub fragments: Vec<ExplosionFragment>,
    pub sound: Option<Resource<GameSound>>,
}

#[derive(Debug)]
pub struct ExplosionFragment {
    pub body: TransientBallisticBody,
    pub trail: ParticleEmitter,
}

#[derive(Debug)]
pub struct ExplosionParticleEmission {
    pub emission: ParticleEmission,
    pub count: Curve<usize>,
    pub off_center_speed: Curve<f32>,
    pub speed_factor: f32,
}

#[derive(Debug)]
pub struct Location {
    pub sky: Sky,
    pub size: Vec2,
    pub start_pos_norm: Vec2,
    pub background_objects: Vec<BackgroundObject>,
    pub bots: Vec<Bot>,
    pub content: LocationContent,
    pub lava_damage_by_height_per_sec_norm: Option<Curve<f32>>,
    pub progression: Vec<ProgressRule>,
    pub journal: Vec<JournalEntry>,
    pub default_weapon: Option<PlaneWeapon>,
}

#[derive(Debug)]
pub struct ProgressRule {
    pub objective: Option<&'static str>,
    pub journal_entry: Option<Vec<&'static str>>,
    pub display_condition: ProgressPredicate,
    pub complete_condition: ProgressPredicate,
    pub output_flags: Vec<ProgressFlag>,
}

pub struct ProgressPredicate(pub ProgressPredicateFn);
pub type ProgressPredicateFn = fn(&GameProgressCtx) -> bool;

impl Debug for ProgressPredicate {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        Ok(())
    }
}

#[derive(Clone, Debug)]
pub struct GameProgressCtx<'a> {
    pub player: &'a PlayerState,
    pub progression: &'a GameProgression,
    pub loot: &'a Pool<LootId, LootState>,
}

#[derive(Debug)]
pub enum LocationContent {
    Generator001(LocationGenerator001),
    Generator002(LocationGenerator002),
}

#[derive(Debug)]
pub struct Game {
    pub combat: Resource<Location>,
    pub training: Resource<Location>,
    pub player_plane: Resource<Plane>,
    pub standard_pain_material: Resource<MaterialInstance>,
    pub advanced_weapons: WeaponSelector,
    pub shop_assortment: Vec<Resource<ShopLot>>,
    pub rpg: Pool<ImprovementId, Improvement>,
    pub sound_level_up: Option<Resource<GameSound>>,
    pub sound_skill_up: Option<Resource<GameSound>>,
}

#[derive(Copy, Clone, Hash, Eq, PartialEq, Debug, Ord, PartialOrd)]
pub struct ImprovementId(i64);

impl PoolKey for ImprovementId {
    fn initial() -> Self { ImprovementId(1) }
    fn next(&self) -> Self { ImprovementId(self.0 + 1) }
}

#[derive(Debug)]
pub struct Improvement {
    pub title: ImprovementTitle,
    pub category: ImprovementCategory,
    pub levels: Vec<ImprovementLevel>,
    pub description: [&'static str; 3],
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, Sequence)]
pub enum ImprovementCategory {
    Passives,
    Weapons,
    Utility,
}

#[derive(Debug)]
pub enum ImprovementTitle {
    Hard(&'static str),
    FromBuff(Resource<Buff>),
    FromWeapon(Resource<Cannon>),
}

#[derive(Debug)]
pub struct ImprovementLevel {
    pub spec: ImprovementSpec,
    pub points: u32,
}

#[derive(Debug)]
pub enum ImprovementSpec {
    WeaponSkill(PlaneWeapon),
    BuffSkill(PlaneBuff),
    Passive {
        spec: Resource<BuffSpec>,
    },
    Bonus {
        spec: BonusSpec,
    },
}

#[derive(Debug, Clone, PartialEq)]
pub struct PlaneWeapon {
    pub spec: Resource<Cannon>,
    pub energy_per_shot: f32,
    pub order: DeviceOrder,
}

#[derive(Debug, Clone, PartialEq)]
pub struct PlaneBuff {
    pub spec: Resource<Buff>,
    pub energy_per_second: f32,
    pub order: DeviceOrder,
}

//
// #[derive(Debug)]
// pub enum Skill {
//     Missiles,
//     NitroCannon,
//     Repair,
//     NitroSteer,
//     NitroThrust,
//     Generator(U3),
//     Armor(U3),
//     MiniMissiles,
// }

#[derive(Debug)]
pub enum U3 {
    _1,
    _2,
    _3,
}

#[derive(Debug)]
pub struct SpriteClip {
    pub frames: Vec<Sprite>,
    pub rate: f32,
    pub mods: Vec<SpriteClipMod>,
    pub on_end: OnClipEnd,
}

#[derive(Debug, Clone)]
pub enum OnClipEnd {
    Clamp,
    Repeat,
}

#[derive(Debug)]
pub enum SpriteClipMod {
    Empty,
    Alpha(Curve<f32>),
    Scale(Curve<f32>),
    ColorMult(Color),
}

#[derive(Debug)]
pub struct Sky {
    pub color_by_height: Curve<Color>,
}

#[derive(Debug)]
pub struct MaterialInstance {
    pub material: Resource<Material>,
    pub uniforms: Vec<(&'static str, UniformSupplier)>,
}

#[derive(Debug)]
pub enum UniformSupplier {
    Color(Color),
}

#[derive(Debug)]
pub struct TransientBallisticBody {
    pub initial_speed: f32,
    pub sprite: Option<Resource<Sprite>>,
    pub drag: f32,
    pub seconds_to_live: Option<f32>,
}

#[derive(Debug, Clone)]
pub struct CollisionCircleNorm {
    pub center_norm: Vec2,
    pub radius_norm: f32,
}

#[derive(Debug, Clone, Copy)]
pub enum Collider {
    Circle(CollisionCircle),
    Ray(CollisionRay),
}

#[derive(Debug, Clone, Copy)]
pub struct CollisionCircle {
    pub center: Vec2,
    pub radius: f32,
}

#[derive(Debug, Clone, Copy)]
pub struct CollisionRay {
    pub origin: Vec2,
    pub dir: Vec2,
    pub distance: f32,
    pub thickness: f32,
}

#[derive(Debug, Clone)]
pub struct GameSound {
    pub sound: Sound,
    pub base_volume: f32,
    pub throttling_sec: Option<f32>,
}

#[derive(Debug, Clone)]
pub struct Loot {
    pub sprite: Resource<Sprite>,
    pub content: Vec<Item>,
    pub collider_unscaled: CollisionCircle,
    pub pick_sound: Option<Resource<GameSound>>,
}

#[derive(Debug, Clone)]
pub enum Item {
    Resource {
        count: u32,
        resource: GameResource,
    },
}

#[derive(Debug, Clone)]
pub struct ShopLot {
    pub price: HashMap<GameResource, u32>,
    pub item: Obtainable,
}

#[derive(Debug, Clone)]
pub enum Obtainable {
    Weapon {
        slot: DeviceSlot,
        weapon: Resource<Cannon>,
        ammo: Ammo,
    },
    HP {
        title: &'static str,
        amount: f32,
    },
    Consumable {
        slot: DeviceSlot,
        def: Resource<Buff>,
        reserve_sec: f32,
    },
    Passive {
        def: Resource<Buff>,
    },
    PassiveReset {
        title: &'static str
    },
}

#[derive(Debug, Clone, Copy)]
pub enum DeviceSlot {
    Primary,
    Secondary,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, Sequence)]
pub enum GameResource {
    A,
    B,
    C,
}

#[derive(Debug)]
pub struct JournalEntry {
    pub condition: ProgressPredicate,
    pub text: Vec<&'static str>,
}

use std::cell::RefCell;
use std::collections::{HashMap, HashSet, VecDeque};
use std::hash::Hash;
use std::ops::Mul;
use std::rc::Rc;
use enum_iterator::Sequence;

use macroquad::audio::Sound;
use macroquad::color::Color;
use macroquad::math::Rect;
use macroquad::prelude::{f32, Vec3, Vec4Swizzles};
use macroquad::prelude::Material;
use macroquad::prelude::Vec2;
use macroquad::telemetry::Frame;
use macroquad::texture::Texture2D;

use crate::common::angle::Angle;
use crate::common::curve::Curve;
use crate::common::metrics::Metrics;
use crate::common::pool::{Pool, PoolKey};
use crate::common::prefs::Pref;
use crate::common::resource::Resource;
use crate::common::unsorted::{ModifyColor, ToColor};
use crate::{Game};
use crate::common::camera::ViewPort;
use crate::model::def::{Buff, BuffSpec, Cannon, CannonPodProps, CircleEffect, CollisionCircle, CollisionRay, GameResource, GameSound, HitScanRay, Isle, Location, Loot, MaterialInstance, Mob, MobAnimation, MobAttack, MobRank, Plane, Projectile, ShopLot, ImprovementId, Sprite, SpriteClip, TrailSource, PlaneWeapon, PlaneBuff, ProgressFlag, Burst};

#[derive(Clone, Debug)]
pub enum AppState {
    Intro,
    Title {
        menu: MenuState
    },
    Game {
        game: GameState,
    },
    GameMenu {
        game: GameState,
        menu: MenuState,
    },
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum AppStateEvent {
    NewGame { location: Resource<Location> },
    Quit,
    Intro,
    Hover(Box<AppStateEvent>),
}

impl AppStateEvent {
    pub fn hover(event: &AppStateEvent) -> AppStateEvent {
        AppStateEvent::Hover(event.clone().into())
    }
}

#[derive(Clone, Debug)]
pub struct MenuState {}

#[derive(Clone, Debug)]
pub struct GameState {
    pub paused: bool,
    pub location: Resource<Location>,
    pub def: Resource<Game>,
    pub player: PlayerState,
    pub planes: Pool<PlaneId, PlaneState>,
    pub background_objects: Vec<BackgroundObjectState>,
    pub particles: ParticlesState,
    pub rays: Vec<RayState>,
    pub bots: Vec<BotState>,
    pub projectiles: Vec<ProjectileState>,
    pub metrics: Rc<Metrics>,
    pub unpause_one_frame: bool,
    pub show_colliders: bool,
    pub isles: Pool<IsleId, IsleState>,
    pub mobs: Pool<MobId, MobState>,
    pub loot: Pool<LootId, LootState>,
    pub gids: Gids,
    pub subsystems: SubSystems,
    pub commands: VecDeque<GameCommand>,
    pub ui_commands: VecDeque<WindowsAction>,
    pub progression: GameProgression,
    pub journal: Vec<JournalStatePage>,
    pub reachable_mobs: Vec<MobId>,
}

#[derive(Clone, Debug)]
pub struct JournalStatePage {
    pub lines: Vec<&'static str>,
}

#[derive(Clone, Debug)]
pub struct LootManager {
    pub accumulators: HashMap<Resource<Loot>, f32>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum UiEvent {
    HoverTopItem(usize),
    ClickShopItem(Resource<ShopLot>),
    ClickSkillsItem(ImprovementId),
    EquipmentClickItem(DeviceId),
    EquipmentClickBinding(Option<EquipmentBinding>),
    JournalAbs { page: usize },
    Hover(Box<UiEvent>),
}

impl UiEvent {
    pub fn to_hover(&self) -> UiEvent {
        UiEvent::Hover(Box::from(self.clone()))
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash, Ord, PartialOrd, Sequence)]
pub enum EquipmentBinding {
    Primary,
    Secondary,
    _1,
    _2,
    _3,
    _4,
    _5,
}

#[derive(Clone, Debug)]
pub enum GameCommand {
    Damage { amount: f32, source: WeaponOwner, target: DamageTarget },
    FireCannon { bal: TransState, rot: RotState, owner: WeaponOwner, cannon: Resource<Cannon>, initial_angle: Angle },
    NewRay(RayState),
    Drop(Resource<Loot>, RelativePos),
}

#[derive(Clone, Debug)]
pub enum WindowsAction {
    Buy(Resource<ShopLot>),
}

#[derive(Clone, Debug, Copy)]
pub enum DamageTarget {
    Mob(MobId),
    Plane(PlaneId),
}

#[derive(Clone, Debug)]
pub struct SubSystems {
    pub audio: AudioManager,
    pub loot: LootManager,
}

#[derive(Clone, Debug)]
pub struct GameProgression {
    pub flags: HashSet<ProgressFlag>,
}

#[derive(Clone, Debug)]
pub struct LoopedSoundState {
    pub def: Resource<GameSound>,
    pub instance_volume: f32,
}

#[derive(Clone, Debug)]
pub struct AudioManager {
    pub volume: Pref<f32>,
    pub looped: Vec<LoopedSoundState>,
    pub cool_down_sec: RefCell<HashMap<Resource<GameSound>, f32>>,
}

#[derive(Clone, Debug)]
pub struct Gids {
    pub prev_gid: i64,
}

#[derive(Copy, Clone, Hash, Eq, PartialEq, Debug)]
pub struct IsleId(i64);

impl PoolKey for IsleId {
    fn initial() -> Self { IsleId(1) }
    fn next(&self) -> Self { IsleId(self.0 + 1) }
}

#[derive(Clone, Debug)]
pub struct ParticlesState {
    pub fixed: Vec<FixedSpriteClipState>,
    pub moving: Vec<MovingSpriteClipState>,
}

#[derive(Copy, Clone, Hash, Eq, PartialEq, Debug)]
pub struct PlaneId(i64);

impl PoolKey for PlaneId {
    fn initial() -> Self { PlaneId(1) }
    fn next(&self) -> Self { PlaneId(self.0 + 1) }
}

#[derive(Clone, Debug)]
pub struct PlaneState {
    pub def: Resource<Plane>,
    pub trans: TransState,
    pub rot: RotState,
    pub desired_rot: Angle,
    pub gear: usize,
    pub effective_gear: usize,
    pub effective_gear_prev: Option<usize>,
    pub trail: Option<ParticleEmitterState>,
    pub primary: DeviceState,
    pub secondary: Option<DeviceState>,
    pub passive_buff: Option<Resource<Buff>>,
    pub passive_buffs: Vec<Resource<BuffSpec>>,
    pub durable: Durable,
    pub active_buffs: Vec<Resource<Buff>>,
    pub energy: f32,
}

#[derive(Clone, Debug)]
pub struct RpgState {
    pub xp: u32,
    pub level: u16,
    pub skill_points: u32,
    pub skills: HashMap<ImprovementId, u16>,
}

#[derive(Clone, Debug)]
pub struct DeviceState {
    pub spec: DeviceSpec,
    pub binding: Option<EquipmentBinding>,
    pub order: DeviceOrder,
}

#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq)]
pub struct DeviceOrder(pub i32);

#[derive(Copy, Clone, Hash, Eq, PartialEq, Debug, Ord, PartialOrd)]
pub struct DeviceId(i64);

impl PoolKey for DeviceId {
    fn initial() -> Self { DeviceId(1) }
    fn next(&self) -> Self { DeviceId(self.0 + 1) }
}

impl DeviceState {
    pub fn new(spec: DeviceSpec) -> Self {
        DeviceState { spec, binding: None, order: DeviceOrder(-9000) }
    }

    pub fn weapon(weapon: CannonState) -> Self {
        DeviceState { spec: DeviceSpec::Weapon(weapon), binding: None, order: DeviceOrder(-9000) }
    }

    pub fn booster(booster: ManualBuffState) -> Self {
        DeviceState { spec: DeviceSpec::Booster(booster), binding: None, order: DeviceOrder(-9000) }
    }
}

#[derive(Clone, Debug)]
pub enum DeviceSpec {
    Weapon(CannonState),
    Booster(ManualBuffState),
}

#[derive(Clone, Debug)]
pub enum Durable {
    Good {
        hp: f32,
        hp_prev: f32,
        pain_remaining_seconds: f32,
    },
    Destroyed(WeaponOwner),
}

#[derive(Clone, Debug)]
pub struct CannonState {
    pub pod: Resource<CannonPodProps>,
    pub def: Resource<Cannon>,
    pub recovery_seconds: f32,
    pub trigger: bool,
    pub ammo: Ammo,
}

#[derive(Clone, Debug)]
pub struct ManualBuffState {
    pub def: Resource<Buff>,
    pub trigger: bool,
    pub reserve: ManualBuffAmmo,
}

#[derive(Clone, Debug)]
pub enum ManualBuffAmmo {
    Hard { reserve_sec: f32 },
    Energy { energy_per_second: f32 },
}

impl ManualBuffState {
    pub fn new(def: Resource<Buff>, reserve: ManualBuffAmmo) -> ManualBuffState {
        ManualBuffState {
            def,
            trigger: false,
            reserve,
        }
    }
}

#[derive(Clone, Debug, Copy)]
pub enum Ammo {
    Infinite,
    Finite(u32),
    Energy { energy_per_shot: f32 },
}

impl CannonState {
    pub fn new(pod: &Resource<CannonPodProps>, def: &Resource<Cannon>, ammo: Ammo) -> Self {
        CannonState {
            pod: pod.clone(),
            def: def.clone(),
            recovery_seconds: 0.0,
            trigger: false,
            ammo,
        }
    }
}

#[derive(Clone, Debug)]
pub struct TransState {
    pub pos: Vec2,
    pub velocity: Vec2,
}

impl TransState {
    pub fn new(pos: Vec2) -> TransState {
        TransState { pos, velocity: Vec2::ZERO }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum WeaponOwner {
    Plane(PlaneId),
    Mob,
    Environment,
}

#[derive(Clone, Debug)]
pub struct ProjectileState {
    pub mods: Vec<ProjectileStateMod>,
    pub def: Resource<Projectile>,
    pub owner: WeaponOwner,
    pub trans: TransState,
    pub rot: RotState,
    pub remaining_seconds: Option<f32>,
    pub trail: Option<ParticleEmitterState>,
    pub exhaust_clip: Option<SpriteClipState>,
}

#[derive(Clone, Debug)]
pub enum ProjectileStateMod {
    Homing(ProjectileHomingState)
}

#[derive(Clone, Copy, Debug)]
pub enum ProjectileHomingState {
    Plane(PlaneId),
}

#[derive(Clone, Debug)]
pub struct RotState {
    pub angle: Angle,
    pub ang_velocity_rad: f32,
}

#[derive(Clone, Debug)]
pub struct ParticleEmitterState {
    pub def: Resource<TrailSource>,
    pub emission_queue: f32,
}

#[derive(Clone, Debug)]
pub struct BotState {
    pub plane: PlaneId,
    pub direction_x: f32,
}

#[derive(Clone, Debug)]
pub struct PlayerState {
    pub steering: bool,
    pub god: bool,
    pub plane: Option<PlaneId>,
    pub camera_pos: Vec2,
    pub resources: HashMap<GameResource, u32>,
    pub windows: VecDeque<UiWindow>,
    pub rpg: RpgState,
    pub equipment: Pool<DeviceId, DeviceState>,
    pub death_count: u32,
    pub hp_max: f32,
    pub energy_max: f32,
    pub thrust_tech_level: u32,
}

#[derive(Clone, Debug)]
pub enum UiWindow {
    Shop(ShopWindow),
    Improvements,
    Equipment(EquipmentWindow),
    Journal(JournalWindow),
    Help,
}

#[derive(Clone, Debug)]
pub struct JournalWindow {
    pub page: usize,
}

#[derive(Clone, Debug)]
pub struct ShopWindow {
    pub items: Vec<Resource<ShopLot>>,
}

#[derive(Clone, Debug)]
pub struct EquipmentWindow {
    pub selected_item: Option<DeviceId>,
}

#[derive(Clone, Debug)]
pub struct LimitedCannon {
    pub ammo: u32,
    pub def: Resource<Cannon>,
}


#[derive(Clone, Debug)]
pub struct WeaponSelector {
    pub items: Vec<Resource<Cannon>>,
    pub selected: usize,
}

#[derive(Clone, Debug)]
pub struct BackgroundObjectState {
    pub size: f32,
    pub pos: Vec3,
    pub sprite: Resource<Sprite>,
    pub material: Option<Resource<MaterialInstance>>,
}

#[derive(Clone, Debug)]
pub struct FixedSpriteClipState {
    pub clip: SpriteClipState,
    pub pos: Vec2,
    pub scale: f32,
    pub rot: Angle,
}

#[derive(Clone, Debug)]
pub struct SpriteClipState {
    pub clip: Resource<SpriteClip>,
    pub frame: f64,
    pub rate: f32,
    pub delay: f32,
}

#[derive(Clone, Debug)]
pub struct MovingSpriteClipState {
    pub clip: SpriteClipState,
    pub pos: Vec2,
    pub scale: f32,
    pub velocity: Vec2,
    pub rot: Angle,
}

#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
pub struct MobId(i32);

impl PoolKey for MobId {
    fn initial() -> Self { MobId(1) }
    fn next(&self) -> Self { MobId(self.0 + 1) }
}

#[derive(Clone, Debug)]
pub struct MobBaseState {
    pub gid: i64,
    pub mission: MobMission,
    pub def: Resource<Mob>,
    pub clip_state: Option<SpriteClipState>,
    pub phase: MobPhase,
    pub animation: Option<MobAnimation>,
    pub durable: Durable,
    pub xp_awarded: bool,
    pub death_initiated: bool,
    pub animation_ended: bool,
    pub debug_once: bool,
    pub debug: bool,
    pub charge_spent: bool,
    pub dir: f32,
    pub hold_effect: Option<CircleEffectState>,
    pub burst_rem: u16,
}

#[derive(Clone, Debug, Copy)]
pub enum MobMission {
    IsleGuard(IsleId),
}

#[derive(Clone, Debug)]
pub struct CircleEffectState {
    pub def: Resource<CircleEffect>,
    pub life_sec: f32,
}

#[derive(Clone, Copy, Debug)]
pub struct IsleTrans {
    pub pos_local: Vec2,
    pub isle_id: IsleId,
}

#[derive(Clone, Debug)]
pub struct MobState {
    pub base: MobBaseState,
    pub anchor: MobAnchor,
}

#[derive(Clone, Debug)]
pub enum MobAnchor {
    Isle(IsleId, WalkingMobState),
    Global(FlyingMobState),
}

#[derive(Clone, Debug)]
pub struct WalkingMobState {
    pub pos_local: Vec2,
}

#[derive(Clone, Debug)]
pub struct FlyingMobState {
    pub pos: Vec2,
    pub extra_velocity: Vec2,
    pub swing_phase: f32,
}

#[derive(Clone, Debug)]
pub enum MobPhase {
    WaitSeconds { action: WaitSecondsAction, seconds_remaining: f32 },
    WaitAnimationEnd(WaitAnimationEndAction),
}

#[derive(Clone, Debug)]
pub enum WaitSecondsAction {
    Idle,
    Move(MoveAction),
    Charge {
        velocity: Vec2,
        payload: Resource<MobAttack>,
    },
    AttackHold(Aim, Resource<MobAttack>),
    AttackBurst(Aim, BurstState),
}

#[derive(Clone, Debug)]
pub struct BurstState {
    pub def: Resource<Burst>,
    pub remaining_rounds: u16,
    pub attack: Resource<MobAttack>,
}

#[derive(Clone, Debug)]
pub enum Aim {
    Angle(Angle),
    Plane {
        plane: PlaneId,
        fallback: Angle,
    },
}

#[derive(Clone, Debug)]
pub enum MoveAction {
    IsleBound { velocity_x: f32 },
    FreeFly { velocity: Vec2 },
}

#[derive(Clone, Debug)]
pub enum WaitAnimationEndAction {
    LayDead,
    Die,
    AttackFinish(Resource<MobAttack>),
    AttackWindup(Aim, Resource<MobAttack>),
    AttackDeliver(Aim, Resource<MobAttack>),
}

#[derive(Clone, Debug)]
pub struct IsleState {
    pub order: i32,
    pub def: Resource<Isle>,
    pub trans: TransState,
    pub course: Vec2,
    pub course_change_interval_last: f32,
    pub course_seconds_remaining: f32,
    pub guard_count_threshold: u32,
    pub guard_rank: Option<MobRank>,
}

#[derive(Clone, Debug)]
pub struct LootState {
    pub def: Resource<Loot>,
    pub pos: RelativePos,
}

#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
pub struct LootId(i32);

impl PoolKey for LootId {
    fn initial() -> Self { LootId(1) }
    fn next(&self) -> Self { LootId(self.0 + 1) }
}

#[derive(Clone, Debug, Copy)]
pub enum RelativePos {
    Isle(IsleId, Vec2),
    Global(Vec2),
}

#[derive(Clone, Debug)]
pub struct RayState {
    pub def: Resource<HitScanRay>,
    pub trans: RayTrans,
    pub length: f32,
    pub life_sec: f32,
}

#[derive(Clone, Debug)]
pub enum RayTrans {
    Global {
        pos: Vec2,
        dir_norm: Vec2,
    },
    Plane(PlaneId),
}

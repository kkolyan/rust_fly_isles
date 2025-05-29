use std::cmp::min;
use std::collections::hash_map::IterMut;
use std::f32::consts::PI;
use std::iter::Filter;
use std::ops::Sub;
use std::rc::Rc;

use macroquad::math::clamp;
use macroquad::miniquad::debug;
use macroquad::prelude::error;
use rust_macroquad_ui::common::to_vec::ToVec;

use WaitAnimationEndAction::{AttackDeliver, LayDead};
use WaitSecondsAction::{AttackHold, Idle, Move};

use crate::{GameState, PlaneState, PlaneId, PlayerState, Vec2};
use crate::common::angle::Angle;
use crate::common::camera::ViewPort;
use crate::common::contract::{Get, GetMut, InsertSimple};
use crate::common::frame::FrameCtx;
use crate::common::pool::{Pool, PoolKey};
use crate::common::resource::Resource;
use crate::common::sprite_clip;
use crate::common::sprite_clip::do_nothing;
use crate::common::unsorted::{gen_range, RangeAdd, ToAngle};
use crate::game::{cannon, durable, isles, loot, mobs};
use crate::game::loot::LootPos;
use crate::model::def::{Burst, CannonPodProps, Isle, Mob, MobAnimation, MobAttack, MobAttackPattern};
use crate::model::state::{Aim, ProjectileState, Durable, FlyingMobState, Gids, IsleId, IsleState, LootState, MobAnchor, MobBaseState, MobPhase, MobState, MoveAction, RelativePos, RotState, SubSystems, SpriteClipState, TransState, WaitAnimationEndAction, WaitSecondsAction, WalkingMobState, WeaponOwner, GameCommand, MobId, DamageTarget, MobMission, BurstState};
use crate::model::state::Durable::{Destroyed, Good};
use crate::model::state::MobPhase::{WaitAnimationEnd, WaitSeconds};
use crate::model::state::WaitAnimationEndAction::{AttackFinish, AttackWindup, Die};
use crate::model::state::WaitSecondsAction::Charge;
use crate::resources::constants::{ANIM_END_PHASE_TIMEOUT_SEC, FLYING_SWING_ACCELERATION, FLYING_SWING_PERIOD, GRAVITY, PAIN_SECONDS};

pub fn spawn_mob<I>(pool: &mut I, def: &Resource<Mob>, pos: RelativePos, mission: MobMission, gids: &mut Gids)
    where I: InsertSimple<MobState>
{
    pool.insert_simple(MobState {
        base: spawn_mob_base(def, mission, gids),
        anchor: match pos {
            RelativePos::Isle(isle_id, pos) => MobAnchor::Isle(isle_id, WalkingMobState {
                pos_local: pos,
            }),
            RelativePos::Global(pos) => MobAnchor::Global(FlyingMobState {
                extra_velocity: Vec2::ZERO,
                pos,
                swing_phase: gen_range(0.0..1.0),
            }),
        },
    });
}

fn spawn_mob_base(def: &Resource<Mob>, mission: MobMission, gids: &mut Gids) -> MobBaseState {
    MobBaseState {
        gid: gids.next_gid(),
        mission,
        def: def.clone(),
        clip_state: None,
        phase: WaitSeconds { action: Idle, seconds_remaining: 0.0 },
        animation: None,
        durable: Durable::new(def.hp),
        xp_awarded: false,
        death_initiated: false,
        animation_ended: false,
        debug_once: false,
        debug: false,
        charge_spent: false,
        dir: 1.0,
        hold_effect: None,
        burst_rem: 0,
    }
}


fn is_reachable(isles: &Pool<IsleId, IsleState>, player: &PlayerState, mob: &MobState) -> bool {
    player.camera_pos.distance(mob.anchor.get_pos_rel().get_abs(isles)) < 2000.0
}

pub fn update(state: &mut GameState, dt: &FrameCtx) {
    state.reachable_mobs = state.mobs.iter()
        .filter(|(_, mob)| is_reachable(&state.isles, &state.player, mob))
        .map(|(id, _)| *id)
        .to_vec();

    for mob_id in state.reachable_mobs.iter() {
        if let Some(mob) = state.mobs.get_mut(mob_id) {
            match &mut mob.anchor {
                MobAnchor::Isle(isle_id, spec) => {
                    if let WaitSeconds { action: Move(move_action), .. } = &mob.base.phase {
                        match move_action {
                            MoveAction::IsleBound { velocity_x } => {
                                spec.pos_local.x += velocity_x * dt.dt;
                                mob.base.dir = velocity_x.signum();

                                let bounds = &state.isles.get_mut(isle_id).unwrap().def.bounds;
                                spec.pos_local.x = clamp(spec.pos_local.x, bounds.start, bounds.end);
                            }
                            MoveAction::FreeFly { .. } => {
                                panic!("WTF?")
                            }
                        }
                    }
                }
                MobAnchor::Global(spec) => {
                    if let Destroyed(..) = mob.base.durable {
                        spec.extra_velocity += Vec2::new(0.0, 4.0 * GRAVITY * dt.dt);
                    } else {
                        spec.swing_phase += dt.dt / FLYING_SWING_PERIOD;
                        spec.swing_phase = spec.swing_phase.fract();
                        spec.pos.y += (spec.swing_phase * PI * 2.0).cos() * FLYING_SWING_ACCELERATION * dt.dt;
                    }
                    spec.pos += spec.extra_velocity * dt.dt;
                }
            }

            let mut dead = false;
            update_mob_base(
                dt,
                &state.subsystems,
                &mut mob.base,
                &state.player,
                &state.planes,
                &state.isles,
                &mut state.projectiles,
                &mut state.commands,
                mob.anchor.get_pos_rel().get_abs(&state.isles),
                |base, planes| {
                    delegate_ai(&state.player, planes, base, &mut mob.anchor, dt, &state.isles);
                },
                || dead = true,
            );
            if dead {
                loot::spawn_loot(
                    &mob.base.def,
                    &state.progression,
                    &state.player,
                    &mut state.subsystems.loot,
                    &mut state.commands,
                    &state.loot,
                    &mob.anchor,
                )
            }

            match &mut mob.anchor {
                MobAnchor::Isle(_, _) => {}
                MobAnchor::Global(spec) => {
                    if let WaitSeconds { action: Move(move_action), .. } = &mob.base.phase {
                        match move_action {
                            MoveAction::IsleBound { .. } => {
                                panic!("WTF?")
                            }
                            MoveAction::FreeFly { velocity } => {
                                spec.pos += *velocity * dt.dt;
                                mob.base.dir = velocity.x.signum();
                            }
                        }
                    }
                    if let WaitSeconds { action: Charge { velocity, payload }, .. } = mob.base.phase.clone() {
                        spec.pos += velocity * dt.dt;
                        mob.base.dir = velocity.x.signum();
                        if !mob.base.charge_spent {
                            mob.base.charge_spent = deliver_attack(
                                &mut mob.base,
                                &state.player,
                                &state.planes,
                                &mut state.projectiles,
                                spec.pos,
                                velocity.to_angle(),
                                &payload,
                                &state.subsystems,
                                &state.isles,
                                &mut state.commands,
                            );
                        }
                    }
                }
            }
        }
    }

    sprite_clip::update_pool(dt.dt, mobs::animation_ended, &mut state.mobs, |it| it.base.clip_state.as_mut());
}

fn update_mob_base<F, D>(
    dt: &FrameCtx,
    settings: &SubSystems,
    mob: &mut MobBaseState,
    player: &PlayerState,
    planes: &Pool<PlaneId, PlaneState>,
    isles: &Pool<IsleId, IsleState>,
    projectiles: &mut Vec<ProjectileState>,
    commands: &mut impl InsertSimple<GameCommand>,
    pos: Vec2,
    mut decide: F,
    mut on_dead: D,
) where
    F: FnMut(
        &mut MobBaseState,
        &Pool<PlaneId, PlaneState>,
    ),
    D: FnMut()
{
    if mob.debug_once {
        mob.debug_once = false;
        debug!("{:?}", mob);
    }
    if mob.debug {
        // just for breakpoint
        mob.debug = true;
    }
    match &mob.durable {
        Durable::Good { .. } => {}
        Destroyed(_) => {
            if !mob.death_initiated {
                mob.death_initiated = true;
                set_phase(dt, mob, WaitAnimationEnd(Die));
                on_dead();
            }
        }
    }

    let mut time = dt.dt;
    loop {
        let mut repeat = false;

        if mob.animation_ended {
            mob.animation_ended = false;

            match mob.phase.clone() {
                WaitSeconds { .. } => {}
                WaitAnimationEnd(sub_phase) => match sub_phase {
                    LayDead => {}
                    Die => set_phase(dt, mob, WaitAnimationEnd(LayDead)),
                    AttackFinish(attack) => {
                        set_phase(dt, mob, WaitSeconds { action: Idle, seconds_remaining: attack.cooldown_sec.random() });
                        repeat = true;
                    }
                    AttackWindup(angle, attack) => {
                        set_phase(dt, mob, WaitSeconds {
                            action: AttackHold(angle, attack.clone()),
                            seconds_remaining: attack.hold_sec,
                        });
                        repeat = true;
                    }
                    AttackDeliver(aim, attack) => {
                        let angle = resolve_aim_angle(planes, &aim, pos);
                        if let Some(burst) = &attack.burst {
                            set_phase_burst(dt, mob, &aim, &attack, burst, burst.rounds_in_row);
                        } else {
                            deliver_attack(mob, player, planes, projectiles, pos, angle, &attack, settings, isles, commands);
                            set_phase(dt, mob, WaitAnimationEnd(AttackFinish(attack.clone())));
                        }
                        repeat = true;
                    }
                }
            }
        }
        let mut act = false;
        if let WaitSeconds { action: phase, seconds_remaining } = &mut mob.phase {
            *seconds_remaining -= time;
            if *seconds_remaining <= 0.0 {
                time = -*seconds_remaining;
                act = true;
            }
        }
        if act {
            if let WaitSeconds { action: phase, seconds_remaining } = &mob.phase.clone() {
                match phase {
                    WaitSecondsAction::Idle => decide(mob, planes),
                    WaitSecondsAction::Move { .. } => decide(mob, planes),
                    WaitSecondsAction::AttackHold(aim, attack) => {
                        let attack = attack.clone();
                        if let Some(charge) = &attack.charge {
                            let angle = resolve_aim_angle(planes, aim, pos);
                            mob.dir = angle.to_vec2_norm().x.signum();
                            mob.charge_spent = false;
                            set_phase(dt, mob, WaitSeconds {
                                action: WaitSecondsAction::Charge { velocity: angle.to_vec2_norm().normalize() * charge.velocity, payload: attack.clone() },
                                seconds_remaining: charge.duration_sec,
                            });
                            repeat = true;
                        } else {
                            let aim = aim.clone();
                            let angle = resolve_aim_angle(planes, &aim, pos);
                            mob.dir = angle.to_vec2_norm().x.signum();
                            set_phase(dt, mob, WaitAnimationEnd(AttackDeliver(aim, attack)));
                            repeat = true;
                        }
                    }
                    WaitSecondsAction::Charge { payload, .. } => {
                        let attack = payload.clone();
                        set_phase(dt, mob, WaitAnimationEnd(AttackFinish(attack)));
                        repeat = true;
                    }
                    WaitSecondsAction::AttackBurst(aim, burst) => {
                        let angle = resolve_aim_angle(planes, aim, pos);
                        let initial_angle = angle + Angle::degrees(burst.def.cannon.spread_degrees.random());
                        mob.dir = angle.to_vec2_norm().x.signum();
                        deliver_attack(mob, player, planes, projectiles, pos, initial_angle, &burst.attack, settings, isles, commands);
                        if burst.remaining_rounds > 0 {
                            set_phase_burst(dt, mob, aim, &burst.attack, &burst.def, burst.remaining_rounds - 1);
                        } else {
                            set_phase(dt, mob, WaitAnimationEnd(AttackFinish(burst.attack.clone())));
                        }
                        repeat = true;
                    }
                }
            }
        }
        if !repeat {
            break;
        }
    }
}

fn set_phase_burst(dt: &FrameCtx, mob: &mut MobBaseState, angle: &Aim, attack: &Resource<MobAttack>, burst: &Resource<Burst>, remaining_rounds: u16) {
    set_phase(dt, mob, WaitSeconds {
        action: WaitSecondsAction::AttackBurst(angle.clone(), BurstState {
            def: burst.clone(),
            remaining_rounds,
            attack: attack.clone(),
        }),
        seconds_remaining: 1.0 / burst.cannon.rate,
    });
}

fn deliver_attack(
    mob: &MobBaseState,
    player: &PlayerState,
    planes: &Pool<PlaneId, PlaneState>,
    projectiles: &mut Vec<ProjectileState>,
    pos: Vec2,
    angle: Angle,
    attack: &Resource<MobAttack>,
    settings: &SubSystems,
    isles: &Pool<IsleId, IsleState>,
    commands: &mut impl InsertSimple<GameCommand>,
) -> bool {
    let mut connected = false;
    match &attack.pattern {
        MobAttackPattern::Melee { connect_range, damage } => {
            let player = player.plane.and_then(|it| planes.get(&it).map(|v| (it, v)));
            if let Some((player_plane_id, player)) = player {
                let pos = pos + mob.def.collider_unscaled.center * mob.def.scale;
                let dir = player.trans.pos - pos;
                if (dir.x > 0.0) == (angle.to_vec2_norm().x > 0.0) && dir.length() < *connect_range {
                    commands.insert_simple(GameCommand::Damage {
                        target: DamageTarget::Plane(player_plane_id),
                        amount: damage.random(),
                        source: WeaponOwner::Mob,
                    });
                    connected = true;
                }
            }
        }
        MobAttackPattern::Distant { cannon } => {
            let pos = pos + calc_offset(mob);
            commands.insert_simple(GameCommand::FireCannon {
                cannon: cannon.clone(),
                owner: WeaponOwner::Mob,
                rot: RotState { angle, ang_velocity_rad: 0.0 },
                bal: TransState { pos, velocity: Vec2::ZERO },
                initial_angle: angle,
            });
        }
    }
    connected
}

pub fn calc_offset(mob: &MobBaseState) -> Vec2 {
    let mut offset = mob.def.pod.offset;
    if mob.get_dir() < 0.0 {
        offset.x *= -1.0;
    }
    offset
}

fn delegate_ai(
    player: &PlayerState,
    planes: &Pool<PlaneId, PlaneState>,
    base_mob: &mut MobBaseState,
    anchor: &mut MobAnchor,
    dt: &FrameCtx,
    isles: &impl Get<IsleId, IsleState>,
) {
    let enemies = player.plane
        .map_or_else(Vec::new, |it| vec![it]);
    let phase = match anchor {
        MobAnchor::Isle(isle_id, isle_mob) => {
            let isle = isles.get(isle_id).unwrap();
            make_decision_walker(&isle.trans, &isle.def, &enemies[..], planes, isle_mob, base_mob)
        }
        MobAnchor::Global(flier_mob) => {
            make_decision_flier(&enemies[..], planes, base_mob, flier_mob, dt)
        }
    };
    if let WaitAnimationEnd(AttackWindup(aim, _)) = &phase {
        let pos = anchor.get_pos_rel();
        let dir = match anchor {
            MobAnchor::Isle(isle_id, spec) => &mut base_mob.dir,
            MobAnchor::Global(spec) => &mut base_mob.dir,
        };
        *dir = resolve_aim_angle(planes, aim, pos.get_abs(isles)).to_vec2_norm().x.signum();
    }
    set_phase(dt, base_mob, phase);
    // set_phase(base_mob, WaitSeconds(Idle, 1.0));
}

fn resolve_aim_angle(planes: &Pool<PlaneId, PlaneState>, aim: &Aim, pos: Vec2) -> Angle {
    match aim {
        Aim::Angle(angle) => *angle,
        Aim::Plane { plane, fallback } => planes
            .get(&plane)
            .map(|it| (it.trans.pos - pos).to_angle())
            .unwrap_or(*fallback),
    }
}

fn set_phase(dt: &FrameCtx, mob: &mut MobBaseState, phase: MobPhase) {
    // println!("frame: {}, mob: {}, phase: {:?}", dt.frame, mob.gid, phase);
    let prev_animation = mob.animation;
    mob.phase = phase;
    let animation = resolve_animation(&mob.phase);
    if prev_animation.map(|it| it != animation).unwrap_or(true) {
        let clip = &mob.def.sprite_set.clips[&animation];
        mob.animation = Some(animation);
        mob.clip_state = Some(SpriteClipState::new(clip));
        if clip.frames.len() == 0 {
            mob.animation_ended = true;
        }
    }
}

fn resolve_animation(phase: &MobPhase) -> MobAnimation {
    match phase {
        WaitSeconds { action, .. } => match action {
            WaitSecondsAction::Idle => MobAnimation::Idle,
            WaitSecondsAction::Move { .. } => MobAnimation::Move,
            WaitSecondsAction::AttackHold(..) => MobAnimation::AttackHold,
            WaitSecondsAction::Charge { .. } => MobAnimation::AttackDeliver,
            WaitSecondsAction::AttackBurst(..) => MobAnimation::Burst,
        },
        WaitAnimationEnd(phase) => match phase {
            Die => MobAnimation::Die,
            AttackFinish(..) => MobAnimation::AttackFinish,
            LayDead => MobAnimation::Dead,
            AttackWindup(..) => MobAnimation::AttackWindup,
            AttackDeliver(..) => MobAnimation::AttackDeliver,
        },
    }
}

fn make_decision_walker(isle_trans: &TransState, isle_def: &Isle, enemies: &[PlaneId], planes: &Pool<PlaneId, PlaneState>, state: &WalkingMobState, base_state: &MobBaseState) -> MobPhase {
    let def = base_state.def.clone();
    let nearest_enemy = get_nearest_enemy(enemies, planes, isle_trans.pos + state.pos_local, &def);

    if let Some((distance, dir, nearest_enemy, nearest_enemy_id)) = nearest_enemy
    {
        for attack in &def.attacks {
            if distance <= attack.trigger_range {
                let aim = if attack.late_aim {
                    Aim::Plane { plane: nearest_enemy_id, fallback: dir.to_angle() }
                } else {
                    Aim::Angle(dir.to_angle())
                };
                return WaitAnimationEnd(AttackWindup(aim, attack.clone()));
            }
        }
    }

    match gen_range(0.0..1.0) {
        action_rng if action_rng < 0.3 => {
            let pos_norm = {
                let rel_pos = state.pos_local.x - isle_def.bounds.start;
                rel_pos / (isle_def.bounds.end - isle_def.bounds.start)
            };
            let dir = match gen_range(0.0..1.0) {
                dir_rng if dir_rng < 0.5 => -1.0,
                _ => 1.0,
            };
            WaitSeconds { action: Move(MoveAction::IsleBound { velocity_x: def.move_speed * dir }), seconds_remaining: def.move_seconds.random() }
        }
        _ => WaitSeconds { action: Idle, seconds_remaining: def.idle_seconds.random() }
    }
}

fn make_decision_flier(enemies: &[PlaneId], planes: &Pool<PlaneId, PlaneState>, base_state: &MobBaseState, flier_mob: &FlyingMobState, dt: &FrameCtx) -> MobPhase {
    let def = base_state.def.clone();
    let nearest_enemy = get_nearest_enemy(enemies, planes, flier_mob.pos, &def);

    if let Some((distance, dir, nearest_enemy, nearest_enemy_id)) = nearest_enemy
    {
        for attack in &def.attacks {
            if gen_range(0.0..1.0) <= base_state.def.attack_chance {
                if distance <= attack.trigger_range {
                    let aim = if attack.late_aim {
                        Aim::Plane { plane: nearest_enemy_id, fallback: dir.to_angle() }
                    } else {
                        Aim::Angle(dir.to_angle())
                    };
                    return WaitAnimationEnd(AttackWindup(aim, attack.clone()));
                }
            }
        }

        if distance <= base_state.def.flier_aggro_distance {
            let step = def.flier_chase_step.min(distance / (def.move_speed * dt.dt));
            return WaitSeconds { action: Move(MoveAction::FreeFly { velocity: dir.normalize() * def.move_speed }), seconds_remaining: def.flier_chase_step };
        }
    }

    match gen_range(0.0..1.0) {
        action_rng if action_rng < 0.3 => {
            let dir = match gen_range(0.0..1.0) {
                dir_rng if dir_rng < 0.5 => -1.0,
                _ => 1.0,
            };
            WaitSeconds { action: Move(MoveAction::FreeFly { velocity: Vec2::new(def.move_speed * dir, 0.0) }), seconds_remaining: def.move_seconds.random() }
        }
        _ => WaitSeconds { action: Idle, seconds_remaining: def.idle_seconds.random() }
    }
}

fn get_nearest_enemy<'a>(
    enemies: &[PlaneId],
    planes: &'a Pool<PlaneId, PlaneState>,
    pos: Vec2,
    def: &Resource<Mob>,
) -> Option<(f32, Vec2, &'a PlaneState, PlaneId)> {
    enemies.iter()
        .filter_map(|id| planes.get(id).map(|it| (it, *id)))
        .map(|(it, id)| (it.trans.pos.sub(pos + def.collider_unscaled.center * def.scale), it, id))
        .map(|(dir, enemy, id)| (dir.length(), dir, enemy, id))
        .min_by_key(|(distance, _, _, _)| *distance as i32)
}

fn animation_ended(mobs: &mut Pool<MobId, MobState>, index: MobId) {
    if let Some(mob) = mobs.get_mut(&index) {
        mob.base.animation_ended = true;
    }
}

impl RelativePos {
    pub fn get_abs(self, isles: &impl Get<IsleId, IsleState>) -> Vec2 {
        match self {
            RelativePos::Isle(isle_id, offset) => isles.get(&isle_id).unwrap().trans.pos + offset,
            RelativePos::Global(pos) => pos,
        }
    }
}

impl MobAnchor {
    pub fn get_pos_rel(&self) -> RelativePos {
        match self {
            MobAnchor::Isle(isle_id, spec) => RelativePos::Isle(*isle_id, spec.pos_local),
            MobAnchor::Global(spec) => RelativePos::Global(spec.pos),
        }
    }
}

impl MobBaseState {
    pub fn get_dir(&self) -> f32 {
        self.dir
    }
}

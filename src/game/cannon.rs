use std::cell::Cell;
use std::collections::VecDeque;
use std::ops::Not;
use std::rc::Rc;

use macroquad::math::Rect;
use macroquad::prelude::{Mat2, Vec2};
use macroquad::prelude::WHITE;
use macroquad::shapes::draw_line;
use rust_macroquad_ui::common::to_vec::ToVec;

use crate::{GameState, PlaneState, PlaneId, PlayerState};
use crate::common::{physics, sprite};
use crate::common::angle::{Angle, AsRadians};
use crate::common::resource::Resource;
use crate::common::camera::ViewPort;
use crate::common::contract::{Get, InsertSimple};
use crate::common::curve::Curve;
use crate::common::frame::FrameCtx;
use crate::common::line_circle::Line;
use crate::common::metrics::Metrics;
use crate::common::pool::Pool;
use crate::common::sound::PlaySound;
use crate::common::sprite::draw_sprite;
use crate::common::unsorted::{gen_range, IndexRange, ToAngle, WithMut};
use crate::game::{particles, plane};
use crate::model::def::{Projectile, Cannon, CannonBarrel, CollisionCircle, Location, GameSound, Collider, Mob, CollisionRay, HitScanLook, ProjectileRot, ProjectileMod};
use crate::model::state::{AudioManager, ProjectileState, CannonState, Durable, IsleId, IsleState, MobBaseState, MobState, ParticleEmitterState, ParticlesState, RotState, SpriteClipState, SubSystems, TransState, WeaponOwner, GameCommand, MobId, RayState, ProjectileHomingState, ProjectileStateMod, Ammo, RayTrans, DeviceSpec};
use crate::model::state::DamageTarget;

pub fn update_projectiles(state: &mut GameState, dt: &FrameCtx, vp: &ViewPort) {
    for i in state.projectiles.indices().rev() {
        let mut projectile = state.projectiles.get_mut(i).unwrap();

        let action = update_projectile(
            dt,
            &state.location,
            &state.metrics,
            projectile,
            &state.planes,
            &mut state.isles,
            &mut state.particles,
            &state.mobs,
            &state.subsystems,
            vp,
            &state.player,
            &mut state.commands,
            &state.reachable_mobs,
        );
        match action {
            HitAction::Proceed => {}
            HitAction::Hit { .. } => {
                let projectile = state.projectiles.remove(i);
                if let Some(explosion) = &projectile.def.explosion {
                    let pos = TransState {
                        pos: projectile.trans.pos + Mat2::from_angle(projectile.rot.angle.to_rad()) * explosion.offset,
                        //explosion shouldn't move as missile
                        velocity: Vec2::ZERO,
                    };
                    particles::emit_explosion(&explosion.explosion, &pos, &mut state.particles, &state.subsystems);
                }
                if let Some(splash_damage) = &projectile.def.splash_damage {
                    for mob_id in &state.reachable_mobs {
                        if let Some(mob) = state.mobs.get(mob_id) {
                            let mob_pos = mob.anchor.get_pos_rel().get_abs(&state.isles) + mob.base.def.collider_unscaled.center * mob.base.def.scale;
                            let mob_radius = mob.base.def.collider_unscaled.radius * mob.base.def.scale;
                            let distance = mob_pos.distance(projectile.trans.pos) - mob_radius;
                            let distance_norm = (distance / splash_damage.radius).max(0.0);
                            if distance_norm < 1.0 {
                                state.commands.insert_simple(GameCommand::Damage {
                                    amount: splash_damage.damage.random() * splash_damage.damage_factor_by_distance_norm.lerp(distance_norm),
                                    source: projectile.owner,
                                    target: DamageTarget::Mob(*mob_id),
                                })
                            }
                        }
                    }
                }
            }
            HitAction::SilentDiscard => {
                state.projectiles.remove(i);
            }
        }
    }

    for i in state.rays.indices().rev() {
        let ray = state.rays.get_mut(i).unwrap();
        ray.life_sec += dt.dt;
        if ray.life_sec > ray.def.duration_sec {
            state.rays.remove(i);
        }
    }
}

fn is_on_screen(vp: &ViewPort, projectile: &ProjectileState) -> bool {
    let collider_abs = CollisionCircle {
        center: projectile.trans.pos,
        radius: projectile.def.collision_radius,
    };
    let on_screen = Cell::new(false);
    vp.port(collider_abs.center, 1.0, |ported| {
        let r = collider_abs.radius * ported.screen_scale;
        if Rect::new(-r, -r, vp.screen_size.x + r, vp.screen_size.y + r).contains(ported.screen_pos) {
            on_screen.set(true);
        }
    });
    on_screen.get()
}

enum HitAction {
    Proceed,
    Hit { point: Vec2 },
    SilentDiscard,
}

fn update_projectile(
    dt: &FrameCtx,
    location: &Location,
    metrics: &Metrics,
    projectile: &mut ProjectileState,
    planes: &Pool<PlaneId, PlaneState>,
    isles: &mut Pool<IsleId, IsleState>,
    particles: &mut ParticlesState,
    mobs: &Pool<MobId, MobState>,
    settings: &SubSystems,
    vp: &ViewPort,
    player: &PlayerState,
    commands: &mut impl InsertSimple<GameCommand>,
    reachable_mobs: &Vec<MobId>,
) -> HitAction {
    let timeout = if let Some(rem) = &mut projectile.remaining_seconds {
        if *rem <= 0.0 {
            true
        } else {
            *rem -= dt.dt;
            false
        }
    } else { false };

    if timeout && !is_on_screen(vp, projectile) {
        return HitAction::SilentDiscard;
    }

    let is_player = player.plane
        .map(|it| WeaponOwner::Plane(it) == projectile.owner)
        .unwrap_or(false);
    if is_player && !is_on_screen(vp, projectile) {
        return HitAction::SilentDiscard;
    }

    let mut homing = Option::None;

    for m in &projectile.mods {
        match m {
            ProjectileStateMod::Homing(m) => {
                match m {
                    ProjectileHomingState::Plane(target) => {
                        if let Some(target) = planes.get(&target) {
                            homing = Some((target.trans.pos - projectile.trans.pos).to_angle());
                        }
                    }
                }
            }
        }
    }

    if let Some(stabilization) = &projectile.def.stabilization {
        let desired_rot = homing.unwrap_or(projectile.rot.angle);
        physics::apply_steering(&mut projectile.trans, &mut projectile.rot, &stabilization.steer, desired_rot, metrics, dt.dt);
    } else {
        assert!(homing.is_none(), "homing not supported for projectiles without stabilization")
    }
    if let Some(acceleration) = &projectile.def.acceleration {
        physics::apply_thrust(metrics, dt.dt, *acceleration, &projectile.rot, &mut projectile.trans)
    }
    physics::apply_gravity(&mut projectile.trans, metrics, dt.dt);
    physics::apply_drag(&mut projectile.trans, &metrics, dt.dt, dt.dt);
    if let Some(stabilization) = &projectile.def.stabilization {
        physics::apply_slide(metrics, dt.dt, dt.dt, &mut projectile.trans, &projectile.rot, &stabilization.slide);
    }
    physics::apply_velocity(&mut projectile.trans, &location, dt.dt);
    physics::apply_rotation(&mut projectile.rot, dt.dt);

    if let Some(trail) = &mut projectile.trail {
        particles::update_particles(trail, &projectile.trans, &projectile.rot, dt.dt, particles, 1.0);
    }

    try_hit(
        planes,
        isles,
        mobs,
        commands,
        settings,
        Threat {
            pos: projectile.trans.pos,
            source: projectile.owner,
            damage: projectile.def.damage.clone(),
            hit_sound: projectile.def.hit_sound.clone(),
            collider: Collider::Ray(CollisionRay {
                origin: projectile.trans.pos,
                dir: projectile.trans.velocity.normalize(),
                distance: projectile.trans.velocity.length() * dt.dt,
                thickness: projectile.def.collision_radius,
            }),
        },
        reachable_mobs,
    )
}

struct Threat {
    pos: Vec2,
    source: WeaponOwner,
    damage: Curve<f32>,
    hit_sound: Option<Resource<GameSound>>,
    collider: Collider,
}

fn check_ray(ray: CollisionRay, candidate_pos: Vec2, candidate_radius: f32) -> bool {
    // for "intrusiveness", especially for explosive missiles
    let effective_radius = candidate_radius + ray.thickness;
    let effective_origin = ray.origin - ray.dir * ray.thickness;
    let line = Line { p1: effective_origin, p2: effective_origin + ray.dir.normalize() * ray.distance };
    line.p1.distance(candidate_pos) < effective_radius
        || line.p2.distance(candidate_pos) < effective_radius
        || line.circle_intersections(candidate_pos.x, candidate_pos.y, effective_radius, true).is_empty().not()
}

fn try_hit(
    planes: &Pool<PlaneId, PlaneState>,
    isles: &Pool<IsleId, IsleState>,
    mobs: &Pool<MobId, MobState>,
    commands: &mut impl InsertSimple<GameCommand>,
    subsystems: &SubSystems,
    threat: Threat,
    reachable_mobs: &Vec<MobId>,
) -> HitAction {
    for (id, plane) in planes.iter() {
        match plane.durable {
            Durable::Good { .. } => {}
            Durable::Destroyed(_) => continue,
        }
        let me = match threat.source {
            WeaponOwner::Plane(plane_id) => plane_id == *id,
            WeaponOwner::Mob => false,
            WeaponOwner::Environment => false,
        };
        if me {
            continue;
        }
        let hit = match threat.collider {
            Collider::Circle(collider) => {
                let distance = plane.trans.pos.distance(collider.center);
                distance < collider.radius + plane.def.collision_radius
            }
            Collider::Ray(ray) => {
                check_ray(ray, plane.trans.pos, plane.def.collision_radius)
            }
        };
        if hit {
            commands.insert_simple(GameCommand::Damage {
                amount: threat.damage.random(),
                source: threat.source,
                target: DamageTarget::Plane(*id),
            });
            threat.hit_sound.play_once(&subsystems.audio);
            return HitAction::Hit { point: plane.trans.pos };
        }
    }

    let mut sorted_mobs = reachable_mobs.iter()
        .filter_map(|it| mobs.get(it).map(|mob| (it, mob)))
        .to_vec();
    sorted_mobs.sort_by_key(|(_, it)| it.anchor.get_pos_rel().get_abs(isles).distance(threat.pos) as i32);
    for (id, mob) in sorted_mobs {
        if let Some(hit) = try_hit_mob(
            *id,
            &mob.base,
            mob.anchor.get_pos_rel().get_abs(isles),
            subsystems,
            &threat,
            commands,
        ) {
            return hit;
        }
    }
    HitAction::Proceed
}

fn try_hit_mob(
    mob_id: MobId,
    base_state: &MobBaseState,
    mob_pos: Vec2,
    settings: &SubSystems,
    threat: &Threat,
    commands: &mut impl InsertSimple<GameCommand>,
) -> Option<HitAction> {
    let me = match threat.source {
        WeaponOwner::Plane(plane_id) => false,
        WeaponOwner::Mob => true,
        WeaponOwner::Environment => false,
    };
    if me {
        return None;
    }
    if let Durable::Good { ref hp, .. } = base_state.durable {
        if let Some(clip) = &base_state.clip_state {
            let candidate_center = base_state.def.collider_unscaled.center * base_state.def.scale + mob_pos;

            let hit = match threat.collider {
                Collider::Circle(collider) => {
                    let distance = candidate_center.distance(collider.center);
                    distance < collider.radius + base_state.def.collider_unscaled.radius * base_state.def.scale
                }
                Collider::Ray(ray) => {
                    check_ray(ray, candidate_center, base_state.def.collider_unscaled.radius * base_state.def.scale)
                }
            };
            if hit {
                let damage = threat.damage.random();
                commands.insert_simple(GameCommand::Damage {
                    target: DamageTarget::Mob(mob_id),
                    source: threat.source,
                    amount: damage,
                });
                threat.hit_sound.play_once(&settings.audio);
                return Some(HitAction::Hit { point: mob_pos });
            }
        }
    }
    None
}

pub fn update_cannon(
    cannon: &mut CannonState,
    bal: &TransState,
    rot: &RotState,
    owner: WeaponOwner,
    frame: &FrameCtx,
    commands: &mut impl InsertSimple<GameCommand>,
    energy: &mut f32,
) {
    if cannon.recovery_seconds > 0.0 {
        cannon.recovery_seconds -= frame.dt;
    }
    if cannon.trigger && cannon.recovery_seconds <= 0.0 {
        cannon.recovery_seconds += 1.0 / cannon.def.rate;

        let enough_ammo = match &mut cannon.ammo {
            Ammo::Infinite => { true }
            Ammo::Finite(ammo) => {
                if *ammo < 1 {
                    false
                } else {
                    *ammo -= 1;
                    true
                }
            }
            Ammo::Energy { energy_per_shot } => {
                if *energy < *energy_per_shot {
                    false
                } else {
                    *energy -= *energy_per_shot;
                    true
                }
            }
        };

        if enough_ammo {
            let initial_angle = rot.angle + Angle::degrees(cannon.def.spread_degrees.random());
            commands.insert_simple(GameCommand::FireCannon {
                bal: bal.clone(),
                rot: rot.clone(),
                owner,
                cannon: cannon.def.clone(),
                initial_angle,
            });
        }
    }
}

pub fn fire(
    player: &PlayerState,
    trans: &TransState,
    rot: &RotState,
    owner: WeaponOwner,
    def: &CannonBarrel,
    initial_angle: Angle,
    projectiles: &mut Vec<ProjectileState>,
    cannon: &Cannon,
    settings: &SubSystems,
    planes: &Pool<PlaneId, PlaneState>,
    isles: &Pool<IsleId, IsleState>,
    mobs: &Pool<MobId, MobState>,
    reachable_mobs: &Vec<MobId>,
    commands: &mut impl InsertSimple<GameCommand>,
) {
    cannon.sound.play_once(&settings.audio);
    match def {
        CannonBarrel::Projectile(def) => {
            let mut mods = vec![];
            for m in &def.mods {
                match m {
                    ProjectileMod::Homing => {
                        if let Some(player) = player.plane {
                            mods.push(ProjectileStateMod::Homing(ProjectileHomingState::Plane(player)));
                        }
                    }
                }
            }
            projectiles.push(ProjectileState {
                mods,
                def: def.clone(),
                owner,
                trans: TransState {
                    pos: trans.pos,
                    velocity: trans.velocity + (initial_angle).to_vec2_norm() * def.body.initial_speed,
                },
                rot: rot.clone().with_mut(|it| {
                    it.ang_velocity_rad = match &def.rotation {
                        ProjectileRot::InitialVelocity => 0.0,
                        ProjectileRot::Spinning { degrees_per_second } => degrees_per_second.to_radians(),
                    }
                }),
                remaining_seconds: def.body.seconds_to_live,
                trail: def.trail.as_ref().map(|trail_pod| ParticleEmitterState {
                    def: trail_pod.clone(),
                    emission_queue: 0.0,
                }),
                exhaust_clip: def.exhaust_clip.as_ref().map(|it| {
                    SpriteClipState::new(it)
                }),
            })
        }
        CannonBarrel::HitScan(hit_scan) => {
            let result = try_hit(
                planes,
                isles,
                mobs,
                commands,
                settings,
                Threat {
                    pos: trans.pos,
                    source: owner,
                    damage: hit_scan.action.damage.clone(),
                    hit_sound: None,
                    collider: Collider::Ray(CollisionRay {
                        origin: trans.pos,
                        dir: initial_angle.to_vec2_norm(),
                        distance: hit_scan.action.range,
                        thickness: hit_scan.action.collider_thickness,
                    }),
                },
                reachable_mobs,
            );
            match &hit_scan.look {
                HitScanLook::None => {}
                HitScanLook::Ray(ray) => {
                    let mut length = hit_scan.action.range;
                    if let HitAction::Hit { point } = result {
                        length = trans.pos.distance(point);
                    }
                    match owner {
                        WeaponOwner::Plane(plane) => {
                            commands.insert_simple(GameCommand::NewRay(RayState {
                                def: ray.clone(),
                                trans: RayTrans::Plane(plane),
                                length,
                                life_sec: 0.0,
                            }));
                        }
                        WeaponOwner::Mob | WeaponOwner::Environment => {
                            commands.insert_simple(GameCommand::NewRay(RayState {
                                def: ray.clone(),
                                trans: RayTrans::Global {
                                    pos: trans.pos,
                                    dir_norm: initial_angle.to_vec2_norm(),
                                },
                                length,
                                life_sec: 0.0,
                            }));
                        }
                    }
                }
            }
        }
    }
}

pub fn draw_projectiles(state: &GameState, view_port: &ViewPort) {
    for projectile in &state.projectiles {
        if let Some(sprite) = &projectile.def.body.sprite {
            view_port.port(projectile.trans.pos, 1.0, |ported| {
                draw_sprite(sprite, ported.screen_pos, |it| {
                    it.screen_scale = ported.screen_scale;
                    it.angle = projectile.rot.angle;
                    if let Some(pulse) = &projectile.def.pulsation {
                        it.screen_scale *= pulse.scale.random();
                    }
                });
            });
        }
    }

    for ray in &state.rays {
        let f = ray.life_sec / ray.def.duration_sec;
        let width = ray.def.width.lerp(f);
        let color = ray.def.color.lerp(f);
        let trans = match ray.trans {
            RayTrans::Global { pos, dir_norm } => { Some((pos, dir_norm)) }
            RayTrans::Plane(plane) => {
                state.planes.get(&plane).map(|plane| {
                    let offset = match &plane.primary.spec {
                        DeviceSpec::Weapon(weapon) => { plane::pod_offset(weapon, &plane.rot) }
                        DeviceSpec::Booster(_) => { Vec2::ZERO }
                    };
                    let pos = (plane.trans.pos + offset);
                    let dir_norm = plane.rot.angle.to_vec2_norm();
                    (pos, dir_norm)
                })
            }
        };
        if let Some((pos, dir_norm)) = trans {
            view_port.port(pos, 1.0, |ported| {
                let p0 = ported.screen_pos;
                let p1 = ported.screen_pos + dir_norm * ported.screen_scale * ray.length;
                draw_line(p0.x, p0.y, p1.x, p1.y, width, color);
            });
        }
    }
}

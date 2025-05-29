use std::f32::consts::PI;
use macroquad::color::{RED, WHITE, YELLOW};
use crate::common::curve::{Curve};
use crate::common::resource::{Resource, ResourceLoad, ResourceLoadAsync};
use crate::model::def::{Cannon, CannonBarrel, ExplosionSource, HitScan, HitScanAction, HitScanLook, HitScanRay, Projectile, ProjectileMod, ProjectilePulsation, ProjectileRot, SlideStabilization, SplashDamage, Sprite, Stabilization, SteerStabilization, TrailSource, TransientBallisticBody};
use crate::{ResourceGet, ResourceManager, Vec2};
use crate::common::curve::Point::{Transition, Value};
use crate::resources::constants::{DRAG, MISSILE_THRUST, SCALE_SPEED, SLIDE};
use crate::resources::fx::settings::{exhaust_trail_missile, missile_explosion_simple};
use crate::resources::objects::objects::standard_stabilization;
use crate::resources::sounds::{sound_cannon_001, sound_cannon_002, sound_hit_001, sound_missile_001, sound_plasma, sound_rail};
use crate::resources::sprites::fire_facepalm11_clip::fire_facepalm11_clip;
use crate::resources::sprites::sprites::{bullet_sprite, bullet_sprite_big, missile_sprite, missile_sprite_yellow, sprite_plasma_001};

pub const cannon_default: ResourceLoad<Cannon> = |rm| Cannon {
    title: "Auto Cannon",
    rate: 6.0,
    barrel: CannonBarrel::Projectile(bullet(&rm, 1200.0, Curve::new([8.0, 10.0]), bullet_sprite)),
    spread_degrees: Curve::new([
        -2.0,
        2.0,
    ]),
    sound: Some(sound_cannon_001.get(&rm)),
};

pub const cannon_gatling: ResourceLoad<Cannon> = |rm| Cannon {
    title: "Gatling Cannon",
    rate: 12.0,
    barrel: CannonBarrel::Projectile(bullet(&rm, 1200.0, Curve::new([6.0, 12.0]), bullet_sprite)),
    spread_degrees: Curve::new([
        -4.0,
        4.0,
    ]),
    sound: Some(sound_cannon_001.get(&rm)),
};

pub const cannon_gatling_robot: ResourceLoad<Cannon> = |rm| Cannon {
    title: "Gatling Cannon Jagger",
    rate: 20.0,
    barrel: CannonBarrel::Projectile(bullet(&rm, 1200.0, Curve::new([6.0, 10.0]), bullet_sprite_big)),
    spread_degrees: Curve::new([
        -4.0,
        4.0,
    ]),
    sound: Some(sound_cannon_001.get(&rm)),
};

pub const cannon_plasma: ResourceLoad<Cannon> = |rm| Cannon {
    title: "Auto Plasma",
    rate: 6.0,
    barrel: CannonBarrel::Projectile(bullet_ext(
        &rm,
        600.0,
        Curve::new([10.0, 30.0]),
        ProjectileRot::Spinning { degrees_per_second: 12345.0 },
        Some(ProjectilePulsation { scale: Curve::new([0.5, 1.3]) }),
        1.5,
        sprite_plasma_001
    )),
    spread_degrees: Curve::new([
        -2.0,
        2.0,
    ]),
    sound: Some(sound_plasma.get(&rm)),
};

pub const cannon_rail: ResourceLoad<Cannon> = |rm| Cannon {
    title: "Mob Laser",
    rate: 2.0,
    barrel: CannonBarrel::HitScan(Resource::detached( HitScan {
        action: HitScanAction {
            damage: Curve::new([40.0, 80.0]),
            range: 1600.0,
            collider_thickness: 0.0
        },
        look: HitScanLook::Ray(Resource::detached(HitScanRay{
            width: Curve::new([7.0, 0.0]),
            color: Curve::new([WHITE, YELLOW, RED]),
            duration_sec: 0.3
        })),
    })),
    spread_degrees: Curve::new([
        0.0,
    ]),
    sound: Some(sound_rail.get(&rm)),
};

pub const cannon_rail2: ResourceLoad<Cannon> = |rm| {
    let rate = 8.0;
    Cannon {
        title: "Impulse Laser",
        rate,
        barrel: CannonBarrel::HitScan(Resource::detached(HitScan {
            action: HitScanAction {
                damage: Curve::new([10.0, 15.0]),
                range: 1600.0,
                collider_thickness: 0.0
            },
            look: HitScanLook::Ray(Resource::detached(HitScanRay {
                width: Curve::new([5.0, 0.0]),
                color: Curve::new([WHITE, YELLOW]),
                duration_sec: 1.0 / rate
            })),
        })),
        spread_degrees: Curve::new([
            0.0,
        ]),
        sound: Some(sound_rail.get(&rm)),
    }
};

pub const cannon_rail_player: ResourceLoad<Cannon> = |rm| {
    let rate = 8.0;
    Cannon {
        title: "Impulse Laser",
        rate,
        barrel: CannonBarrel::HitScan(Resource::detached(HitScan {
            action: HitScanAction {
                damage: Curve::new([10.0, 15.0]),
                range: 1600.0,
                collider_thickness: 0.0
            },
            look: HitScanLook::Ray(Resource::detached(HitScanRay {
                width: Curve::new([5.0, 0.0]),
                color: Curve::new([WHITE, YELLOW]),
                duration_sec: 1.0 / rate
            })),
        })),
        spread_degrees: Curve::new([
            0.0,
        ]),
        sound: Some(sound_rail.get(&rm)),
    }
};

pub const launcher_player: ResourceLoad<Cannon> = |rm| Cannon {
    title: "Missile Launcher",
    rate: 0.9,
    barrel: CannonBarrel::Projectile(Resource::detached(Projectile {
        body: TransientBallisticBody {
            initial_speed: 0.0,
            sprite: Some(missile_sprite.get(&rm)),
            drag: DRAG,
            seconds_to_live: Some(2.0),
        },
        collision_radius: 32.0 * SCALE_SPEED,
        splash_damage: Some(SplashDamage {
            damage: Curve::new([30.0, 70.0]),
            radius: 200.0,
            damage_factor_by_distance_norm: Curve::new([1.0, 0.0]),
        }),
        damage: Curve::new([0.0]),
        explosion: Some(ExplosionSource {
            offset: Vec2::new(60.0, 0.0),
            explosion: missile_explosion_simple.get(&rm),
        }),
        acceleration: Some(MISSILE_THRUST),
        stabilization: Some(standard_stabilization.get(&rm)),
        trail: Some(Resource::detached(TrailSource {
            emitter: exhaust_trail_missile.get(&rm),
            offset: Vec2::new(-40.0, 0.0),
        })),
        exhaust_clip: Some(fire_facepalm11_clip.get(&rm)),
        hit_sound: None,
        rotation: ProjectileRot::InitialVelocity,
        pulsation: None,
        mods: vec![]
    })),
    spread_degrees: Curve::new([
        0.0,
    ]),
    sound: Some(sound_missile_001.get(&rm)),
};

pub const launcher_jagger: ResourceLoad<Cannon> = |rm| Cannon {
    title: "Missiles (Jagger)",
    rate: 1.0,
    barrel: missile_mob(&rm, 0.0, 16.0, Curve::new([30.0, 120.0]), vec![], &missile_sprite_yellow),
    spread_degrees: Curve::new([
        0.0,
    ]),
    sound: Some(sound_missile_001.get(&rm)),
};

pub const launcher_jagger_homing: ResourceLoad<Cannon> = |rm| Cannon {
    title: "Homing Missiles (Jagger)",
    rate: 1.0,
    barrel: missile_mob(&rm, 0.0, 32.0, Curve::new([30.0, 120.0]), vec![ProjectileMod::Homing], &missile_sprite_yellow),
    spread_degrees: Curve::new([
        0.0,
    ]),
    sound: Some(sound_missile_001.get(&rm)),
};

pub const launcher_jagger_homing_fast: ResourceLoad<Cannon> = |rm| Cannon {
    title: "Homing Missiles (Jagger)",
    rate: 3.0,
    barrel: missile_mob(&rm, 0.0, 32.0, Curve::new([10.0, 40.0]), vec![ProjectileMod::Homing], &missile_sprite_yellow),
    spread_degrees: Curve::new([
        0.0,
    ]),
    sound: Some(sound_missile_001.get(&rm)),
};

fn bullet(rm: &ResourceManager, initial_speed: f32, damage: Curve<f32>, sprite: ResourceLoadAsync<Sprite>) -> Resource<Projectile> {
    bullet_ext(rm, initial_speed, damage, ProjectileRot::InitialVelocity, None, 1.0, sprite)
}

fn bullet_ext(rm: &ResourceManager, initial_speed: f32, damage: Curve<f32>, rotation: ProjectileRot, pulsation: Option<ProjectilePulsation>, seconds_to_live: f32, sprite: ResourceLoadAsync<Sprite>) -> Resource<Projectile> {
    Resource::detached(Projectile {
        body: TransientBallisticBody {
            initial_speed,
            sprite: Some(sprite.get(&rm)),
            drag: DRAG,
            seconds_to_live: Some(seconds_to_live),
        },
        collision_radius: 8.0 * SCALE_SPEED,
        splash_damage: None,
        damage,
        explosion: None,
        acceleration: None,
        stabilization: None,
        trail: None,
        exhaust_clip: None,
        hit_sound: None,
        rotation,
        pulsation,
        mods: vec![]
    })
}

fn missile_mob(rm: &ResourceManager, initial_speed: f32, collision_radius: f32, damage: Curve<f32>, mods: Vec<ProjectileMod>, sprite: &'static ResourceLoadAsync<Sprite>) -> CannonBarrel {
    CannonBarrel::Projectile(
        Resource::detached(Projectile {
            body: TransientBallisticBody {
                initial_speed,
                sprite: Some(sprite.get(&rm)),
                drag: DRAG,
                seconds_to_live: Some(2.0),
            },
            collision_radius: collision_radius * SCALE_SPEED,
            splash_damage: None,
            damage,
            explosion: Some(ExplosionSource {
                offset: Vec2::new(60.0, 0.0),
                explosion: missile_explosion_simple.get(&rm),
            }),
            acceleration: Some(MISSILE_THRUST),
            stabilization: Some(Resource::detached(Stabilization {
                slide: SlideStabilization {
                    slide_by_speed: Curve::from_function(|it| SLIDE * it.powi(2)),
                    slide_by_attack: Curve::from_function(|it| (it * PI).cos().abs()),
                },
                steer: SteerStabilization {
                    max_angular_acceleration: PI * 2.0,
                    steering_by_speed: Curve::new_ext(&[
                        Value(1.0),
                    ]),
                    steering_by_attack: Curve::new_ext(&[
                        Value(1.0),
                    ]),
                }
            })),
            trail: Some(Resource::detached(TrailSource {
                emitter: exhaust_trail_missile.get(&rm),
                offset: Vec2::new(-40.0, 0.0),
            })),
            exhaust_clip: Some(fire_facepalm11_clip.get(&rm)),
            hit_sound: None,
            rotation: ProjectileRot::InitialVelocity,
            pulsation: None,
            mods
        })
    )
}


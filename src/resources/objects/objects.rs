#![allow(non_upper_case_globals)]

use std::any::Any;
use std::collections::HashMap;
use std::f32::consts::PI;
use std::ops::Deref;
use std::rc::Rc;

use futures::future::BoxFuture;
use futures::FutureExt;
use macroquad::prelude::{Vec2, Vec4Swizzles, YELLOW};
use macroquad::color::{BLUE, RED, WHITE};
use macroquad::prelude::{BLACK, GRAY};
use rust_macroquad_ui::common::to_vec::ToVec;

use crate::model::def::{BackgroundObject, Bot, Cannon, CannonPod, CannonPodProps, Explosion, ExplosionFragment, ExplosionParticleEmission, ExplosionSource, GameSound, Gear, HitScan, HitScanAction, HitScanLook, HitScanRay, Location, MaterialInstance, ParticleEmission, ParticleEmitter, Plane, PlaneArms, Projectile, ProjectilePulsation, ProjectileRot, Sky, SlideStabilization, SplashDamage, SpriteClip, Stabilization, SteerStabilization, TrailSource, UniformSupplier};
use crate::common::{sprite, unsorted};
use crate::common::angle::{Angle, AsRadians};
use crate::common::resource::{Resource, ResourceLoadAsync};
use crate::common::curve::{Curve};
use crate::common::curve::Point::{Transition, Value};
use crate::common::resource::{ResourceLoad, ResourceManagerRc};
use crate::common::sprite::load_sprites_from_sheet;
use crate::common::unsorted::ToColor;
use crate::model::def::CannonBarrel;
use crate::model::def::Sprite;
use crate::model::def::TransientBallisticBody;
use crate::{ResourceGet, ResourceManager};
use crate::resources::constants::{DRAG, INITIAL_ENERGY, INITIAL_ENERGY_RESTORE_PER_SEC, INITIAL_HP, MISSILE_THRUST, PLANE_THRUST_NOMINAL, SCALE_SPEED, SECONDS_TO_RESTORE_FULL_ENERGY, SLIDE};
use crate::resources::fx::settings::{exhaust_trail_missile, exhaust_trail_plane, missile_explosion_simple, plane_explosion_composite};
use crate::resources::materials::fog::fog_material;
use crate::resources::fx::sprites::explosion3_clip;
use crate::resources::objects::arms::{cannon_default, launcher_player};
use crate::resources::sounds::{engine_001_sound, sound_cannon_001, sound_death_001, sound_explosion_001, sound_hit_001, sound_missile_001, sound_pain, sound_plasma, sound_rail};
use crate::resources::sprites::explosion_facepalm33_clip::explosion_facepalm33_clip;
use crate::resources::sprites::fire_facepalm11_clip::fire_facepalm11_clip;
use crate::resources::sprites::sprites::{bullet_sprite, missile_sprite, missile_sprite_blue, missile_sprite_yellow, plane_sprite, sprite_plasma_001};

pub const standard_stabilization: ResourceLoad<Stabilization> = |rm| {
    Stabilization {
        slide: SlideStabilization {
            slide_by_speed: Curve::from_function(|it| SLIDE * it.powi(2)),
            slide_by_attack: Curve::from_function(|it| (it * PI).cos().abs()),
        },
        steer: SteerStabilization {
            max_angular_acceleration: PI * 2.0,
            steering_by_speed: Curve::new_ext(&[
                Value(0.0),
                // Value(0.8),
                Value(1.0),
            ]),
            steering_by_attack: Curve::new_ext(&[
                Value(1.0),
                Value(0.3),
                Transition(2),
                Value(0.1),
            ]),
        },
    }
};

pub const plane001: ResourceLoad<Plane> = |rm| Plane {
    sprite: plane_sprite.get(&rm),
    hp: INITIAL_HP,
    gears: vec![
        Gear { thrust: 0.0, energy_per_sec: 0.0, tech_level: 0 },
        Gear { thrust: PLANE_THRUST_NOMINAL * 0.5, energy_per_sec: INITIAL_ENERGY_RESTORE_PER_SEC * 0.25, tech_level: 0 },
        Gear { thrust: PLANE_THRUST_NOMINAL * 1.0, energy_per_sec: INITIAL_ENERGY_RESTORE_PER_SEC * 0.50, tech_level: 0 },
        Gear { thrust: PLANE_THRUST_NOMINAL * 1.5, energy_per_sec: INITIAL_ENERGY_RESTORE_PER_SEC * 1.00, tech_level: 0 },
        Gear { thrust: PLANE_THRUST_NOMINAL * 2.0, energy_per_sec: INITIAL_ENERGY_RESTORE_PER_SEC * 2.00, tech_level: 0 },
        Gear { thrust: PLANE_THRUST_NOMINAL * 3.0, energy_per_sec: INITIAL_ENERGY_RESTORE_PER_SEC * 3.00, tech_level: 1 },
        Gear { thrust: PLANE_THRUST_NOMINAL * 4.5, energy_per_sec: INITIAL_ENERGY_RESTORE_PER_SEC * 4.50, tech_level: 2 },
        Gear { thrust: PLANE_THRUST_NOMINAL * 5.5, energy_per_sec: INITIAL_ENERGY_RESTORE_PER_SEC * 5.50, tech_level: 3 },
    ],
    default_gear: 2,
    trail: Some(Resource::detached(TrailSource {
        emitter: exhaust_trail_plane.get(&rm),
        offset: Vec2::new(-40.0, 0.0),
    })),
    stabilization: standard_stabilization.get(&rm),
    explosion: Some(plane_explosion_composite.get(&rm)),
    arms: PlaneArms {
        primary_default: (cannon_default.get(&rm)),
        primary: Resource::detached(CannonPodProps { offset: Vec2::new(25.0, 9.0) * SCALE_SPEED }),
        secondary: Resource::detached(CannonPodProps { offset: Vec2::new(25.0, 9.0) * SCALE_SPEED }),
    },
    collision_radius: 25.0 * SCALE_SPEED,
    engine_sound: engine_001_sound.iter()
        .map(|it| it.as_ref().map(|it| it.get(&rm)))
        .to_vec(),
    death_sound: Some(sound_death_001.get(&rm)),
    pain_sound: Some(sound_pain.get(&rm)),
};

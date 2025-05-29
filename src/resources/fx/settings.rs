use crate::common::curve::{Curve};
use crate::common::resource::{ResourceGet, ResourceLoad, ResourceManagerRc};
use crate::FutureExt;
use crate::model::def::{Explosion, ExplosionFragment, ExplosionParticleEmission, ParticleEmission, ParticleEmitter, TransientBallisticBody};
use crate::resources::constants::SCALE_SPEED;
use crate::resources::fx::sprites::{smoke_clip_001, smoke_clip_001_dark, smoke_clip_plane_explosion};
use crate::resources::sounds::sound_explosion_001;
use crate::resources::sprites::explosion_facepalm33_clip::explosion_facepalm33_clip;
use crate::resources::sprites::explosion_facepalm34_clip::explosion_facepalm34_clip;

pub const missile_explosion_simple: ResourceLoad<Explosion> = |rm| Explosion {
    particles: vec![
        ExplosionParticleEmission {
            emission: ParticleEmission {
                clip_variants: vec![
                    explosion_facepalm33_clip.get(&rm),
                    explosion_facepalm34_clip.get(&rm),
                ],
                scale: 1.0.into(),
                spread_distance: 0.0,
                rate: 1.0.into(),
                delay: 0.0.into(),
            },
            count: 1.into(),
            off_center_speed: 100.0.into(),
            speed_factor: 0.1,
        }
    ],
    fragments: vec![],
    sound: Some(sound_explosion_001.get(&rm)),
};

pub const plane_explosion_composite: ResourceLoad<Explosion> = |rm| Explosion {
    particles: vec![
        ExplosionParticleEmission {
            emission: ParticleEmission {
                clip_variants: vec![
                    smoke_clip_plane_explosion.get(&rm),
                ],
                spread_distance: 80.0 * SCALE_SPEED,
                scale: Curve::new([0.5, 0.9]),
                rate: Curve::new([0.1, 0.4]),
                delay: Curve::new([0.3, 0.3]),
            },
            count: Curve::new([10]),
            off_center_speed: Curve::new([0.0, 0.0 * SCALE_SPEED]),
            speed_factor: 0.5,
        },
        ExplosionParticleEmission {
            emission: ParticleEmission {
                clip_variants: vec![
                    explosion_facepalm33_clip.get(&rm),
                    explosion_facepalm34_clip.get(&rm),
                ],
                spread_distance: 0.0 * SCALE_SPEED,
                scale: Curve::new([1.0]),
                // rate: Curve::new([0.8, 1.25]),
                rate: Curve::new([1.0]),
                delay: Curve::new([0.0]),
            },
            count: Curve::new([1]),
            off_center_speed: Curve::new([0.0]),
            speed_factor: 0.5,
        },
    ],
    fragments: vec![
        ExplosionFragment {
            body: TransientBallisticBody {
                initial_speed: 0.0,
                sprite: None,
                drag: 0.0,
                seconds_to_live: None,
            },
            trail: ParticleEmitter {
                emission: ParticleEmission {
                    clip_variants: vec![smoke_clip_001.get(&rm)],
                    spread_distance: 16.0 * SCALE_SPEED,
                    scale: Curve::new([2.0]),
                    rate: 1.0.into(),
                    delay: 0.0.into(),
                },
                spawn_rate: 0.0,
            },
        }
    ],
    sound: Some(sound_explosion_001.get(&rm)),
};

pub const exhaust_trail_plane: ResourceLoad<ParticleEmitter> = |rm| ParticleEmitter {
    emission: ParticleEmission {
        clip_variants: vec![smoke_clip_001.get(&rm)],
        spread_distance: 32.0,
        scale: Curve::new([0.5]),
        rate: 1.0.into(),
        delay: 0.0.into(),
    },
    spawn_rate: 16.0,
};

pub const exhaust_trail_missile: ResourceLoad<ParticleEmitter> = |rm| ParticleEmitter {
    emission: ParticleEmission {
        clip_variants: vec![smoke_clip_001_dark.get(&rm)],
        spread_distance: 32.0,
        scale: Curve::new([0.8]),
        rate: 1.0.into(),
        delay: 0.0.into(),
    },
    spawn_rate: 24.0,
};

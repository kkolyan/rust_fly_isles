use crate::common::curve::Curve;
use crate::common::resource::ResourceLoad;
use crate::model::def::Isle;
use crate::ResourceGet;
use crate::resources::sprites::isles::{isle1_sprite, isle_empy_1_sprite};

pub const isle_001: ResourceLoad<Isle> = |rm| Isle {
    sprite: isle1_sprite.get(&rm),
    scale: 0.5,
    bounds: -160.0..160.0,
    course_change_interval_seconds: Curve::new([25.0, 35.0]),
    drift_speed: 20.0,
};

pub const isle_slow: ResourceLoad<Isle> = |rm| Isle {
    sprite: isle_empy_1_sprite.get(&rm),
    scale: 0.5,
    bounds: -160.0..160.0,
    course_change_interval_seconds: Curve::new([25.0, 35.0]),
    drift_speed: 5.0,
};

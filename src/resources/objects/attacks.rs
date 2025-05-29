use macroquad::color::{RED, WHITE, YELLOW};
use crate::common::curve::{Curve, Point};
use crate::common::resource::{Resource, ResourceLoad};
use crate::model::def::{CannonPodProps, CircleEffect, HoldEffect, MobAttack};
use crate::model::def::MobAttackPattern::Distant;
use crate::{ResourceGet, Vec2};
use crate::common::unsorted::ToColor;
use crate::resources::objects::arms::{cannon_plasma, cannon_rail, cannon_rail2};

pub const attack_railgun: ResourceLoad<MobAttack> = |rm| {
    MobAttack {
        trigger_range: 500.0,
        hold_sec: 0.5,
        cooldown_sec: Curve::new([0.7]),
        pattern: Distant { cannon: cannon_rail.get(&rm) },
        charge: None,
        burst: None,
        late_aim: false,
        hold_effect: Some(HoldEffect::Circle(Resource::detached(CircleEffect {
            radius: Curve::new([0.0, 10.0, 15.0, 20.0, 0.0]),
            color: Curve::new([RED, YELLOW, WHITE]),
        }))),
    }
};

pub const attack_railgun2: ResourceLoad<MobAttack> = |rm| {
    MobAttack {
        trigger_range: 500.0,
        hold_sec: 0.1,
        cooldown_sec: Curve::new([0.7]),
        pattern: Distant { cannon: cannon_rail2.get(&rm) },
        charge: None,
        burst: None,
        late_aim: false,
        hold_effect: Some(HoldEffect::Circle(Resource::detached(CircleEffect {
            radius: Curve::new([0.0, 10.0, 15.0, 20.0, 0.0]),
            color: Curve::new([RED, YELLOW, WHITE]),
        }))),
    }
};

pub const attack_plasma: ResourceLoad<MobAttack> = |rm| {
    MobAttack {
        trigger_range: 500.0,
        hold_sec: 1.0,
        cooldown_sec: Curve::new([1.2]),
        pattern: Distant { cannon: cannon_plasma.get(&rm) },
        charge: None,
        burst: None,
        late_aim: true,
        hold_effect: Some(HoldEffect::Circle(Resource::detached(CircleEffect {
            radius: Curve::new_ext(&[
                Point::Value(0.0),
                Point::Transition(5),
                Point::Value(20.0),
                Point::Value(10.0),
            ]),
            color: Curve::new(["#9AFF19".to_color(), WHITE]),
        }))),
    }
};

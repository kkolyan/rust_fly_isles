use std::collections::HashMap;
use std::f32::consts::PI;
use crate::{Game, ResourceGet};
use crate::common::curve::Curve;
use crate::common::enum_maps::new_enum_map;
use crate::common::pool::Pool;
use crate::common::resource::{Resource, ResourceLoad};
use crate::model::def::{Cannon, GameResource, Obtainable, ShopLot, DeviceSlot, Buff, BuffSpec, SteerStabilization, Improvement, ImprovementLevel, ImprovementSpec, BonusSpec, ImprovementTitle, ImprovementCategory, PlaneWeapon, PlaneBuff};
use crate::model::def::Obtainable::Weapon;
use crate::model::state::{Ammo, DeviceOrder, LimitedCannon, WeaponSelector};
use crate::model::state::Ammo::{Energy, Finite, Infinite};
use crate::resources::constants::{PLANE_THRUST_NOMINAL, SPEED_ABS_MAX};
use crate::resources::materials::pain::pain_material;
use crate::resources::objects::arms::{cannon_gatling, cannon_plasma, launcher_player, cannon_rail, launcher_jagger_homing, cannon_rail2, cannon_default, cannon_rail_player};
use crate::resources::objects::locations::location002::location002;
use crate::resources::objects::locations::location001::*;
use crate::resources::objects::locations::location003::location003;
use crate::resources::objects::locations::location003_training::location003_training;
use crate::resources::objects::objects::plane001;
use crate::resources::sounds::{sound_levelup, sound_skillup};

pub const game_001: ResourceLoad<Game> = |rm| Game {
    // location: location001.get(&rm),
    combat: location003.get(&rm),
    training: location003_training.get(&rm),
    player_plane: plane001.get(&rm),
    standard_pain_material: pain_material.get(&rm),
    advanced_weapons: WeaponSelector {
        items: vec![
            cannon_gatling.get(&rm),
            launcher_player.get(&rm),
            cannon_rail.get(&rm),
            cannon_plasma.get(&rm),
        ],
        selected: 0,
    },
    shop_assortment: vec![
        shop_lot(price(0, 0, 0), weapon(DeviceSlot::Primary, Energy { energy_per_shot: 2.5 }, cannon_gatling.get(&rm))),
        shop_lot(price(1, 0, 0), Obtainable::HP { title: "20HP", amount: 20.0 }),
        shop_lot(price(0, 0, 0), Obtainable::HP { title: "Cheat: 120HP", amount: 120.0 }),
        shop_lot(price(0, 0, 0), consumable("Jet Nitro", 30.0, booster_add_spec(12.0))),
        shop_lot(price(0, 0, 0), consumable("Jet Steer", 30.0, steer_booster_spec())),
        shop_lot(price(0, 0, 0), consumable("Jet Steer", 30.0, steer_booster_spec())),
        shop_lot(price(0, 0, 0), passive("Boosters", Some(booster_mul_spec(5.0)))),
        shop_lot(price(0, 0, 0), passive("None", None)),
        shop_lot(price(0, 0, 0), weapon(DeviceSlot::Primary, Infinite, cannon_rail.get(&rm))),
        shop_lot(price(0, 0, 0), weapon(DeviceSlot::Primary, Infinite, cannon_rail2.get(&rm))),
        shop_lot(price(0, 0, 0), weapon(DeviceSlot::Primary, Energy { energy_per_shot: 1.5 }, cannon_default.get(&rm))),
        shop_lot(price(0, 0, 0), weapon(DeviceSlot::Secondary, Finite(10), launcher_player.get(&rm))),
        shop_lot(price(2, 1, 0), weapon(DeviceSlot::Secondary, Finite(50), cannon_plasma.get(&rm))),
        shop_lot(price(3, 2, 1), weapon(DeviceSlot::Secondary, Finite(5), launcher_jagger_homing.get(&rm))),
    ],
    rpg: Pool::from([
        Improvement {
            title: ImprovementTitle::Hard("Extra Energy"),
            category: ImprovementCategory::Passives,
            levels: vec![
                ImprovementLevel { spec: ImprovementSpec::Bonus { spec: BonusSpec::Power { extra_energy: 25.0 } }, points: 1 },
                ImprovementLevel { spec: ImprovementSpec::Bonus { spec: BonusSpec::Power { extra_energy: 35.0 } }, points: 1 },
                ImprovementLevel { spec: ImprovementSpec::Bonus { spec: BonusSpec::Power { extra_energy: 40.0 } }, points: 1 },
                ImprovementLevel { spec: ImprovementSpec::Bonus { spec: BonusSpec::Power { extra_energy: 50.0 } }, points: 2 },
                ImprovementLevel { spec: ImprovementSpec::Bonus { spec: BonusSpec::Power { extra_energy: 50.0 } }, points: 3 },
            ],
            description: [
                "Energy used for everything - for flying, for shooting, for buffing.",
                "Make a pig of oneself with up to +100% Energy divided into 3 upgrades.",
                "Increases energy regen proportionally.",
            ],
        },
        Improvement {
            title: ImprovementTitle::Hard("Extra Armor"),
            category: ImprovementCategory::Passives,
            levels: vec![
                ImprovementLevel { spec: ImprovementSpec::Bonus { spec: BonusSpec::Armor { extra_hp: 25.0 } }, points: 1 },
                ImprovementLevel { spec: ImprovementSpec::Bonus { spec: BonusSpec::Armor { extra_hp: 35.0 } }, points: 1 },
                ImprovementLevel { spec: ImprovementSpec::Bonus { spec: BonusSpec::Armor { extra_hp: 40.0 } }, points: 1 },
                ImprovementLevel { spec: ImprovementSpec::Bonus { spec: BonusSpec::Armor { extra_hp: 50.0 } }, points: 2 },
                ImprovementLevel { spec: ImprovementSpec::Bonus { spec: BonusSpec::Armor { extra_hp: 50.0 } }, points: 3 },
            ],
            description: ["Endure more damage with up to +100% HP divided into 3 upgrades.", "", ""],
        },
        Improvement {
            title: ImprovementTitle::Hard("Extra Acceleration"),
            category: ImprovementCategory::Passives,
            levels: vec![
                ImprovementLevel { spec: ImprovementSpec::Bonus { spec: BonusSpec::ThrustTechInc }, points: 1 },
                ImprovementLevel { spec: ImprovementSpec::Bonus { spec: BonusSpec::ThrustTechInc }, points: 1 },
                ImprovementLevel { spec: ImprovementSpec::Bonus { spec: BonusSpec::ThrustTechInc }, points: 1 },
            ],
            description: [
                "Fly faster if you like with additional acceleration stages.",
                "With additional Energy, of course.", "",
            ],
        },
        skill_weapon(cannon_gatling.get(&rm), 1, 2.5, 1, [
            "Stuff enemies with a lead salad on 90000 rounds per minute.", "", ""
        ]),
        skill_weapon(launcher_player.get(&rm), 2, 10.0, 1, [
            "Devastate groups of enemies with high-explosive missiles.", "", ""]),
        skill_weapon(cannon_rail_player.get(&rm), 3, 8.0, 1, [
            "Burn them all on a speed-of-light using continuous instantly reaching beam.", "", ""
        ]),
        skill_buff(Resource::detached(Buff { title: "In-Flight Repair", spec: BuffSpec::Repair { hp_per_sec: 40.0 } }), 4, 30.0, 1, [
            "Restore HP using Energy", "", ""
        ]),
        skill_buff(Resource::detached(Buff { title: "Nitro-Jet", spec: BuffSpec::Nitro {
            smoke_factor: 5.0,
            top_speed: SPEED_ABS_MAX,
            acceleration_by_speed: Curve::new([PLANE_THRUST_NOMINAL * 30.0, 0.0]),
        } }), 5, 50.0, 1, [
            "Easily get out of trouble with temporary huge acceleration boost.",
            "Drains Energy in a seconds.", ""
        ]),
        skill_buff(Resource::detached(Buff { title: "Side Thrusters", spec: steer_booster_spec() }), 6, 10.0, 1, [
            "Feel yourself mobile as UFO with jet side thrusters.",
            "While it drains your energy, of course.", ""
        ]),
    ]),
    sound_level_up: Some(sound_levelup.get(&rm)),
    sound_skill_up: Some(sound_skillup.get(&rm)),
};

fn skill_weapon(spec: Resource<Cannon>, order: i32, energy_per_shot: f32, skill_points: u32, description: [&'static str; 3]) -> Improvement {
    Improvement {
        title: ImprovementTitle::FromWeapon(spec.clone()),
        category: ImprovementCategory::Weapons,
        levels: vec![ImprovementLevel { spec: ImprovementSpec::WeaponSkill(PlaneWeapon { spec, energy_per_shot, order: DeviceOrder(order) }), points: skill_points }],
        description,
    }
}

fn skill_buff(spec: Resource<Buff>, order: i32, energy_per_second: f32, skill_points: u32, description: [&'static str; 3]) -> Improvement {
    Improvement {
        title: ImprovementTitle::FromBuff(spec.clone()),
        category: ImprovementCategory::Utility,
        levels: vec![ImprovementLevel { spec: ImprovementSpec::BuffSkill(PlaneBuff { spec, energy_per_second, order: DeviceOrder(order) }), points: skill_points }],
        description,
    }
}

fn shop_lot(price: HashMap<GameResource, u32>, item: Obtainable) -> Resource<ShopLot> {
    Resource::detached(ShopLot { price, item })
}

fn weapon(slot: DeviceSlot, ammo: Ammo, weapon: Resource<Cannon>) -> Obtainable {
    Obtainable::Weapon { slot, weapon, ammo }
}

fn passive(title: &'static str, spec: Option<BuffSpec>) -> Obtainable {
    match spec {
        Some(spec) => Obtainable::Passive {
            def: Resource::detached(Buff {
                title,
                spec,
            })
        },
        None => Obtainable::PassiveReset { title },
    }
}

fn consumable(title: &'static str, reserve_sec: f32, spec: BuffSpec) -> Obtainable {
    Obtainable::Consumable {
        slot: DeviceSlot::Secondary,
        def: Resource::detached(Buff {
            title,
            spec,
        }),
        reserve_sec,
    }
}

fn booster_add_spec(factor: f32) -> BuffSpec {
    BuffSpec::ThrustAddendum {
        extra_acceleration: PLANE_THRUST_NOMINAL * factor,
    }
}

fn booster_mul_spec(factor: f32) -> BuffSpec {
    BuffSpec::ThrustMultiplier {
        acceleration_multiplier: factor,
        smoke_factor_rel: factor,
    }
}

fn steer_booster_spec() -> BuffSpec {
    BuffSpec::SteerBooster {
        stabilization: SteerStabilization {
            steering_by_speed: Curve::new([1.0]),
            steering_by_attack: Curve::new([1.0]),
            max_angular_acceleration: PI * 4.0,
        },
        smoke_factor_abs: 8.0,
    }
}

fn price(a: u32, b: u32, c: u32) -> HashMap<GameResource, u32> {
    new_enum_map(|it| match it {
        GameResource::A => a,
        GameResource::B => b,
        GameResource::C => c,
    })
}

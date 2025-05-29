use std::collections::HashSet;
use enum_iterator::All;
use crate::common::contract::{Get, GetMut, Insert};
use crate::{GameState, Plane, PlaneState, PlayerState};
use crate::common::resource::Resource;
use crate::common::sound::PlaySound;
use crate::model::def::{BonusSpec, Cannon, Improvement, ImprovementId, ImprovementSpec, ImprovementTitle, PlaneBuff, PlaneWeapon};
use crate::model::state::{Ammo, CannonState, DeviceId, DeviceSpec, DeviceState, Durable, EquipmentBinding, ManualBuffAmmo, ManualBuffState, UiWindow, WeaponOwner};
use crate::resources::constants::{next_level_xp, XP_MUL};

pub fn update(state: &mut GameState) {
    let rpg = &mut state.player.rpg;
    let next_level_xp = next_level_xp(rpg.level);
    if rpg.xp >= next_level_xp {
        rpg.level += 1;
        rpg.skill_points += 1;
        state.def.sound_level_up.play_once(&state.subsystems.audio);
    }
    for (_, mob) in state.mobs.iter_mut() {
        if let Durable::Destroyed(killer) = mob.base.durable {
            if !mob.base.xp_awarded {
                match killer {
                    WeaponOwner::Plane(plane) => {
                        if Some(plane) == state.player.plane {
                            state.player.rpg.xp += (mob.base.def.xp_reward as f32 * XP_MUL) as u32;
                        }
                    }
                    WeaponOwner::Mob => {}
                    WeaponOwner::Environment => {}
                }
                mob.base.xp_awarded = true;
                mob.base.def.death_sound.play_once(&state.subsystems.audio);
            }
        }
    }
}

pub(crate) fn click_skill(state: &mut GameState, skill_id: &ImprovementId) {
    if let Some(player) = state.player.plane.and_then(|it| state.planes.get_mut(&it)) {
        let level = state.player.rpg.skills.get(skill_id).copied().unwrap_or(0);
        if let Some(skill) = state.def.rpg.get(skill_id) {
            if let Some(value) = skill.levels.get(usize::from(level)) {
                if state.player.rpg.skill_points >= value.points {
                    match &value.spec {
                        ImprovementSpec::WeaponSkill(weapon) => {
                            let device = weapon_to_device(&player.def, weapon);
                            give_equipment(&mut state.player, device);
                        }
                        ImprovementSpec::BuffSkill(PlaneBuff {spec, energy_per_second, order}) => {
                            let device = DeviceState {
                                spec: DeviceSpec::Booster(ManualBuffState::new(
                                    spec.clone(),
                                    ManualBuffAmmo::Energy { energy_per_second: *energy_per_second }
                                )),
                                binding: None,
                                order: *order
                            };
                            give_equipment(&mut state.player, device);
                        }
                        ImprovementSpec::Passive { spec } => {
                            player.passive_buffs.push(spec.clone());
                        }
                        ImprovementSpec::Bonus { spec } => {
                            match spec {
                                BonusSpec::ThrustTechInc => {
                                    state.player.thrust_tech_level += 1;
                                }
                                BonusSpec::Power { extra_energy } => {
                                    state.player.energy_max += extra_energy;
                                    player.energy += extra_energy;
                                }
                                BonusSpec::Armor { extra_hp } => {
                                    state.player.hp_max += extra_hp;
                                    match &mut player.durable {
                                        Durable::Good { hp, .. } => { *hp += extra_hp }
                                        Durable::Destroyed(_) => {}
                                    }
                                }
                            }
                        }
                    }
                    state.player.rpg.skill_points -= value.points;
                    state.player.rpg.skills.insert(*skill_id, level + 1);
                    state.def.sound_skill_up.play_once(&state.subsystems.audio);
                }
            }
        }
    }
}

pub fn give_equipment(player: &mut PlayerState, mut device: DeviceState) {
    let occupied: HashSet<EquipmentBinding> = HashSet::from_iter(player.equipment.iter().filter_map(|(_, it)| it.binding));
    device.binding = enum_iterator::all::<EquipmentBinding>()
        .find(|it| !occupied.contains(it));
    player.equipment.insert(device);
}

pub fn weapon_to_device(plane: &Resource<Plane>, weapon: &PlaneWeapon) -> DeviceState {
    let resource = plane;
    DeviceState {
        spec: DeviceSpec::Weapon(CannonState::new(
            &resource.arms.primary,
            &weapon.spec,
            Ammo::Energy { energy_per_shot: weapon.energy_per_shot },
        )),
        binding: None,
        order: weapon.order,
    }
}

pub fn improvement_title(improvement: &Improvement) -> &'static str {
    match &improvement.title {
        ImprovementTitle::Hard(title) => { title }
        ImprovementTitle::FromBuff(buff) => { buff.title }
        ImprovementTitle::FromWeapon(weapon) => { weapon.title }
    }
}

pub(crate) fn bind_equipment(state: &mut GameState, item: &DeviceId, binding: &Option<EquipmentBinding>) {
    if let Some(player) = state.player.plane.and_then(|it| state.planes.get_mut(&it)) {
        for (id, device) in state.player.equipment.iter_mut() {
            if device.binding == *binding {
                device.binding = None;
            }
        }
        if let Some(device) = state.player.equipment.get_mut(item) {
            device.binding = *binding;
        }
        if let Some(UiWindow::Equipment(window)) = state.player.windows.back_mut() {
            window.selected_item = None;
        }
    }
}

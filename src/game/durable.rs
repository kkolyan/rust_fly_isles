use std::rc::Rc;
use crate::common::contract::{Get, GetMut};
use crate::common::frame::FrameCtx;
use crate::common::resource::Resource;
use crate::common::sound::PlaySound;
use crate::GameState;
use crate::model::def::MaterialInstance;
use crate::model::state::{Durable, WeaponOwner};
use crate::resources::constants::PAIN_SECONDS;


impl Durable {
    pub fn new(hp: f32) -> Self {
        Durable::Good { hp, hp_prev: 0.0, pain_remaining_seconds: 0.0 }
    }

    pub fn accept_damage(&mut self, damage: f32, offender: WeaponOwner) {
        match self {
            Durable::Good { ref mut hp, .. } => {
                *hp -= damage;
                if *hp < 0.0 {
                    *self = Durable::Destroyed(offender)
                }
            },
            Durable::Destroyed(_) => {}
        }
    }
}

pub fn update(state: &mut GameState, dt: &FrameCtx) {
    for mob_id in state.reachable_mobs.iter() {
        if let Some(mob) = state.mobs.get_mut(mob_id) {
            update_durable(dt,  &mut mob.base.durable, || {
                mob.base.def.pain_sound.play_once(&state.subsystems.audio);
            });
        }
    }

    for (_, plane) in state.planes.iter_mut() {
        update_durable(dt, &mut plane.durable, || {
            plane.def.pain_sound.play_once(&state.subsystems.audio);
        });
    }
}

fn update_durable<OnDamage: FnOnce()>(dt: &FrameCtx, durable: &mut Durable, on_damage: OnDamage)  {
    if let Durable::Good { hp, ref mut hp_prev, ref mut pain_remaining_seconds } = durable {
        *pain_remaining_seconds -= dt.dt;

        if *hp_prev != *hp {
            if *hp_prev > *hp {
                *pain_remaining_seconds = PAIN_SECONDS;
                on_damage();
            }
            *hp_prev = *hp;
        }
    }
}

pub fn pain_option(state: &GameState, durable: &Durable) -> Option<Resource<MaterialInstance>> {
    match durable {
        Durable::Good { pain_remaining_seconds, .. } => if *pain_remaining_seconds > 0.0 {
            Some(state.def.standard_pain_material.clone())
        } else {
            None
        },
        Durable::Destroyed(_) => None
    }
}

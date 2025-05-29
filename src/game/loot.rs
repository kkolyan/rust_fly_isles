use std::collections::{HashSet, VecDeque};
use std::ops::{Add, AddAssign};
use crate::common::camera::ViewPort;
use crate::common::resource::Resource;
use crate::common::sprite::draw_sprite;
use crate::model::def::{ProgressFlag, Item, Loot, Mob, GameResource, GameProgressCtx};
use crate::{FrameCtx, GameState, Plane, PlaneState, PlayerState, Vec2};
use crate::common::contract::{Get, InsertSimple};
use crate::common::curve::{Curve, Point};
use crate::common::pool::Pool;
use crate::common::sound::PlaySound;
use crate::common::unsorted::{gen_range, IndexRange};
use crate::model::state::{LootState, IsleId, SubSystems, LootId, GameProgression, LootManager, GameCommand, MobAnchor};

pub enum LootPos {
    Isle(IsleId, Vec2),
    Flying(Vec2),
}

pub fn spawn_loot(
    mob: &Mob,
    progress: &GameProgression,
    player: &PlayerState,
    loot_manager: &mut LootManager,
    commands: &mut dyn InsertSimple<GameCommand>,
    loots: &Pool<LootId, LootState>,
    anchor: &MobAnchor,
) {
    for loot in mob.loot_chances.iter() {
        if (loot.rule.0)(&GameProgressCtx { player, progression: progress, loot: loots }) {
            let acc = loot_manager.accumulators.entry(loot.loot.clone()).or_insert(0.0);
            *acc += loot.probability.clamp(0.0, 1.0);
            while *acc >= 1.0 {
                commands.insert_simple(GameCommand::Drop(loot.loot.clone(), anchor.get_pos_rel()));
                *acc -= 1.0;
            }
        }
    }
}

pub fn draw(pos: Vec2, loot: &Loot, vp: &ViewPort) {
    vp.port(pos, 1.0, |ported| {
        draw_sprite(&loot.sprite, ported.screen_pos, |it| {
            it.screen_scale = ported.screen_scale;
        });
    });
}

pub fn update(state: &mut GameState, ctx: &FrameCtx) {
    if let Some(player) = state.player.plane.and_then(|it| state.planes.get(&it)) {
        let keys: Vec<LootId> = state.loot.iter().map(|(it, _)| it).copied().collect();
        for i in keys {
            if let Some(loot) = state.loot.get(&i) {
                match update_loot(&loot.def, loot.pos.get_abs(&state.isles), player, &mut state.player, &state.subsystems) {
                    LootUpdateResult::Left => {}
                    LootUpdateResult::Picked => { state.loot.remove(i); }
                }
            }
        }
    }
}

enum LootUpdateResult {
    Left,
    Picked,
}

fn update_loot(loot: &Loot, pos: Vec2, player_plane: &PlaneState, player_state: &mut PlayerState, settings: &SubSystems) -> LootUpdateResult {
    if (pos + loot.collider_unscaled.center).distance(player_plane.trans.pos) < player_plane.def.collision_radius + loot.collider_unscaled.radius {
        for item in &loot.content {
            match item {
                Item::Resource { count, resource } => {
                    player_state.resources.get_mut(resource).unwrap().add_assign(count);
                }
            }
        }
        loot.pick_sound.play_once(&settings.audio);
        return LootUpdateResult::Picked;
    }
    LootUpdateResult::Left
}
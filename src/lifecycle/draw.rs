use std::collections::HashMap;
use std::ops::Not;
use std::rc::Rc;
use macroquad::hash;

use macroquad::prelude::{BLACK, Rect, RED, screen_height, screen_width, WHITE};
use macroquad::prelude::Vec2;
use macroquad::shapes::{draw_circle, draw_rectangle};
use macroquad::ui::{root_ui, Skin};

use crate::common::camera::ViewPort;
use crate::common::contract::Get;
use crate::common::fps::FpsCounter;
use crate::common::pool::{Pool, PoolKey};
use crate::common::sprite_clip;
use crate::common::sprite_clip::{SpriteDrawer, SpriteDrawingItem, SpriteDrawingOption};
use crate::common::sprite_clip::SpriteDrawingOption::{Empty, FlipX, Material, Rot, Scale};
use crate::common::unsorted::{ColorOps, RectExtOps, ToColor};
use crate::game::{cannon, control_guard, durable, game_viewport, isles, loot, mobs, plane, sky, ui};
use crate::{AppState, debug, GameState};
use crate::game::ui::legacy_hud;
use crate::model::def::{HoldEffect, MaterialInstance, MobAttackPattern};
use crate::model::state::{Durable, IsleId, IsleState, MobAnchor, MobPhase, MobState, RelativePos, UiWindow, WaitSecondsAction};
use crate::resources::constants::LOGIC_RESOLUTION;

pub struct DrawState {
    pub fps_counter: FpsCounter,
}

pub fn draw_state(app: &mut AppState, draw_state: &mut DrawState) {
    match app {
        AppState::Title { .. } => {

        }
        AppState::Game { game } => {
            draw_game_state(game, draw_state);
        }
        AppState::GameMenu { game, .. } => {
            draw_game_state(game, draw_state);
        }
        AppState::Intro => {}
    }
}

pub fn draw_game_state(state: &mut GameState, draw_state: &mut DrawState) {
    let view_port = &game_viewport::create_viewport(state);
    draw_state.fps_counter.update_fps();

    sky::draw_sky(state, view_port);

    let mut stats = Stats { drawn_clouds: 0 };
    sky::draw_clouds(state, &mut stats, view_port);

    let sprite_drawer = SpriteDrawer {
        stats: &mut stats,
        view_port,
    };

    struct DrawSet<'a, T> {
        flying: Vec<&'a T>,
        lying: HashMap<IsleId, Vec<&'a T>>,
    }

    fn create<K: PoolKey, T, F: Fn(&T) -> Option<IsleId>>(items: &Pool<K, T>, grouper: F) -> DrawSet<T> {
        let mut lying = HashMap::new();
        let mut flying = Vec::new();
        for (_, item) in items.iter() {
            let group = grouper(&item);
            match group {
                None => flying.push(item),
                Some(isle_id) => lying.entry(isle_id)
                    .or_insert_with(|| Vec::new())
                    .push(item),
            }
        }
        DrawSet {
            flying,
            lying,
        }
    }

    let mut mobs = create(&state.mobs, |it| match it.anchor {
        MobAnchor::Isle(isle_id, _) => Some(isle_id),
        MobAnchor::Global(_) => None,
    });

    let mut loot = create(&state.loot, |it| match it.pos {
        RelativePos::Isle(isle_id, _) => Some(isle_id),
        RelativePos::Global(_) => None,
    });

    let mut isles: Vec<(&IsleId, &IsleState)> = state.isles.iter().collect();
    isles.sort_by_key(|(_, isle)| isle.order);
    for (isle_id, isle) in isles {
        isles::draw_isle(isle, view_port);
        if let Some(mobs) = mobs.lying.get(isle_id) {
            draw_mobs(state, mobs, &sprite_drawer, view_port);
        }

        if let Some(loots) = loot.lying.get(isle_id) {
            for loot in loots {
                loot::draw(loot.pos.get_abs(&state.isles), &loot.def, &view_port);
            }
        }
    }

    draw_mobs(state, &mobs.flying, &sprite_drawer, view_port);

    for loot in &loot.flying {
        loot::draw(loot.pos.get_abs(&state.isles), &loot.def, &view_port);
    }

    sprite_drawer.draw(&state.particles.fixed, |it| Some(SpriteDrawingItem {
        pos: it.pos,
        clip: &it.clip,
        options: [Scale(it.scale), Rot(it.rot)],
    }));
    sprite_drawer.draw(&state.particles.moving, |it| Some(SpriteDrawingItem {
        pos: it.pos,
        clip: &it.clip,
        options: [Scale(it.scale), Rot(it.rot)],
    }));
    plane::draw_planes(state, &view_port);
    sprite_drawer.draw(&state.projectiles, |projectile| projectile.exhaust_clip.as_ref().map(|exhaust_clip| {
        SpriteDrawingItem {
            pos: projectile.trans.pos,
            clip: exhaust_clip,
            options: [
                Rot(projectile.rot.angle),
            ],
        }
    }));
    cannon::draw_projectiles(state, &view_port);
    control_guard::draw(state, &stats, draw_state, &view_port);
    if state.show_colliders {
        let draw_collider = |center: Vec2, radius: f32| {
            view_port.port(center, 1.0, |ported| {
                draw_circle(ported.screen_pos.x, ported.screen_pos.y, ported.screen_scale * radius, "#F0F".to_color().with_alpha(0.4))
            });
        };
        for (_, mob) in state.mobs.iter() {
            let pos = mob.anchor.get_pos_rel().get_abs(&state.isles);
            let center = pos + mob.base.def.collider_unscaled.center * mob.base.def.scale;
            let radius = mob.base.def.collider_unscaled.radius * mob.base.def.scale;
            draw_collider(center, radius);
            view_port.port(pos, 1.0, |ported| {
                draw_circle(ported.screen_pos.x, ported.screen_pos.y, ported.screen_scale * 10.0, "#F0F".to_color().with_alpha(0.8))
            });
        }
        for (_, loot) in state.loot.iter() {
            let pos = loot.pos.get_abs(&state.isles);
            let center = pos + loot.def.collider_unscaled.center;
            let radius = loot.def.collider_unscaled.radius;
            draw_collider(center, radius);
            view_port.port(pos, 1.0, |ported| {
                draw_circle(ported.screen_pos.x, ported.screen_pos.y, ported.screen_scale * 10.0, "#F0F".to_color().with_alpha(0.8))
            });
        }
        for (_, plane) in state.planes.iter() {
            let center = plane.trans.pos;
            let radius = plane.def.collision_radius;
            draw_collider(center, radius);
        }
        for projectile in &state.projectiles {
            let center = projectile.trans.pos;
            let radius = projectile.def.collision_radius;
            draw_collider(center, radius);
        }
    }
}

fn draw_mobs(state: &GameState, mobs: &Vec<&MobState>, sprite_drawer: &SpriteDrawer, view_port: &ViewPort) {
    sprite_drawer.draw(mobs, |mob| {
        mob.base.clip_state.as_ref().map(|clip| {
            // panic!("Mob: {}", mob.flier.pos);
            if mob.base.debug {
                debug!("animation =  {:?}, frame: {:?}", mob.base.animation, clip.frame);
            }
            SpriteDrawingItem {
                pos: mob.anchor.get_pos_rel().get_abs(&state.isles),
                clip,
                options: [
                    FlipX(mob.base.get_dir() < 0.0),
                    Scale(mob.base.def.scale),
                    durable::pain_option(state, &mob.base.durable)
                        .map(Material)
                        .or_else(|| mob.base.def.material.clone().map(Material))
                        .unwrap_or(Empty),
                ],
            }
        })
    });
    for mob in mobs {
        match &mob.base.phase {
            MobPhase::WaitSeconds { action, seconds_remaining } => {
                match action {
                    WaitSecondsAction::Idle => {}
                    WaitSecondsAction::Move(_) => {}
                    WaitSecondsAction::Charge { .. } => {}
                    WaitSecondsAction::AttackHold(_, attack) => {
                        if let Some(effect) = &attack.hold_effect {
                            match effect {
                                HoldEffect::Circle(effect) => {
                                    view_port.port(mob.anchor.get_pos_rel().get_abs(&state.isles), 1.0, |ported| {
                                        let offset = match &attack.pattern {
                                            MobAttackPattern::Melee { .. } => Vec2::ZERO,
                                            MobAttackPattern::Distant { cannon } =>
                                                mobs::calc_offset(&mob.base),
                                        };
                                        let pos = ported.screen_pos + offset * ported.screen_scale;
                                        let f = 1.0 - seconds_remaining / attack.hold_sec;
                                        let radius = effect.radius.lerp(f) * ported.screen_scale;
                                        let color = effect.color.lerp(f);
                                        draw_circle(pos.x, pos.y, radius, color);
                                    });
                                }
                            }
                        }
                    }
                    WaitSecondsAction::AttackBurst(_, _) => {}
                }
            }
            MobPhase::WaitAnimationEnd(_) => {}
        }
    }
}

pub struct Stats {
    pub drawn_clouds: i32,
}

impl DrawState {
    pub fn new() -> DrawState {
        DrawState {
            fps_counter: FpsCounter::new()
        }
    }
}

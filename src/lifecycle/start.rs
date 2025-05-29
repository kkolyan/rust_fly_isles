use std::collections::{HashMap, VecDeque};
use std::f32::consts::PI;
use std::ops::Range;
use std::rc::Rc;
use macroquad::audio::PlaySoundParams;

use macroquad::prelude::{Vec2, Vec3};
use macroquad::rand::{rand, RandomRange};

use crate::{GameState, Plane, PlaneId, PlayerState};
use crate::common::angle::AsRadians;
use crate::common::contract::GetMut;
use crate::common::enum_maps::{new_enum_map, new_enum_map_async};
use crate::common::pool::Pool;
use crate::common::prefs::Pref;
use crate::common::resource::{Resource, ResourceGet, ResourceManager, ResourceManagerRc};
use crate::common::unsorted;
use crate::common::sound::PlaySound;
use crate::resources::objects::objects::plane001;
use crate::game::{bots, isles, mobs, plane, rpg, sky, sounds};
use crate::model::def::{BackgroundObject, Game, ProgressFlag, Location, MaterialInstance};
use crate::model::state::{AudioManager, BackgroundObjectState, BotState, GameProgression, Gids, LootManager, ParticlesState, RpgState, SubSystems, WeaponSelector};
use crate::rand::{ChooseRandom, gen_range};
use crate::resources::constants::INITIAL_ENERGY;
use crate::resources::materials::pain::pain_material;
use crate::resources::objects::arms::cannon_gatling;
use crate::resources::objects::locations::location001::location001;
use crate::resources::objects::objects;

pub fn new_game(def: &Resource<Game>, location: &Resource<Location>) -> GameState {
    let mut state = GameState {
        paused: false,
        location: location.clone(),
        player: PlayerState {
            steering: false,
            plane: None,
            camera_pos: Default::default(),
            god: false,
            resources: new_enum_map(|it| 0),
            windows: Default::default(),
            equipment: Pool::new(),
            rpg: RpgState {
                xp: 0,
                level: 1,
                skill_points: 0,
                skills: Default::default(),
            },
            death_count: 0,
            hp_max: def.player_plane.hp,
            energy_max: INITIAL_ENERGY,
            thrust_tech_level: 0
        },
        def: def.clone(),
        planes: Pool::new(),
        background_objects: vec![],
        particles: ParticlesState {
            fixed: vec![],
            moving: vec![],
        },
        rays: vec![],
        bots: vec![],
        projectiles: vec![],
        metrics: Default::default(),
        unpause_one_frame: false,
        show_colliders: false,
        isles: Pool::new(),
        mobs: Pool::new(),
        loot: Pool::new(),
        gids: Gids {
            prev_gid: 0
        },
        subsystems: SubSystems {
            audio: AudioManager {
                volume: Pref::new("audio.volume", 1.0),
                looped: vec![],
                cool_down_sec: Default::default()
            },
            loot: LootManager { accumulators: Default::default() }
        },
        commands: Default::default(),
        ui_commands: Default::default(),
        progression: GameProgression { flags: Default::default() },
        journal: vec![],
        reachable_mobs: vec![]
    };
    spawn_player_plane(&mut state);
    if let Some(weapon) = &state.location.default_weapon {
        rpg::give_equipment(&mut state.player, rpg::weapon_to_device(&def.player_plane, weapon))
    }
    // me.body.velocity = Vec2::ZERO;
    // me.gear = 0;
    // me.body.rot = 15.0 / 180.0 * PI;
    // me.body.rot = 75.0 / 180.0 * PI;
    // me.body.rot = (0.0 / 180.0 * PI).as_radians();
    sky::init_clouds(&mut state);

    bots::init_bots(&mut state);

    isles::init_isles(&mut state);

    state
}

pub fn spawn_player_plane(mut state: &mut GameState) {
    let plane = &state.def.player_plane;
    state.player.plane = Some(plane::allocate_plane(
        &mut state.planes,
        plane.clone(),
        state.location.size * state.location.start_pos_norm,
        0.0f32.as_radians(),
        &state.def,
        &state.player
    ));
    let me = state.planes.get_mut(&state.player.plane.unwrap()).unwrap();
    sounds::on_start(&mut state);
}
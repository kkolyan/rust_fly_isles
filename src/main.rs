#![allow(unused_imports)]
#![allow(unused)]

// `x <= 0` is better even when x is unsigned, because I don't want to think about sighness of x
// when I read code and `< 0` is common mistake in signed numbers.
#![allow(clippy::absurd_extreme_comparisons)]

// separated ifs are mor readable in a lot of cases
#![allow(clippy::collapsible_if)]

extern crate core;

use std::process::exit;
use std::time::{Duration, Instant};

use futures::{AsyncReadExt, FutureExt};
use macroquad::audio::PlaySoundParams;
use macroquad::prelude::*;

use lifecycle::{draw, input, start, update};
use lifecycle::draw::DrawState;
use model::def::Plane;
use model::state::{GameState, PlaneId, PlaneState, PlayerState};
use resources::constants::MAX_FPS;

use crate::common::frame::FrameCtx;
use crate::common::perf::{perf_report, perf_task};
use crate::common::resource::{ResourceGet, ResourceManager, ResourceManagerRc};
use crate::game::{game_viewport, ui};
use crate::game::sounds::on_pause;
use crate::model::def::Game;
use crate::model::state::{AppState, MenuState};
use crate::resources::constants::{DT_MAX, DT_MIN};
use crate::resources::games::game_001;

mod game;
mod common;
mod lifecycle;
mod resources;
pub mod model;
mod conf;

fn main() {
    perf_task("program started");
    macroquad::Window::from_config(conf::deal_with_config_file(Conf {
        window_title: "Forbidden Islands".to_owned(),
        window_width: 1366,
        window_height: 768,
        fullscreen: true,
        ..Default::default()
    }), async_main());
    perf_task("main thread done")
}

async fn async_main() {
    next_frame().await;
    perf_task("async_main entered");
    let rm = ResourceManager::new(|enqueued, completed| {
        async move {
            if is_key_pressed(KeyCode::Escape) {
                exit(0);
            }
            let progress = if enqueued > 0 { completed as f32 / enqueued as f32 } else { 0.0 };
            clear_background(BLACK);
            let message = format!("loading: {:.00}% ({} of {})", progress * 100.0, completed, enqueued);
            // let message = format!("loading: {:.00}%", progress * 100.0);
            info!("[INIT] {}", message);
            let font_size = 72.0;
            let dimensions = measure_text(message.as_str(), None, font_size as u16, 1.0);
            let sw = screen_width();
            let sh = screen_height();
            let tw = dimensions.width;
            let th = dimensions.height;
            draw_text(message.as_str(), sw * 0.5 - tw * 0.5, sh * 0.5 - th * 0.5, font_size, WHITE);
            next_frame().await;
        }.boxed_local()
    });

    let mut app = AppState::Intro;

    let game = game_001.get(&rm);

    rm.poll_tasks().await;

    perf_task("GameState constructed");

    rm.perform_debug_checks();

    let mut draw_state = DrawState::new();


    perf_report();

    let target_duration = Duration::from_secs_f64(1.0 / MAX_FPS);

    let mut last_sync = std::time::Instant::now();
        get_time();

    let mut frame = 0i64;
    let mut paused_prev = false;
    loop {
        let dt = FrameCtx {
            dt: get_frame_time().clamp(DT_MIN, DT_MAX),
            frame,
        };

        app = input::process_input(app);

        let mut paused = true;
        match &mut app {
            AppState::Title { .. } => {}
            AppState::Game { game, .. } => {
                if !game.paused {
                    let view_port = game_viewport::create_viewport(game);
                    if game.player.windows.is_empty() {
                        paused = false;
                        update::update_game_state(game, &dt, &view_port);
                    } else if game.unpause_one_frame {
                        update::update_game_state(game, &dt, &view_port);
                        paused = false;
                        game.unpause_one_frame = false;
                    }
                    update::update_command_queue(game);
                }
            }
            AppState::GameMenu { .. } => {}
            AppState::Intro => {}
        };
        if paused != paused_prev {
            match &mut app {
                AppState::Title { .. } => {}
                AppState::Game { game, .. } | AppState::GameMenu { game, .. } => {
                    on_pause(game, paused);
                }
                AppState::Intro => {}
            }
            paused_prev = paused;
        }

        draw::draw_state(&mut app, &mut draw_state);

        app = ui::do_ui(app, &draw_state, &game);

        let mut now: Instant;
        loop {
            now = Instant::now();
            let actual_duration = now - last_sync;
            if actual_duration >= target_duration {
                break;
            }
            let rem =  target_duration - actual_duration;
        }
        last_sync = now;

        next_frame().await;

        frame += 1;
    }
}

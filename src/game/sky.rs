use macroquad::prelude::{Vec2, Vec3, Vec3Swizzles};
use macroquad::color::{BLUE, GREEN};
use macroquad::material::gl_use_default_material;
use macroquad::math::Rect;
use macroquad::models::{draw_mesh, Mesh, Vertex};
use macroquad::prelude::{Color, GRAY, WHITE};
use crate::lifecycle::draw::Stats;
use crate::GameState;
use crate::common::{camera, sprite, unsorted};
use crate::common::angle::Angle;
use crate::common::unsorted::{RectExtOps, WithMut};
use crate::common::camera::ViewPort;
use crate::common::sprite::{draw_sprite};
use crate::model::state::BackgroundObjectState;
use crate::rand::{ChooseRandom, gen_range};
use crate::resources::constants::LOGIC_RESOLUTION;

pub fn init_clouds(state: &mut GameState) {
    for cloud in &state.location.background_objects {
        let count = cloud.count.lerp(gen_range(0.0, 1.0));
        for _ in 0..count {
            let z = cloud.z.lerp(gen_range(0.0, 1.0));
            let furthest_size = state.location.size;
            let pos = Vec2::new(
                unsorted::gen_range(0.0..furthest_size.x),
                cloud.height_normal.clone()
                    .map(|it| it.random() * furthest_size.y)
                    .unwrap_or_else(|| unsorted::gen_range(0.0..furthest_size.y))
                ,
            );
            state.background_objects.push(BackgroundObjectState {
                sprite: cloud.sprite.choose().unwrap().clone(),
                pos: pos.extend(z),
                size: cloud.size.lerp(gen_range(0.0, 1.0)),
                material: cloud.material.clone(),
            });
        }
    }
    state.background_objects.sort_by_key(|it| (it.pos.z * -1000.0) as i32);
}

pub fn draw_sky(state: &GameState, view_port: &ViewPort) {
    let hy = LOGIC_RESOLUTION.1 * 1.0;
    let top = (state.player.camera_pos.y - hy) / state.location.size.y;
    let bottom = (state.player.camera_pos.y + hy) / state.location.size.y;

    let top_color = state.location.sky.color_by_height.lerp(top);
    let bottom_color = state.location.sky.color_by_height.lerp(bottom);

    gl_use_default_material();
    draw_mesh(&Mesh {
        vertices: vec![
            Vertex { position: Vec3::new(0.0, 0.0, 0.0), uv: Vec2::ZERO, color: top_color },
            Vertex { position: Vec3::new(view_port.screen_size.x, 0.0, 0.0), uv: Vec2::ZERO, color: top_color },
            Vertex { position: Vec3::new(view_port.screen_size.x, view_port.screen_size.y, 0.0), uv: Vec2::ZERO, color: bottom_color },
            Vertex { position: Vec3::new(0.0, view_port.screen_size.y, 0.0), uv: Vec2::ZERO, color: bottom_color },
        ],
        indices: vec![0, 1, 2, 0, 2, 3],
        texture: None,
    })
}

pub fn draw_clouds(state: &GameState, stats: &mut Stats, view_port: &ViewPort) {
    for cloud in &state.background_objects {
        view_port.draw_deep(cloud.pos.xy(), cloud.size, Some(cloud.pos.z), |ported| {
            draw_sprite(&cloud.sprite, ported.screen_pos, |it| {
                it.material = cloud.material.clone();
                it.screen_scale = ported.screen_scale;
            });
        });
    }
}
use std::cell::Cell;
use std::ops::{Mul, Range};
use std::rc::Rc;
use futures::future::LocalBoxFuture;
use macroquad::prelude::{f32, IVec2, Mat3, Vec2, Vec3, Vec3Swizzles, Vec4Swizzles};
use macroquad::color::WHITE;
use macroquad::material::Material;
use macroquad::miniquad::gl::glUniform3f;
use macroquad::models::{draw_mesh, Mesh, Vertex};
use macroquad::prelude::{Color, gl_use_default_material, gl_use_material, load_material, Rect, Texture2D};
use crate::common::angle::Angle;
use crate::common::resource::Resource;
use crate::common::unsorted::{ModifyColor, ToColor, WithMut};
use crate::model::data::SpriteMod;
use crate::model::def::{MaterialInstance, Sprite, SpriteRegion, UniformSupplier};

use crate::resources::constants::SCALE_SPEED;

thread_local! {
    pub static DEBUG_TINT: Cell<Color> = Cell::new(WHITE);
}

pub async fn load_sprite(data: &[u8], sprite_mods: Vec<SpriteMod>) -> Sprite {
    let texture = Texture2D::from_file_with_format(data, None);
    let size = Vec2::new(texture.width(), texture.height());
    let origin = Vec2::ONE * 0.5;
    let region = SpriteRegion { x0: 0.0, y0: 0.0, x1: 1.0, y1: 1.0 };
    let scale = 1.0;
    let mut sprite = Sprite { texture, origin_normalized: origin, size, scale, angle: Angle::ZERO, region, collision_circle_normalized: None };
    apply_sprite_mods(&mut sprite, sprite_mods);
    sprite
}

pub fn apply_sprite_mods(sprite: &mut Sprite, sprite_mods: Vec<SpriteMod>) {
    for sprite_mod in sprite_mods {
        match sprite_mod {
            SpriteMod::Scale(v) => sprite.scale *= v,
            SpriteMod::OriginNorm(v) => sprite.origin_normalized = v,
            SpriteMod::Rot(v) => sprite.angle = v,
        }
    }
}

#[derive(Debug, Clone)]
pub enum SheetRegion {
    All,
    Range(Range<(usize, usize)>),
    Cells(Vec<(usize, usize)>),
}

pub async fn load_sprites_from_files(files: Vec<&[u8]>, sprite_mods: Vec<SpriteMod>) -> Vec<Sprite>
{
    let mut sprites = vec![];
    for file in files {
        sprites.push(load_sprite(file, sprite_mods.clone()).await);
    }
    sprites
}

pub fn load_sprites_from_sheet<F: Fn(&mut Sprite)>(data: &[u8], sheet_size: (usize, usize), region: SheetRegion, f: F) -> Vec<Sprite> {
    let cells: Vec<(usize, usize)> = match region {
        SheetRegion::All => (0..sheet_size.1)
            .flat_map(|y| (0..sheet_size.0)
                .map(move |x| (x, y)))
            .collect(),
        SheetRegion::Range(range) => (range.start.1..range.end.1)
            .flat_map(|y| (range.start.0..range.end.0)
                .map(move |x| (x, y)))
            .collect(),
        SheetRegion::Cells(cells) => cells
    };
    let texture = Texture2D::from_file_with_format(data, None);
    let size = Vec2::new(texture.width() / sheet_size.0 as f32, texture.height() / sheet_size.1 as f32);
    let cell_size = (1.0 / sheet_size.0 as f32, 1.0 / sheet_size.1 as f32);
    cells
        .iter()
        .map(|cell| {
            let u0 = cell.0 as f32 / sheet_size.0 as f32;
            let v0 = cell.1 as f32 / sheet_size.1 as f32;
            let mut sprite = Sprite {
                texture,
                origin_normalized: Vec2::ONE * 0.5,
                size,
                scale: 1.0,
                angle: Angle::ZERO,
                region: SpriteRegion { x0: u0, y0: v0, x1: u0 + cell_size.0, y1: v0 + cell_size.1 },
                collision_circle_normalized: None
            };
            f(&mut sprite);
            sprite
        })
        .collect()
}


#[derive(Clone)]
pub struct SpriteDraw {
    pub screen_scale: f32,
    pub angle: Angle,
    pub flip_x: bool,
    pub flip_y: bool,
    pub tint: Color,
    pub material: Option<Resource<MaterialInstance>>,
}

pub fn draw_sprite<F>(sprite: &Sprite, screen_pos: Vec2, f: F) where
    F: Fn(&mut SpriteDraw)
{
    let mut state = SpriteDraw {
        screen_scale: 1.0,
        angle: Angle::ZERO,
        flip_y: false,
        flip_x: false,
        tint: WHITE,
        material: None,
    };

    f(&mut state);

    let mut origin = sprite.origin_normalized;
    if state.flip_x {
        origin.x = 1.0 - origin.x;
    }
    if state.flip_y {
        origin.y = 1.0 - origin.y;
    }
    let origin = origin * sprite.size;

    let mut m: Mat3 = Mat3::IDENTITY;

    m = Mat3::from_scale(sprite.size) * m;
    m = Mat3::from_translation(-origin) * m;
    m = Mat3::from_rotation_z((state.angle + sprite.angle).to_rad()) * m;
    m = Mat3::from_scale(Vec2::ONE * state.screen_scale * SCALE_SPEED) * m;
    m = Mat3::from_scale(Vec2::ONE * sprite.scale) * m;
    m = Mat3::from_translation(screen_pos) * m;

    let uv = &sprite.region;


    let color = DEBUG_TINT.with(|it| {
        let a = state.tint.to_vec();
        let b = it.get().to_vec();
        a * b
    }).to_color();

    let mut vertices = vec!(
        Vertex { position: Vec3::new(0.0, 0.0, 0.0), uv: Vec2::new(uv.x0, uv.y0), color },
        Vertex { position: Vec3::new(1.0, 0.0, 0.0), uv: Vec2::new(uv.x1, uv.y0), color },
        Vertex { position: Vec3::new(1.0, 1.0, 0.0), uv: Vec2::new(uv.x1, uv.y1), color },
        Vertex { position: Vec3::new(0.0, 1.0, 0.0), uv: Vec2::new(uv.x0, uv.y1), color }
    );

    for v in vertices.iter_mut() {
        v.position = m.transform_point2(v.position.xy()).extend(0.0);
    }

    let mut swap_uv = |a: usize, b: usize| {
        let temp = vertices[a].uv;
        vertices[a].uv = vertices[b].uv;
        vertices[b].uv = temp;
    };

    if state.flip_y {
        swap_uv(0, 3);
        swap_uv(1, 2);
    }

    if state.flip_x {
        swap_uv(0, 1);
        swap_uv(3, 2);
    }

    let reset_after = match &state.material {
        None => true,
        Some(material) => {
            gl_use_material(*material.material);
            for (name, supplier) in &material.uniforms {
                match supplier {
                    UniformSupplier::Color(color) => {
                        material.material.set_uniform(name, color.to_vec())
                    }
                }
            }
            true
        }
    };
    draw_mesh(&Mesh {
        vertices,
        indices: vec![0, 1, 2, 0, 2, 3],
        texture: Some(sprite.texture),
    });
    if reset_after {
        gl_use_default_material();
    }
}

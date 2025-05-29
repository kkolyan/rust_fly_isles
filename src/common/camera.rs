use std::rc::Rc;

use macroquad::prelude::Vec2;
use macroquad::color::WHITE;
use macroquad::math::Rect;
use macroquad::prelude::{Color, GRAY};

use crate::common::angle::Angle;
use crate::common::sprite;
use crate::model::def::Sprite;
use crate::resources::constants::TINT_DUPLICATE;

#[derive(Clone, Debug)]
pub struct ViewPort {
    pub camera_pos: Vec2,
    pub screen_size: Vec2,
    pub view_scale: f32,
    pub bounds: Rect,
    pub loc_size: Vec2,
    pub left_pass: bool,
    pub right_pass: bool,
}

#[derive(Copy, Clone, Debug)]
pub struct Ported {
    pub screen_pos: Vec2,
    pub screen_scale: f32,
}

impl ViewPort {
    pub fn port(&self, pos: Vec2, scale: f32, task: impl Fn(Ported)) {
        self.draw_deep(pos, scale, None, task);
    }

    pub fn screen_to_world(&self, pos_screen: Vec2, z: Option<f32>) -> Vec2 {
        (pos_screen - self.screen_size * 0.5) * z.unwrap_or(1.0) / self.view_scale + self.camera_pos
    }

    pub fn draw_deep(&self, pos: Vec2, scale: f32, z: Option<f32>, task: impl Fn(Ported)) {
        let mut do_pass = |dx: f32| {
            let z = z.unwrap_or(1.0);

            // game space
            let xy = pos + Vec2::new(dx, 0.0);

            let pos_axially = xy - self.camera_pos;

            // screen space
            let pos_screen = pos_axially / z * self.view_scale + self.screen_size * 0.5;

            if self.bounds.contains(pos_screen) {
                task(Ported {
                    screen_pos: pos_screen,
                    screen_scale: scale / z * self.view_scale,
                })
            }
        };
        do_pass(0.0);

        sprite::DEBUG_TINT.with(|it| {
            let old = it.replace(TINT_DUPLICATE);
            if self.left_pass {
                do_pass(-self.loc_size.x);
            }
            if self.right_pass {
                do_pass(self.loc_size.x);
            }
            it.set(old)
        });
    }
}

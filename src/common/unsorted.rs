use std::f32::consts::PI;
use std::ops::{Add, Mul, Range};
use std::rc::Rc;
use macroquad::audio::{play_sound, play_sound_once, PlaySoundParams, set_sound_volume, Sound, stop_sound};
use macroquad::prelude::{DVec4, f32, Vec2, Vec4, Vec4Swizzles};
use macroquad::color::Color;
use macroquad::prelude::{GREEN, Rect, RED};
use macroquad::rand::RandomRange;
use crate::common::angle::{Angle, AsRadians};
use crate::common::resource::Resource;
use crate::common::curve::Lerp;
use crate::model::def::GameSound;
use crate::model::state::AudioManager;

pub fn gen_range<T: RandomRange + Clone>(range: Range<T>) -> T {
    T::gen_range(range.start.clone(), range.end)
}

pub trait ToColor {
    fn to_color(self) -> Color;
}

impl ToColor for &str {
    fn to_color(self) -> Color {
        let c = csscolorparser::parse(self).unwrap();
        Color::from_vec(DVec4::from(c.rgba()).as_f32())
    }
}

pub trait ColorOps {
    fn with_alpha(&self, a: f32) -> Color;
    fn rgb_mul(&self, f: f32) -> Color;
}

impl ColorOps for Color {
    fn with_alpha(&self, a: f32) -> Color {
        self.to_vec()
            .xyz()
            .extend(a)
            .to_color()
    }

    fn rgb_mul(&self, f: f32) -> Color {
        let a = self.a;
        self.to_vec()
            .xyz()
            .mul(f)
            .extend(a)
            .to_color()
    }
}

pub trait ToAngle {
    fn to_angle(self) -> Angle;
}

impl ToAngle for Vec2 {
    fn to_angle(self) -> Angle {
        Vec2::X.angle_between(self).as_radians()
    }
}

impl ToColor for Vec4 {
    fn to_color(self) -> Color {
        Color::from_vec(self)
    }
}

impl Lerp<Color> for Color {
    fn lerp(a: Color, b: Color, s: f32) -> Color { Color::from_vec(a.to_vec().lerp(b.to_vec(), s)) }
}

pub trait RectExtOps {
    fn scale_with_pos(&self, scale: f32) -> Self;
}

impl RectExtOps for Rect {
    fn scale_with_pos(&self, scale: f32) -> Self {
        Rect::new(self.x * scale, self.y * scale, self.w * scale, self.h * scale)
    }
}

pub trait IndexRange {
    fn indices(&self) -> Range<usize>;
}

impl<T> IndexRange for Vec<T> {
    fn indices(&self) -> Range<usize> { 0..self.len() }
}

pub trait RangeAdd<T> {
    fn add(&self, other: T) -> Self;
}

impl<T> RangeAdd<T> for Range<T>
    where T: Add<T, Output=T>,
          T: Clone
{
    fn add(&self, other: T) -> Self {
        (self.start.clone() + other.clone())..(self.end.clone() + other.clone())
    }
}

pub trait ModifyColor {
    fn modify_color(&self, original: Color) -> Color;
}

pub fn identity_fn<T, F: Fn(T) -> T>() -> impl Fn(T) -> T {
    |it| it
}

pub trait WithMut: Sized {
    fn with_mut<F: Fn(&mut Self)>(self, f: F) -> Self {
        let mut v = self;
        f(&mut v);
        v
    }
}

impl<T: Sized> WithMut for T {}

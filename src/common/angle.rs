use std::f32::consts::PI;
use std::fmt::{Display, Formatter, Pointer, UpperExp};
use std::ops::{Add, AddAssign, Sub};
use macroquad::prelude::{Mat2, Vec2};
use crate::common::unsorted::gen_range;

#[derive(Copy, Clone, PartialOrd, PartialEq, Debug)]
pub struct Angle {
    radians: f32,
}

impl Angle {
    pub fn random() -> Angle {
        gen_range(-PI..PI).as_radians()
    }
}

pub trait AsRadians {
    fn as_radians(&self) -> Angle;
}

impl AsRadians for f32 {
    fn as_radians(&self) -> Angle {
        Angle { radians: *self }
    }
}

impl Display for Angle {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}°", self.to_deg())
    }
}

impl AddAssign for Angle {
    fn add_assign(&mut self, rhs: Self) {
        self.radians += rhs.radians;
    }
}

impl Sub for Angle {
    type Output = Angle;

    fn sub(self, rhs: Self) -> Self::Output {
        (self.radians - rhs.radians).as_radians()
    }
}

impl Add for Angle {
    type Output = Angle;

    fn add(self, rhs: Self) -> Self::Output {
        (self.radians + rhs.radians).as_radians()
    }
}

impl Angle {
    pub const ZERO: Angle = Angle { radians: 0.0 };

    pub fn degrees(degrees: f32) -> Angle {
        Angle { radians: degrees.to_radians() }
    }

    pub fn to_vec2_norm(self) -> Vec2 {
        Vec2::new(self.radians.cos(), self.radians.sin())
    }

    pub fn to_deg(self) -> f32 {
        self.radians * 180.0 / PI
    }

    pub fn to_rad(self) -> f32 {
        self.radians
    }

    pub fn normalize(self) -> Angle {
        let mut rot = self.radians;
        while rot >= PI  { rot -= PI * 2.0; }
        while rot < -PI { rot += PI * 2.0; }
        rot.as_radians()
    }
}

#[test]
fn test4() {
    let a = 50.0;
    let old = Mat2::from_angle(a) * Vec2::new(1.0, 0.0);
    let new = a.as_radians().to_vec2_norm();
    assert_eq!(old, new);
}
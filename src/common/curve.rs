use std::fmt::{Debug, Display, format, Formatter};
use std::ops::Mul;
use std::rc::Rc;

use macroquad::math::clamp;

use crate::common::angle::Angle;
use crate::common::unsorted::gen_range;

type F<T> = dyn Fn(f32) -> T;

#[derive(Clone, Debug)]
pub enum Curve<T: Clone> {
    Manual {
        points: Vec<T>,
    },
    Function(CurveFunction<T>),
}

impl <T: Clone> Curve<T> {
    pub fn from_function<F: 'static +  Fn(f32) -> T>(f: F) -> Self {
        Curve::Function(CurveFunction(Rc::new(f)))
    }
}

#[derive(Clone)]
pub struct CurveFunction<T>(Rc<dyn Fn(f32) -> T>);

impl <T> Debug for CurveFunction<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "CurveFunction")
    }
}

impl <T: Clone> Curve<T> {
    pub fn new<const N: usize>(points: [T; N]) -> Curve<T> {
        assert!(!points.is_empty(), "curve must contain at least one point");
        Curve::Manual { points: Vec::from(points) }
    }
}

pub enum Point<T> {
    Value(T),
    Transition(u8),
}

pub trait Lerp<T>: Clone {
    fn lerp(a: T, b: T, s: f32) -> T;
}

impl<T: Lerp<T>> Curve<T> {
    pub fn new_ext(points: &[Point<T>]) -> Curve<T> {
        let mut results = Vec::new();
        for (i, point) in points.iter().enumerate() {
            match point {
                Point::Value(value) => {
                    results.push(value.clone());
                }
                Point::Transition(n) => {
                    let left = match &points.get(i - 1) {
                        Some(Point::Value(value)) => value,
                        _ => panic!("there is no value at the left of transition")
                    };
                    let right = match &points.get(i + 1) {
                        Some(Point::Value(value)) => value,
                        _ => panic!("there is no value at the right of transition")
                    };
                    for j in 0..*n {
                        results.push(T::lerp(left.clone(), right.clone(), (j + 1) as f32 / (n + 1) as f32));
                    }
                }
            }
        }
        Curve::Manual {
            points: results
        }
    }

    pub fn lerp(&self, key: f32) -> T {
        match self {
            Curve::Manual { points } => {
                let key = clamp(key, 0.0, 1.0);
                let f_index = key * (points.len() - 1) as f32;
                let left = f_index.floor() as usize;
                let right = f_index.ceil() as usize;
                if left == right {
                    return points[left].clone();
                }

                T::lerp(points[left].clone(), points[right].clone(), f_index.fract())
            }
            Curve::Function(CurveFunction(f)) => f(key)
        }
    }

    pub fn random(&self) -> T {
        self.lerp(gen_range(0.0..1.0))
    }
}

impl<T: 'static> Mul<f32> for Curve<T>
    where
        T: Clone,
        T: Mul<f32, Output=T>
{
    type Output = Self;

    fn mul(self, rhs: f32) -> Self::Output {
        match self {
            Curve::Manual { points } => Curve::Manual {points : points.iter().map(|it| it.clone() * rhs).collect() },
            Curve::Function(CurveFunction(f)) => Curve::Function(CurveFunction(Rc::new(move |it| {
                f(it) * rhs
            })))
        }
    }
}

impl Lerp<f32> for f32 {
    fn lerp(a: f32, b: f32, s: f32) -> f32 {
        a + (b - a) * s
    }
}

impl Lerp<i32> for i32 {
    fn lerp(a: i32, b: i32, s: f32) -> i32 {
        a + ((b - a) as f32 * s).round() as i32
    }
}

impl Lerp<u32> for u32 {
    fn lerp(a: u32, b: u32, s: f32) -> u32 {
        a + ((b - a) as f32 * s).round() as u32
    }
}

impl Lerp<usize> for usize {
    fn lerp(a: usize, b: usize, s: f32) -> usize {
        a + ((b as f32 - a as f32) * s).round() as usize
    }
}

#[cfg(test)]
mod tests {
    use crate::common::curve::{Curve, Lerp, Point};

    #[test]
    fn test_ext2() {
        if let Curve::Manual { points } = Curve::new_ext(&[Point::Value(1.0), Point::Transition(2), Point::Value(10.0)]) {
            assert_eq!(points, vec![1.0, 4.0, 7.0, 10.0])
        } else {
            panic!();
        }
    }

    #[test]
    fn test_ext1() {
        if let Curve::Manual { points } = Curve::new_ext(&[Point::Value(1.0), Point::Transition(1), Point::Value(10.0)]) {
            assert_eq!(points, vec![1.0, 5.5, 10.0])
        } else {
            panic!();
        }
    }

    #[test]
    #[should_panic(expected = "curve must contain at least one point")]
    fn test_0() {
        let _ = Curve::<f32>::new([]);
    }

    #[test]
    fn test_out_left() {
        let curve = Curve::new([7.0, 10.0]);
        assert_eq!(curve.lerp(-1.5), 7.0);
    }

    #[test]
    fn test_out_right() {
        let curve = Curve::new([7.0, 10.0]);
        assert_eq!(curve.lerp(2.5), 10.0);
    }

    #[test]
    fn test1() {
        let curve = Curve::new([7.0]);
        assert_eq!(curve.lerp(0.5), 7.0);
        assert_eq!(curve.lerp(0.0), 7.0);
        assert_eq!(curve.lerp(0.1), 7.0);
    }

    #[test]
    fn test2() {
        let curve = Curve::new([7.0, 10.0]);
        assert_eq!(curve.lerp(0.0), 7.0);
        assert_eq!(curve.lerp(0.5), 8.5);
        assert_eq!(curve.lerp(1.0), 10.0);
    }

    #[test]
    fn test2_inverse() {
        let curve = Curve::new([10.0, 7.0]);
        assert_eq!(curve.lerp(0.0), 10.0);
        assert_eq!(curve.lerp(0.5), 8.5);
        assert_eq!(curve.lerp(1.0), 7.0);
    }

    #[test]
    fn test3() {
        let curve = Curve::new([7.0, 10.0, 11.0]);
        assert_eq!(curve.lerp(0.5), 10.0);
        assert_eq!(curve.lerp(0.25), 8.5);
        assert_eq!(curve.lerp(0.75), 10.5);
        assert_eq!(curve.lerp(0.0), 7.0);
        assert_eq!(curve.lerp(1.0), 11.0);
    }

    #[test]
    fn test_lerp_i32() {
        assert_eq!(i32::lerp(1, 10, 1.0), 10);
        assert_eq!(i32::lerp(1, 10, 0.0), 1);
        assert_eq!(i32::lerp(1, 10, 0.5), 6);
        assert_eq!(i32::lerp(1, 9, 0.5), 5);
    }
}

impl From<i32> for Curve<i32> {
    fn from(value: i32) -> Self {
        Curve::new([value])
    }
}

impl From<usize> for Curve<usize> {
    fn from(value: usize) -> Self {
        Curve::new([value])
    }
}

impl From<f32> for Curve<f32> {
    fn from(value: f32) -> Self {
        Curve::new([value])
    }
}
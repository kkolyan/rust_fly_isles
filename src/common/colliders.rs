// use std::marker::PhantomData;
// use macroquad::prelude::Vec2;
//
// #[derive(Debug, Clone)]
// pub enum Collider {
//     Circle {
//         center: Vec2,
//         radius: f32,
//     }
// }
//
// pub struct ColliderHandle {}
//
// #[derive(Clone, Debug)]
// pub struct CollisionField {
//     items: Vec<ColliderBox>,
//
// }
//
// struct ColliderBox {
//
// }
//
// impl CollisionField {
//     pub fn create_collider<T: Copy, F: Fn(T) -> Vec2>(&mut self, collider: Collider, key: T, get_pos: F) -> ColliderHandle {
//
//         todo!()
//     }
// }
#![allow(non_upper_case_globals)]

use futures::FutureExt;
use macroquad::miniquad::{BlendState, Equation};
use macroquad::prelude::{Color, load_material, Material, MaterialParams, PipelineParams, UniformType};
use macroquad::prelude::Vec4Swizzles;
use crate::common::resource::{ResourceGet, ResourceLoad, ResourceManagerRc};
use crate::common::unsorted::{ToColor, ColorOps};
use crate::miniquad::{BlendFactor, BlendValue};
use crate::model::def::{MaterialInstance, UniformSupplier};
use crate::ResourceManager;

const VERTEX: &str = include_str!("default.vert");
const FRAGMENT: &str = include_str!("huer.frag");

pub const U_SOURCE_COLOR: &str = "SourceColor";
pub const U_TARGET_COLOR: &str = "TargetColor";

pub const huer_shader: ResourceLoad<Material> = |rm| {
    let params = PipelineParams {
        color_blend: Some(BlendState::new(
            Equation::Add,
            BlendFactor::Value(BlendValue::SourceAlpha),
            BlendFactor::OneMinusValue(BlendValue::SourceAlpha),
        )),
        ..Default::default()
    };
    load_material(VERTEX, FRAGMENT, MaterialParams {
        pipeline_params: params,
        uniforms: vec![
            (U_SOURCE_COLOR.to_owned(), UniformType::Float4),
            (U_TARGET_COLOR.to_owned(), UniformType::Float4),
        ],
        textures: vec![],
    }).unwrap()
};

pub fn create_huer_material(rm: &ResourceManager, source: Color, target: Color) -> MaterialInstance {
    MaterialInstance {
        material: huer_shader.get(rm),
        uniforms: vec![
            (U_SOURCE_COLOR, UniformSupplier::Color(source)),
            (U_TARGET_COLOR, UniformSupplier::Color(target)),
        ],
    }
}

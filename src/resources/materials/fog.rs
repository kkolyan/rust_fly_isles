#![allow(non_upper_case_globals)]

use futures::FutureExt;
use macroquad::miniquad::{BlendState, Equation};
use macroquad::prelude::{load_material, Material, MaterialParams, PipelineParams, UniformType};
use macroquad::prelude::Vec4Swizzles;
use crate::common::resource::{ResourceGet, ResourceLoad, ResourceManagerRc};
use crate::common::unsorted::{ToColor, ColorOps};
use crate::miniquad::{BlendFactor, BlendValue};
use crate::model::def::{MaterialInstance, UniformSupplier};

const VERTEX: &str = include_str!("default.vert");
const FRAGMENT: &str = include_str!("fog.frag");

pub const U_FOG_COLOR: &str = "FogColor";

pub const fog_shader: ResourceLoad<Material> = |rm| {
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
            (U_FOG_COLOR.to_owned(), UniformType::Float4)
        ],
        textures: vec![],
    }).unwrap()
};

pub const fog_material: ResourceLoad<MaterialInstance> = |rm| MaterialInstance {
    material: fog_shader.get(&rm),
    uniforms: vec![
        (U_FOG_COLOR, UniformSupplier::Color("#CCC".to_color().with_alpha(0.5)))
    ],
};

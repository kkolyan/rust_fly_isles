use crate::common::resource::{ResourceGet, ResourceLoad, ResourceManagerRc};
use crate::common::unsorted::{ToColor, ColorOps};
use crate::model::def::{MaterialInstance, UniformSupplier};
use crate::resources::materials::fog::{fog_shader, U_FOG_COLOR};
use crate::{FutureExt, Vec4Swizzles};

pub const pain_material: ResourceLoad<MaterialInstance> = |rm|
    MaterialInstance {
        material: fog_shader.get(&rm),
        uniforms: vec![
            (U_FOG_COLOR, UniformSupplier::Color("#FFF".to_color().with_alpha(0.5)))
        ],
    };

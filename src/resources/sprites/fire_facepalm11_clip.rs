use crate::common::angle::Angle;
use crate::common::curve::{Curve};
use crate::common::resource::{ResourceLoad, ResourceLoadAsync};
use crate::lifecycle::loading;
use crate::model::data::{GenericSplitSpriteClipData, SplitSpriteClipData};
use crate::model::data::SpriteMod::{OriginNorm, Rot};
use crate::model::def::OnClipEnd::{Clamp, Repeat};
use crate::model::def::SpriteClip;
use crate::Vec2;

pub const fire_facepalm11_clip: ResourceLoadAsync<SpriteClip> = |rm| loading::clip_split(rm, SplitSpriteClipData {
    scale: 0.2,
    alpha: None,
    data: GenericSplitSpriteClipData::MultiFile {
        rate: 90.0,
        sprite_mod: vec![
            Rot(Angle::degrees(-90.0)),
            OriginNorm(Vec2::new(0.5, 0.98))
        ],
        data: vec![
            include_bytes!("../../../art/FacePalms 2D Explosions and Effects/EXTRAS/FIRE/FIRE_1/FIRE_1.1/FIRE_1.1-0001.png"),
            include_bytes!("../../../art/FacePalms 2D Explosions and Effects/EXTRAS/FIRE/FIRE_1/FIRE_1.1/FIRE_1.1-0002.png"),
            include_bytes!("../../../art/FacePalms 2D Explosions and Effects/EXTRAS/FIRE/FIRE_1/FIRE_1.1/FIRE_1.1-0003.png"),
            include_bytes!("../../../art/FacePalms 2D Explosions and Effects/EXTRAS/FIRE/FIRE_1/FIRE_1.1/FIRE_1.1-0004.png"),
            include_bytes!("../../../art/FacePalms 2D Explosions and Effects/EXTRAS/FIRE/FIRE_1/FIRE_1.1/FIRE_1.1-0005.png"),
            include_bytes!("../../../art/FacePalms 2D Explosions and Effects/EXTRAS/FIRE/FIRE_1/FIRE_1.1/FIRE_1.1-0006.png"),
            include_bytes!("../../../art/FacePalms 2D Explosions and Effects/EXTRAS/FIRE/FIRE_1/FIRE_1.1/FIRE_1.1-0007.png"),
            include_bytes!("../../../art/FacePalms 2D Explosions and Effects/EXTRAS/FIRE/FIRE_1/FIRE_1.1/FIRE_1.1-0008.png"),
            include_bytes!("../../../art/FacePalms 2D Explosions and Effects/EXTRAS/FIRE/FIRE_1/FIRE_1.1/FIRE_1.1-0009.png"),
            include_bytes!("../../../art/FacePalms 2D Explosions and Effects/EXTRAS/FIRE/FIRE_1/FIRE_1.1/FIRE_1.1-0010.png"),
            include_bytes!("../../../art/FacePalms 2D Explosions and Effects/EXTRAS/FIRE/FIRE_1/FIRE_1.1/FIRE_1.1-0011.png"),
            include_bytes!("../../../art/FacePalms 2D Explosions and Effects/EXTRAS/FIRE/FIRE_1/FIRE_1.1/FIRE_1.1-0012.png"),
            include_bytes!("../../../art/FacePalms 2D Explosions and Effects/EXTRAS/FIRE/FIRE_1/FIRE_1.1/FIRE_1.1-0013.png"),
            include_bytes!("../../../art/FacePalms 2D Explosions and Effects/EXTRAS/FIRE/FIRE_1/FIRE_1.1/FIRE_1.1-0014.png"),
            include_bytes!("../../../art/FacePalms 2D Explosions and Effects/EXTRAS/FIRE/FIRE_1/FIRE_1.1/FIRE_1.1-0015.png"),
            include_bytes!("../../../art/FacePalms 2D Explosions and Effects/EXTRAS/FIRE/FIRE_1/FIRE_1.1/FIRE_1.1-0016.png"),
            include_bytes!("../../../art/FacePalms 2D Explosions and Effects/EXTRAS/FIRE/FIRE_1/FIRE_1.1/FIRE_1.1-0017.png"),
            include_bytes!("../../../art/FacePalms 2D Explosions and Effects/EXTRAS/FIRE/FIRE_1/FIRE_1.1/FIRE_1.1-0018.png"),
            include_bytes!("../../../art/FacePalms 2D Explosions and Effects/EXTRAS/FIRE/FIRE_1/FIRE_1.1/FIRE_1.1-0019.png"),
            include_bytes!("../../../art/FacePalms 2D Explosions and Effects/EXTRAS/FIRE/FIRE_1/FIRE_1.1/FIRE_1.1-0020.png"),
            include_bytes!("../../../art/FacePalms 2D Explosions and Effects/EXTRAS/FIRE/FIRE_1/FIRE_1.1/FIRE_1.1-0021.png"),
            include_bytes!("../../../art/FacePalms 2D Explosions and Effects/EXTRAS/FIRE/FIRE_1/FIRE_1.1/FIRE_1.1-0022.png"),
            include_bytes!("../../../art/FacePalms 2D Explosions and Effects/EXTRAS/FIRE/FIRE_1/FIRE_1.1/FIRE_1.1-0023.png"),
            include_bytes!("../../../art/FacePalms 2D Explosions and Effects/EXTRAS/FIRE/FIRE_1/FIRE_1.1/FIRE_1.1-0024.png"),
            include_bytes!("../../../art/FacePalms 2D Explosions and Effects/EXTRAS/FIRE/FIRE_1/FIRE_1.1/FIRE_1.1-0025.png"),
        ],
        on_end: Repeat
    },
});

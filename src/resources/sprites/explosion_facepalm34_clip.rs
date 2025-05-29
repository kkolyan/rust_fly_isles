use crate::common::resource::{ResourceLoad, ResourceLoadAsync};
use crate::lifecycle::loading;
use crate::model::data::{GenericSplitSpriteClipData, SplitSpriteClipData};
use crate::model::def::OnClipEnd::Clamp;
use crate::model::def::SpriteClip;

pub const explosion_facepalm34_clip: ResourceLoadAsync<SpriteClip> = |rm| loading::clip_split(rm, SplitSpriteClipData {
    scale: 0.5,
    alpha: None,
    data: GenericSplitSpriteClipData::MultiFile {
        rate: 30.0,
        sprite_mod: vec![],
        data: vec![
            include_bytes!("../../../art/FacePalms 2D Explosions and Effects/EXPLOSIONS_3/EXPLOSION_3.4/EXPLOSION_3.4-0001.png"),
            include_bytes!("../../../art/FacePalms 2D Explosions and Effects/EXPLOSIONS_3/EXPLOSION_3.4/EXPLOSION_3.4-0002.png"),
            include_bytes!("../../../art/FacePalms 2D Explosions and Effects/EXPLOSIONS_3/EXPLOSION_3.4/EXPLOSION_3.4-0003.png"),
            include_bytes!("../../../art/FacePalms 2D Explosions and Effects/EXPLOSIONS_3/EXPLOSION_3.4/EXPLOSION_3.4-0004.png"),
            include_bytes!("../../../art/FacePalms 2D Explosions and Effects/EXPLOSIONS_3/EXPLOSION_3.4/EXPLOSION_3.4-0005.png"),
            include_bytes!("../../../art/FacePalms 2D Explosions and Effects/EXPLOSIONS_3/EXPLOSION_3.4/EXPLOSION_3.4-0006.png"),
            include_bytes!("../../../art/FacePalms 2D Explosions and Effects/EXPLOSIONS_3/EXPLOSION_3.4/EXPLOSION_3.4-0007.png"),
            include_bytes!("../../../art/FacePalms 2D Explosions and Effects/EXPLOSIONS_3/EXPLOSION_3.4/EXPLOSION_3.4-0008.png"),
            include_bytes!("../../../art/FacePalms 2D Explosions and Effects/EXPLOSIONS_3/EXPLOSION_3.4/EXPLOSION_3.4-0009.png"),
            include_bytes!("../../../art/FacePalms 2D Explosions and Effects/EXPLOSIONS_3/EXPLOSION_3.4/EXPLOSION_3.4-0010.png"),
            include_bytes!("../../../art/FacePalms 2D Explosions and Effects/EXPLOSIONS_3/EXPLOSION_3.4/EXPLOSION_3.4-0011.png"),
            include_bytes!("../../../art/FacePalms 2D Explosions and Effects/EXPLOSIONS_3/EXPLOSION_3.4/EXPLOSION_3.4-0012.png"),
            include_bytes!("../../../art/FacePalms 2D Explosions and Effects/EXPLOSIONS_3/EXPLOSION_3.4/EXPLOSION_3.4-0013.png"),
            include_bytes!("../../../art/FacePalms 2D Explosions and Effects/EXPLOSIONS_3/EXPLOSION_3.4/EXPLOSION_3.4-0014.png"),
            include_bytes!("../../../art/FacePalms 2D Explosions and Effects/EXPLOSIONS_3/EXPLOSION_3.4/EXPLOSION_3.4-0015.png"),
            include_bytes!("../../../art/FacePalms 2D Explosions and Effects/EXPLOSIONS_3/EXPLOSION_3.4/EXPLOSION_3.4-0016.png"),
            include_bytes!("../../../art/FacePalms 2D Explosions and Effects/EXPLOSIONS_3/EXPLOSION_3.4/EXPLOSION_3.4-0017.png"),
            include_bytes!("../../../art/FacePalms 2D Explosions and Effects/EXPLOSIONS_3/EXPLOSION_3.4/EXPLOSION_3.4-0018.png"),
            include_bytes!("../../../art/FacePalms 2D Explosions and Effects/EXPLOSIONS_3/EXPLOSION_3.4/EXPLOSION_3.4-0019.png"),
            include_bytes!("../../../art/FacePalms 2D Explosions and Effects/EXPLOSIONS_3/EXPLOSION_3.4/EXPLOSION_3.4-0020.png"),
            include_bytes!("../../../art/FacePalms 2D Explosions and Effects/EXPLOSIONS_3/EXPLOSION_3.4/EXPLOSION_3.4-0021.png"),
            include_bytes!("../../../art/FacePalms 2D Explosions and Effects/EXPLOSIONS_3/EXPLOSION_3.4/EXPLOSION_3.4-0022.png"),
            include_bytes!("../../../art/FacePalms 2D Explosions and Effects/EXPLOSIONS_3/EXPLOSION_3.4/EXPLOSION_3.4-0023.png"),
        ],
        on_end: Clamp,
    },
});

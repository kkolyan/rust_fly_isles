use crate::common::resource::{ResourceLoad, ResourceLoadAsync};
use crate::lifecycle::loading;
use crate::model::data::{GenericSplitSpriteClipData, SplitSpriteClipData};
use crate::model::def::OnClipEnd::Clamp;
use crate::model::def::SpriteClip;

pub const explosion_facepalm33_clip: ResourceLoadAsync<SpriteClip> = |rm| loading::clip_split(rm, SplitSpriteClipData {
    scale: 0.5,
    alpha: None,
    data: GenericSplitSpriteClipData::MultiFile {
        rate: 30.0,
        sprite_mod: vec![],
        data: vec![
            include_bytes!("../../../art/FacePalms 2D Explosions and Effects/EXPLOSIONS_3/EXPLOSION_3.3/EXPLOSION_3.3-0001.png"),
            include_bytes!("../../../art/FacePalms 2D Explosions and Effects/EXPLOSIONS_3/EXPLOSION_3.3/EXPLOSION_3.3-0002.png"),
            include_bytes!("../../../art/FacePalms 2D Explosions and Effects/EXPLOSIONS_3/EXPLOSION_3.3/EXPLOSION_3.3-0003.png"),
            include_bytes!("../../../art/FacePalms 2D Explosions and Effects/EXPLOSIONS_3/EXPLOSION_3.3/EXPLOSION_3.3-0004.png"),
            include_bytes!("../../../art/FacePalms 2D Explosions and Effects/EXPLOSIONS_3/EXPLOSION_3.3/EXPLOSION_3.3-0005.png"),
            include_bytes!("../../../art/FacePalms 2D Explosions and Effects/EXPLOSIONS_3/EXPLOSION_3.3/EXPLOSION_3.3-0006.png"),
            include_bytes!("../../../art/FacePalms 2D Explosions and Effects/EXPLOSIONS_3/EXPLOSION_3.3/EXPLOSION_3.3-0007.png"),
            include_bytes!("../../../art/FacePalms 2D Explosions and Effects/EXPLOSIONS_3/EXPLOSION_3.3/EXPLOSION_3.3-0008.png"),
            include_bytes!("../../../art/FacePalms 2D Explosions and Effects/EXPLOSIONS_3/EXPLOSION_3.3/EXPLOSION_3.3-0009.png"),
            include_bytes!("../../../art/FacePalms 2D Explosions and Effects/EXPLOSIONS_3/EXPLOSION_3.3/EXPLOSION_3.3-0010.png"),
            include_bytes!("../../../art/FacePalms 2D Explosions and Effects/EXPLOSIONS_3/EXPLOSION_3.3/EXPLOSION_3.3-0011.png"),
            include_bytes!("../../../art/FacePalms 2D Explosions and Effects/EXPLOSIONS_3/EXPLOSION_3.3/EXPLOSION_3.3-0012.png"),
            include_bytes!("../../../art/FacePalms 2D Explosions and Effects/EXPLOSIONS_3/EXPLOSION_3.3/EXPLOSION_3.3-0013.png"),
            include_bytes!("../../../art/FacePalms 2D Explosions and Effects/EXPLOSIONS_3/EXPLOSION_3.3/EXPLOSION_3.3-0014.png"),
            include_bytes!("../../../art/FacePalms 2D Explosions and Effects/EXPLOSIONS_3/EXPLOSION_3.3/EXPLOSION_3.3-0015.png"),
            include_bytes!("../../../art/FacePalms 2D Explosions and Effects/EXPLOSIONS_3/EXPLOSION_3.3/EXPLOSION_3.3-0016.png"),
            include_bytes!("../../../art/FacePalms 2D Explosions and Effects/EXPLOSIONS_3/EXPLOSION_3.3/EXPLOSION_3.3-0017.png"),
            include_bytes!("../../../art/FacePalms 2D Explosions and Effects/EXPLOSIONS_3/EXPLOSION_3.3/EXPLOSION_3.3-0018.png"),
            include_bytes!("../../../art/FacePalms 2D Explosions and Effects/EXPLOSIONS_3/EXPLOSION_3.3/EXPLOSION_3.3-0019.png"),
            include_bytes!("../../../art/FacePalms 2D Explosions and Effects/EXPLOSIONS_3/EXPLOSION_3.3/EXPLOSION_3.3-0020.png"),
            include_bytes!("../../../art/FacePalms 2D Explosions and Effects/EXPLOSIONS_3/EXPLOSION_3.3/EXPLOSION_3.3-0021.png"),
            include_bytes!("../../../art/FacePalms 2D Explosions and Effects/EXPLOSIONS_3/EXPLOSION_3.3/EXPLOSION_3.3-0022.png"),
            include_bytes!("../../../art/FacePalms 2D Explosions and Effects/EXPLOSIONS_3/EXPLOSION_3.3/EXPLOSION_3.3-0023.png"),
        ],
        on_end: Clamp,
    },
});

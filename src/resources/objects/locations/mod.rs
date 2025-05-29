pub mod location001;
pub mod location002;
pub mod location003;
pub mod location003_training;
mod location003_objectives;
mod tutorial_objectives;

pub(crate) const DEV_SCALE: f32 = 1.0;

pub(crate) fn dev_scale(value: i32) -> i32 {
    (value as f32 * DEV_SCALE) as i32
}

pub(crate) fn dev_scale_usize(value: u32) -> u32 {
    (value as f32 * DEV_SCALE) as u32
}

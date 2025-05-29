use macroquad::prelude::get_frame_time;

pub struct FpsCounter {
    buffer: [f32; 64],
    buffer_index: usize,
    pub smooth: f32,
}

impl FpsCounter {
    pub fn new() -> FpsCounter {
        FpsCounter {
            buffer_index: 0,
            buffer: [0.0; 64],
            smooth: 0.0,
        }
    }
}

impl FpsCounter {
    pub fn update_fps(&mut self) {
        self.buffer[self.buffer_index] = 1.0 / get_frame_time();
        self.buffer_index += 1;
        self.buffer_index %= self.buffer.len();

        let sum: f32 = self.buffer.iter().sum();
        self.smooth = sum / self.buffer.len() as f32;
    }
}
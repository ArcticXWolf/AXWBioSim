use rand::Rng;

use crate::world::WorldSettings;

#[derive(Debug, Default, Clone)]
pub struct Agent {
    pub hash: u32,
    pub x: usize,
    pub y: usize,
}

impl Agent {
    pub fn new(hash: u32, x: usize, y: usize) -> Self {
        Self { hash, x, y }
    }

    pub fn spawn(world_settings: &WorldSettings) -> Self {
        let mut thread_rng = rand::thread_rng();
        Self {
            hash: thread_rng.gen(),
            x: thread_rng.gen_range(0..world_settings.width),
            y: thread_rng.gen_range(0..world_settings.height),
        }
    }

    pub fn step(&self) -> (usize, usize) {
        // Just random movement for now
        let mut thread_rng = rand::thread_rng();
        match thread_rng.gen_range(0..4) {
            0 => (self.x.saturating_sub(1), self.y),
            1 => (self.x + 1, self.y),
            2 => (self.x, self.y.saturating_sub(1)),
            3 => (self.x, self.y + 1),
            _ => unimplemented!(),
        }
    }

    pub fn set_pos(&mut self, x: usize, y: usize) {
        self.x = x;
        self.y = y;
    }
}

use rand::{seq::IteratorRandom, Rng};

use crate::world::WorldSettings;

#[derive(Debug, Default, Clone)]
pub struct Agent {
    pub direction: u8,
    pub x: usize,
    pub y: usize,
}

impl Agent {
    pub fn new(direction: u8, x: usize, y: usize) -> Self {
        Self { direction, x, y }
    }

    pub fn spawn(world_settings: &WorldSettings) -> Self {
        let mut thread_rng = rand::thread_rng();
        Self {
            direction: thread_rng.gen_range(0..4),
            x: thread_rng.gen_range(0..world_settings.width),
            y: thread_rng.gen_range(0..world_settings.height),
        }
    }

    pub fn spawn_from_parents(world_settings: &WorldSettings, parents: &[Self]) -> Self {
        let mut thread_rng = rand::thread_rng();
        let direction = parents
            .iter()
            .map(|a| a.direction)
            .choose(&mut thread_rng)
            .unwrap();
        Self {
            direction,
            x: thread_rng.gen_range(0..world_settings.width),
            y: thread_rng.gen_range(0..world_settings.height),
        }
    }

    pub fn step(&self) -> (usize, usize) {
        match self.direction {
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

    pub fn color(&self) -> [u8; 3] {
        match self.direction {
            0 => [0x0, 0x0, 0xFF],
            1 => [0x0, 0xFF, 0x0],
            2 => [0xFF, 0x0, 0x0],
            3 => [0xFF, 0xFF, 0xFF],
            _ => unimplemented!(),
        }
    }
}

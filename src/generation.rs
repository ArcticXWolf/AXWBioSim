use crate::{
    agent::Agent,
    world::{World, WorldSettings},
};
use image::{ImageBuffer, Rgb};
use imageproc::drawing::draw_filled_circle_mut;
const TILE_SIZE: usize = 6;

pub struct Generation {
    pub index: usize,
    pub current_age: usize,
    pub max_age: usize,
    pub world: World,
    pub agents: Vec<Agent>,
}

impl Generation {
    pub fn new(
        index: usize,
        max_age: usize,
        agents: Vec<Agent>,
        world_settings: WorldSettings,
    ) -> Self {
        let mut world = World::new(world_settings);
        for agent in &agents {
            world.spawn_agent(agent);
        }

        Self {
            index,
            current_age: 0,
            max_age,
            world,
            agents,
        }
    }

    pub fn step(&mut self) {
        for agent in self.agents.iter_mut() {
            let agent_intention = agent.process_neural_net();

            if let Some(move_intention) = agent_intention.move_intention {
                let (new_x, new_y) =
                    self.world
                        .validate_move_intention(agent.x, agent.y, move_intention);
                agent.set_pos(new_x, new_y);
            }
        }
        self.current_age += 1;
    }

    pub fn run(&mut self) {
        while self.current_age < self.max_age {
            self.step();
        }
    }

    pub fn run_and_save_gif(&mut self, save_path: &str) {
        self.save_current_state(save_path);

        while self.current_age < self.max_age {
            self.step();
            self.save_current_state(save_path);
        }
    }

    pub fn save_current_state(&self, save_path: &str) {
        let _ = std::fs::create_dir_all(format!("{}/{:03}/", save_path, self.index)).unwrap();
        let file_path = format!(
            "{}/{:03}/{:03}.png",
            save_path, self.index, self.current_age
        );
        let mut imgbuf: ImageBuffer<Rgb<u8>, Vec<_>> = ImageBuffer::new(
            (self.world.settings.width * TILE_SIZE) as u32,
            (self.world.settings.height * TILE_SIZE) as u32,
        );
        for agent in &self.agents {
            let color = Rgb::from(agent.color());
            draw_filled_circle_mut(
                &mut imgbuf,
                (
                    (agent.x * TILE_SIZE + TILE_SIZE / 2) as i32,
                    (agent.y * TILE_SIZE + TILE_SIZE / 2) as i32,
                ),
                (TILE_SIZE / 2) as i32,
                color,
            );
        }
        imgbuf.save(file_path).unwrap();
    }
}

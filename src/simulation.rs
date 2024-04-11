use crate::{
    agent::Agent, death_criterium::DeathCriterium, generation::Generation, world::WorldSettings,
};

pub struct SimulationSettings {
    pub num_generations: usize,
    pub generations_to_save: Vec<usize>,
    pub save_path: String,
    pub generation_age: usize,
    pub agents_per_generation: usize,
    pub death_criterium: Box<dyn DeathCriterium>,
    pub world_settings: WorldSettings,
}

pub struct Simulation {
    pub settings: SimulationSettings,
    pub generations: Vec<Generation>,
}

impl Simulation {
    pub fn new(settings: SimulationSettings) -> Self {
        let generations = vec![];
        Self {
            settings,
            generations,
        }
    }

    pub fn current_generation(&self) -> &Generation {
        self.generations.last().unwrap()
    }

    pub fn current_generation_mut(&mut self) -> &mut Generation {
        self.generations.last_mut().unwrap()
    }

    fn seed_new_generation(&mut self) {
        if self.generations.is_empty() {
            let mut agents = Vec::with_capacity(self.settings.agents_per_generation);
            for _ in 0..self.settings.agents_per_generation {
                agents.push(Agent::spawn(&self.settings.world_settings));
            }

            self.generations.push(Generation::new(
                0,
                self.settings.generation_age,
                agents,
                self.settings.world_settings.clone(),
            ));
        } else {
            let survivors = self
                .settings
                .death_criterium
                .survivors(self.current_generation().agents.clone());

            let mut agents = Vec::with_capacity(self.settings.agents_per_generation);
            for i in 0..self.settings.agents_per_generation {
                let parents_id = i * 2 % survivors.len();
                let parents = &survivors[parents_id..parents_id + 1];
                agents.push(Agent::spawn_from_parents(
                    &self.settings.world_settings,
                    parents,
                ));
            }

            self.generations.push(Generation::new(
                self.generations.len(),
                self.settings.generation_age,
                agents,
                self.settings.world_settings.clone(),
            ));
        }
    }

    pub fn run_generation(&mut self) {
        if self
            .settings
            .generations_to_save
            .contains(&self.current_generation().index)
        {
            println!(
                "Running Generation [{}/{}] and saving...",
                self.generations.len(),
                self.settings.num_generations
            );
            let save_path = self.settings.save_path.clone();
            self.current_generation_mut()
                .run_and_save_gif(save_path.as_str());
        } else {
            println!(
                "Running Generation [{}/{}]...",
                self.generations.len(),
                self.settings.num_generations
            );
            self.current_generation_mut().run();
        }
    }

    pub fn run(&mut self) {
        while self.generations.len() < self.settings.num_generations {
            self.seed_new_generation();
            self.run_generation();
        }
    }
}

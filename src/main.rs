pub mod agent;
pub mod death_criterium;
pub mod generation;
pub mod simulation;
pub mod world;

fn main() {
    let world_settings = world::WorldSettings {
        width: 100,
        height: 100,
    };
    let death_crit = death_criterium::StaticAreaDeathCriterium {
        min_x: 0,
        max_x: world_settings.width / 2,
        min_y: 0,
        max_y: world_settings.height,
    };
    let simulation_settings = simulation::SimulationSettings {
        generation_age: 100,
        agents_per_generation: 1000,
        world_settings,
        num_generations: 100,
        generations_to_save: vec![0, 99],
        save_path: String::from("target/images"),
        death_criterium: Box::new(death_crit),
    };
    let mut sim = simulation::Simulation::new(simulation_settings);

    sim.run();
}

use crate::agent::Agent;

pub trait DeathCriterium {
    fn survivors(&self, agents: Vec<Agent>) -> Vec<Agent>;
}

pub struct StaticAreaDeathCriterium {
    pub min_x: usize,
    pub max_x: usize,
    pub min_y: usize,
    pub max_y: usize,
}

impl DeathCriterium for StaticAreaDeathCriterium {
    fn survivors(&self, agents: Vec<Agent>) -> Vec<Agent> {
        agents
            .into_iter()
            .filter(|a| {
                a.x >= self.min_x && a.x < self.max_x && a.y >= self.min_y && a.y < self.max_y
            })
            .collect()
    }
}

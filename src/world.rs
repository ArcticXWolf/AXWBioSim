use crate::agent::{Agent, MoveIntention};

#[derive(Debug, Clone)]
pub struct WorldSettings {
    pub width: usize,
    pub height: usize,
}

#[derive(Debug)]
pub struct World {
    pub settings: WorldSettings,
    board: Vec<bool>,
}

impl World {
    pub fn new(settings: WorldSettings) -> Self {
        let board = vec![false; settings.width * settings.height];
        Self { settings, board }
    }

    pub fn spawn_agent(&mut self, agent: &Agent) {
        self.board[agent.y * self.settings.width + agent.x] = true;
    }

    pub fn validate_move_intention(
        &mut self,
        old_x: usize,
        old_y: usize,
        move_intention: MoveIntention,
    ) -> (usize, usize) {
        let (new_x, new_y) = match move_intention {
            MoveIntention::North => (old_x, old_y.saturating_sub(1)),
            MoveIntention::East => (old_x + 1, old_y),
            MoveIntention::South => (old_x, old_y + 1),
            MoveIntention::West => (old_x.saturating_sub(1), old_y),
        };

        if new_x >= self.settings.width || new_y >= self.settings.height {
            return (old_x, old_y);
        }

        if *self.board.get(new_y * self.settings.width + new_x).unwrap() {
            return (old_x, old_y);
        }

        self.board[old_y * self.settings.width + old_x] = false;
        self.board[new_y * self.settings.width + new_x] = true;
        return (new_x, new_y);
    }
}

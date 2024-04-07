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

    pub fn make_move(&mut self, old_x: usize, old_y: usize, new_x: usize, new_y: usize) -> bool {
        if new_x >= self.settings.width || new_y >= self.settings.height {
            return false;
        }

        if let Some(true) = self.board.get(new_y * self.settings.width + new_x) {
            return false;
        }

        self.board
            .insert(old_y * self.settings.width + old_x, false);
        self.board.insert(new_y * self.settings.width + new_x, true);
        return true;
    }
}

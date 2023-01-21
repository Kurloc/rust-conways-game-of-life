pub struct Tile {
    pub alive: bool,
}

impl Tile {
    pub fn default() -> Tile {
        return Tile {
            alive: false,
        };
    }

    pub fn set_alive(&mut self, value: bool) {
        self.alive = value;
    }
}

use std::collections::HashMap;
use crate::tile::Tile;
use crate::world_display::MoveDirections;
use std::collections::hash_map::Iter;

#[allow(dead_code)]
pub enum OscillatorOrientation {
    Vertical,
    Horizontal,
}

pub struct World {
    pub tiles: HashMap<(usize, usize), Tile>,
    pub alive_tile_keys: HashMap<(usize, usize), bool>,
    pub x_size: usize,
    pub y_size: usize,
    pub chunk_x_size: usize,
    pub chunk_y_size: usize,
    pub current_chunk_address: (i32, i32),
    pub allotted_read_input_time: usize,
    pub frame_interval_ms: usize,
}

impl World {
    pub fn new(x_size: usize,
               y_size: usize,
               chunk_x_size: usize,
               chunk_y_size: usize,
               frame_interval_ms: usize,
               allotted_read_input_time: usize
    ) -> World {
        let mut tiles = HashMap::new();
        let alive_tile_keys = HashMap::new();
        for y in 0..y_size {
            for x in 0..x_size {
                let tile = Tile::default();
                tiles.insert((x, y), tile);
            }
        }

        let is_chunk_1_by_1 = x_size / chunk_x_size == 1 && y_size / chunk_y_size == 1;

        return World {
            x_size,
            y_size,
            tiles,
            chunk_x_size,
            chunk_y_size,
            alive_tile_keys,
            current_chunk_address: (0, 0),
            allotted_read_input_time: (
                if is_chunk_1_by_1 {
                    if allotted_read_input_time == 0 {
                        0
                    } else {
                        10
                    }
                } else {
                    if allotted_read_input_time == 0 {
                        0
                    } else {
                        frame_interval_ms / 5
                    }
                }
            ),
            frame_interval_ms,
        };
    }

    pub fn reset_world(&mut self) {
        self.tiles.clear();
        self.alive_tile_keys.clear();
        for y in 0..self.y_size {
            for x in 0..self.x_size {
                self.tiles.insert((x, y), Tile::default());
            }
        }
    }

    pub fn insert_position(
        tiles: &mut HashMap<(usize, usize), Tile>,
        alive_tile_keys: &mut HashMap<(usize, usize), bool>,
        key: (usize, usize),
    ) {
        tiles.get_mut(&key).unwrap().set_alive(true);
        alive_tile_keys.insert(key, true);
    }

    pub fn move_chunk(&mut self, move_direction: MoveDirections) {
        let mut adjustment_amount: (i32, i32) = (0, 0);
        match move_direction {
            MoveDirections::Left => {
                adjustment_amount = (adjustment_amount.0 - 1, adjustment_amount.1)
            }
            MoveDirections::Right => {
                adjustment_amount = (adjustment_amount.0 + 1, adjustment_amount.1)
            }
            MoveDirections::Up => {
                adjustment_amount = (adjustment_amount.0, adjustment_amount.1 + 1)
            }
            MoveDirections::Down => {
                adjustment_amount = (adjustment_amount.0, adjustment_amount.1 - 1)
            }
        }

        let original_chunk_x = self.current_chunk_address.0;
        let original_chunk_y = self.current_chunk_address.1;
        let mut new_x = original_chunk_x + adjustment_amount.0;
        let mut new_y = original_chunk_y + adjustment_amount.1;
        if new_x < 0 { new_x = 0; }
        if new_x > ((self.x_size / self.chunk_x_size) as i32) - 1 {
            new_x = ((self.x_size / self.chunk_x_size) as i32) - 1;
        }
        if new_y < 0 { new_y = 0; }
        if new_y > ((self.y_size / self.chunk_y_size) as i32) - 1 {
            new_y = ((self.y_size / self.chunk_y_size) as i32) - 1;
        }
        self.current_chunk_address = (new_x, new_y);
    }

    pub fn reset_game_world(
        &mut self,
        tiles_to_set_as_alive: Iter<(u16, u16), bool>,
    ) {
        self.reset_world();
        for key in tiles_to_set_as_alive {
            if *key.1 {
                let key_post = (key.0.0 as usize, key.0.1 as usize);
                World::insert_position(&mut self.tiles, &mut self.alive_tile_keys, key_post);
            }
        }
    }

    pub fn handle_generation(
        &mut self,
        mut keys_to_remove: &mut Vec<(i128, i128)>,
        mut keys_to_add: &mut Vec<(i128, i128)>,
    ) {
        let mut keys_to_search = Vec::new();
        for i in self.alive_tile_keys.keys().into_iter() {
            keys_to_search.push((i.0, i.1));
        }
        self.handle_top_generation(
            keys_to_search,
            &mut keys_to_remove,
            &mut keys_to_add,
            0,
        );
        keys_to_remove.clear();
    }

    #[allow(dead_code)]
    pub fn insert_blinker_box(mut tiles: &mut HashMap<(usize, usize), Tile>, mut alive_tile_keys: &mut HashMap<(usize, usize), bool>) {
        Self::insert_position(&mut tiles, &mut alive_tile_keys, (5, 5));
        Self::insert_position(&mut tiles, &mut alive_tile_keys, (5, 6));
        Self::insert_position(&mut tiles, &mut alive_tile_keys, (6, 7));
        Self::insert_position(&mut tiles, &mut alive_tile_keys, (7, 7));
        Self::insert_position(&mut tiles, &mut alive_tile_keys, (8, 7));
        Self::insert_position(&mut tiles, &mut alive_tile_keys, (6, 5));
        Self::insert_position(&mut tiles, &mut alive_tile_keys, (5, 5));
        Self::insert_position(&mut tiles, &mut alive_tile_keys, (7, 5));
        Self::insert_position(&mut tiles, &mut alive_tile_keys, (8, 5));
        Self::insert_position(&mut tiles, &mut alive_tile_keys, (9, 5));
        Self::insert_position(&mut tiles, &mut alive_tile_keys, (9, 6));
    }

    #[allow(dead_code)]
    pub fn insert_oscillator(
        mut tiles: &mut HashMap<(usize, usize), Tile>,
        mut alive_tile_keys: &mut HashMap<(usize, usize), bool>,
        position: (usize, usize),
        orientation: OscillatorOrientation,
    ) {
        match orientation {
            OscillatorOrientation::Vertical => {
                Self::insert_position(&mut tiles, &mut alive_tile_keys, (position.0 + 1, position.1));
                Self::insert_position(&mut tiles, &mut alive_tile_keys, (position.0 + 1, position.1 - 1));
                Self::insert_position(&mut tiles, &mut alive_tile_keys, (position.0 + 1, position.1 - 2));
            }
            OscillatorOrientation::Horizontal => {
                Self::insert_position(&mut tiles, &mut alive_tile_keys, (position.0, position.1 - 1));
                Self::insert_position(&mut tiles, &mut alive_tile_keys, (position.0 + 1, position.1 - 1));
                Self::insert_position(&mut tiles, &mut alive_tile_keys, (position.0 + 2, position.1 - 1));
            }
        }
    }

    #[allow(dead_code)]
    pub fn self_insert_blinker_box(
        &mut self,
        position: (usize, usize),
    ) -> &mut World {
        Self::insert_position(&mut self.tiles, &mut self.alive_tile_keys, (position.0 + 5, position.1 + 5));
        Self::insert_position(&mut self.tiles, &mut self.alive_tile_keys, (position.0 + 5, position.1 + 6));
        Self::insert_position(&mut self.tiles, &mut self.alive_tile_keys, (position.0 + 6, position.1 + 7));
        Self::insert_position(&mut self.tiles, &mut self.alive_tile_keys, (position.0 + 7, position.1 + 7));
        Self::insert_position(&mut self.tiles, &mut self.alive_tile_keys, (position.0 + 8, position.1 + 7));
        Self::insert_position(&mut self.tiles, &mut self.alive_tile_keys, (position.0 + 6, position.1 + 5));
        Self::insert_position(&mut self.tiles, &mut self.alive_tile_keys, (position.0 + 5, position.1 + 5));
        Self::insert_position(&mut self.tiles, &mut self.alive_tile_keys, (position.0 + 7, position.1 + 5));
        Self::insert_position(&mut self.tiles, &mut self.alive_tile_keys, (position.0 + 8, position.1 + 5));
        Self::insert_position(&mut self.tiles, &mut self.alive_tile_keys, (position.0 + 9, position.1 + 5));
        Self::insert_position(&mut self.tiles, &mut self.alive_tile_keys, (position.0 + 9, position.1 + 6));
        return self;
    }

    #[allow(dead_code)]
    pub fn self_insert_oscillator(
        &mut self,
        position: (usize, usize),
        orientation: OscillatorOrientation,
    ) -> &mut World {
        match orientation {
            OscillatorOrientation::Vertical => {
                Self::insert_position(&mut self.tiles, &mut self.alive_tile_keys, (position.0 + 1, position.1));
                Self::insert_position(&mut self.tiles, &mut self.alive_tile_keys, (position.0 + 1, position.1 - 1));
                Self::insert_position(&mut self.tiles, &mut self.alive_tile_keys, (position.0 + 1, position.1 - 2));
            }
            OscillatorOrientation::Horizontal => {
                Self::insert_position(&mut self.tiles, &mut self.alive_tile_keys, (position.0, position.1 - 1));
                Self::insert_position(&mut self.tiles, &mut self.alive_tile_keys, (position.0 + 1, position.1 - 1));
                Self::insert_position(&mut self.tiles, &mut self.alive_tile_keys, (position.0 + 2, position.1 - 1));
            }
        }
        return self;
    }

    #[allow(dead_code)]
    pub fn self_insert_chaos_cloverleaf(
        &mut self,
        position: (usize, usize),
    ) -> &mut World {
        Self::insert_position(&mut self.tiles, &mut self.alive_tile_keys, (position.0 + 10, position.1 - 4));
        Self::insert_position(&mut self.tiles, &mut self.alive_tile_keys, (position.0 + 10, position.1 - 3));
        Self::insert_position(&mut self.tiles, &mut self.alive_tile_keys, (position.0 + 9, position.1 - 2));
        Self::insert_position(&mut self.tiles, &mut self.alive_tile_keys, (position.0 + 8, position.1 - 2));
        Self::insert_position(&mut self.tiles, &mut self.alive_tile_keys, (position.0 + 7, position.1 - 2));
        Self::insert_position(&mut self.tiles, &mut self.alive_tile_keys, (position.0 + 7, position.1 - 1));
        Self::insert_position(&mut self.tiles, &mut self.alive_tile_keys, (position.0 + 6, position.1 - 3));
        Self::insert_position(&mut self.tiles, &mut self.alive_tile_keys, (position.0 + 6, position.1 - 5));
        Self::insert_position(&mut self.tiles, &mut self.alive_tile_keys, (position.0 + 6, position.1 - 7));
        Self::insert_position(&mut self.tiles, &mut self.alive_tile_keys, (position.0 + 6, position.1 - 9));
        Self::insert_position(&mut self.tiles, &mut self.alive_tile_keys, (position.0 + 8, position.1 - 8));
        Self::insert_position(&mut self.tiles, &mut self.alive_tile_keys, (position.0 + 8, position.1 - 7));
        Self::insert_position(&mut self.tiles, &mut self.alive_tile_keys, (position.0 + 9, position.1 - 7));
        Self::insert_position(&mut self.tiles, &mut self.alive_tile_keys, (position.0 + 5, position.1 - 1));
        Self::insert_position(&mut self.tiles, &mut self.alive_tile_keys, (position.0 + 5, position.1 - 2));
        Self::insert_position(&mut self.tiles, &mut self.alive_tile_keys, (position.0 + 4, position.1 - 4));
        Self::insert_position(&mut self.tiles, &mut self.alive_tile_keys, (position.0 + 4, position.1 - 5));
        Self::insert_position(&mut self.tiles, &mut self.alive_tile_keys, (position.0 + 4, position.1 - 2));
        Self::insert_position(&mut self.tiles, &mut self.alive_tile_keys, (position.0 + 3, position.1 - 5));
        Self::insert_position(&mut self.tiles, &mut self.alive_tile_keys, (position.0 + 3, position.1 - 2));
        Self::insert_position(&mut self.tiles, &mut self.alive_tile_keys, (position.0 + 2, position.1 - 3));
        Self::insert_position(&mut self.tiles, &mut self.alive_tile_keys, (position.0 + 2, position.1 - 4));
        Self::insert_position(&mut self.tiles, &mut self.alive_tile_keys, (position.0 + 2, position.1 - 8));
        Self::insert_position(&mut self.tiles, &mut self.alive_tile_keys, (position.0 + 2, position.1 - 9));
        Self::insert_position(&mut self.tiles, &mut self.alive_tile_keys, (position.0 + 3, position.1 - 10));
        Self::insert_position(&mut self.tiles, &mut self.alive_tile_keys, (position.0 + 11, position.1 - 11));
        Self::insert_position(&mut self.tiles, &mut self.alive_tile_keys, (position.0 + 3, position.1 - 11));
        Self::insert_position(&mut self.tiles, &mut self.alive_tile_keys, (position.0 + 3, position.1 - 12));
        Self::insert_position(&mut self.tiles, &mut self.alive_tile_keys, (position.0 + 4, position.1 - 12));
        Self::insert_position(&mut self.tiles, &mut self.alive_tile_keys, (position.0 + 4, position.1 - 13));
        Self::insert_position(&mut self.tiles, &mut self.alive_tile_keys, (position.0 + 4, position.1 - 14));
        Self::insert_position(&mut self.tiles, &mut self.alive_tile_keys, (position.0 + 5, position.1 - 15));
        Self::insert_position(&mut self.tiles, &mut self.alive_tile_keys, (position.0 + 7, position.1 - 14));
        Self::insert_position(&mut self.tiles, &mut self.alive_tile_keys, (position.0 + 7, position.1 - 15));
        Self::insert_position(&mut self.tiles, &mut self.alive_tile_keys, (position.0 + 8, position.1 - 14));
        Self::insert_position(&mut self.tiles, &mut self.alive_tile_keys, (position.0 + 9, position.1 - 14));
        Self::insert_position(&mut self.tiles, &mut self.alive_tile_keys, (position.0 + 10, position.1 - 13));
        Self::insert_position(&mut self.tiles, &mut self.alive_tile_keys, (position.0 + 10, position.1 - 12));
        Self::insert_position(&mut self.tiles, &mut self.alive_tile_keys, (position.0 + 9, position.1 - 11));
        Self::insert_position(&mut self.tiles, &mut self.alive_tile_keys, (position.0 + 8, position.1 - 11));
        Self::insert_position(&mut self.tiles, &mut self.alive_tile_keys, (position.0 + 8, position.1 - 12));
        return self;
    }

    #[allow(dead_code)]
    pub fn self_insert_cloverleaf(
        &mut self,
        position: (usize, usize),
    ) -> &mut World {
        Self::insert_position(&mut self.tiles, &mut self.alive_tile_keys, (position.0 + 10, position.1 - 4));
        Self::insert_position(&mut self.tiles, &mut self.alive_tile_keys, (position.0 + 10, position.1 - 3));
        Self::insert_position(&mut self.tiles, &mut self.alive_tile_keys, (position.0 + 9, position.1 - 2));
        Self::insert_position(&mut self.tiles, &mut self.alive_tile_keys, (position.0 + 8, position.1 - 2));
        Self::insert_position(&mut self.tiles, &mut self.alive_tile_keys, (position.0 + 7, position.1 - 2));
        Self::insert_position(&mut self.tiles, &mut self.alive_tile_keys, (position.0 + 7, position.1 - 1));
        Self::insert_position(&mut self.tiles, &mut self.alive_tile_keys, (position.0 + 6, position.1 - 3));
        Self::insert_position(&mut self.tiles, &mut self.alive_tile_keys, (position.0 + 6, position.1 - 5));
        Self::insert_position(&mut self.tiles, &mut self.alive_tile_keys, (position.0 + 6, position.1 - 7));
        Self::insert_position(&mut self.tiles, &mut self.alive_tile_keys, (position.0 + 6, position.1 - 9));
        Self::insert_position(&mut self.tiles, &mut self.alive_tile_keys, (position.0 + 5, position.1 - 1));
        Self::insert_position(&mut self.tiles, &mut self.alive_tile_keys, (position.0 + 5, position.1 - 2));
        Self::insert_position(&mut self.tiles, &mut self.alive_tile_keys, (position.0 + 4, position.1 - 4));
        Self::insert_position(&mut self.tiles, &mut self.alive_tile_keys, (position.0 + 4, position.1 - 5));
        Self::insert_position(&mut self.tiles, &mut self.alive_tile_keys, (position.0 + 4, position.1 - 2));
        Self::insert_position(&mut self.tiles, &mut self.alive_tile_keys, (position.0 + 3, position.1 - 5));
        Self::insert_position(&mut self.tiles, &mut self.alive_tile_keys, (position.0 + 3, position.1 - 2));
        Self::insert_position(&mut self.tiles, &mut self.alive_tile_keys, (position.0 + 2, position.1 - 3));
        Self::insert_position(&mut self.tiles, &mut self.alive_tile_keys, (position.0 + 2, position.1 - 4));
        Self::insert_position(&mut self.tiles, &mut self.alive_tile_keys, (position.0 + 2, position.1 - 8));
        Self::insert_position(&mut self.tiles, &mut self.alive_tile_keys, (position.0 + 2, position.1 - 9));
        Self::insert_position(&mut self.tiles, &mut self.alive_tile_keys, (position.0 + 3, position.1 - 10));
        Self::insert_position(&mut self.tiles, &mut self.alive_tile_keys, (position.0 + 3, position.1 - 11));
        Self::insert_position(&mut self.tiles, &mut self.alive_tile_keys, (position.0 + 3, position.1 - 12));
        Self::insert_position(&mut self.tiles, &mut self.alive_tile_keys, (position.0 + 4, position.1 - 12));
        Self::insert_position(&mut self.tiles, &mut self.alive_tile_keys, (position.0 + 4, position.1 - 13));
        Self::insert_position(&mut self.tiles, &mut self.alive_tile_keys, (position.0 + 4, position.1 - 14));
        Self::insert_position(&mut self.tiles, &mut self.alive_tile_keys, (position.0 + 5, position.1 - 15));
        Self::insert_position(&mut self.tiles, &mut self.alive_tile_keys, (position.0 + 7, position.1 - 14));
        Self::insert_position(&mut self.tiles, &mut self.alive_tile_keys, (position.0 + 7, position.1 - 15));
        Self::insert_position(&mut self.tiles, &mut self.alive_tile_keys, (position.0 + 8, position.1 - 14));
        Self::insert_position(&mut self.tiles, &mut self.alive_tile_keys, (position.0 + 9, position.1 - 14));
        Self::insert_position(&mut self.tiles, &mut self.alive_tile_keys, (position.0 + 10, position.1 - 13));
        Self::insert_position(&mut self.tiles, &mut self.alive_tile_keys, (position.0 + 10, position.1 - 12));
        Self::insert_position(&mut self.tiles, &mut self.alive_tile_keys, (position.0 + 9, position.1 - 11));
        Self::insert_position(&mut self.tiles, &mut self.alive_tile_keys, (position.0 + 8, position.1 - 11));
        Self::insert_position(&mut self.tiles, &mut self.alive_tile_keys, (position.0 + 8, position.1 - 12));
        return self;
    }

    fn handle_top_generation(
        &mut self,
        keys_to_search: Vec<(usize, usize)>,
        keys_to_remove: &mut Vec<(i128, i128)>,
        keys_to_add: &mut Vec<(i128, i128)>,
        depth: usize,
    ) {
        let mut dead_keys_to_seen: HashMap<(i128, i128), usize> = HashMap::new();
        let mut live_neighbors: usize = 0;

        for i in keys_to_search {
            let x = i.0;
            let y = i.1;

            live_neighbors = self.calculate_northern_neighbors(
                &mut dead_keys_to_seen,
                live_neighbors,
                x,
                y,
            );
            live_neighbors = self.calculate_western_neighbors(
                &mut dead_keys_to_seen,
                live_neighbors,
                x,
                y,
            );
            live_neighbors = self.calculate_eastern_neighbors(
                &mut dead_keys_to_seen,
                live_neighbors,
                x,
                y,
            );
            live_neighbors = self.calculate_southern_neighbors(
                &mut dead_keys_to_seen,
                live_neighbors,
                x,
                y,
            );

            let tile = self.tiles.get_mut(&i);
            if !tile.is_none() {
                if tile.unwrap().alive {
                    if live_neighbors < 2 {
                        keys_to_remove.push((i.0 as i128, i.1 as i128));
                    }
                    if live_neighbors == 2 || live_neighbors == 3 {
                        keys_to_add.push((i.0 as i128, i.1 as i128));
                    }
                    if live_neighbors > 3 {
                        keys_to_remove.push((i.0 as i128, i.1 as i128));
                    }
                } else {
                    if live_neighbors == 3 {
                        keys_to_add.push((i.0 as i128, i.1 as i128));
                    }
                }
            }
            live_neighbors = 0;
        }
        for i in keys_to_add.iter() {
            let t = self.tiles.get_mut(&(i.0 as usize, i.1 as usize)).unwrap();
            t.set_alive(true);
            self.alive_tile_keys.insert((i.0 as usize, i.1 as usize), true);
        }

        let mut keys_to_search = Vec::new();
        for (key, _) in dead_keys_to_seen.into_iter() {
            keys_to_search.push((key.0 as usize, key.1 as usize));
        }
        if keys_to_search.len() > 0 && depth == 0 {
            self.handle_top_generation(
                keys_to_search,
                keys_to_remove,
                keys_to_add,
                1,
            )
        }

        for i in keys_to_remove.iter() {
            let t = self.tiles.get_mut(&(i.0 as usize, i.1 as usize)).unwrap();
            t.set_alive(false);
            self.alive_tile_keys.remove(&(i.0 as usize, i.1 as usize));
        }
        keys_to_add.clear();
    }

    fn calculate_southern_neighbors(
        &mut self,
        mut dead_keys_to_seen: &mut HashMap<(i128, i128), usize>,
        live_neighbors: usize,
        x: usize,
        y: usize,
    ) -> usize {
        let mut live_neighbors = live_neighbors;
        if y > 0 {
            let s_key = (x, y - 1);
            let s_neighbor = self.tiles.get_mut(&s_key);
            if !s_neighbor.is_none() {
                if s_neighbor.unwrap().alive {
                    live_neighbors += 1;
                }
                World::handle_neighbor_key_seen(&mut dead_keys_to_seen, s_key);
            }
            if x > 0 {
                let sw_key = (x - 1, y - 1);
                let sw_neighbor = self.tiles.get_mut(&sw_key);
                if !sw_neighbor.is_none() {
                    if sw_neighbor.unwrap().alive {
                        live_neighbors += 1;
                    }
                    World::handle_neighbor_key_seen(&mut dead_keys_to_seen, sw_key);
                }
            }
            if x < self.x_size {
                let se_key = (x + 1, y - 1);
                let se_neighbor = self.tiles.get_mut(&se_key);
                if !se_neighbor.is_none() {
                    if se_neighbor.unwrap().alive {
                        live_neighbors += 1;
                    }
                    World::handle_neighbor_key_seen(&mut dead_keys_to_seen, se_key);
                }
            }
        }
        return live_neighbors;
    }

    fn calculate_eastern_neighbors(
        &mut self,
        mut dead_keys_to_seen: &mut HashMap<(i128, i128), usize>,
        live_neighbors: usize,
        x: usize,
        y: usize,
    ) -> usize {
        let mut live_neighbors = live_neighbors;
        if x < self.x_size {
            let e_key = (x + 1, y);
            let e_neighbor = self.tiles.get_mut(&e_key);
            if !e_neighbor.is_none() {
                if e_neighbor.unwrap().alive {
                    live_neighbors += 1;
                }
                World::handle_neighbor_key_seen(&mut dead_keys_to_seen, e_key);
            }
        }
        return live_neighbors;
    }

    fn calculate_western_neighbors(
        &mut self,
        mut dead_keys_to_seen: &mut HashMap<(i128, i128), usize>,
        live_neighbors: usize,
        x: usize,
        y: usize,
    ) -> usize {
        let mut live_neighbors = live_neighbors;
        if x > 0 {
            let w_key = (x - 1, y);
            let w_neighbor = self.tiles.get_mut(&w_key);
            if !w_neighbor.is_none() {
                if w_neighbor.unwrap().alive {
                    live_neighbors += 1;
                }
                World::handle_neighbor_key_seen(&mut dead_keys_to_seen, w_key);
            }
        }
        return live_neighbors;
    }

    fn calculate_northern_neighbors(
        &mut self,
        mut dead_keys_to_seen: &mut HashMap<(i128, i128), usize>,
        live_neighbors: usize,
        x: usize,
        y: usize,
    ) -> usize {
        let mut live_neighbors = live_neighbors;
        if y < self.y_size {
            let n_key = (x, y + 1);
            let n_neighbor = self.tiles.get_mut(&n_key);
            if !n_neighbor.is_none() {
                if n_neighbor.unwrap().alive {
                    live_neighbors += 1;
                }
                World::handle_neighbor_key_seen(&mut dead_keys_to_seen, n_key);
            }
            if x > 0 {
                let nw_key = (x - 1, y + 1);
                let nw_neighbor = self.tiles.get_mut(&nw_key);
                if !nw_neighbor.is_none() {
                    if nw_neighbor.unwrap().alive {
                        live_neighbors += 1;
                    }
                }
                World::handle_neighbor_key_seen(&mut dead_keys_to_seen, nw_key);
            }
            if x < self.x_size {
                let ne_key = (x + 1, y + 1);
                let ne_neighbor = self.tiles.get_mut(&ne_key);
                if !ne_neighbor.is_none() {
                    if ne_neighbor.unwrap().alive {
                        live_neighbors += 1;
                    }
                }
                World::handle_neighbor_key_seen(&mut dead_keys_to_seen, ne_key);
            }
        }
        return live_neighbors;
    }

    fn handle_neighbor_key_seen(
        dead_keys_to_seen: &mut HashMap<(i128, i128), usize>,
        x: (usize, usize),
    ) {
        let temp_key = (x.0 as i128, x.1 as i128);
        let dk = dead_keys_to_seen.get(&temp_key).unwrap_or(&(0));
        let dk_count = *dk + 1;
        dead_keys_to_seen.insert(temp_key, dk_count);
    }
}

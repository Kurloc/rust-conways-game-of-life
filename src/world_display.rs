use crossterm::style::Stylize;
use crate::tile::Tile;
use crate::world::World;

pub enum MoveDirections {
    Left,
    Right,
    Up,
    Down,
}

pub struct GameWorldDisplay { }

impl GameWorldDisplay {
    pub fn print_chunk(world: &World) {
        let chunk_address = world.current_chunk_address;
        print!(
            "Chunk: ({}, {}) of ({}, {}) :: WorldSize: {}x{} :: TilesVisible: {}\n",
            chunk_address.0,
            chunk_address.1,
            (world.x_size / world.chunk_x_size) - 1,
            (world.y_size / world.chunk_y_size) - 1,
            world.x_size,
            world.y_size,
            world.chunk_y_size * world.chunk_x_size
        );
        let iter_chunk_x = world.chunk_x_size - 1;
        for y in 0..world.chunk_y_size {
            for x in 0..world.chunk_x_size {
                let to_fetch = (
                    x + chunk_address.0 as usize * world.chunk_x_size,
                    y + chunk_address.1 as usize * world.chunk_y_size
                );

                let tile = world
                    .tiles
                    .get(&(to_fetch))
                    .unwrap_or(&Tile { alive: true });

                if tile.alive {
                    if x == iter_chunk_x {
                        print!("{}", format!("X\n").yellow());
                    } else {
                        print!("{}", format!("X").yellow());
                    }
                } else {
                    if x == iter_chunk_x {
                        print!("{}", " \n");
                    } else {
                        print!("{}", " ");
                    }
                }
            }
        }
    }
}

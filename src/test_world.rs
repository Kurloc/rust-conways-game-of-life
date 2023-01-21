#[cfg(test)]
pub mod test_board {
    use std::io::stdout;
    use test_case::test_case;
    use crate::world::{handle_generation, OscillatorOrientation, World};
    use crate::world_display::GameWorldDisplay;

    #[test]
    fn test_game_draw() {
        let mut stdout = stdout();
        let mut console_off = false;
        let mut debug = true;
        let mut running = true;
        let mut world = World::new(1000, 1000, 10, 10, 100);
        let mut keys_to_remove: Vec<(i128, i128)> = Vec::new();
        let mut keys_to_add: Vec<(i128, i128)> = Vec::new();
        let mut keys_to_search: Vec<(usize, usize)> = Vec::new();
        let mut how_long_we_slept: i128 = 0;
        let mut how_long_a_frame_took: i128 = 0;
        let mut current_generation: u128 = 0;

        World::insert_oscillator(
            &mut world.tiles,
            &mut world.alive_tile_keys,
            (3, 3),
            OscillatorOrientation::Vertical
        );
        assert_eq!(world.alive_tile_keys.len(), 3);
        GameWorldDisplay::print_chunk(&world);

        handle_generation(debug, &mut world, &mut keys_to_remove, &mut keys_to_add, &mut keys_to_search);
        current_generation = current_generation + (1 as u128);

        let mut keys_alive: Vec<(usize, usize)> = Vec::new();
        for keys in world.alive_tile_keys.keys() {
            keys_alive.push((keys.0, keys.0));
        }

        GameWorldDisplay::print_chunk(&world);
        assert_eq!(world.alive_tile_keys.len(), 3);
        println!("");

        handle_generation(debug, &mut world, &mut keys_to_remove, &mut keys_to_add, &mut keys_to_search);
        current_generation = current_generation + (1 as u128);

        GameWorldDisplay::print_chunk(&world);
        assert_eq!(world.alive_tile_keys.len(), 3);

        handle_generation(debug, &mut world, &mut keys_to_remove, &mut keys_to_add, &mut keys_to_search);
        current_generation = current_generation + (1 as u128);

        GameWorldDisplay::print_chunk(&world);
        assert_eq!(world.alive_tile_keys.len(), 3);

        handle_generation(debug, &mut world, &mut keys_to_remove, &mut keys_to_add, &mut keys_to_search);
        current_generation = current_generation + (1 as u128);

        GameWorldDisplay::print_chunk(&world);
        assert_eq!(world.alive_tile_keys.len(), 3);

        handle_generation(debug, &mut world, &mut keys_to_remove, &mut keys_to_add, &mut keys_to_search);
        current_generation = current_generation + (1 as u128);

        GameWorldDisplay::print_chunk(&world);
        assert_eq!(world.alive_tile_keys.len(), 3);

        handle_generation(debug, &mut world, &mut keys_to_remove, &mut keys_to_add, &mut keys_to_search);
        current_generation = current_generation + (1 as u128);

        GameWorldDisplay::print_chunk(&world);
        assert_eq!(world.alive_tile_keys.len(), 3);

        handle_generation(debug, &mut world, &mut keys_to_remove, &mut keys_to_add, &mut keys_to_search);
        current_generation = current_generation + (1 as u128);

        GameWorldDisplay::print_chunk(&world);
        assert_eq!(world.alive_tile_keys.len(), 3);

        handle_generation(debug, &mut world, &mut keys_to_remove, &mut keys_to_add, &mut keys_to_search);
        current_generation = current_generation + (1 as u128);

        GameWorldDisplay::print_chunk(&world);
        assert_eq!(world.alive_tile_keys.len(), 3);

        handle_generation(debug, &mut world, &mut keys_to_remove, &mut keys_to_add, &mut keys_to_search);
        current_generation = current_generation + (1 as u128);

        GameWorldDisplay::print_chunk(&world);
        assert_eq!(world.alive_tile_keys.len(), 3);

        handle_generation(debug, &mut world, &mut keys_to_remove, &mut keys_to_add, &mut keys_to_search);
        current_generation = current_generation + (1 as u128);

        GameWorldDisplay::print_chunk(&world);
        assert_eq!(world.alive_tile_keys.len(), 3);

        handle_generation(debug, &mut world, &mut keys_to_remove, &mut keys_to_add, &mut keys_to_search);
        current_generation = current_generation + (1 as u128);

        GameWorldDisplay::print_chunk(&world);
        assert_eq!(world.alive_tile_keys.len(), 3);

        handle_generation(debug, &mut world, &mut keys_to_remove, &mut keys_to_add, &mut keys_to_search);
        current_generation = current_generation + (1 as u128);

        GameWorldDisplay::print_chunk(&world);
        assert_eq!(world.alive_tile_keys.len(), 3);
        assert_eq!(current_generation, 12);
    }

    #[test]
    fn test_comput_allotted_read_input_time() {
        let mut world = World::new(1000, 1000, 10, 10, 100);
        println!("{}", world.allotted_read_input_time);
        let mut world = World::new(100, 100, 10, 10, 10);
        println!("{}", world.allotted_read_input_time);
        let mut world = World::new(1000, 1000, 10, 10, 50);
        println!("{}", world.allotted_read_input_time);
    }
}

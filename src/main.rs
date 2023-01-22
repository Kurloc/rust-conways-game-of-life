extern crate crossterm;
use crate::design_world::DesignWorld;
use crate::design_world_display::DesignWorldDisplay;

mod tile;
mod design_world;
mod design_world_display;
mod world;
mod world_display;

use std::io::{stdout, Stdout};
use crossterm::{
    execute,
    cursor,
    style::Print,
    event::{Event, KeyCode},
    event::{KeyEvent, KeyModifiers, poll, read},
    terminal::{
        Clear,
        ClearType,
        disable_raw_mode,
        enable_raw_mode,
    },
};
use std::{
    thread::sleep,
    time::{Duration, Instant},
};

use world_display::{MoveDirections, GameWorldDisplay};
use crate::world::World;


fn main() {
    let empty_string = "";
    enable_raw_mode().unwrap();
    let mut stdout = stdout();

    let mut reset_game = false;
    let mut edit_game_settings = false;
    let mut design_world = true;
    let mut is_game_running = true;

    // @TODO make these values for world size allowed to passed in via args
    // @TODO also make the values editable during the game loop, they can already be resized
    // safely just need to add keybindings.
    let mut world = World::new(
        250,
        250,
        75,
        50,
        36,
        0
    );
    let mut design_world_instance = DesignWorld::new(125, 50);
    
    let mut current_generation = 1;
    let mut how_long_we_slept = 0;
    let mut how_long_rendering_took = 0;
    let mut how_long_generation_took = 0;
    let mut how_long_input_took = 0;
    let mut how_long_a_frame_took = 0;
    let mut sleep_duration_ms;
    
    execute!(stdout, cursor::Hide, Print("")).unwrap();
    while is_game_running {
        let now_total = Instant::now();
        let mut now = Instant::now();
        if edit_game_settings {
            // @TODO: Add more stuff here to edit game values like:
            //      world.x_size
            //      world.y_size
            //      world.chunk_x_size
            //      world.chunk_y_size
            //      world.sleep_duration
            execute!(stdout, Clear(ClearType::FromCursorUp), cursor::MoveTo(0, 0),
                Print(
                    "Howdy howdy\n\
                    return: go back\n"
                )
            ).unwrap();
            if poll(Duration::from_millis((10) as u64)).unwrap() {
                let key_event = read().unwrap();
                match key_event {
                    Event::Key(
                        KeyEvent {
                            code: KeyCode::Backspace,
                            modifiers: KeyModifiers::NONE,
                            ..
                        }) => {
                        edit_game_settings = false;
                        design_world = true;
                        continue;
                    }
                    _ => (),
                }
            }
            sleep_duration_ms = 10;
        }
        else if design_world {
            let now = Instant::now();
            // @TODO: Add more functionality here for editing, mainly QOL life stuff
            //      1) Ability to copy a section and paste it else where on the world
            //      2) Clear the design map, so you don't need to reset the app to clear it or untoggle everything manually.

            // Render
            DesignWorldDisplay::print_design_world(&mut stdout, &design_world_instance);

            // handle input
            (is_game_running, design_world, edit_game_settings) = handle_design_world_input(
                &mut design_world_instance
            );
            
            // init the game world w/ our edits
            if !design_world {
                world.reset_game_world(design_world_instance.marked_positions.iter());
                current_generation = 1;
            }
            sleep_duration_ms = (32 as i32) - (now.elapsed().as_millis() as i32);
            if sleep_duration_ms < 0 {
                sleep_duration_ms = 0;
            }
        }
        else {
            // Render
            render_game_world(
                current_generation,
                how_long_we_slept,
                how_long_a_frame_took,
                how_long_rendering_took,
                how_long_generation_took,
                how_long_input_took,
                &world,
                &mut stdout
            );
            how_long_rendering_took = now.elapsed().as_millis() as i128;
            now = Instant::now();
            
            // handle input
            (is_game_running, design_world, reset_game) = handle_game_play_input(&mut world);
            how_long_input_took = now.elapsed().as_millis() as i128;
            now = Instant::now();
            if reset_game {
                world.reset_game_world(design_world_instance.marked_positions.iter());
                current_generation = 1;
                continue;
            }

            // handle update
            handle_gameplay_loop(&mut world);
            how_long_generation_took = now.elapsed().as_millis() as i128;

            current_generation += 1;
            sleep_duration_ms = world.frame_interval_ms as i32;
        }

        how_long_a_frame_took = now_total.elapsed().as_millis() as i128;
        how_long_we_slept = (sleep_duration_ms as i128) - how_long_a_frame_took;
        if how_long_we_slept > 0 {
            sleep(Duration::from_millis(how_long_we_slept as u64));
        }
        how_long_a_frame_took = how_long_a_frame_took + how_long_we_slept
    }

    execute!(stdout, Clear(ClearType::All), Print(empty_string)).unwrap();
    disable_raw_mode().unwrap();
}

fn handle_game_play_input(world: &mut World) -> (bool, bool, bool) {
    if poll(Duration::from_millis((world.allotted_read_input_time) as u64)).unwrap() {
        let key_event = read().unwrap();
        match key_event {
            Event::Key(KeyEvent {
                           code: KeyCode::Char('w'),
                           modifiers: KeyModifiers::NONE, ..
                       }) => world.move_chunk(MoveDirections::Up),
            Event::Key(KeyEvent {
                           code: KeyCode::Char('a'),
                           modifiers: KeyModifiers::NONE, ..
                       }) => world.move_chunk(MoveDirections::Left),
            Event::Key(KeyEvent {
                           code: KeyCode::Char('s'),
                           modifiers: KeyModifiers::NONE, ..
                       }) => world.move_chunk(MoveDirections::Down),
            Event::Key(KeyEvent {
                           code: KeyCode::Char('d'),
                           modifiers: KeyModifiers::NONE, ..
                       }) => world.move_chunk(MoveDirections::Right),
            Event::Key(KeyEvent {
                           code: KeyCode::Char('e'),
                           modifiers: KeyModifiers::CONTROL, ..
                       }) => return (true, true, false),
            Event::Key(KeyEvent {
                           code: KeyCode::Char('r'),
                           modifiers: KeyModifiers::NONE, ..
                       }) => return (true, false, true),
            Event::Key(KeyEvent {
                           code: KeyCode::Char('q'),
                           modifiers: KeyModifiers::CONTROL, ..
                       }) => return (false, false, false),
            Event::Key(KeyEvent {
                           code: KeyCode::Char('c'),
                           modifiers: KeyModifiers::CONTROL, ..
                       }) => return (false, false, false),
            _ => (),
        }
    }
    return (true, false, false)
}

fn handle_design_world_input(
    design_world_instance: &mut DesignWorld
) -> (bool, bool, bool) {
    if poll(Duration::from_millis((10) as u64)).unwrap() {
        let key_event = read().unwrap();
        match key_event {
            Event::Key(
                KeyEvent {
                    code: KeyCode::Enter,
                    modifiers: KeyModifiers::NONE,
                    ..
                }) => {
                let current_tile = design_world_instance
                    .marked_positions
                    .get(&design_world_instance.current_position);

                if *current_tile.unwrap_or(&false) {
                    design_world_instance.marked_positions.insert(
                        design_world_instance.current_position,
                        false,
                    );
                } else {
                    design_world_instance.marked_positions.insert(
                        design_world_instance.current_position,
                        true,
                    );
                }
            }
            Event::Key(KeyEvent {
                           code: KeyCode::Char('e'),
                           modifiers: KeyModifiers::CONTROL,
                           ..
                       }) => return (true, false, true),
            Event::Key(KeyEvent {
                           code: KeyCode::Char('c'),
                           modifiers: KeyModifiers::CONTROL,
                           ..
                       }) => return (false, false, false),
            Event::Key(KeyEvent {
                           code: KeyCode::Char('p'),
                           modifiers: KeyModifiers::CONTROL,
                           ..
                       }) => return (true, false, false),
            Event::Key(KeyEvent {
                           code: KeyCode::Char('w'),
                           modifiers: KeyModifiers::NONE, ..
                       }) => design_world_instance.move_chunk(MoveDirections::Down),
            Event::Key(KeyEvent {
                           code: KeyCode::Char('a'),
                           modifiers: KeyModifiers::NONE, ..
                       }) => design_world_instance.move_chunk(MoveDirections::Left),
            Event::Key(KeyEvent {
                           code: KeyCode::Char('s'),
                           modifiers: KeyModifiers::NONE, ..
                       }) => design_world_instance.move_chunk(MoveDirections::Up),
            Event::Key(KeyEvent {
                           code: KeyCode::Char('d'),
                           modifiers: KeyModifiers::NONE, ..
                       }) => design_world_instance.move_chunk(MoveDirections::Right),
            _ => (),
        }
    }
    return (true, true, false);
}

fn handle_gameplay_loop(world: &mut World) {
    let mut keys_to_remove: Vec<(i128, i128)> = Vec::new();
    let mut keys_to_add: Vec<(i128, i128)> = Vec::new();
    world.handle_generation(&mut keys_to_remove, &mut keys_to_add);
}

fn render_game_world(
    current_generation: u128,
    how_long_we_slept: i128,
    how_long_a_frame_took: i128,
    how_long_rendering_took: i128,
    how_long_generation_took: i128,
    how_long_input_took: i128,
    world: &World,
    stdout: &mut Stdout,
) {
    execute!(stdout, Clear(ClearType::FromCursorUp), cursor::MoveTo(0, 0), Print(
        format!(
            "Current generation: {}\n  \
             Read_time: {}ms\n  \
             Rendering took {}ms\n  \
             Generation_took: {}ms\n  \
             Sleep Time: {}ms\n  \
             Requested frame interval: {}\n  \
             Total Frame Time: {}ms\n",
            current_generation,
            how_long_input_took,
            how_long_rendering_took,
            how_long_generation_took,
            how_long_we_slept,
            world.frame_interval_ms,
            how_long_a_frame_took
        ),
    )).unwrap();
    GameWorldDisplay::print_chunk(&world);
}

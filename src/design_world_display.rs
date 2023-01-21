use std::io::Stdout;
use crossterm::execute;
use crossterm::style::Stylize;
use crossterm::style::Print;
use crossterm::{
    cursor,
    terminal::{
        Clear,
        ClearType,
    },
};
use crate::design_world::DesignWorld;

pub struct DesignWorldDisplay {}

impl DesignWorldDisplay {
    pub fn print_design_world(stdout: &mut Stdout, world: &DesignWorld) {
        let chunk_address = world.current_position;
        execute!(stdout, Clear(ClearType::FromCursorUp), cursor::MoveTo(0, 0), Print("")).unwrap();
        print!(
                    "CurrentPosition: ({}, {})\n\
                    e+ctrl: edit settings\n\
                    c+ctrl: quit\n\
                    p+ctrl: play\n\
                    w,a,s,d: move\n\
                    enter: mark a tile\n",
                    world.current_position.0, world.current_position.1
        );
        let x_check = world.max_x - 1;
        let y_check = world.max_y - 1;
        for y in 0..world.max_y {
            for x in 0..world.max_x {
                let address = (x, y);
                if address == chunk_address {
                    print!("{}", format!("X").yellow());
                } else if *world.marked_positions.get(&address).unwrap_or(&false) {
                    print!("{}", format!("M").blue());
                } else {
                    if y == 0 || x == 0 || x == x_check || y == y_check {
                        print!("+");
                    } else {
                        print!("{}", " ");
                    }
                }
            }
            print!("\n");
        }
    }
}

use std::collections::HashMap;
use crate::world_display::MoveDirections;

pub struct DesignWorld {
    pub current_position: (u16, u16),
    pub max_x: u16,
    pub max_y: u16,
    pub marked_positions: HashMap<(u16, u16), bool>,
}

impl DesignWorld {
    pub fn new(max_x: u16, max_y: u16) -> DesignWorld {
        let mut marked_positions: HashMap<(u16, u16), bool> = HashMap::new();
        for y in 0..max_y {
            for x in 0..max_x {
                marked_positions.insert((x, y), false);
            }
        }

        return DesignWorld {
            current_position: ((1), (1)),
            max_x,
            max_y,
            marked_positions,
        };
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

        let original_chunk_x = self.current_position.0;
        let original_chunk_y = self.current_position.1;
        let mut new_x = (original_chunk_x as i16) + (adjustment_amount.0 as i16);
        let mut new_y = (original_chunk_y as i16) + (adjustment_amount.1 as i16);
        if new_x == 0
            || new_y == 0
            || new_x < 0
            || new_y < 0
            || new_x == (self.max_x - 1) as i16
            || new_y == (self.max_y - 1) as i16
         {
            return;
        }

        self.current_position = (new_x as u16, new_y as u16);
    }
}

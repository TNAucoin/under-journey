use crate::prelude::*;

pub struct Player {
    pub position: Point,
}

impl Player {
    pub fn new(position: Point) -> Self {
        Self { position }
    }

    pub fn update(&mut self, ctx: &mut BTerm, map: &Map, camera: &mut Camera) {
        if let Some(key) = ctx.key {
            // Create a delta point direction based on keypress
            let delta = match key {
                VirtualKeyCode::A => Point::new(-1, 0),
                VirtualKeyCode::D => Point::new(1, 0),
                VirtualKeyCode::W => Point::new(0, -1),
                VirtualKeyCode::S => Point::new(0, 1),
                _ => Point::zero(),
            };
            // add the delta position to current, to get new potential location
            let new_position = self.position + delta;
            // if the location is valid, set the current position to new_position
            if map.can_enter_tile(new_position) {
                self.position = new_position;
                // Move the camera to the new player position
                camera.on_player_move(new_position);
            }
        }
    }

    pub fn render(&self, ctx: &mut BTerm, camera: &Camera) {
        // Set to object rendering layer
        ctx.set_active_console(1);
        // Render player in relation to the camera
        ctx.set(
            self.position.x - camera.left_x,
            self.position.y - camera.top_y,
            WHITE,
            BLACK,
            to_cp437('@'),
        )
    }
}

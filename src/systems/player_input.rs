use crate::prelude::*;

#[system]
#[write_component(Point)]
#[read_component(Player)]
pub fn player_input(
    ecs: &mut SubWorld,
    #[resource] map: &Map,
    #[resource] key: &Option<VirtualKeyCode>,
    #[resource] camera: &mut Camera,
) {
    if let Some(key) = key {
        let delta = match key {
            VirtualKeyCode::A => Point::new(-1, 0),
            VirtualKeyCode::D => Point::new(1, 0),
            VirtualKeyCode::W => Point::new(0, -1),
            VirtualKeyCode::S => Point::new(0, 1),
            _ => Point::zero(),
        };

        if delta.x != 0 || delta.y != 0 {
            // Query for mut Point ref, filter for entities with the Player tag component
            let mut players = <&mut Point>::query().filter(component::<Player>());
            // iterate over query results returning a mut point ref from the query
            players.iter_mut(ecs).for_each(|pos| {
                // deref the pos value and add the delta
                let destination = *pos + delta;
                // check if this is a valid pos
                if map.can_enter_tile(destination) {
                    //update the pos value with delta, and change camera
                    *pos = destination;
                    camera.on_player_move(destination);
                }
            })
        }
    }
}

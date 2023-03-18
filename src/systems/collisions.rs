use crate::prelude::*;

#[system]
#[read_component(Point)]
#[read_component(Player)]
#[read_component(Enemy)]
// CommandBuffer allows you to issue a set of instructions after
// this system has finished
pub fn collisions(ecs: &mut SubWorld, commands: &mut CommandBuffer) {
    // store the player position for later comparison
    let mut player_position = Point::zero();
    // get the player position
    let mut players = <&Point>::query().filter(component::<Player>());
    players.iter(ecs).for_each(|pos| player_position = *pos);
    // get a list of all enemies <Entity,Pos> components
    let mut enemies = <(Entity, &Point)>::query().filter(component::<Enemy>());
    enemies
        .iter(ecs)
        // if this enemy is == to the player position
        .filter(|(_, pos)| **pos == player_position)
        // submit a command to remove this enemy from the game at the end of the frame.
        .for_each(|(entity, _)| {
            commands.remove(*entity);
        })
}

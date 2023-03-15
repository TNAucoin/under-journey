mod camera;
mod map;
mod map_builder;
mod player;

mod prelude {
    pub use bracket_lib::prelude::*;
    pub const SCREEN_WIDTH: i32 = 80;
    pub const SCREEN_HEIGHT: i32 = 50;
    pub const DISPLAY_WIDTH: i32 = SCREEN_WIDTH / 2;
    pub const DISPLAY_HEIGHT: i32 = SCREEN_HEIGHT / 2;
    pub use crate::camera::*;
    pub use crate::map::*;
    pub use crate::map_builder::*;
    pub use crate::player::*;
}

use prelude::*;

pub const TILESET_NAME: &str = "tileset.jpg";
pub const TILE_WIDTH: i32 = 32;
pub const TILE_HEIGHT: i32 = 32;

//Holds our global game state
struct State {
    map: Map,
    player: Player,
    camera: Camera,
}

impl State {
    fn new() -> Self {
        // Create new instance of RNG, and pass it to the map builder constructor
        let mut rng = RandomNumberGenerator::new();
        let map_builder = MapBuilder::new(&mut rng);
        // Set the current map as the resulting map from map_builder
        // and set player's start position in this map.
        Self {
            map: map_builder.map,
            player: Player::new(map_builder.player_start),
            camera: Camera::new(map_builder.player_start),
        }
    }
}

// Implements the gameloop on our State
impl GameState for State {
    // Primary Update Loop
    fn tick(&mut self, ctx: &mut BTerm) {
        ctx.set_active_console(0);
        ctx.cls();
        ctx.set_active_console(1);
        ctx.cls();
        self.player.update(ctx, &self.map, &mut self.camera);
        self.map.render(ctx, &self.camera);
        self.player.render(ctx, &self.camera);
    }
}

fn main() -> BError {
    // Setup game window, and graphics
    let context = BTermBuilder::new()
        .with_title("Under Journey")
        .with_fps_cap(30.0)
        .with_dimensions(DISPLAY_WIDTH, DISPLAY_HEIGHT)
        .with_tile_dimensions(TILE_WIDTH, TILE_HEIGHT)
        .with_resource_path("resources/")
        .with_font(TILESET_NAME, TILE_WIDTH, TILE_HEIGHT)
        .with_simple_console(DISPLAY_WIDTH, DISPLAY_HEIGHT, TILESET_NAME)
        .with_simple_console_no_bg(DISPLAY_WIDTH, DISPLAY_HEIGHT, TILESET_NAME)
        .build()?;
    // Run the game
    main_loop(context, State::new())
}

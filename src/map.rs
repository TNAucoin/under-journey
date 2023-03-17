use crate::prelude::*;

// Calculate the total number of tiles we will need
// usize will match preferred cpu so 64bit cpu will result in 64bit usize
const NUM_TILES: usize = (SCREEN_WIDTH * SCREEN_HEIGHT) as usize;

#[derive(Copy, Clone, PartialEq)]
pub enum TileType {
    Wall,
    Floor,
}

pub struct Map {
    pub tiles: Vec<TileType>,
}

// Y-first ordering of map
// each row of data is placed together in y-order
pub fn map_idx(x: i32, y: i32) -> usize {
    // vectors are indexed using usize
    // We calculate the x,y position using the y-first ordering
    // then convert to usize for the right vec index.
    ((y * SCREEN_WIDTH) + x) as usize
}

impl Map {
    pub fn new() -> Self {
        Self {
            tiles: vec![TileType::Floor; NUM_TILES],
        }
    }

    // Check if the point is within the bounds of the current map
    pub fn in_bounds(&self, point: Point) -> bool {
        point.x >= 0 && point.x < SCREEN_WIDTH && point.y >= 0 && point.y < SCREEN_HEIGHT
    }

    // Check if an entity can enter this tile
    pub fn can_enter_tile(&self, point: Point) -> bool {
        self.in_bounds(point) && self.tiles[map_idx(point.x, point.y)] == TileType::Floor
    }

    // Checks if the point falls within the map and if it is valid returns the vec idx of that location.
    pub fn try_idx(&self, point: Point) -> Option<usize> {
        if !self.in_bounds(point) {
            None
        } else {
            Some(map_idx(point.x, point.y))
        }
    }

    pub fn render(&self, ctx: &mut BTerm, camera: &Camera) {
        // Set active rendering layer to the background
        ctx.set_active_console(0);
        // Only render what is in view of the camera, and within the map bounds
        for y in camera.top_y..camera.bottom_y {
            for x in camera.left_x..camera.right_x {
                if self.in_bounds(Point::new(x, y)) {
                    let idx = map_idx(x, y);
                    match self.tiles[idx] {
                        TileType::Floor => ctx.set(
                            x - camera.left_x,
                            y - camera.top_y,
                            WHITE,
                            BLACK,
                            to_cp437('.'),
                        ),
                        TileType::Wall => ctx.set(
                            x - camera.left_x,
                            y - camera.top_y,
                            WHITE,
                            GREY10,
                            to_cp437('#'),
                        ),
                    }
                }
            }
        }
    }
}

use crate::prelude::*;

/// Map for game, Player, rooms, corridors, monsters, treasure all on it.

const NUM_TILES: usize = (SCREEN_HEIGHT * SCREEN_WIDTH) as usize;

/// Indicate how tile should be rendered
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum TileType {
    Wall,
    Floor,
}

/// We use vec for saving map, (alternative approach is array)
pub struct Map {
    pub tiles: Vec<TileType>,
}

impl Map {
    pub fn new() -> Self {
        Map {
            tiles: vec![TileType::Floor; NUM_TILES],
        }
    }

    pub fn map_idx(x: i32, y: i32) -> usize {
        ((SCREEN_WIDTH * y) + x) as usize
    }

    /// Get type of tiles.
    pub fn type_on(&self, point: Point) -> Option<TileType> {
        if let Some(idx) = self.try_index(point) {
            Some(self.tiles[idx])
        } else {
            None
        }
    }

    /// Rendering functionality move to ECS.
    // pub fn render(&self, ctx: &mut BTerm, camera: &Camera) {
    //     ctx.set_active_console(0);
    //     for y in camera.top_y..camera.bottom_y {
    //         for x in camera.left_x..camera.right_x {
    //             if let Some(tile_type) = self.type_on(Point::new(x, y)) {
    //                 match tile_type {
    //                     TileType::Floor => ctx.set(
    //                         x - camera.left_x,
    //                         y - camera.top_y,
    //                         WHITE,
    //                         BLACK,
    //                         to_cp437('.'),
    //                     ),
    //                     TileType::Wall => ctx.set(
    //                         x - camera.left_x,
    //                         y - camera.top_y,
    //                         WHITE,
    //                         BLACK,
    //                         to_cp437('#'),
    //                     ),
    //                 }
    //             }
    //         }
    //     }
    // }

    /// Check if a point is out of map
    pub fn in_bound(&self, point: Point) -> bool {
        point.x >= 0
            && point.x < SCREEN_WIDTH
            && point.y >= 0
            && point.y < SCREEN_HEIGHT
    }

    /// Check if play can enter the point
    pub fn can_enter_tile(&self, point: Point) -> bool {
        self.in_bound(point)
            && self.tiles[Map::map_idx(point.x, point.y)] == TileType::Floor
    }

    /// Get index of point in vec.
    pub fn try_index(&self, point: Point) -> Option<usize> {
        if self.in_bound(point) {
            Some(Map::map_idx(point.x, point.y))
        } else {
            None
        }
    }
}

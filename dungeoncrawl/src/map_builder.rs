use crate::prelude::*;

const NUM_ROOMS: usize = 100;

pub struct MapBuilder {
    pub map: Map,
    pub rooms: Vec<Rect>,
}

impl MapBuilder {
    pub fn new(rng: &mut RandomNumberGenerator) -> Self {
        let mut mb = Self {
            map: Map::new(),
            rooms: Vec::new(),
        };

        mb.fill(TileType::Wall);
        mb.generate_rooms(rng);
        mb.draw_corridors(rng);

        mb
    }

    pub fn fill(&mut self, tile_type: TileType) {
        self.map.tiles.iter_mut().for_each(|f| *f = tile_type);
    }

    /// randomly build some rooms up to NUM_ROOMS
    pub fn generate_rooms(&mut self, rng: &mut RandomNumberGenerator) {
        while self.rooms.len() < NUM_ROOMS {
            let room = Rect::with_size(
                rng.range(1, SCREEN_WIDTH - 10),  // x
                rng.range(1, SCREEN_HEIGHT - 10), // y
                rng.range(2, 10),                 // width
                rng.range(2, 10),                 // height
            );

            let count = self.rooms.iter().filter(|r| r.intersect(&room)).count();

            if count == 0 {
                room.for_each(|p| {
                    if self.map.in_bound(p) {
                        let index = Map::map_idx(p.x, p.y);
                        self.map.tiles[index] = TileType::Floor;
                    }
                });
                self.rooms.push(room);
            }
        }
    }

    fn dig_vertical_tunnel(&mut self, x: i32, y1: i32, y2: i32) {
        use std::cmp::{max, min};
        for y in min(y1, y2)..max(y1, y2) {
            if let Some(idx) = self.map.try_index(Point::new(x, y)) {
                self.map.tiles[Map::map_idx(x, y)] = TileType::Floor;
            }
        }
    }

    fn dig_horizontal_tunnel(&mut self, x1: i32, x2: i32, y: i32) {
        use std::cmp::{max, min};
        for x in min(x1, x2)..max(x1, x2) {
            if let Some(idx) = self.map.try_index(Point::new(x, y)) {
                self.map.tiles[Map::map_idx(x, y)] = TileType::Floor;
            }
        }
    }

    pub fn draw_corridors(&mut self, rng: &mut RandomNumberGenerator) {
        let mut rooms = self.rooms.clone();

        rooms.sort_by(|a, b| a.center().x.cmp(&b.center().x));

        for (i, current) in rooms.iter().enumerate().skip(1) {
            let prev = rooms[i - 1].center();
            let current = current.center();
            if rng.rand() {
                self.dig_horizontal_tunnel(prev.x, current.x, prev.y);
                self.dig_vertical_tunnel(current.x, prev.y, current.y);
            } else {
                self.dig_vertical_tunnel(prev.x, prev.y, current.y);
                self.dig_horizontal_tunnel(prev.x, current.x, current.y);
            }
        }
    }
}

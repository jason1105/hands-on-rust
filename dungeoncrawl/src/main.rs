#![warn(clippy::all, clippy::pedantic)]

mod map;
mod map_builder;
mod player;
mod prelude {
    pub use crate::map::*;
    pub use crate::map_builder::*;
    pub use crate::player::*;
    pub use bracket_lib::prelude::*;
    pub const SCREEN_HEIGHT: i32 = 50;
    pub const SCREEN_WIDTH: i32 = 80;
}

use prelude::*;

struct State {
    map: Map,
    player: Player,
}

impl State {
    fn new() -> Self {
        Self {
            map: Self::build_map(),
            player: Player::new(Point::new(SCREEN_WIDTH / 2, SCREEN_HEIGHT / 2)),
        }
    }

    fn reset(&mut self, ctx: &mut BTerm) {
        self.map = Self::build_map();
    }

    fn build_map() -> Map {
        let mut rng = RandomNumberGenerator::new();
        let map_builder = MapBuilder::new(&mut rng);
        map_builder.map
    }
}

impl GameState for State {
    fn tick(&mut self, ctx: &mut BTerm) {
        ctx.cls();
        self.map.render(ctx);
        self.player.update(&mut self.map, ctx);
        self.player.render(&self.map, ctx);

        if let Some(VirtualKeyCode::R) = ctx.key {
            self.reset(ctx);
        }
    }
}

fn main() -> BError {
    let context = BTermBuilder::simple80x50()
        .with_title("DungeonCrawl")
        .with_fps_cap(30.0)
        .build()?;
    main_loop(context, State::new())
}

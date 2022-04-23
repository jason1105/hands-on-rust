#![warn(clippy::all, clippy::pedantic)]

mod camera;
mod map;
mod map_builder;
mod player;
mod prelude {
    pub use crate::camera::*;
    pub use crate::map::*;
    pub use crate::map_builder::*;
    pub use crate::player::*;
    pub use bracket_lib::prelude::*;
    pub const SCREEN_HEIGHT: i32 = 50;
    pub const SCREEN_WIDTH: i32 = 80;
    pub const DISPLAY_WIDTH: i32 = SCREEN_WIDTH / 2;
    pub const DISPLAY_HEIGHT: i32 = SCREEN_HEIGHT / 2;
}

use prelude::*;

struct State {
    map: Map,
    player: Player,
    camera: Camera,
}

impl State {
    fn new() -> Self {
        let (map, player_start) = Self::build_map();
        Self {
            map: map,
            player: Player::new(player_start),
            camera: Camera::new(player_start),
        }
    }

    fn reset(&mut self) {
        let mut player_start = Point::zero();
        (self.map, player_start) = Self::build_map();
        self.player = Player::new(player_start);
        self.camera = Camera::new(player_start);
    }

    fn build_map() -> (Map, Point) {
        let mut rng = RandomNumberGenerator::new();
        let map_builder = MapBuilder::new(&mut rng);
        (map_builder.map, map_builder.player_start)
    }
}

impl GameState for State {
    fn tick(&mut self, ctx: &mut BTerm) {
        ctx.set_active_console(0);
        ctx.cls();
        ctx.set_active_console(1);
        ctx.cls();

        self.player.update(&mut self.map, ctx, &mut self.camera);
        self.map.render(ctx, &self.camera);
        self.player.render(&self.map, ctx, &self.camera);

        if let Some(VirtualKeyCode::R) = ctx.key {
            self.reset();
        }
    }
}

fn main() -> BError {
    // let context = BTermBuilder::simple80x50()
    //     .with_title("DungeonCrawl")
    //     .with_fps_cap(30.0)
    //     .build()?;
    let context = BTermBuilder::new()
        .with_title("Dungeon Crawler")
        .with_fps_cap(30.0)
        .with_dimensions(DISPLAY_WIDTH, DISPLAY_HEIGHT)
        .with_tile_dimensions(32, 32)
        .with_resource_path("resources/")
        .with_font("dungeonfont.png", 32, 32) // the size are usually the same as tile dimensions
        .with_simple_console(DISPLAY_WIDTH, DISPLAY_HEIGHT, "dungeonfont.png")
        .with_simple_console_no_bg(DISPLAY_WIDTH, DISPLAY_HEIGHT, "dungeonfont.png")
        .build()?;
    main_loop(context, State::new())
}

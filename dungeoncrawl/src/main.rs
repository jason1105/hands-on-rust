#![warn(clippy::all, clippy::pedantic)]

mod camera;
mod components;
mod map;
mod map_builder;
mod player;
mod spawner;
mod systems;
mod prelude {
    pub use crate::camera::*;
    pub use crate::components::*;
    pub use crate::map::*;
    pub use crate::map_builder::*;
    pub use crate::player::*;
    pub use crate::spawner::*;
    pub use crate::systems::*;
    pub use bracket_lib::prelude::*;
    pub use legion::systems::CommandBuffer;
    pub use legion::world::SubWorld;
    pub use legion::*;
    pub const SCREEN_HEIGHT: i32 = 50;
    pub const SCREEN_WIDTH: i32 = 80;
    pub const DISPLAY_WIDTH: i32 = SCREEN_WIDTH / 2;
    pub const DISPLAY_HEIGHT: i32 = SCREEN_HEIGHT / 2;
}

use prelude::*;

struct State {
    ecs: World,           // context stores all entities and components
    resources: Resources, // hold resources
    systems: Schedule,    // hold systems
}

impl State {
    fn new() -> Self {
        let mut ecs = World::default();

        // init resource.
        let mut resources = Resources::default();
        let (map, player_start) = Self::build_map();
        resources.insert(map);
        resources.insert(Camera::new(player_start));

        // add entity
        spawn_player(&mut ecs, player_start);

        Self {
            ecs,
            resources,
            systems: build_scheduler(),
        }
    }

    fn reset(&mut self) {}

    fn build_map() -> (Map, Point) {
        let mut rng = RandomNumberGenerator::new();
        let map_builder = MapBuilder::new(&mut rng);
        (map_builder.map, map_builder.player_start)
    }
}

impl GameState for State {
    fn tick(&mut self, ctx: &mut BTerm) {
        // clear console
        ctx.set_active_console(0);
        ctx.cls();
        ctx.set_active_console(1);
        ctx.cls();

        // refresh resource of key input
        self.resources.insert(ctx.key);

        // execute all systems
        self.systems.execute(&mut self.ecs, &mut self.resources);

        // flush render buffer
        render_draw_buffer(ctx).expect("Render error");
    }
}

fn main() -> BError {
    // let context = BTermBuilder::simple80x50()
    //     .with_title("DungeonCrawl")
    //     .with_fps_cap(30.0)
    //     .build()?;
    let context = BTermBuilder::new()
        .with_title("Dungeon Crawler")
        .with_fps_cap(30.0) // 30 frames per minute
        .with_dimensions(DISPLAY_WIDTH, DISPLAY_HEIGHT) // window size
        .with_tile_dimensions(32, 32) // tile size
        .with_resource_path("resources/")
        .with_font("dungeonfont.png", 32, 32) // the size are usually the same as tile dimensions
        .with_simple_console(DISPLAY_WIDTH, DISPLAY_HEIGHT, "dungeonfont.png") // a layer used for drawing map
        .with_simple_console_no_bg(
            // a layer used for drawing player
            DISPLAY_WIDTH,
            DISPLAY_HEIGHT,
            "dungeonfont.png",
        )
        .build()?;

    // main loop
    main_loop(context, State::new())
}

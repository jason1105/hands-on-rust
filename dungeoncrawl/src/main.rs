#![warn(clippy::all, clippy::pedantic)]

mod camera;
mod components;
mod map;
mod map_builder;
mod player;
mod spawner;
mod systems;
mod turn_state;
mod prelude {
    pub use crate::camera::*;
    pub use crate::components::*;
    pub use crate::map::*;
    pub use crate::map_builder::*;
    pub use crate::player::*;
    pub use crate::spawner::*;
    pub use crate::systems::*;
    pub use crate::turn_state::*;
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
    ecs: World,               // context stores all entities and components
    resources: Resources,     // hold resources
    input_system: Schedule,   // systems
    player_system: Schedule,  // systems
    monster_system: Schedule, // systems
}

impl State {
    fn new() -> Self {
        let mut ecs = World::default();
        let mut rng = RandomNumberGenerator::new();

        // init resource.
        let mut resources = Resources::default();
        let (map, player_start, rooms) = Self::build_map(&mut rng);
        resources.insert(map);
        resources.insert(Camera::new(player_start));
        resources.insert(TurnState::AwaitingInput); // init turn state

        // add player
        spawn_player(&mut ecs, player_start);

        // add enemy
        rooms
            .iter()
            .skip(1) // Player is in first room.
            .map(|rect| rect.center())
            .for_each(|point| {
                spawn_enemy(&mut ecs, point, &mut rng);
            });

        Self {
            ecs,
            resources,
            input_system: build_input_schedule(),
            player_system: build_player_schedule(),
            monster_system: build_monster_schedule(),
        }
    }

    fn reset(&mut self) {}

    fn build_map(rng: &mut RandomNumberGenerator) -> (Map, Point, Vec<Rect>) {
        let map_builder = MapBuilder::new(rng);
        (map_builder.map, map_builder.player_start, map_builder.rooms)
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

        // execute systems
        let turn_state = self.resources.get::<TurnState>().unwrap().clone(); // Fetch 自动实现 Clone, 因为 TurnState 是Clone
        let system = match turn_state {
            TurnState::AwaitingInput => self
                .input_system
                .execute(&mut self.ecs, &mut self.resources),
            TurnState::PlayerTurn => self
                .player_system
                .execute(&mut self.ecs, &mut self.resources),
            TurnState::MonsterTurn => self
                .monster_system
                .execute(&mut self.ecs, &mut self.resources),
        };

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

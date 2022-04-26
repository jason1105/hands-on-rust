use crate::prelude::*;

use self::collisions::collision_system;
use self::entity_render::entity_render_system;
use self::map_render::map_render_system;
use self::player_input::player_input_system;
mod collisions;
mod entity_render;
mod map_render;
mod player_input;
/// Build systems

pub fn build_scheduler() -> Schedule {
    Schedule::builder()
        .add_system(player_input_system())
        .add_system(map_render_system())
        .add_system(entity_render_system())
        .add_system(collision_system())
        .build()
}

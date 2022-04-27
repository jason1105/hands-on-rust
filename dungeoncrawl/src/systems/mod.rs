use crate::prelude::*;

use self::collisions::collision_system;
use self::end_turn::end_turn_system;
use self::entity_render::entity_render_system;
use self::map_render::map_render_system;
use self::player_input::player_input_system;
use self::random_move::random_move_system;

mod collisions;
mod end_turn;
mod entity_render;
mod map_render;
mod player_input;
mod random_move;

/// Build systems

pub fn build_input_schedule() -> Schedule {
    Schedule::builder()
        .add_system(player_input_system())
        .flush()
        .add_system(map_render_system())
        .add_system(entity_render_system())
        .build()
}

pub fn build_player_schedule() -> Schedule {
    Schedule::builder()
        .add_system(collision_system())
        .flush()
        .add_system(map_render_system())
        .add_system(entity_render_system())
        .add_system(end_turn_system())
        .build()
}

pub fn build_monster_schedule() -> Schedule {
    Schedule::builder()
        .add_system(random_move_system())
        .flush()
        .add_system(collision_system())
        .flush()
        .add_system(map_render_system())
        .add_system(entity_render_system())
        .add_system(end_turn_system())
        .build()
}

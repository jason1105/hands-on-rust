use crate::prelude::*;

use self::combat::combat_system;
use self::end_turn::end_turn_system;
use self::entity_render::entity_render_system;
use self::hud::hud_system;
use self::map_render::map_render_system;
use self::movement::movement_system;
use self::player_input::player_input_system;
use self::random_move::random_move_system;
use self::tooltips::tooltips_system;

mod combat;
mod end_turn;
mod entity_render;
mod hud;
mod map_render;
mod movement;
mod player_input;
mod random_move;
mod tooltips;

///! Build systems

/// handle input event
pub fn build_input_schedule() -> Schedule {
    Schedule::builder()
        // hud
        .add_system(hud_system())
        // handle input, create player movement message
        .add_system(player_input_system())
        .flush()
        // render screen
        .add_system(map_render_system())
        .add_system(entity_render_system())
        .add_system(tooltips_system())
        .build()
}

/// handle player
pub fn build_player_schedule() -> Schedule {
    Schedule::builder()
        // hub
        .add_system(hud_system())
        // process player's movement
        .add_system(movement_system())
        .flush() // flush command buffer
        .add_system(combat_system())
        .flush()
        // render screen
        .add_system(map_render_system())
        .add_system(entity_render_system())
        // show tooltips
        .add_system(tooltips_system())
        // move to next state
        .add_system(end_turn_system())
        .build()
}

/// handle monsters
pub fn build_monster_schedule() -> Schedule {
    Schedule::builder()
        // hud
        .add_system(hud_system())
        // monsters do move
        .add_system(random_move_system())
        .flush()
        // process monster's movement
        .add_system(movement_system())
        .flush() // flush command buffer
        .add_system(combat_system())
        .flush()
        // render screen
        .add_system(map_render_system())
        .add_system(entity_render_system())
        // show tooltips
        .add_system(tooltips_system())
        // move to first state
        .add_system(end_turn_system())
        .build()
}

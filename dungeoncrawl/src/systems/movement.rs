use crate::prelude::*;

/// This is movement system.
/// Handle all of messages of WantsToMove
#[system(for_each)]
#[read_component(Player)]
pub fn movement(
    want_move: &WantsToMove, // condition for query
    entity: &Entity,         // a reference to query result
    #[resource] map: &Map,
    #[resource] camera: &mut Camera,
    ecs: &mut SubWorld,
    command: &mut CommandBuffer,
) {
    if map.can_enter_tile(want_move.position) {
        command.add_component(want_move.entity, want_move.position);
        if ecs
            .entry_ref(want_move.entity)
            .unwrap()
            .get_component::<Player>()
            .is_ok()
        {
            camera.on_player_move(want_move.position);
        }
    }
    command.remove(*entity);
}

/// spawn entities in to world
use crate::prelude::*;

/// Adding player to world
pub fn spawn_player(ecs: &mut World, point: Point) {
    ecs.push((
        Player,
        point,
        Render {
            color: ColorPair::new(WHITE, BLACK),
            glyph: to_cp437('@'),
        },
    ));
}

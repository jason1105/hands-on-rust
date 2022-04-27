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

/// Adding enemy to world
pub fn spawn_enemy(
    ecs: &mut World,
    point: Point,
    rng: &mut RandomNumberGenerator,
) {
    ecs.push((
        Enemy,
        point,
        Render {
            color: ColorPair::new(WHITE, BLACK),
            glyph: match rng.range(0, 4) as i32 {
                0 => to_cp437('E'),
                1 => to_cp437('O'),
                2 => to_cp437('o'),
                _ => to_cp437('g'),
            },
        },
        MovingRandomly,
    ));
}

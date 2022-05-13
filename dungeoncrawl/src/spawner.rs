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
        Health {
            current: 20,
            max: 20,
        },
    ));
}

/// Adding enemy to world
pub fn spawn_enemy(
    ecs: &mut World,
    point: Point,
    rng: &mut RandomNumberGenerator,
) {
    let (health, name, glyph) = match rng.range(1, 10) {
        1..=8 => goblin(),
        _ => orc(),
    };

    ecs.push((
        Enemy,
        point,
        Render {
            color: ColorPair::new(WHITE, BLACK),
            glyph,
        },
        Health {
            current: health,
            max: health,
        },
        MovingRandomly,
        Name(name),
    ));
}

/// Creating monster of goblin
fn goblin() -> (i32, String, FontCharType) {
    return (1, "goblin".into(), to_cp437('g'));
}

/// Creating monster of orc
fn orc() -> (i32, String, FontCharType) {
    return (2, "orc".into(), to_cp437('o'));
}

use crate::prelude::*;

/// Heads-up Display system
#[system]
#[read_component(Health)]
#[read_component(Player)]
pub fn hud(ecs: &mut SubWorld) {
    let zero = Point::zero();

    let health = <&Health>::query()
        .filter(component::<Player>())
        .iter(ecs)
        .nth(0)
        .unwrap();

    let mut draw_batch = DrawBatch::new();
    draw_batch.target(2);

    // draw horizontal bar
    draw_batch.bar_horizontal(
        zero,
        SCREEN_WIDTH * 2,
        health.current, // current health
        health.max,     // max health
        ColorPair::new(RED, BLACK),
    );

    // draw player's Hit Points
    draw_batch.print_color_centered(
        0,
        format!("Health: {} / {}", health.current, health.max),
        ColorPair::new(WHITE, RED),
    );

    // draw information
    draw_batch.print_centered(1, "Explore the Dungeon. Cursor keys to move.");
}

use crate::prelude::*;

#[system]
#[read_component(Enemy)]
#[read_component(Player)]
#[read_component(Point)]
pub fn collision(ecs: &SubWorld, commands: &mut CommandBuffer) {
    let mut player_pos = Point::zero();

    <&Point>::query()
        .filter(component::<Player>())
        .iter(ecs)
        .for_each(|point| {
            player_pos = *point;
        });

    /*
    `<(Entity, &Point)>::query()` means query Entity and get reference to it.
    */
    <(Entity, &Point)>::query()
        .filter(component::<Enemy>())
        .iter(ecs)
        .filter(|(_, pos)| **pos == player_pos)
        .for_each(|(entity, _)| commands.remove(*entity));
}

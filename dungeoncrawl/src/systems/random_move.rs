use crate::prelude::*;

/// Handle movements of enemies
#[system]
#[read_component(MovingRandomly)]
#[write_component(Point)]
pub fn random_move(ecs: &mut SubWorld, commands: &mut CommandBuffer) {
    let mut rng = RandomNumberGenerator::new();
    <(Entity, &mut Point)>::query()
        .filter(component::<MovingRandomly>())
        .iter_mut(ecs)
        .for_each(|(entity, p)| {
            let destination = match rng.range(0, 5) {
                0 => Point::new(-1, 0),
                1 => Point::new(1, 0),
                2 => Point::new(0, 1),
                3 => Point::new(0, -1),
                _ => Point::new(0, 0),
            } + *p;

            commands.push((
                (),
                WantsToMove {
                    entity: *entity,
                    position: destination,
                },
            ));
        });
}

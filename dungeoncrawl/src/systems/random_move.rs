use crate::prelude::*;

/// Handle movements of enemies
#[system]
#[read_component(MovingRandomly)]
#[read_component(Point)]
pub fn random_move(ecs: &SubWorld, commands: &mut CommandBuffer) {
    let mut rng = RandomNumberGenerator::new();

    // player
    let (player, player_pos) = <(Entity, &Point)>::query()
        .filter(component::<Player>())
        .filter(component::<Health>())
        .iter(ecs)
        .nth(0)
        .expect("Error");

    // monsters
    <(Entity, &Point)>::query()
        .filter(component::<MovingRandomly>())
        .iter(ecs)
        .for_each(|(monster, monster_pos)| {
            let destination = match rng.range(0, 5) {
                0 => Point::new(-1, 0),
                1 => Point::new(1, 0),
                2 => Point::new(0, 1),
                3 => Point::new(0, -1),
                _ => Point::new(0, 0),
            } + *monster_pos;

            let mut attack = false;

            // monster attack player
            if destination == *player_pos {
                commands.push((
                    (),
                    WantsToAttack {
                        attacker: *monster,
                        victim: *player,
                    },
                ));
                attack = true;
            }

            // nobody else in front of, monster keep moving
            if !attack {
                commands.push((
                    (),
                    WantsToMove {
                        entity: *monster,
                        position: destination,
                    },
                ));
            }
        });
}

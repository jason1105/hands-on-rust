use crate::prelude::*;

/// A system for player input
/// Page 153
#[system]
#[write_component(Point)]
#[read_component(Player)]
#[read_component(Enemy)]
#[read_component(Health)]
pub fn player_input(
    ecs: &mut SubWorld,
    #[resource] key: &Option<VirtualKeyCode>,
    #[resource] turn_state: &mut TurnState,
    commands: &mut CommandBuffer,
) {
    if let Some(key) = key {
        let delta = match key {
            VirtualKeyCode::Up => Point::new(0, -1),
            VirtualKeyCode::Down => Point::new(0, 1),
            VirtualKeyCode::Left => Point::new(-1, 0),
            VirtualKeyCode::Right => Point::new(1, 0),
            _ => Point::zero(),
        };

        // get player entity and new position
        let (player_entity, player_new_position) = <(Entity, &Point)>::query()
            .filter(component::<Player>())
            .iter(ecs)
            .find_map(|(entity, pos)| Some((*entity, *pos + delta)))
            .expect("Error");

        // a flag to indicate whether to attack enemies, will be used later
        let mut fight = false;

        // fight each enemy if the enemy is at the position where player will move to
        <(Entity, &Enemy, &Point)>::query()
            .filter(component::<Health>())
            .iter(ecs)
            .for_each(|(monster, _, position)| {
                if player_new_position == *position {
                    // player attack monster
                    commands.push((
                        (),
                        WantsToAttack {
                            attacker: player_entity,
                            victim: *monster,
                        },
                    ));
                    fight = true;
                }
            });

        // move to next position if no enemy was found
        if !fight {
            // send a movement message to movement system.
            commands.push((
                (),
                WantsToMove {
                    entity: player_entity,
                    position: player_new_position,
                },
            ));
        }

        // to render something about player
        *turn_state = TurnState::PlayerTurn;
    }
}

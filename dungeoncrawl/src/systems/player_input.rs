use crate::prelude::*;

/// A system for player input

#[system]
#[write_component(Point)]
#[read_component(Player)]
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

        let mut players =
            <(Entity, &mut Point)>::query().filter(component::<Player>());

        players.iter_mut(ecs).for_each(|(player, pos)| {
            let new_position = *pos + delta;

            // send a movement message to movement system.
            commands.push((
                (),
                WantsToMove {
                    entity: *player,
                    position: new_position,
                },
            ));
        });

        *turn_state = TurnState::PlayerTurn;
    }
}

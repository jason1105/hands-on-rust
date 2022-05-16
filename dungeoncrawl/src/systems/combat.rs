use crate::prelude::*;

#[system]
#[read_component(WantsToAttack)]
#[write_component(Health)]
pub fn combat(ecs: &mut SubWorld, commands: &mut CommandBuffer) {
    <(Entity, &WantsToAttack)>::query()
        .iter(ecs)
        .map(|(entity, msg)| (*entity, msg.attacker, msg.victim)) // here is a clone, so the references of WantsToAttack is out of scope, we did not use it anymore
        .collect::<Vec<(Entity, Entity, Entity)>>() // doing this for complete iter() method above.
        .iter()
        .for_each(|(msg, attacker, victim)| {
            if let Ok(health) = ecs
                .entry_mut(*victim)
                .unwrap()
                .get_component_mut::<Health>()
            {
                health.current -= 1;
                println!(
                    "{:?} attacked {:?}, hp: {} / {}",
                    attacker, victim, health.current, health.max
                );
                if health.current < 0 {
                    commands.remove(*victim);
                    println!("{:?} has been removed", victim);
                }
            }
            commands.remove(*msg);
        });
}

/*


Entity 是 Copy, Clone 的, 上面的链式调用 .map(|(entity, msg)| (*entity, msg.victim)) 使得所有的引用值都被复制,
从而避免了 .for_each(|(msg, victim)| { 对 ecs 的借用,   所以 if let Ok(health) = ecs 可以再次借用 ecs.

    .collect::<Vec<(Entity, Entity)>>() 完成了 iter() 方法, 释放了对 ecs 的借用, 在 if let Ok(health) = ecs 这里再次借用也没有问题了.
 */

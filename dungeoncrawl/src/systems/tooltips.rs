use crate::prelude::*;

/// Tooltips system.
/// Show name and health info with monsters when hover mouse over them
#[system]
#[read_component(Name)]
#[read_component(Point)]
#[read_component(Health)]
pub fn tooltips(
    ecs: &SubWorld,
    #[resource] camera: &Camera,
    #[resource] mouse: &Point,
) {
    let mut batch = DrawBatch::new();
    let mut query = <(Entity, &Point, &Name)>::query();

    let offset = Point::new(camera.left_x, camera.top_y);
    let map_pos = *mouse + offset;
    query
        .iter(ecs)
        .filter(|(_, pos, _)| **pos == map_pos)
        .for_each(|(entity, _, name)| {
            let message = if let Ok(e) = ecs.entry_ref(*entity) {
                if let Ok(health) = e.get_component::<Health>() {
                    format!("{} : {} hp", &name.0, health.max)
                } else {
                    format!("{}", &name.0)
                }
            } else {
                format!("{}", &name.0)
            };

            let screen_pos = *mouse * 4;
            batch.target(2);
            batch.print(screen_pos, &message);
        });
    batch.submit(9999).expect("Batch error.");
}

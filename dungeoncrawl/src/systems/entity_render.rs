use crate::prelude::*;

/// A system to render all entities.
#[system]
#[read_component(Point)]
#[read_component(Render)]
pub fn entity_render(
    ecs: &mut SubWorld,
    #[resource] map: &Map,
    #[resource] camera: &Camera,
) {
    let mut batch = DrawBatch::new();

    batch.target(1);

    let camera_offset = Point::new(camera.left_x, camera.top_y);

    <(&Point, &Render)>::query()
        .iter(ecs)
        .for_each(|(p, render)| {
            if map.in_bound(*p) {
                batch.set(*p - camera_offset, render.color, render.glyph);
            }
        });

    batch.submit(5000).expect("Render entities error.");
}

use crate::prelude::*;

/// A system to render map
#[system]
pub fn map_render(#[resource] map: &Map, #[resource] camera: &Camera) {
    let mut batch = DrawBatch::new();
    batch.target(0);

    let camera_offset = Point::new(camera.left_x, camera.top_y);

    for y in camera.top_y..camera.bottom_y {
        for x in camera.left_x..camera.right_x {
            if let Some(tile_type) = map.type_on(Point::new(x, y)) {
                let map_point = Point::new(x, y);
                let glyph;
                match tile_type {
                    TileType::Floor => glyph = to_cp437('.'),
                    TileType::Wall => glyph = to_cp437('#'),
                }
                batch.set(
                    map_point - camera_offset,
                    ColorPair::new(WHITE, BLACK),
                    glyph,
                );
            }
        }
    }
    batch.submit(0).expect("Batch Render error.");
}

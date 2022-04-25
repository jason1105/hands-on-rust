// use crate::{camera, prelude::*};

// pub struct Player {
//     position: Point,
// }

// impl Player {
//     pub fn new(point: Point) -> Self {
//         Self { position: point }
//     }

//     pub fn render(&self, map: &Map, ctx: &mut BTerm, c: &Camera) {
//         ctx.set_active_console(1);
//         if map.in_bound(self.position) {
//             ctx.set(
//                 self.position.x - c.left_x,
//                 self.position.y - c.top_y,
//                 WHITE,
//                 BLACK,
//                 to_cp437('@'),
//             );
//         }
//     }

//     /// update player's position through arrow key
//     pub fn update(&mut self, map: &Map, ctx: &mut BTerm, c: &mut Camera) {
//         if let Some(key) = ctx.key {
//             let delta = match key {
//                 VirtualKeyCode::Up => Point::new(0, -1),
//                 VirtualKeyCode::Down => Point::new(0, 1),
//                 VirtualKeyCode::Left => Point::new(-1, 0),
//                 VirtualKeyCode::Right => Point::new(1, 0),
//                 _ => Point::zero(),
//             };

//             let new_position = self.position + delta;
//             if map.can_enter_tile(new_position) {
//                 self.position = new_position;
//                 c.on_player_move(new_position);
//             }
//         }
//     }
// }

use bracket_lib::prelude::*;

struct State {
    component: Option<Bounce>,
}

impl GameState for State {
    fn tick(&mut self, ctx: &mut BTerm) {
        ctx.cls();
        let (x, y) = self.change_position(ctx);
        ctx.print(x, y, "hello world.");
    }
}

impl State {
    fn change_position(&mut self, ctx: &BTerm) -> (u32, u32) {
        match &mut self.component {
            Some(bounce) => {
                match bounce.x_dir {
                    true => bounce.x += 2,
                    false => {
                        bounce.x = bounce.x.saturating_sub(2);
                    }
                }
                match bounce.y_dir {
                    true => bounce.y += 1,
                    false => {
                        bounce.y = bounce.y.saturating_sub(1);
                    }
                }

                match bounce.x {
                    80 => bounce.x_dir = false,
                    0 => bounce.x_dir = true,
                    _ => (),
                }
                match bounce.y {
                    50 => bounce.y_dir = false,
                    0 => bounce.y_dir = true,
                    _ => (),
                }

                (bounce.x, bounce.y)
            }
            None => {
                let _ = self.component.insert(Bounce {
                    x: 0,
                    x_dir: true,
                    y: 0,
                    y_dir: true,
                });
                (0, 0)
            }
        }
    }
}

struct Bounce {
    x: u32,
    x_dir: bool,
    y: u32,
    y_dir: bool,
}

fn main() -> BError {
    let context = BTermBuilder::simple80x50().with_title("flappy").build()?;
    main_loop(context, State { component: None })
}

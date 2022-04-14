use bracket_lib::prelude::*;

struct State {
    mode: GameMode, 
}

impl GameState for State {
    fn tick(&mut self, ctx: &mut BTerm) {
        match self.mode {
            GameMode::Menu => (self.main_menu(ctx)),
            GameMode::Playing => (self.play(ctx)),
            GameMode::End => (self.dead(ctx)),
        }
    }
}

impl State {
    fn new() -> State {
        State {
            mode: GameMode::Menu,
        }
    }
    fn main_menu(&mut self, ctx: &mut BTerm) {
        // TODO 
        ctx.cls();
        ctx.print_centered(5, "Welcom to Flappy Dragon");
        ctx.print_centered(8, "(P) Play Game");
        ctx.print_centered(10, "(Q) Quit Game");
        match ctx.key {
           Some(VirtualKeyCode::P) => self.mode = GameMode::Playing,
           Some(VirtualKeyCode::Q) => ctx.quitting = true,
           _ => ()

        }
    }

    fn play(&mut self, ctx: &mut BTerm) {
        // TODO
       self.mode = GameMode::End; 
    }

    fn dead(&mut self, ctx: &mut BTerm) {
        // TODO
        ctx.cls();
        ctx.print_centered(5, "You are dead.");
        ctx.print_centered(8, "(P) Play Again");
        ctx.print_centered(10, "(Q) Quit Game");
        match ctx.key {
           Some(VirtualKeyCode::P) => self.mode = GameMode::Playing,
           Some(VirtualKeyCode::Q) => ctx.quitting = true,
           _ => ()

        }
    }

    fn restart(&mut self, ctx: &mut BTerm) {
         // TODO
         self.mode = GameMode::Playing;
    }
}

enum GameMode {
    Menu,
    Playing,
    End,
}

fn main() -> BError {
    let context = BTermBuilder::simple80x50().with_title("flappy").build()?;
    main_loop(context, State::new())
}

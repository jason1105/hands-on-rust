use bracket_lib::prelude::*;

struct State {
    player: Player,
    frame_time: f32,
    obstacle: Obstacle,
    mode: GameMode,
    score: i32,
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
            player: Player::new(5, 25),
            frame_time: 0.0,
            obstacle: Obstacle::new(SCREEN_HEIGHT, 0),
            mode: GameMode::Menu,
            score: 0,
        }
    }
    fn main_menu(&mut self, ctx: &mut BTerm) {
        // TODO
        ctx.cls();
        ctx.print_centered(5, "Welcome to Flappy Dragon");
        ctx.print_centered(8, "(P) Play Game");
        ctx.print_centered(10, "(Q) Quit Game");
        match ctx.key {
            Some(VirtualKeyCode::P) => self.mode = GameMode::Playing,
            Some(VirtualKeyCode::Q) => ctx.quitting = true,
            _ => (),
        }
    }

    fn play(&mut self, ctx: &mut BTerm) {
        // TODO
        ctx.cls_bg(NAVY);
        ctx.print(0, 0, "Press SPACE to flap.");
        ctx.print(0, 1, format!("Score: {}", self.score));
        self.frame_time += ctx.frame_time_ms;
        if self.frame_time > FRAME_TIME {
            self.player.gravity_and_move();
            self.frame_time = 0.0;
        }

        if let Some(VirtualKeyCode::Space) = ctx.key {
            self.player.flap();
        }

        self.obstacle.render(ctx, self.player.x);
        self.player.render(ctx);

        if self.player.x > self.obstacle.x {
            self.score += 1;
            self.obstacle = Obstacle::new(self.player.x + SCREEN_WIDTH, self.score);
        }

        if (self.player.y > SCREEN_HEIGHT
            || self.obstacle.hit_obstacle(self.player.x, self.player.y))
        {
            self.mode = GameMode::End;
        }
    }

    fn dead(&mut self, ctx: &mut BTerm) {
        // TODO
        ctx.cls();
        ctx.print_centered(8, "Your are dead.");
        ctx.print_centered(10, format!("Your score: {}", self.score));
        ctx.print_centered(12, "(P) Play Again");
        ctx.print_centered(14, "(Q) Quit Game");
        match ctx.key {
            Some(VirtualKeyCode::P) => self.restart(ctx),
            Some(VirtualKeyCode::Q) => ctx.quitting = true,
            _ => (),
        }
    }

    fn restart(&mut self, ctx: &mut BTerm) {
        // TODO
        self.player = Player::new(5, 25);
        self.frame_time = 0.0;
        self.mode = GameMode::Playing;
        self.score = 0;
        self.obstacle = Obstacle::new(SCREEN_WIDTH, 0);
    }
}

struct Player {
    x: i32,
    y: i32,
    velocity: f32,
}

// I am a player backend, and responsible for behavior of players.
impl Player {
    fn new(x: i32, y: i32) -> Self {
        Player {
            x,
            y,
            velocity: 0.0,
        }
    }

    fn render(&mut self, ctx: &mut BTerm) {
        ctx.set(0, self.y, YELLOW, BLACK, to_cp437('@'));
    }

    fn gravity_and_move(&mut self) {
        if self.velocity < 2.0 {
            self.velocity += 0.2;
        }

        self.y += self.velocity as i32;
        self.x += 1;

        if self.y == 0 {
            self.y = 0;
        }
    }

    fn flap(&mut self) {
        self.velocity = -2.0;
    }
}

struct Obstacle {
    x: i32,     // where this obstacle put
    gap_y: i32, // position in obstacle where has a gap
    size: i32,  // size of gap
}

impl Obstacle {
    fn new(x: i32, score: i32) -> Self {
        let mut random = RandomNumberGenerator::new();
        Obstacle {
            x,
            gap_y: random.range(10, 40),
            size: i32::max(2, 20 - score),
        }
    }

    fn render(&self, ctx: &mut BTerm, player_x: i32) {
        let screen_x = self.x - player_x; // player is moving, so obstacle is moving too.
        for y in (0..self.gap_y - self.size / 2) {
            ctx.set(screen_x, y, RED, BLACK, to_cp437('|'));
        }
        for y in (self.gap_y + self.size / 2..SCREEN_HEIGHT) {
            ctx.set(screen_x, y, RED, BLACK, to_cp437('|'));
        }
    }

    fn hit_obstacle(&self, player_x: i32, player_y: i32) -> bool {
        let gap_top = self.gap_y - self.size / 2;
        let gap_end = self.gap_y + self.size / 2;
        if player_x == self.x
            && ((player_y > 0 && player_y < gap_top)
                || (player_y < SCREEN_HEIGHT && player_y > gap_end))
        {
            return true;
        }
        false
    }
}

enum GameMode {
    Menu,
    Playing,
    End,
}

const SCREEN_HEIGHT: i32 = 50;
const SCREEN_WIDTH: i32 = 80;
const FRAME_TIME: f32 = 75.0;

fn main() -> BError {
    let context = BTermBuilder::simple80x50().with_title("flappy").build()?;
    main_loop(context, State::new())
}

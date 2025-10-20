use bracket_lib::prelude::*;
fn main() -> BError {
    let context=BTermBuilder::simple80x50().with_title("Flappy Dragon").build()?;
    main_loop(context,State::new())?;
    Ok(())
}

enum GameMode {
    Menu,
    Playing,
    End,
}

struct State {
   mode: GameMode,
}

impl GameState for State {
    fn tick(&mut self, ctx: &mut BTerm) {
        // Clear first, then draw the current screen
        ctx.cls();
        match self.mode {
            GameMode::Menu=> self.draw_menu(ctx),
            GameMode::Playing => self.draw_playing(ctx),
            GameMode::End => self.draw_end(ctx),
        }
    }
  
}

impl State {    
    fn new() -> Self {
        Self { mode: GameMode::Menu }
    }

    
    fn draw_menu(&mut self, ctx: &mut BTerm) {
        ctx.cls();
        ctx.print(1, 1, "Welcome to Flappy Dragon");
        ctx.print(1, 3, "Press P to start");
        ctx.print(1, 5, "Press Q to quit");

        if let Some(key)=ctx.key {
            match key {
                VirtualKeyCode::P => self.restart(),
                VirtualKeyCode::Q => ctx.quitting = true,
                _ => {}
            }
        }
    }
    fn draw_playing(&mut self, ctx: &mut BTerm) {
        ctx.print(1, 1, "Playing");
        //todo
        self.mode=GameMode::End;
    }
    fn restart(&mut self) {
        self.mode=GameMode::Playing;
    }
    fn draw_end(&mut self,  ctx: &mut BTerm) {
        ctx.print(1, 1, "Game Over");
    }
}

struct Player {
    x: i32,
    y: i32,
    velocity: f32,
}
impl Player {
    fn new(x: i32, y: i32) -> Self {
        Self { x, y, velocity: 0.0 }
    }
    fn render(&mut self, ctx: &mut BTerm) {
        ctx.set(0, self.y, YELLOW, BLACK,to_cp437('🐉'));
    }
    fn gravity_and_move(&mut self, ctx: &mut BTerm) {
        if self.velocity < 2.0 {
            self.velocity += 1.0;
        }
        self.y += self.velocity as i32;
        self.x += 1;
        if self.y < 0 {
            self.y = 0;
        }
    }
    fn flap(&mut self) {
        self.velocity = -2.0;
    }
}
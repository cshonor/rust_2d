use bracket_lib::prelude::*;
fn main() -> BError {
    
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
        match self.mode {
            GameMode::Menu=> self.draw_menu(ctx),
            GameMode::Playing => self.draw_playing(ctx),
            GameMode::End => self.draw_end(ctx),
        }
        ctx.cls();
        ctx.print(1, 1, "Hello, world!");
    }
  
}

impl State {    
    fn new() -> Self {
        Self { mode: GameMode::Menu }
    }

    
    fn draw_menu(&mut self, ctx: &mut BTerm) {
        ctx.print(1, 1, "Welcome to Flappy Dragon");
        ctx.print(1, 3, "Press SPACE to start");
    }
    fn draw_playing(&mut self, ctx: &mut BTerm) {
        ctx.print(1, 1, "Playing");
    }
    fn draw_end(&mut self,  ctx: &mut BTerm) {
        ctx.print(1, 1, "Game Over");
    }
}


let context=BTermBuilder::simple80x50().with_title("Flappy Dragon").build()?;


main_loop(context,State：：new());


}
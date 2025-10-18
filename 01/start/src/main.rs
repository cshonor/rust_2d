use bracket_lib::prelude::*;
fn main() -> BError {
    
}

struct state {
   
}
impl GameState for state {
    fn tick(&mut self, ctx: &mut BTerm) {
        ctx.cls();
        ctx.print(1, 1, "Hello, world!");
    }
}

let context=BTermBuilder::simple80x50().with_title("Flappy Dragon").build()?;
use bracket_lib::prelude::*;
fn main() {
    
}

struct state {
   
}
impl GameState for state {
    fn tick(&mut self, ctx: &mut BTerm) {
        ctx.cls();
        ctx.print(1, 1, "Hello, world!");
    }
}

mod map;

mod prelude{
   pub  use bracket_lib::prelude::*;
   pub use crate::map::*;
   pub use legion::*;
}
use map::*
fn main() {
    println!("Hello, world!");
}

use bracket_lib::prelude::*;

struct State {}
impl GameState for State {
    fn tick(&mut self, ctx: &mut BTerm) {
        ctx.cls();
        ctx.print(1, 1, "Test");
    }
}

pub const WINDOW_WIDTH: i32 = 80;
pub const WINDOW_HEIGHT: i32 = 60;

fn main() {
    let context = BTermBuilder::simple(WINDOW_WIDTH, WINDOW_HEIGHT)
        .with_title("TCC")
        .build();
    let gs = State{ };
    bracket_lib::prelude::main_loop(context, gs);
}


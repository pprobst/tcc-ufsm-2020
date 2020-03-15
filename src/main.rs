use bracket_lib::prelude::*;
use specs::prelude::*;

mod state;
use state::State;
mod components;
pub use components::*;
mod player;

pub const WINDOW_WIDTH: i32 = 80;
pub const WINDOW_HEIGHT: i32 = 60;

fn main() {
    let context = BTermBuilder::simple(WINDOW_WIDTH, WINDOW_HEIGHT)
        .with_title("TCC")
        .build();

    let mut world = World::new();

    // Register the components.
    world.register::<Position>();
    world.register::<Renderable>();
    world.register::<Player>();

    let mut gs = State::new(world);

    let player = gs.ecs
            .create_entity()
            .with(Position { x: WINDOW_HEIGHT/2, y: WINDOW_WIDTH/2 })
            .with(Renderable {
                glyph: to_cp437('@'),
                fg: RGB::named(WHITE),
                bg: RGB::named(BLACK),
            })
            .with(Player{})
            .build();

    gs.ecs.insert(Point::new(0, 0));
    gs.ecs.insert(player);

    bracket_lib::prelude::main_loop(context, gs);
}


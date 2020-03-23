use bracket_lib::prelude::*;
use specs::prelude::*;

mod state;
use state::State;
mod components;
pub use components::*;
mod renderer;
mod player;
mod systems;
pub mod map_gen;
mod spawner;

pub const WINDOW_WIDTH: i32 = 80;
pub const WINDOW_HEIGHT: i32 = 60;
pub const POSTPROCESS: bool = true;

fn main() {
    let mut context = BTermBuilder::simple(WINDOW_WIDTH, WINDOW_HEIGHT)
        .with_title("TCC")
        //.with_font("terminal_10x16.png", 10, 16)
        .build();
    context.with_post_scanlines(POSTPROCESS);

    let mut world = World::new();

    // Register the components (see components.rs).
    world.register::<Position>();
    world.register::<Renderable>();
    world.register::<Player>();
    world.register::<Mob>();
    world.register::<Fov>();
    world.register::<Blocker>();

    let mut gs = State::new(world, POSTPROCESS);

    gs.ecs.insert(map_gen::Map::new(80, 80));
    let map = gs.generate_map();

    spawner::spawn_map(&mut gs.ecs, &map);

    bracket_lib::prelude::main_loop(context, gs);
}


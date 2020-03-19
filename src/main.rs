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

pub const WINDOW_WIDTH: i32 = 80;
pub const WINDOW_HEIGHT: i32 = 60;
pub const POSTPROCESS: bool = true;

fn main() {
    let mut context = BTermBuilder::simple(WINDOW_WIDTH, WINDOW_HEIGHT)
        .with_title("TCC")
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

    let player = gs.ecs
            .create_entity()
            .with(Position { x: WINDOW_HEIGHT/2, y: WINDOW_WIDTH/2 })
            .with(Renderable {
                glyph: to_cp437('@'),
                fg: RGB::named(WHITE),
                bg: RGB::named(BLACK),
            })
            .with(Player{})
            .with(Fov { range: 50, visible_pos: Vec::new(), dirty: true })
            .build();

    gs.ecs.insert(map_gen::Map::new(80, 60));
    gs.ecs.insert(Point::new(0, 0));
    gs.ecs.insert(player);

    gs.generate_map();

    bracket_lib::prelude::main_loop(context, gs);
}


use bracket_lib::prelude::*;
use specs::prelude::*;

mod state;
use state::{State, RunState};
mod components;
pub use components::*;
mod utils;
mod input;
mod renderer;
mod killer;
mod ui;
mod log;
mod player;
mod systems;
mod map_gen;
mod spawner;

pub const WINDOW_WIDTH: i32 = 80;
pub const WINDOW_HEIGHT: i32 = 60;
//pub const WINDOW_WIDTH: i32 = 79;
//pub const WINDOW_HEIGHT: i32 = 39;
//pub const X_OFFSET: i32 = 13; // Left box
//pub const Y_OFFSET: i32 = 5;  // Bottom box
pub const X_OFFSET: i32 = 18; // Left box
pub const Y_OFFSET: i32 = 7;  // Bottom box
pub const TILE_WIDTH: i32 = 10;
pub const TILE_HEIGHT: i32 = 12;

pub const POSTPROCESS: bool = false;

//embedded_resource!(TILE_FONT, "../resources/vga8x16.png");

fn main() -> BError {
    //link_resource!(TILE_FONT, "resources/terminal_12x12");
    let term = BTermBuilder::new()
        .with_dimensions(WINDOW_WIDTH, WINDOW_HEIGHT)
        .with_title("TCC")
        .with_tile_dimensions(TILE_WIDTH, TILE_HEIGHT)
        .with_font("Nice-curses-10x12.png", 10, 12)
        .with_sparse_console(WINDOW_WIDTH, WINDOW_HEIGHT-Y_OFFSET, "Nice-curses-10x12.png")
        //.with_sparse_console(WINDOW_WIDTH, WINDOW_HEIGHT/2, "terminal_12x12.png")
        //.with_fullscreen(true)
        .build()?;
    /*
    let term = BTermBuilder::simple80x50()
        .with_title("TCC")
        .build();
    */

    let mut world = World::new();

    // Register the components (see components.rs).
    world.register::<Position>();
    world.register::<Renderable>();
    world.register::<Player>();
    world.register::<Mob>();
    world.register::<Name>();
    world.register::<Fov>();
    world.register::<Blocker>();
    world.register::<Health>();
    world.register::<BaseStats>();
    world.register::<SufferDamage>();
    world.register::<MeleeAttack>();
    world.register::<MissileAttack>();
    world.register::<MeleeWeapon>();
    world.register::<MissileWeapon>();
    world.register::<Target>();

    // Create game state.
    let mut game_state = State::new(world, POSTPROCESS);

    // Insert map into the ECS and generate it.
    game_state.ecs.insert(map_gen::Map::new(80, 80));
    let map = game_state.generate_map();

    // Spawn entities on the map.
    spawner::spawn_map(&mut game_state.ecs, &map);

    // Insert initial state (Start) on the ECS.
    game_state.ecs.insert(RunState::Start);

    // Insert the Log into the ECS.
    let mut log = log::Log::new();
    log.add("Test test test 1", RGB::named(WHITE));
    game_state.ecs.insert(log);

    bracket_lib::prelude::main_loop(term, game_state)
}


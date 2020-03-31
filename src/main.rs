use bracket_lib::prelude::*;
use specs::prelude::*;

mod state;
use state::{State, RunState};
mod components;
pub use components::*;
mod utils;
mod input;
mod renderer;
mod ui;
mod killer;
mod player;
mod systems;
mod map_gen;
mod spawner;

//pub const WINDOW_WIDTH: i32 = 75;
//pub const WINDOW_HEIGHT: i32 = 39;
pub const WINDOW_WIDTH: i32 = 80;
pub const WINDOW_HEIGHT: i32 = 60;
//pub const X_OFFSET: i32 = 13; // Left box
//pub const Y_OFFSET: i32 = 5;  // Bottom box
pub const X_OFFSET: i32 = 18; // Left box
pub const Y_OFFSET: i32 = 8;  // Bottom box
pub const TILE_WIDTH: i32 = 16;
pub const TILE_HEIGHT: i32 = 16;

pub const POSTPROCESS: bool = false;

embedded_resource!(TILE_FONT, "../resources/terminal_12x12.png");

fn main() {
    link_resource!(TILE_FONT, "resources/terminal_12x12");
    let term = BTermBuilder::new()
        .with_dimensions(WINDOW_WIDTH, WINDOW_HEIGHT)
        .with_title("TCC")
        .with_tile_dimensions(TILE_WIDTH, TILE_HEIGHT)
        .with_font("terminal_12x12.png", 12, 12)
        .with_sparse_console(WINDOW_WIDTH, WINDOW_HEIGHT-Y_OFFSET, "terminal_12x12.png")
        //.with_fullscreen(true)
        .build();
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

    // Generate map.
    game_state.ecs.insert(map_gen::Map::new(80, 80));
    let map = game_state.generate_map();

    // Spawn entities on the map.
    spawner::spawn_map(&mut game_state.ecs, &map);

    // Insert initial state (Start) on the ECS.
    game_state.ecs.insert(RunState::Start);

    bracket_lib::prelude::main_loop(term, game_state);
}


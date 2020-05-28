use bracket_lib::prelude::*;
use specs::prelude::*;

mod state;
use state::{RunState, State};
mod components;
pub use components::*;
mod input;
mod killer;
mod log;
mod map_gen;
mod player;
mod renderer;
mod rexloader;
mod spawner;
mod systems;
mod ui;
mod utils;

pub const X_OFFSET: i32 = 20; // Left box
pub const Y_OFFSET: i32 = 10; // Bottom box
pub const WINDOW_WIDTH: i32 = 80;
pub const WINDOW_HEIGHT: i32 = 60;
//pub const WINDOW_HEIGHT: i32 = 80+Y_OFFSET;
pub const TILE_WIDTH: i32 = 16;
pub const TILE_HEIGHT: i32 = 16;

//embedded_resource!(TILE_FONT, "../resources/vga8x16.png");

fn main() -> BError {
    //link_resource!(TILE_FONT, "resources/terminal_12x12");
    let term = BTermBuilder::new()
        .with_dimensions(WINDOW_WIDTH, WINDOW_HEIGHT)
        .with_title("TCC")
        .with_tile_dimensions(TILE_WIDTH, TILE_HEIGHT)
        .with_font("Sapphos-square-16x16.png", 16, 16)
        .with_font("Anikki-square-16x16.png", 16, 16)
        .with_simple_console(WINDOW_WIDTH, WINDOW_HEIGHT, "Sapphos-square-16x16.png")
        .with_sparse_console(WINDOW_WIDTH, WINDOW_HEIGHT, "Anikki-square-16x16.png")
        .with_fullscreen(true)
        .with_fps_cap(60.0)
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
    world.register::<Item>();
    world.register::<Consumable>();
    world.register::<PickupItem>();
    world.register::<DropItem>();
    world.register::<ConsumeItem>();
    world.register::<InBackpack>();
    world.register::<SelectedItem>();

    // Create game state.
    let mut game_state = State::new(world);

    //game_state.ecs.insert(rexloader::RexResource::new("../rex_resources/dungeon80x60.xp"));
    rexloader::load_dungeons();

    // Insert map into the ECS and generate it.
    let (height, width) = (80, 60);
    game_state.ecs.insert(map_gen::Map::new(height, width));
    let map = game_state.generate_map(height, width);

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

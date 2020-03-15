use bracket_lib::prelude::{RGB};
use specs::{prelude::*, Component};

#[derive(Component)]
pub struct Position {
    pub x: i32,
    pub y: i32,
}

#[derive(Component)]
pub struct Renderable {
    pub glyph: u8,
    pub fg: RGB,
    pub bg: RGB,
}

#[derive(Component)]
pub struct Player {}

#[derive(Component)]
// Entities with this component will "block" movement over them.
// After all, you can't walk over enemies (unless you're flying!).
pub struct Blocker {}

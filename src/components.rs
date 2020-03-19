use bracket_lib::prelude::{RGB, Point};
use specs::{prelude::*, Component};
//use std::collections::HashSet;

/*#[derive(Component)]
pub struct Position {
    pub x: i32,
    pub y: i32,
}*/

pub type Position = Point;

#[derive(Component)]
pub struct Renderable {
    pub glyph: u8,
    pub fg: RGB,
    pub bg: RGB,
}

#[derive(Component)]
pub struct Player {}

#[derive(Component)]
// Enemies & NPCs.
pub struct Mob {}

#[derive(Component, PartialEq)]
// An entity's field of view (fov).
pub struct Fov {
    pub range: i32,
    //pub visible_tiles: HashSet<Position>,
    //pub revealed_tiles: HashSet<Position>,
    pub visible_tiles: Vec<Position>,
    pub revealed_tiles: Vec<Position>,
    pub dirty: bool
}

#[derive(Component)]
// Entities with this component will "block" movement over them.
// After all, you can't walk over enemies (unless you're flying!).
pub struct Blocker {}

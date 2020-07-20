use super::Renderable;
use serde::Deserialize;
//use std::collections::HashMap;

#[derive(Deserialize, Debug)]
pub struct Mob {
    pub name: String,
    pub renderable: Option<Renderable>,
    pub fov_range: i32,
    pub blocker: bool,
    pub stats: Stats,
}

#[derive(Deserialize, Debug)]
pub struct Stats {
    pub hp: i32,
    pub max_hp: i32,
    pub attack: i32,
    pub defense: i32,
}

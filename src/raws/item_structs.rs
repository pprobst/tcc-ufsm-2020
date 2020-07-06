use serde::Deserialize;
use std::collections::HashMap;

#[derive(Deserialize, Debug)]
pub struct Item {
    pub name: String,
    pub renderable: Option<Renderable>,
    pub consumable: Option<Consumable>,
}

#[derive(Deserialize, Debug)]
pub struct Renderable {
    pub glyph: char,
    pub fg: String,
    pub bg: String,
    pub layer: i32,
}

#[derive(Deserialize, Debug)]
pub struct Consumable {
    pub effects: HashMap<String, String>,
}

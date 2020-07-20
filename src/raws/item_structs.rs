use super::Renderable;
use serde::Deserialize;
use std::collections::HashMap;

#[derive(Deserialize, Debug)]
pub struct Item {
    pub name: String,
    pub renderable: Option<Renderable>,
    pub consumable: Option<Consumable>,
    pub equipable: Option<Equipable>,
    pub melee: Option<Melee>,
}

#[derive(Deserialize, Debug)]
pub struct Consumable {
    pub effects: HashMap<String, i32>,
}

#[derive(Deserialize, Debug)]
pub struct Melee {
    pub damage: i32,
}

#[derive(Deserialize, Debug)]
pub struct Equipable {
    pub slot: String,
}

use super::Renderable;
use serde::Deserialize;
use std::collections::HashMap;

#[derive(Deserialize, Debug)]
pub struct Item {
    pub name: String,
    pub descr: String,
    pub renderable: Option<Renderable>,
    pub consumable: Option<Consumable>,
    pub equipable: Option<Equipable>,
    pub melee: Option<Melee>,
    pub armor: Option<Armor>,
}

#[derive(Deserialize, Debug)]
pub struct Consumable {
    pub effects: HashMap<String, i32>,
}

#[derive(Deserialize, Debug)]
pub struct Melee {
    pub damage: i32,
    pub class: String,
}

#[derive(Deserialize, Debug)]
pub struct Armor {
    pub defense: i32,
}

#[derive(Deserialize, Debug)]
pub struct Equipable {
    pub slot: String,
}

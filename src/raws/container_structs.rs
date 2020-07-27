use super::Renderable;
use serde::Deserialize;
use std::collections::HashMap;

#[derive(Deserialize, Debug)]
pub struct Container {
    pub name: String,
    pub descr: String,
    pub renderable: Option<Renderable>,
    pub max_items: u8,
    pub tiers: Vec<u8>,
}

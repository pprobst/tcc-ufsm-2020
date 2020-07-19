use super::Raws;
use crate::components::{Consumable, Item, Name, Position, Renderable};
use crate::utils::colors::color;
use bracket_lib::prelude::{to_cp437, ColorPair};
use specs::prelude::*;
use std::collections::HashMap;

#[derive(Debug)]
pub struct RawMaster {
    pub raws: Raws,
    item_index: HashMap<String, usize>,
}

impl RawMaster {
    pub fn empty() -> Self {
        RawMaster {
            raws: Raws { items: Vec::new() },
            item_index: HashMap::new(),
        }
    }

    pub fn load(&mut self, raws: Raws) {
        self.raws = raws;

        for (i, item) in self.raws.items.iter().enumerate() {
            self.item_index.insert(item.name.clone(), i);
        }
    }
}

pub fn spawn_item(
    name: &str,
    x: i32,
    y: i32,
    entity: EntityBuilder,
    raws: &RawMaster,
) -> Option<Entity> {
    if raws.item_index.contains_key(name) {
        let item = &raws.raws.items[raws.item_index[name]];
        let mut ent = entity;

        ent = ent.with(Name {
            name: item.name.clone(),
        });
        ent = ent.with(Position { x, y });
        ent = ent.with(Item {});

        if let Some(renderable) = &item.renderable {
            ent = ent.with(Renderable {
                glyph: to_cp437(renderable.glyph),
                color: ColorPair::new(color(&renderable.fg, 1.0), color(&renderable.bg, 1.0)),
                layer: renderable.layer as u8,
            });
        }

        if let Some(consumable) = &item.consumable {
            for effect in consumable.effects.iter() {
                let effname = effect.0.as_str();
                match effname {
                    "heal" => {
                        ent = ent.with(Consumable {
                            heal: effect.1.parse::<i32>().unwrap(),
                        });
                    }
                    _ => return None,
                }
            }
        }

        Some(ent.build());
    }

    None
}

use super::{Colorscheme, Raws, item_structs};
use bracket_lib::prelude::{ColorPair, to_cp437};
use crate::components::{Position, Item, Consumable, Renderable, Name};
use crate::utils::colors::color;
use specs::prelude::*;
use std::collections::HashMap;

#[derive(Debug)]
pub struct RawMaster {
    pub raws: Raws,
    item_index: HashMap<String, usize>,
    pub color_index: HashMap<String, usize>,
    curr_colorscheme: usize,
}

impl RawMaster {
    pub fn empty() -> Self {
        RawMaster {
            raws: Raws {
                items: Vec::new(),
                colorschemes: Vec::new(),
            },
            item_index: HashMap::new(),
            color_index: HashMap::new(),
            curr_colorscheme: 0,
        }
    }

    pub fn load(&mut self, raws: Raws) {
        self.raws = raws;
        //self.item_index = HashMap::new();

        for (i, item) in self.raws.items.iter().enumerate() {
            self.item_index.insert(item.name.clone(), i);
        }
        for (i, color) in self.raws.colorschemes.iter().enumerate() {
            self.color_index.insert(color.name.clone(), i);
        }
    }

    pub fn set_curr_colorscheme(&mut self, colorscheme: &str) {
        self.curr_colorscheme = self.color_index[colorscheme];
    }

    pub fn get_curr_colorscheme(&self) -> &Colorscheme {
        &self.raws.colorschemes[self.curr_colorscheme]
    }

    /*
    pub fn get_ent_color(&self) -> &str {
        
    }
    */
}

pub fn spawn_item(name: &str, x: i32, y: i32, entity: EntityBuilder, raws: &RawMaster) -> Option<Entity> {
    if raws.item_index.contains_key(name) {
        let item = &raws.raws.items[raws.item_index[name]];
        let mut ent = entity;
        println!("{}", item.name);
        ent = ent.with(Name {name: item.name.clone()});
        ent = ent.with(Position {x, y});
        ent = ent.with(Item {});

        println!("AQUI 1");
        if let Some(renderable) = &item.renderable {
            println!("{}", &renderable.fg);
            ent = ent.with(Renderable {
                glyph: to_cp437(renderable.glyph),
                // Ok, vou ter que criar um Raw novo aqui porque não posso acessar o mesmo por
                // causa do lock no mutex, óbvio...
                color: ColorPair::new(color(&renderable.fg, 1.0), color(&renderable.bg, 1.0)),
                layer: renderable.layer as u8,
            });
        }
        println!("AQUI 2");

        if let Some(consumable) = &item.consumable {
            for effect in consumable.effects.iter() {
                let effname = effect.0.as_str();
                match effname {
                    "heal" => { ent = ent.with(Consumable { heal: effect.1.parse::<i32>().unwrap(), }); },
                    _ => return None
                }
            }
        }

        println!("AQUI FIM!!!");
        Some(ent.build());
    }

    None
}

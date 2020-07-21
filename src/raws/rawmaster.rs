use super::{common_structs, Raws};
use crate::components::{
    BaseStats, Blocker, Consumable, Description, EquipSlot, Equipable, Fov, Health, Item,
    MeleeWeapon, Mob, Name, Position, Renderable,
};
use crate::utils::colors::color;
use bracket_lib::prelude::{to_cp437, ColorPair};
use specs::prelude::*;
use std::collections::HashMap;

#[derive(Debug)]
pub struct RawMaster {
    pub raws: Raws,
    item_index: HashMap<String, usize>,
    mob_index: HashMap<String, usize>,
}

impl RawMaster {
    pub fn empty() -> Self {
        RawMaster {
            raws: Raws {
                items: Vec::new(),
                mobs: Vec::new(),
            },
            item_index: HashMap::new(),
            mob_index: HashMap::new(),
        }
    }

    pub fn load(&mut self, raws: Raws) {
        self.raws = raws;

        for (i, item) in self.raws.items.iter().enumerate() {
            self.item_index.insert(item.name.clone(), i);
        }

        for (i, mob) in self.raws.mobs.iter().enumerate() {
            self.mob_index.insert(mob.name.clone(), i);
        }
    }

    pub fn get_renderable(&self, name: &str) -> &Option<common_structs::Renderable> {
        if self.item_index.contains_key(name) {
            return &self.raws.items[self.item_index[name]].renderable;
        }

        if self.mob_index.contains_key(name) {
            return &self.raws.mobs[self.mob_index[name]].renderable;
        }

        &None
    }
}

fn set_renderable(render: &common_structs::Renderable) -> Renderable {
    Renderable {
        glyph: to_cp437(render.glyph),
        color: ColorPair::new(color(&render.fg, 1.0), color(&render.bg, 1.0)),
        layer: render.layer as u8,
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
        ent = ent.with(Description {
            descr: item.descr.clone(),
        });
        ent = ent.with(Position { x, y });
        ent = ent.with(Item {});

        if let Some(renderable) = &item.renderable {
            ent = ent.with(set_renderable(renderable));
        }

        if let Some(consumable) = &item.consumable {
            for effect in consumable.effects.iter() {
                let effname = effect.0.as_str();
                match effname {
                    "heal" => {
                        ent = ent.with(Consumable { heal: *effect.1 });
                    }
                    _ => return None,
                }
            }
        }

        if let Some(equip) = &item.equipable {
            match equip.slot.as_str() {
                "weapon1" => {
                    ent = ent.with(Equipable {
                        slot: EquipSlot::Weapon1,
                    })
                }
                _ => return None,
            }
        }

        if let Some(melee) = &item.melee {
            ent = ent.with(MeleeWeapon {
                base_damage: melee.damage,
            })
        }

        Some(ent.build());
    }

    None
}

pub fn spawn_mob(
    name: &str,
    x: i32,
    y: i32,
    entity: EntityBuilder,
    raws: &RawMaster,
) -> Option<Entity> {
    if raws.mob_index.contains_key(name) {
        let mob = &raws.raws.mobs[raws.mob_index[name]];
        let mut ent = entity;

        ent = ent.with(Mob {});
        ent = ent.with(Name {
            name: mob.name.clone(),
        });
        ent = ent.with(Description {
            descr: mob.descr.clone(),
        });
        ent = ent.with(Position { x, y });
        ent = ent.with(Fov {
            range: mob.fov_range,
            dirty: true,
            visible_pos: Vec::new(),
        });
        if mob.blocker {
            ent = ent.with(Blocker {});
        }
        ent = ent.with(BaseStats {
            health: Health {
                max_hp: mob.stats.max_hp,
                hp: mob.stats.hp,
            },
            defense: mob.stats.defense,
            attack: mob.stats.attack,
            god: false,
        });

        if let Some(renderable) = &mob.renderable {
            ent = ent.with(set_renderable(renderable));
        }

        Some(ent.build());
    }

    None
}

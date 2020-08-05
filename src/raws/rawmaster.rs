use super::{common_structs, Raws};
use crate::components::{
    Armor, BaseStats, Blocker, Consumable, Container, Description, EquipSlot, Equipable, Fov,
    Health, Item, MeleeWeapon, MeleeWeaponClass, Mob, Name, Position, Renderable,
};
use crate::utils::colors::color;
use bracket_lib::prelude::{to_cp437, ColorPair, RandomNumberGenerator};
use specs::prelude::*;
use std::collections::HashMap;

#[derive(Debug)]
pub struct RawMaster {
    pub raws: Raws,
    item_index: HashMap<String, usize>,
    container_index: HashMap<String, usize>,
    furniture_index: HashMap<String, usize>,
    mob_index: HashMap<String, usize>,
}

impl RawMaster {
    pub fn empty() -> Self {
        RawMaster {
            raws: Raws {
                items: Vec::new(),
                containers: Vec::new(),
                furnitures: Vec::new(),
                mobs: Vec::new(),
            },
            item_index: HashMap::new(),
            container_index: HashMap::new(),
            furniture_index: HashMap::new(),
            mob_index: HashMap::new(),
        }
    }

    pub fn load(&mut self, raws: Raws) {
        self.raws = raws;

        for (i, item) in self.raws.items.iter().enumerate() {
            self.item_index.insert(item.name.clone(), i);
        }
        for (i, container) in self.raws.containers.iter().enumerate() {
            self.container_index.insert(container.name.clone(), i);
        }
        for (i, furniture) in self.raws.furnitures.iter().enumerate() {
            self.furniture_index.insert(furniture.name.clone(), i);
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

pub fn get_random_possible_equips(
    name: &str,
    raws: &RawMaster,
    rng: &mut RandomNumberGenerator,
) -> Option<Vec<String>> {
    if raws.mob_index.contains_key(name) {
        let mut equips: Vec<String> = Vec::new();
        let mob = &raws.raws.mobs[raws.mob_index[name]];
        if let Some(eq) = &mob.equips {
            if let Some(wpn) = &eq.weapons {
                equips.push(rng.random_slice_entry(wpn).unwrap().to_string());
            }
            if let Some(head) = &eq.head {
                equips.push(rng.random_slice_entry(head).unwrap().to_string());
            }
            if let Some(torso) = &eq.torso {
                equips.push(rng.random_slice_entry(torso).unwrap().to_string());
            }
            if let Some(hds) = &eq.hands {
                equips.push(rng.random_slice_entry(hds).unwrap().to_string());
            }
            if let Some(lgs) = &eq.legs {
                equips.push(rng.random_slice_entry(lgs).unwrap().to_string());
            }
            if let Some(feet) = &eq.feet {
                equips.push(rng.random_slice_entry(feet).unwrap().to_string());
            }
            if let Some(bck) = &eq.back {
                equips.push(rng.random_slice_entry(bck).unwrap().to_string());
            }
            if let Some(flt) = &eq.floating {
                equips.push(rng.random_slice_entry(flt).unwrap().to_string());
            }
        }

        return Some(equips);
    }

    None
}

pub fn get_items_tier(tier: u8, raws: &RawMaster) -> Vec<String> {
    let items = &raws.raws.items;
    items
        .iter()
        .filter(|x| x.tier == tier)
        .map(|x| x.name.clone())
        .collect::<Vec<String>>()
}

pub fn spawn_container(
    name: &str,
    pos: Position,
    entity: EntityBuilder,
    raws: &RawMaster,
) -> Option<Entity> {
    if raws.container_index.contains_key(name) {
        let container = &raws.raws.containers[raws.container_index[name]];
        let mut ent = entity;

        ent = ent.with(Name {
            name: container.name.clone(),
        });
        ent = ent.with(Description {
            descr: container.descr.clone(),
        });
        ent = ent.with(Position { x: pos.x, y: pos.y });
        ent = ent.with(Blocker {});
        ent = ent.with(Container {
            tiers: container.tiers.clone(),
            max_items: container.max_items,
        });

        if let Some(renderable) = &container.renderable {
            ent = ent.with(set_renderable(renderable));
        }

        return Some(ent.build());
    }

    None
}

pub fn spawn_item(
    name: &str,
    position: Option<Position>,
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
        ent = ent.with(Item { tier: item.tier });

        if let Some(pos) = position {
            ent = ent.with(Position { x: pos.x, y: pos.y });
        }
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
                "torso" => {
                    ent = ent.with(Equipable {
                        slot: EquipSlot::Torso,
                    })
                }
                "legs" => {
                    ent = ent.with(Equipable {
                        slot: EquipSlot::Legs,
                    })
                }
                _ => return None,
            }
        }
        if let Some(melee) = &item.melee {
            match melee.class.as_str() {
                "dagger" => {
                    ent = ent.with(MeleeWeapon {
                        base_damage: melee.damage,
                        class: MeleeWeaponClass::Dagger,
                    })
                }
                "axe" => {
                    ent = ent.with(MeleeWeapon {
                        base_damage: melee.damage,
                        class: MeleeWeaponClass::Axe,
                    })
                }

                _ => return None,
            }
        }
        if let Some(armor) = &item.armor {
            ent = ent.with(Armor {
                defense: armor.defense,
            })
        }

        return Some(ent.build());
    }

    None
}

pub fn spawn_furniture(
    name: &str,
    pos: Position,
    entity: EntityBuilder,
    raws: &RawMaster,
) -> Option<Entity> {
    if raws.furniture_index.contains_key(name) {
        let furniture = &raws.raws.furnitures[raws.furniture_index[name]];
        let mut ent = entity;
        ent = ent.with(Name {
            name: furniture.name.clone(),
        });
        ent = ent.with(Description {
            descr: furniture.descr.clone(),
        });
        ent = ent.with(Position { x: pos.x, y: pos.y });

        if let Some(_blocker) = &furniture.blocker {
            ent = ent.with(Blocker {});
        }

        if let Some(renderable) = &furniture.renderable {
            ent = ent.with(set_renderable(renderable));
        }

        Some(ent.build());
    }
    None
}

pub fn spawn_mob(
    name: &str,
    pos: Position,
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
        ent = ent.with(Position { x: pos.x, y: pos.y });
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

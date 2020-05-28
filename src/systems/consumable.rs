use crate::components::{InBackpack, Name, BaseStats, ConsumeItem, Consumable};
use crate::log::Log;
use bracket_lib::prelude::{RGB, WHITE};
use specs::prelude::*;

/*
 *
 * consumable.rs
 * -------------
 * Manages the consuming (food, potions, etc.) of items from the player's inventory.
 *
 */

pub struct ConsumableSystem {}

impl<'a> System<'a> for ConsumableSystem {
    type SystemData = (
        ReadExpect<'a, Entity>,
        ReadStorage<'a, Name>,
        ReadStorage<'a, Consumable>,
        WriteExpect<'a, Log>,
        WriteStorage<'a, ConsumeItem>,
        WriteStorage<'a, InBackpack>,
        WriteStorage<'a, BaseStats>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (player, name, consumable, mut log, mut to_consume, mut backpack, mut stats) = data;

        for c in to_consume.join() {
            let item = consumable.get(c.item).unwrap();
            let mut target_stats = stats.get_mut(c.target).unwrap();
            target_stats.health.hp = i32::min(target_stats.health.max_hp, target_stats.health.hp + item.heal);
            backpack.remove(c.item);

            if c.target == *player {
                log.add(
                    format!("You consume the {}, healing {} hp.", name.get(c.item).unwrap().name, item.heal),
                    RGB::named(WHITE),
                );
            }
        }

        to_consume.clear();
    }
}

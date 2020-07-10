use crate::components::{BaseStats, Consumable, ConsumeItem, InBackpack, InventoryCapacity, Name};
use crate::log::Log;
use crate::utils::colors::*;
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
        WriteStorage<'a, InventoryCapacity>,
        WriteStorage<'a, ConsumeItem>,
        WriteStorage<'a, InBackpack>,
        WriteStorage<'a, BaseStats>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (
            player,
            name,
            consumable,
            mut log,
            mut capacity,
            mut to_consume,
            mut backpack,
            mut stats,
        ) = data;

        let mut inventory_cap = capacity.get_mut(*player).unwrap();
        let white = color("BrightWhite", 1.0);

        for c in to_consume.join() {
            let item = consumable.get(c.item).unwrap();
            let mut target_stats = stats.get_mut(c.target).unwrap();
            target_stats.health.hp = i32::min(
                target_stats.health.max_hp,
                target_stats.health.hp + item.heal,
            );
            backpack.remove(c.item);

            if c.target == *player {
                log.add(
                    format!(
                        "You consume the {}, healing {} hp.",
                        name.get(c.item).unwrap().name,
                        item.heal
                    ),
                    white,
                );
                inventory_cap.curr -= 1;
            }
        }

        to_consume.clear();
    }
}

use crate::components::{DropItem, Inventory, InventoryCapacity, Name, Position};
use crate::log::Log;
use crate::utils::colors::*;
use specs::prelude::*;

/*
 *
 * item_drop.rs
 * ------------
 * Manages the dropping of items from the player's inventory.
 *
 */

pub struct ItemDropSystem {}

impl<'a> System<'a> for ItemDropSystem {
    type SystemData = (
        ReadExpect<'a, Entity>,
        ReadStorage<'a, Name>,
        WriteExpect<'a, Log>,
        WriteStorage<'a, InventoryCapacity>,
        WriteStorage<'a, Position>,
        WriteStorage<'a, DropItem>,
        WriteStorage<'a, Inventory>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (player, name, mut log, mut capacity, mut pos, mut drop, mut inventory) = data;
        let white = color("BrightWhite", 1.0);

        let mut inventory_cap = capacity.get_mut(*player).unwrap();
        for d in drop.join() {
            let drop_pos = pos.get(d.dropper).unwrap().clone();
            pos.insert(d.item, Position::new(drop_pos.x, drop_pos.y))
                .expect("Unable to insert position");
            inventory.remove(d.item);

            if d.dropper == *player {
                inventory_cap.curr -= 1;
                log.add(
                    format!("You drop the {}", name.get(d.item).unwrap().name),
                    white,
                );
            }
        }
        drop.clear();
    }
}

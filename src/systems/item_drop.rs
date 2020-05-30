use crate::components::{DropItem, InBackpack, Name, Position};
use crate::log::Log;
use bracket_lib::prelude::{RGB, WHITE};
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
        WriteStorage<'a, Position>,
        WriteStorage<'a, DropItem>,
        WriteStorage<'a, InBackpack>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (player, name, mut log, mut pos, mut drop, mut backpack) = data;

        for d in drop.join() {
            let drop_pos = pos.get(d.dropper).unwrap().clone();
            pos.insert(d.item, Position::new(drop_pos.x, drop_pos.y))
                .expect("Unable to insert position");
            backpack.remove(d.item);

            if d.dropper == *player {
                log.add(
                    format!("You drop the {}", name.get(d.item).unwrap().name),
                    RGB::named(WHITE),
                );
            }
        }
        drop.clear();
    }
}

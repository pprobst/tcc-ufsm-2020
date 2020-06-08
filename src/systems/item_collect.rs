use crate::components::{InBackpack, Name, CollectItem, Position};
use crate::log::Log;
use bracket_lib::prelude::{RGB, WHITE};
use specs::prelude::*;

/*
 *
 * item_collect.rs
 * ---------------
 * Manages the acquiring of items on the map, inserting them in the player's backpack.
 *
 */

pub struct ItemCollectSystem {}

impl<'a> System<'a> for ItemCollectSystem {
    type SystemData = (
        ReadExpect<'a, Entity>,
        ReadStorage<'a, Name>,
        WriteExpect<'a, Log>,
        WriteStorage<'a, Position>,
        WriteStorage<'a, CollectItem>,
        WriteStorage<'a, InBackpack>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (player, name, mut log, mut pos, mut collect, mut backpack) = data;
        for p in collect.join() {
            pos.remove(p.item);
            backpack
                .insert(p.item, InBackpack { owner: p.collector })
                .expect("FAILED to insert item in backpack.");
            if p.collector == *player {
                log.add(
                    format!("You pick up {}.", name.get(p.item).unwrap().name),
                    RGB::named(WHITE),
                );
            }
        }
        collect.clear();
    }
}

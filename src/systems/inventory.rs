use crate::components::{InBackpack, Name, Pickup, Position};
use crate::log::Log;
use bracket_lib::prelude::{RGB, WHITE};
use specs::prelude::*;

/*
 *
 * inventory.rs
 * --------
 * Manages the acquiring of items on the map, inserting them in the player's backpack.
 *
 */

pub struct PickupSystem {}

impl<'a> System<'a> for PickupSystem {
    type SystemData = (
        ReadExpect<'a, Entity>,
        ReadStorage<'a, Name>,
        WriteExpect<'a, Log>,
        WriteStorage<'a, Position>,
        WriteStorage<'a, Pickup>,
        WriteStorage<'a, InBackpack>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (player, name, mut log, mut pos, mut pickup, mut backpack) = data;
        for p in pickup.join() {
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
        pickup.clear();
    }
}

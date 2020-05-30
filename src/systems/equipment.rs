use crate::components::{BaseStats, Equipable, Equipment, InBackpack, Name, TryEquip};
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

pub struct EquipmentSystem {}

impl<'a> System<'a> for EquipmentSystem {
    type SystemData = (
        ReadExpect<'a, Entity>,
        ReadStorage<'a, Name>,
        ReadStorage<'a, Equipment>,
        WriteExpect<'a, Log>,
        WriteStorage<'a, Equipable>,
        WriteStorage<'a, InBackpack>,
        WriteStorage<'a, BaseStats>,
        WriteStorage<'a, TryEquip>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (player, name, equip, mut log, mut to_equip, mut backpack, mut stats, mut try_equip) =
            data;

        for c in try_equip.join() {
            println!("{:?}", c.equipment);
            // TODO: finish this
        }

        try_equip.clear();
    }
}

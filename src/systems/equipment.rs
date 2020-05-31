use crate::components::{Equipable, Equipment, InBackpack, Name, TryEquip};
use crate::log::Log;
use bracket_lib::prelude::{RGB, WHITE};
use specs::prelude::*;

/*
 *
 * equipment.rs
 * -------------
 * Manages equipping stuff, and unequiping if needed.
 *
 */

pub struct EquipmentSystem {}

impl<'a> System<'a> for EquipmentSystem {
    type SystemData = (
        ReadExpect<'a, Entity>,
        ReadStorage<'a, Name>,
        WriteStorage<'a, Equipment>,
        WriteExpect<'a, Log>,
        ReadStorage<'a, Equipable>,
        WriteStorage<'a, InBackpack>,
        WriteStorage<'a, TryEquip>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (player, name, mut equips, mut log, equipable, mut backpack, mut try_equip) = data;

        for e in try_equip.join() {
            let to_equip_slot = &equipable.get(e.equipment.equip).unwrap().slot;
            let to_equip_name = &name.get(e.equipment.equip).unwrap().name;
            let to_equip_user = e.equipment.user;
            let mut to_unequip: Vec<Entity> = Vec::new();

            // Iterate through all already equipped itens.
            for (equip, name, equipab) in (&equips, &name, &equipable).join() {
                if equipab.slot == *to_equip_slot && equip.user == to_equip_user {
                    to_unequip.push(equip.equip);
                    if equip.user == *player {
                        log.add(format!("You unequip {}.", name.name), RGB::named(WHITE));
                    }
                }
            }
            for ue in to_unequip {
                equips.remove(ue);
                backpack
                    .insert(
                        ue,
                        InBackpack {
                            owner: to_equip_user,
                        },
                    )
                    .expect("FAILED inserting item in backpack.");
            }
            backpack.remove(e.equipment.equip);
            equips
                .insert(
                    e.equipment.equip,
                    Equipment {
                        user: to_equip_user,
                        equip: e.equipment.equip,
                    },
                )
                .expect("FAILED equipping item.");

            if to_equip_user == *player {
                log.add(format!("You equip {}.", to_equip_name), RGB::named(WHITE));
            }
        }

        try_equip.clear();
    }
}

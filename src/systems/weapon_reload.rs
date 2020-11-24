use crate::components::{MissileWeapon, Name, TryReload, Inventory, Ammunition, AmmoType};
use crate::log::Log;
use crate::utils::colors::*;
use specs::prelude::*;

/*
 *
 * missile.rs
 * ----------
 * Resposible for managing every missile (ranged) attack performed.
 *
 */

pub struct WeaponReloadSystem {}

impl<'a> System<'a> for WeaponReloadSystem {
    type SystemData = (
        Entities<'a>,
        WriteStorage<'a, MissileWeapon>,
        WriteStorage<'a, TryReload>,
        ReadStorage<'a, Inventory>,
        WriteStorage<'a, Ammunition>,
        ReadExpect<'a, Entity>,
        WriteExpect<'a, Log>,
        ReadStorage<'a, Name>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (entities, mut missile_weapon, mut try_reload, inventory, mut ammo, player, mut log, names) = data;

        for (ent, reload) in (&entities, &try_reload).join() {
            if let Some(w) = missile_weapon.get_mut(reload.weapon) {
                let ammo_type = &w.ammo.ammo_type;
                for (e, _inv) in (&entities, &inventory).join() {
                    if let Some(amm) = ammo.get_mut(e) {
                        if amm.ammo_type == *ammo_type {
                            match ammo_type {
                                AmmoType::_32 => {
                                    amm.ammo -= 1;
                                    // TODO remove ammo from inventory if it reaches 0. Maybe add item drop?
                                    w.ammo.ammo += 1;
                                },
                                _ => {}
                            }
                        }
                        if ent == *player {
                            log.add(
                                format!("You reload the {}.", names.get(reload.weapon).unwrap().name),
                                color("BrightWhite", 1.0),
                            );
                        }
                    }
                }
            }
        }
        try_reload.clear();
    }
}

use crate::components::{BaseStats, MeleeAttack, Name, SufferDamage, ActiveWeapon, MeleeWeapon, Equipment};
use bracket_lib::prelude::RandomNumberGenerator;
use crate::log::Log;
use crate::utils::colors::*;
use specs::prelude::*;

/*
 *
 * melee.rs
 * --------
 * Resposible for managing every melee (physical) attack performed.
 *
 */

pub struct MeleeSystem {}

impl<'a> System<'a> for MeleeSystem {
    type SystemData = (
        Entities<'a>,
        ReadStorage<'a, BaseStats>,
        ReadStorage<'a, Equipment>,
        ReadStorage<'a, ActiveWeapon>,
        WriteStorage<'a, MeleeAttack>,
        WriteStorage<'a, SufferDamage>,
        ReadStorage<'a, MeleeWeapon>,
        ReadExpect<'a, Entity>,
        WriteExpect<'a, Log>,
        WriteExpect<'a, RandomNumberGenerator>,
        ReadStorage<'a, Name>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (entities, base_stats, equipment, active_wpn, mut melee_attack, mut do_damage, melee_wpns, player, 
             mut log, mut rng, names) = data;
        let white = color("BrightWhite", 1.0);

        for (entity, melee, attacker_stats, name) in
            (&entities, &melee_attack, &base_stats, &names).join()
        {
            let attacker_hp = attacker_stats.health.hp;
            let victim_stats = base_stats.get(melee.target).unwrap();
            let victim_hp = victim_stats.health.hp;
            let victim_name = names.get(melee.target).unwrap();

            let mut has_weapon_equipped = false;

            if attacker_hp > 0 && victim_hp > 0 {
                for (_active_wpn, melee_wpn, equip, name_wpn) in (&active_wpn, &melee_wpns, &equipment, &names).join() {
                    if equip.user == entity {
                        has_weapon_equipped = true;
                        let wpn_stats = &melee_wpn.stats;
                        let total_intended_damage = rng.roll_dice(wpn_stats.dice_n, wpn_stats.dice_faces) + wpn_stats.dice_bonus;
                        let damage = i32::max(0, total_intended_damage - victim_stats.defense);
                        log.add(
                            format!(
                            "{} hits {} with {} for {} hp!",
                            &name.name, &victim_name.name, &name_wpn.name, damage
                            ),
                        white,
                        );
                        SufferDamage::add_damage(&mut do_damage, melee.target, damage, entity == *player);
                        break;
                    }
                }
                if !has_weapon_equipped {
                    let damage = i32::max(0, attacker_stats.attack - victim_stats.defense);
                    log.add(
                        format!(
                            "{} hits {} for {} hp!",
                            &name.name, &victim_name.name, damage
                        ),
                        white,
                    );
                    SufferDamage::add_damage(&mut do_damage, melee.target, damage, entity == *player);
                }
            }
        }
        melee_attack.clear();
    }
}

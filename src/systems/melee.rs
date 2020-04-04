use bracket_lib::prelude::{RGB, WHITE};
use specs::prelude::*;
use crate::components::{MeleeAttack, BaseStats, SufferDamage, Name};
use crate::log::Log;

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
        WriteStorage<'a, MeleeAttack>,
        WriteStorage<'a, SufferDamage>,
        ReadExpect<'a, Entity>,
        WriteExpect<'a, Log>,
        ReadStorage<'a, Name>
    );


    fn run(&mut self, data: Self::SystemData) {
        let (entities, base_stats, mut melee_attack, mut do_damage, player, 
             mut log, names) = data;

        for (entity, melee, attacker_stats, name) in (&entities, &melee_attack, &base_stats, &names).join() {
            let attacker_hp = attacker_stats.health.hp;
            let victim_stats = base_stats.get(melee.target).unwrap();
            let victim_hp = victim_stats.health.hp;

            if attacker_hp > 0 && victim_hp > 0 {
                // TODO: let damage come from weapon stats.
                let damage = i32::max(0, attacker_stats.attack - victim_stats.defense);
                let victim_name = names.get(melee.target).unwrap();
                log.add(format!("{} hits {} for {} hp!", &name.name, &victim_name.name, damage), RGB::named(WHITE));
                SufferDamage::add_damage(
                    &mut do_damage,
                    melee.target,
                    damage,
                    entity == *player,
                );
            }
        }
        melee_attack.clear();
    }
}


use crate::components::{BaseStats, MissileAttack, Name, SufferDamage};
use crate::log::Log;
use bracket_lib::prelude::{RGB, WHITE};
use specs::prelude::*;

/*
 *
 * missile.rs
 * ----------
 * Resposible for managing every missile (ranged) attack performed.
 *
 */

pub struct MissileSystem {}

impl<'a> System<'a> for MissileSystem {
    type SystemData = (
        Entities<'a>,
        ReadStorage<'a, BaseStats>,
        WriteStorage<'a, MissileAttack>,
        WriteStorage<'a, SufferDamage>,
        ReadExpect<'a, Entity>,
        WriteExpect<'a, Log>,
        ReadStorage<'a, Name>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (entities, base_stats, mut missile_attack, mut do_damage, player, mut log, names) =
            data;

        for (entity, missile, attacker_stats, name) in
            (&entities, &missile_attack, &base_stats, &names).join()
        {
            let attacker_hp = attacker_stats.health.hp;
            let victim_stats = base_stats.get(missile.target).unwrap();
            let victim_hp = victim_stats.health.hp;

            if attacker_hp > 0 && victim_hp > 0 {
                // TODO: let damage come from weapon stats.
                let damage = i32::max(0, attacker_stats.attack - victim_stats.defense);
                let victim_name = names.get(missile.target).unwrap();
                log.add(
                    format!(
                        "{} shoots {} for {} hp!",
                        &name.name, &victim_name.name, damage
                    ),
                    RGB::named(WHITE),
                );
                SufferDamage::add_damage(&mut do_damage, missile.target, damage, entity == *player);
            }
        }
        missile_attack.clear();
    }
}

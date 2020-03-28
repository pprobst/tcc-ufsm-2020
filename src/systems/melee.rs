//use bracket_lib::prelude::*;
use specs::prelude::*;
use crate::components::{MeleeAttack, BaseStats, SufferDamage};

pub struct MeleeSystem {}

impl<'a> System<'a> for MeleeSystem {
    type SystemData = ( 
        Entities<'a>,
        ReadStorage<'a, BaseStats>,
        WriteStorage<'a, MeleeAttack>,
        WriteStorage<'a, SufferDamage>,
        ReadExpect<'a, Entity>,
    );


    fn run(&mut self, data: Self::SystemData) {
        let (entities, base_stats, mut melee_attack, mut do_damage, player) = data;

        for (entity, melee, attacker_stats) in (&entities, &melee_attack, &base_stats).join() {
            let attacker_hp = attacker_stats.health.hp;
            let victim_stats = base_stats.get(melee.target).unwrap();
            let victim_hp = victim_stats.health.hp;

            if attacker_hp > 0 && victim_hp > 0 && !victim_stats.god {
                let damage = i32::max(0, attacker_stats.attack - victim_stats.defense);
                SufferDamage::new_damage(
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


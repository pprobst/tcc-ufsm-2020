use specs::prelude::*;
use crate::components::{MissileAttack, BaseStats, SufferDamage};

pub struct MissileSystem {}

impl<'a> System<'a> for MissileSystem {
    type SystemData = ( 
        Entities<'a>,
        ReadStorage<'a, BaseStats>,
        WriteStorage<'a, MissileAttack>,
        WriteStorage<'a, SufferDamage>,
        ReadExpect<'a, Entity>,
    );


    fn run(&mut self, data: Self::SystemData) {
        let (entities, base_stats, mut missile_attack, mut do_damage, player) = data;

        for (entity, missile, attacker_stats) in (&entities, &missile_attack, &base_stats).join() {
            let attacker_hp = attacker_stats.health.hp;
            let victim_stats = base_stats.get(missile.target).unwrap();
            let victim_hp = victim_stats.health.hp;

            if attacker_hp > 0 && victim_hp > 0 {
                // TODO: let damage come from weapon stats.
                let damage = i32::max(0, attacker_stats.attack - victim_stats.defense);
                SufferDamage::add_damage(
                    &mut do_damage,
                    missile.target,
                    damage,
                    entity == *player,
                );
            }
        }
        missile_attack.clear();
    }
}


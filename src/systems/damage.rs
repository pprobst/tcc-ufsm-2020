//use bracket_lib::prelude::*;
use specs::prelude::*;
use crate::components::{SufferDamage, BaseStats, Position};
use crate::map_gen::Map;

pub struct DamageSystem {}

impl<'a> System<'a> for DamageSystem {
    type SystemData = (
        WriteStorage<'a, SufferDamage>,
        Entities<'a>,
        ReadExpect<'a, Entity>,
        WriteStorage<'a, BaseStats>,
        WriteExpect<'a, Map>,
        ReadStorage<'a, Position>
    );

    fn run(&mut self, data: Self::SystemData) {
        let (mut damage, entities, _player, mut stats, mut map, position) = data;

        for (damage, _ent, victim_stats, pos) in (&damage, &entities, &mut stats, &position).join() {
            if !victim_stats.god {
                for dmg in damage.amount.iter() {
                    println!("{}", victim_stats.health.hp);
                    victim_stats.health.hp -= dmg.0;
                }
            }
            if victim_stats.health.hp <= 0 {
                let idx = map.idx(pos.x, pos.y);
                map.tiles[idx].block = false;
            }
        }
        damage.clear();
    }
}

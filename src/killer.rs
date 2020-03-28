use specs::prelude::*;
use super::{BaseStats};

pub struct Killer<'a> {
    pub ecs: &'a mut World,
}

pub fn remove_dead_entities(ecs: &mut World) {
    Killer {
        ecs,
    }.kill_all()
}

impl<'a> Killer<'a> {
    pub fn kill_all(&mut self) {
        let mut dead: Vec<Entity> = Vec::new();
        {
            let entities = self.ecs.entities();
            let stats = self.ecs.read_storage::<BaseStats>();

            for (ent, stats) in (&entities, &stats).join() {
                if stats.health.hp <= 0 {
                    dead.push(ent);
                }
            }
        }
        for f in dead {
            self.ecs.delete_entity(f).expect("Unable to remove the dead");
        }
    } 
}

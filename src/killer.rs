use bracket_lib::prelude::{RGB, YELLOW, RED};
use specs::prelude::*;
use super::{BaseStats, Name, Player, log::Log};

/*
 *
 * killer.rs
 * ---------
 * Works as a "cleaner" by deleting the dead entities from the world.
 *
 */

pub struct Killer<'a> {
    pub ecs: &'a mut World,
}

/// Remove all the dead entities from the ECS.
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
            let names = self.ecs.read_storage::<Name>();
            let player = self.ecs.read_storage::<Player>();
            let mut log = self.ecs.fetch_mut::<Log>();

            for (ent, stats, name) in (&entities, &stats, &names).join() {
                if stats.health.hp <= 0 {
                    let p: Option<&Player> = player.get(ent);
                    if let Some(_p) = p {
                        log.add("You died...", RGB::named(RED));
                    } else {
                        log.add(format!("{} dies.", &name.name), RGB::named(YELLOW));
                        dead.push(ent);
                    }
                }
            }
        }
        for f in dead {
            self.ecs.delete_entity(f).expect("Unable to remove the dead");
        }
    } 
}

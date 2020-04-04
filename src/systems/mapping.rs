use specs::prelude::*;
use crate::components::{Position, Blocker};
use crate::map_gen::Map;

/*
 *
 * mapping.rs
 * ----------
 * Responsible for managing the entities on the map tiles.
 *
 */

pub struct MappingSystem {}

impl<'a> System<'a> for MappingSystem {
    type SystemData = (
        Entities<'a>,
        ReadStorage<'a, Position>,
        ReadStorage<'a, Blocker>,
        WriteExpect<'a, Map>,
    );
    
    fn run(&mut self, data: Self::SystemData) {
        let (entities, pos, blockers, mut map) = data;
        let map = &mut *map;

        map.refresh_entities();
        // Iterate through all the entities that have a Position and are Blockers.
        for (ent, pos, _blocker) in (&entities, &pos, &blockers).join() {
            map.add_blocker(pos.x, pos.y);
            let i = map.idx(pos.x, pos.y);
            map.entities[i] = Some(ent.clone());
        }
    }
}

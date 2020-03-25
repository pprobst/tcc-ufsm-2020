use specs::prelude::*;
use crate::components::{Position, Blocker};
use crate::map_gen::Map;

pub struct MappingSystem {}

impl<'a> System<'a> for MappingSystem {
    type SystemData = (
        ReadStorage<'a, Position>,
        ReadStorage<'a, Blocker>,
        WriteExpect<'a, Map>,
    );
    
    fn run(&mut self, data: Self::SystemData) {
        let (pos, blocker, mut map) = data;

        // Iterate through all the entities that have a Position and are Blockers.
        // Blocks the tile in (pos.x, pos.y).
        for (pos, _blocker) in (&pos, &blocker).join() {
            let i = map.idx(pos.x, pos.y);
            map.tiles[i].block = true;
        }
    }
}

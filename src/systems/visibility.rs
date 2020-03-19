use bracket_lib::prelude::*;
use specs::prelude::*;
use crate::components::{Position, Fov, Player};
use crate::map_gen::Map;

// See: https://github.com/thebracket/bracket-lib/blob/master/rltk/examples/ex04-fov.rs

pub struct VisibilitySystem {}

impl<'a> System<'a> for VisibilitySystem {
    type SystemData = (
        Entities<'a>,
        WriteExpect<'a, Map>,
        WriteStorage<'a, Fov>,
        ReadStorage<'a, Position>,
        ReadStorage<'a, Player>
    );

    fn run(&mut self, data: Self::SystemData) {
        let (entities, mut map, mut fov, pos, player) = data;

        for (e, fov, pos) in (&entities, &mut fov, &pos).join() {
            if fov.dirty {
                fov.visible_pos.clear();
                fov.visible_pos = field_of_view(Point::new(pos.x, pos.y), fov.range, &*map);
                fov.visible_pos.retain(|p| map.contain_pos(*p));

                let p: Option<&Player> = player.get(e);
                if let Some(_p) = p {
                    // Reset visible tiles in map.tiles.
                    for tile in map.tiles.iter_mut() { tile.visible = false };
                    // For each visible position (point) in visible_pos, mark the same position 
                    // as visible and revealed in the map tiles.
                    for pos in fov.visible_pos.iter() {
                        let idx = map.idx(pos.x, pos.y);
                        map.tiles[idx].visible = true;
                        map.tiles[idx].revealed = true;
                    }
                }

                fov.dirty = false;
            }
        }
    }
}

use bracket_lib::prelude::*;
use specs::prelude::*;
use crate::components::{Position, Fov, Player};
use crate::map_gen::Map;

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
                fov.visible_tiles.clear();
                fov.visible_tiles = field_of_view(Point::new(pos.x, pos.y), fov.range, &*map);
                fov.visible_tiles.retain(|p| map.contain_pos(*p));

                let p: Option<&Player> = player.get(e);
                if let Some(_p) = p {
                    for tile in map.tiles.iter_mut() { tile.visible = false };
                    for view in fov.visible_tiles.iter() {
                        let idx = map.idx(view.x, view.y);
                        map.tiles[idx].visible = true;
                        map.tiles[idx].revealed = true;
                    }
                }

                fov.dirty = false;
            }
        }
    }
}

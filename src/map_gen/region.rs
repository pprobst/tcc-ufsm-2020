use super::{Map, TileType};
use crate::utils::directions::*;
use bracket_lib::prelude::Point;

pub type Region = Vec<usize>;

pub trait Operations {
    fn is_probably_edge(&self, pt: Point, map: &Map) -> bool;
    fn fill_region(&self, map: &mut Map, ttype: TileType);
}

impl Operations for Region {
    /// Returns true if the point is probably an edge of a region.
    /// While not 100% accurate (it detects blockers not only on edges),
    /// it cuts our distance computations by a lot!
    fn is_probably_edge(&self, pt: Point, map: &Map) -> bool {
        let east = pt + EAST;
        let west = pt + WEST;
        let north = pt + NORTH;
        let south = pt + SOUTH;

        if map.in_map_bounds(east) && map.tiles[map.idx_pt(east)].block {
            return true;
        }
        if map.in_map_bounds(west) && map.tiles[map.idx_pt(west)].block {
            return true;
        }
        if map.in_map_bounds(north) && map.tiles[map.idx_pt(north)].block {
            return true;
        }
        if map.in_map_bounds(south) && map.tiles[map.idx_pt(south)].block {
            return true;
        }

        return false;
    }

    /// Fill a region with tiles of type ttype.
    fn fill_region(&self, map: &mut Map, ttype: TileType) {
        for idx in self {
            map.paint_tile(*idx, ttype);
        }
    }
}

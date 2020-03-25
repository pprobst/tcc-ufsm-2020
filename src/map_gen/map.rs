use bracket_lib::prelude::*;
use crate::components::Position;
//use specs::prelude::*;
use super::{TileType, Tile};

#[derive(Clone)]
pub struct Map {
    pub tiles: Vec<Tile>,
    pub width: i32,
    pub height: i32,
	//pub spawn_point: (i32, i32),
	//pub exit_point: (i32, i32),
}

impl Map {
    pub fn new(width: i32, height: i32) -> Map {
        Self {
            tiles: vec![Tile::floor(); (width*height) as usize],
            width,
            height,
        }
    }

    pub fn idx(&self, x: i32, y: i32) -> usize {
        (y as usize * self.width as usize) + x as usize
    }

    pub fn idx_pos(&self, idx: usize) -> Position {
        let idx_32 = idx as i32;
        let y = idx_32 / self.width;
        let x = idx_32 - y * self.width;
        Position::new(x, y)
    }

    pub fn in_map_bounds(&self, p: Position) -> bool {
        p.x > 0 && p.x < self.width-1 && p.y > 0 && p.y < self.height-1
    }

    pub fn in_map_bounds_xy(&self, x: i32, y: i32) -> bool {
        x > 0 && x < self.width-1 && y > 0 && y < self.height-1
    }

    pub fn clear_blocker(&mut self, x: i32, y: i32) {
        let idx = self.idx(x, y);
        self.tiles[idx].block = false; 
    }

    fn valid_exit(&self, loc: Point, delta: Point) -> Option<usize> {
        let destination = loc + delta;
        let idx = self.point2d_to_index(destination);
        if self.in_map_bounds(destination) && !self.tiles[idx].block {
            Some(idx)
        } else {
            None
        }
    }
}

impl Algorithm2D for Map {
    fn dimensions(&self) -> Point {
        Point::new(self.width, self.height)
    }
}

// https://github.com/thebracket/bracket-lib/tree/master/bracket-pathfinding
// https://github.com/thebracket/bracket-lib/blob/master/bracket-pathfinding/examples/astar/common.rs
impl BaseMap for Map {

    /* 
     * Dijkstra and A-Star need to know what exits are valid from a tile, and the 
     * "cost" of moving to that tile (most of the time you can use 1.0).
     * */

    // Automatically prevents FOV from looking behind opaque tiles.
    fn is_opaque(&self, idx: usize) -> bool {
        let ttype = self.tiles[idx as usize].ttype; 
        ttype == TileType::Wall || ttype == TileType::Tree
    }

    // A* needs this or it won't work!
    fn get_available_exits(&self, idx: usize) -> Vec<(usize, f32)> {
        let mut exits = Vec::new();
        let location = self.idx_pos(idx);

        if let Some(idx) = self.valid_exit(location, Point::new(-1, 0)) {
            exits.push((idx, 1.0))
        }
        if let Some(idx) = self.valid_exit(location, Point::new(1, 0)) {
            exits.push((idx, 1.0))
        }
        if let Some(idx) = self.valid_exit(location, Point::new(0, -1)) {
            exits.push((idx, 1.0))
        }
        if let Some(idx) = self.valid_exit(location, Point::new(0, 1)) {
            exits.push((idx, 1.0))
        }

        if let Some(idx) = self.valid_exit(location, Point::new(-1, -1)) {
            exits.push((idx, 1.4))
        }
        if let Some(idx) = self.valid_exit(location, Point::new(1, -1)) {
            exits.push((idx, 1.4))
        }
        if let Some(idx) = self.valid_exit(location, Point::new(-1, 1)) {
            exits.push((idx, 1.4))
        }
        if let Some(idx) = self.valid_exit(location, Point::new(1, 1)) {
            exits.push((idx, 1.4))
        }

        exits
    }

    fn get_pathing_distance(&self, idx1: usize, idx2: usize) -> f32 {
        DistanceAlg::PythagorasSquared
           .distance2d(self.idx_pos(idx1), self.idx_pos(idx2))
    }
}

use bracket_lib::prelude::*;
use crate::components::Position;
use specs::prelude::{Entity};
use super::{TileType, Tile};

/*
 *
 * map.rs
 * ------
 * Basic structure of a map/level.
 *
 */

#[derive(Clone)]
pub struct Map {
    pub tiles: Vec<Tile>,
    pub size: i32,
    pub width: i32,
    pub height: i32,
    pub entities: Vec<Option<Entity>>
	//pub spawn_point: (i32, i32),
	//pub exit_point: (i32, i32),
}

impl Map {
    pub fn new(width: i32, height: i32) -> Map {
        let map_size = width*height;
        Self {
            tiles: vec![Tile::wall(); map_size as usize],
            size: map_size,
            width,
            height,
            entities: vec![None; map_size as usize]
        }
    }

    /// Returns a map index from a given x, y coordinate.
    pub fn idx(&self, x: i32, y: i32) -> usize {
        (y as usize * self.width as usize) + x as usize
    }

    /// Returns a Point(x, y) from a given map index.
    pub fn idx_pos(&self, idx: usize) -> Position {
        let idx_32 = idx as i32;
        let y = idx_32 / self.width;
        let x = idx_32 - y * self.width;
        Position::new(x, y)
    }

    /// Checks if a certain position (Point) is in the map.
    pub fn in_map_bounds(&self, p: Position) -> bool {
        p.x > 0 && p.x < self.width-1 && p.y > 0 && p.y < self.height-1
    }

    /// Checks if a certain x, y coordinate is in the map.
    pub fn in_map_bounds_xy(&self, x: i32, y: i32) -> bool {
        x > 0 && x < self.width-1 && y > 0 && y < self.height-1
    }

    /// Makes a tile passable.
    pub fn clear_blocker(&mut self, x: i32, y: i32) {
        let idx = self.idx(x, y);
        self.tiles[idx].block = false; 
    }

    /// Makes a tile non-passable.
    pub fn add_blocker(&mut self, x: i32, y: i32) {
        let idx = self.idx(x, y);
        self.tiles[idx].block = true; 
    }

    pub fn refresh_entities(&mut self) {
        for i in 0 .. self.entities.len() {
            self.entities[i] = None
        }
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
    fn get_available_exits(&self, idx: usize) -> SmallVec<[(usize, f32); 10]> {
        let mut exits = SmallVec::new();
        let location = self.index_to_point2d(idx);

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
        if let Some(idx) = self.valid_exit(location, Point::new(-1, 1)) {
            exits.push((idx, 1.4))
        }
        if let Some(idx) = self.valid_exit(location, Point::new(1, -1)) {
            exits.push((idx, 1.4))
        }
        if let Some(idx) = self.valid_exit(location, Point::new(1, 1)) {
            exits.push((idx, 1.4))
        }

        exits
    }

    fn get_pathing_distance(&self, idx1: usize, idx2: usize) -> f32 {
        DistanceAlg::Pythagoras
           .distance2d(self.idx_pos(idx1), self.idx_pos(idx2))
    }
}

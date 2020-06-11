use super::{Tile, TileType, CustomRegion};
use crate::components::Position;
use crate::utils::directions::*;
use bracket_lib::prelude::*;
use specs::prelude::Entity;

/*
 *
 * map.rs
 * ------
 * Basic structure of a map/level.
 *
 */

#[derive(Clone, Debug)]
pub struct Map {
    pub tiles: Vec<Tile>,
    pub region: CustomRegion,
    pub size: i32,
    pub width: i32,
    pub height: i32,
    pub entities: Vec<Option<Entity>>, //pub spawn_point: (i32, i32),
                                       //pub exit_point: (i32, i32)
}

#[allow(dead_code)]
impl Map {
    pub fn new(width: i32, height: i32, empty: bool) -> Map {
        let map_size = width * height;
        let tile = if empty { Tile::floor() } else { Tile::wall() };
        Self {
            tiles: vec![tile; map_size as usize],
            region: CustomRegion::new_rect(0, 0, width, height),
            size: map_size,
            width,
            height,
            entities: vec![None; map_size as usize],
        }
    }

    pub fn get_region(&self) -> CustomRegion {
        self.region.clone()
    }

    /// Makes map chaotic with a chance of floor_chance to change a tile to floor.
    /// Used in mapgen algorithms that require a "chaotic map" like Cellular Automata.
    pub fn make_chaotic(&mut self, floor_chance: u8) {
        let mut rng = RandomNumberGenerator::new();
        let floor = Tile::floor();

        for mut _tile in self.tiles.iter_mut() {
            let chance = rng.range(1, 101);
            if chance <= floor_chance {
                *_tile = floor;
            }
        }
    }

    /// Add solid borders to the map.
    pub fn add_borders(&mut self, ttype: TileType) {
        let mut idx;
        for x in 0..self.width {
            idx = self.idx(x, 0);
            self.paint_tile(idx, ttype);
            idx = self.idx(x, self.height - 1);
            self.paint_tile(idx, ttype);
        }
        for y in 0..self.height {
            idx = self.idx(0, y);
            self.paint_tile(idx, ttype);
            idx = self.idx(self.width - 1, y);
            self.paint_tile(idx, ttype);
        }
    }

    pub fn pretty_walls(&mut self) {
        for y in 1..self.height {
            for x in 1..self.width {
                let idx = self.idx(x, y);
                if self.tiles[idx].ttype == TileType::Wall {
                    let glyph = self.get_wall_glyph(x, y);
                    self.tiles[idx].change_glyph(glyph);
                }
            }
        }
    }

    fn get_wall_glyph(&self, x: i32, y: i32) -> char {
        if !self.in_map_bounds_xy(x, y) {
            return '■';
        }
        let curr_pt = Point { x, y };
        let mut bitmask: u8 = 0;

        // See: https://waa.ai/TY6r
        let north = curr_pt + NORTH;
        let south = curr_pt + SOUTH;
        let west = curr_pt + WEST;
        let east = curr_pt + EAST;

        if self.in_map_bounds(north) {
            if self.tiles[self.idx_pt(north)].ttype == TileType::Wall {
                bitmask += 1;
            }
        }
        if self.in_map_bounds(south) {
            if self.tiles[self.idx_pt(south)].ttype == TileType::Wall {
                bitmask += 2;
            }
        }
        if self.in_map_bounds(west) {
            if self.tiles[self.idx_pt(west)].ttype == TileType::Wall {
                bitmask += 4;
            }
        }
        if self.in_map_bounds(east) {
            if self.tiles[self.idx_pt(east)].ttype == TileType::Wall {
                bitmask += 8;
            }
        }

        match bitmask {
            0 => '■',
            1 => '║',
            2 => '║',
            3 => '║',
            4 => '═',
            5 => '╝',
            6 => '╗',
            7 => '╣',
            8 => '═',
            9 => '╚',
            10 => '╔',
            11 => '╠',
            12 => '═',
            13 => '╩',
            14 => '╦',
            15 => '╬',
            _ => '█',
        }
    }

    pub fn paint_tile(&mut self, idx: usize, ttype: TileType) {
        match ttype {
            TileType::Floor => {
                self.tiles[idx] = Tile::floor();
            }
            TileType::WoodenFloor => {
                self.tiles[idx] = Tile::woodenfloor();
            }
            TileType::ClosedDoor => {
                self.tiles[idx] = Tile::closed_door();
            }
            TileType::OpenDoor => {
                self.tiles[idx] = Tile::open_door();
            }
            TileType::Tree => {
                self.tiles[idx] = Tile::tree();
            }
            TileType::Wall => {
                self.tiles[idx] = Tile::wall();
            }
            TileType::InvisibleWall => {
                self.tiles[idx] = Tile::invisible_wall();
            }
            TileType::ShallowWater => {
                self.tiles[idx] = Tile::shallow_water();
            }
            TileType::DeepWater => {
                self.tiles[idx] = Tile::deep_water();
            }
            TileType::Grass => {
                self.tiles[idx] = Tile::grass();
            }
            TileType::Grass2 => {
                self.tiles[idx] = Tile::grass2();
            }
            TileType::TallGrass => {
                self.tiles[idx] = Tile::tallgrass();
            }
            TileType::Flower => {
                self.tiles[idx] = Tile::flower();
            }
            _ => {
                self.tiles[idx] = Tile::floor();
            }
        }
    }

    pub fn paint_tile_char(&mut self, idx: usize, ch: char) {
        match ch {
            '.' => {
                self.tiles[idx] = Tile::floor();
            }
            '∙' => {
                self.tiles[idx] = Tile::floor2();
            }
            '_' => {
                self.tiles[idx] = Tile::woodenfloor();
            }
            '+' => {
                self.tiles[idx] = Tile::closed_door();
            }
            '/' => {
                self.tiles[idx] = Tile::open_door();
            }
            '#' => {
                self.tiles[idx] = Tile::wall();
            }
            '~' => {
                self.tiles[idx] = Tile::shallow_water();
            }
            '♣' => {
                self.tiles[idx] = Tile::tree();
            }
            '⌠' => {
                self.tiles[idx] = Tile::tallgrass();
            }
            ',' => {
                self.tiles[idx] = Tile::grass();
            }
            _ => {
                self.tiles[idx] = Tile::floor();
            }
        }
    }

    pub fn apply_forest_theme(&mut self) {
        for idx in 0..self.tiles.len() {
            if self.is_wall(idx) {
                self.tiles[idx] = Tile::tree();
            }
        }
    }

    pub fn is_water(&self, idx: usize) -> bool {
        let ttype = self.tiles[idx].ttype;
        match ttype {
            TileType::ShallowWater => true,
            TileType::DeepWater => true,
            _ => false,
        }
    }

    pub fn is_floor(&self, idx: usize) -> bool {
        let ttype = self.tiles[idx].ttype;
        match ttype {
            TileType::Floor => true,
            TileType::Floor2 => true,
            TileType::WoodenFloor => true,
            _ => false,
        }
    }

    pub fn is_wall(&self, idx: usize) -> bool {
        let ttype = self.tiles[idx].ttype;
        match ttype {
            TileType::Wall => true,
            _ => false,
        }
    }

    pub fn is_door(&self, idx: usize) -> bool {
        let ttype = self.tiles[idx].ttype;
        match ttype {
            TileType::OpenDoor => true,
            TileType::ClosedDoor => true,
            _ => false,
        }
    }

    /// Returns a map index from a given x, y coordinate.
    pub fn idx(&self, x: i32, y: i32) -> usize {
        (y as usize * self.width as usize) + x as usize
    }

    /// Returns a map index from a given Point.
    pub fn idx_pt(&self, point: Point) -> usize {
        (point.y as usize * self.width as usize) + point.x as usize
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
        p.x > 0 && p.x < self.width && p.y > 0 && p.y < self.height
    }

    /// Checks if a certain x, y coordinate is in the map.
    pub fn in_map_bounds_xy(&self, x: i32, y: i32) -> bool {
        x > 0 && x < self.width && y > 0 && y < self.height
    }

    /// Checks if a certain idx is in the map.
    pub fn in_map_bounds_idx(&self, idx: usize) -> bool {
        let pos = self.idx_pos(idx);
        self.in_map_bounds(pos)
    }

    pub fn in_map_bounds_neighbors(&self, p: Position) -> bool {
        self.in_map_bounds(p + NORTH)
            && self.in_map_bounds(p + SOUTH)
            && self.in_map_bounds(p + EAST)
            && self.in_map_bounds(p + WEST)
            && self.in_map_bounds(p + NORTHEAST)
            && self.in_map_bounds(p + NORTHWEST)
            && self.in_map_bounds(p + SOUTHEAST)
            && self.in_map_bounds(p + SOUTHWEST)
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

    pub fn reveal(&mut self, idx: usize) {
        self.tiles[idx].revealed = true;
        self.tiles[idx].visible = true;
    }

    pub fn hide(&mut self, idx: usize) {
        self.tiles[idx].revealed = false;
        self.tiles[idx].visible = false;
    }

    pub fn refresh_entities(&mut self) {
        for i in 0..self.entities.len() {
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
        ttype == TileType::Wall || ttype == TileType::Tree || ttype == TileType::ClosedDoor
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
        DistanceAlg::Pythagoras.distance2d(self.idx_pos(idx1), self.idx_pos(idx2))
    }
}

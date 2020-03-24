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
}

impl Algorithm2D for Map {
    fn dimensions(&self) -> Point {
        Point::new(self.width, self.height)
    }
}

// Automatically prevents FOV from looking behind opaque tiles.
impl BaseMap for Map {
    fn is_opaque(&self, idx: usize) -> bool {
        let ttype = self.tiles[idx as usize].ttype; 

        ttype == TileType::Wall || ttype == TileType::Tree
    }
}

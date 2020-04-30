use super::{Direction, TileType};
use std::collections::HashSet;

#[derive(Debug, Clone, PartialEq)]
pub struct MapTile {
    pub idx: usize,
    pub pattern: Vec<TileType>,
    pub compatible: Vec<(Vec<TileType>, Direction)>, // overlaps
    pub size: i32,
}

pub fn tile_idx(tile_size: i32, x: i32, y: i32) -> usize {
    ((y * tile_size) + x) as usize
}

pub fn in_tile_bounds(tile_size: i32, x: i32, y: i32) -> bool {
    x >= 0 && x < tile_size && y >= 0 && y < tile_size
}

// https://www.reddit.com/r/rust/comments/38zzbk/best_way_to_remove_duplicates_from_a_veclist/
pub fn deduplicate(vs: &mut Vec<Vec<TileType>>) {
    let set: HashSet<Vec<TileType>> = vs.drain(..).collect();
    vs.extend(set.into_iter());
}

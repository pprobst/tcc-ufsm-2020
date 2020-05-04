use super::{Cell, MapTile, Point, TileType};
use crate::utils::directions::*;
use bracket_lib::prelude::RandomNumberGenerator;
use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashMap};

#[derive(Debug, Clone)]
pub struct Wave {
    cells: Vec<Cell>,
    pub uncollapsed_cells: usize,
    entropy_queue: BinaryHeap<CoordEntropy>,
    tile_removals: Vec<RemovalUpdate>,
    out_width: i32,
    out_height: i32,
}

#[allow(dead_code)]
impl Wave {
    pub fn new(cells: Vec<Cell>, out_width: i32, out_height: i32) -> Self {
        let cells_len = cells.len(); // or out_width * out_height
        Self {
            cells,
            uncollapsed_cells: cells_len,
            entropy_queue: BinaryHeap::new(),
            tile_removals: Vec::new(),
            out_height,
            out_width,
        }
    }

    pub fn init_entropy_queue(&mut self) {
        let mut i = 0;
        //println!("{} {}", self.out_height, self.out_width);
        for y in 0..self.out_height {
            for x in 0..self.out_width {
                let idx = self.cell_at(x, y);
                let cell = &self.cells[idx];
                i += 1;
                //println!("{:?}, {:?}", cell, MinFloat(cell.entropy()));
                self.entropy_queue.push(CoordEntropy {
                    entropy: MinFloat(cell.entropy()),
                    coord: Point::new(x, y),
                });
            }
        }
        //println!("{}", i);
    }

    fn cell_at(&self, x: i32, y: i32) -> usize {
        (y as usize * self.out_width as usize) + x as usize
    }

    fn in_bound(&self, x: i32, y: i32) -> bool {
        x >= 0 && x < self.out_width && y > 0 && y < self.out_height
    }

    pub fn choose_next_cell(&mut self) -> Point {
        while let Some(entropy_coord) = self.entropy_queue.pop() {
            let idx = self.cell_at(entropy_coord.coord.x, entropy_coord.coord.y);
            let cell = &self.cells[idx];
            if !cell.collapsed {
                return entropy_coord.coord;
            }
        }
        unreachable!("entropy_queue is empty!");
    }

    pub fn collapse_cell_at(
        &mut self,
        pt: Point,
        freq: &HashMap<Vec<TileType>, f32>,
        rng: &mut RandomNumberGenerator,
    ) {
        let idx = self.cell_at(pt.x, pt.y);
        let mut cell = &mut self.cells[idx];
        let locked_tile = cell.choose_tile(freq, rng);

        cell.collapsed = true;

        let patterns = cell.patterns.clone();
        for tile in patterns {
            if tile != locked_tile {
                let t = tile.clone();
                cell.remove_tile(tile, freq);
                self.tile_removals
                    .push(RemovalUpdate { tile: t, coord: pt })
            }
        }
    }

    pub fn propagate(&mut self, freq: &HashMap<Vec<TileType>, f32>) {
        while let Some(removal_update) = self.tile_removals.pop() {
            for i in 0..4 {
                let dir;
                match i {
                    0 => {
                        dir = EAST;
                    }
                    1 => {
                        dir = WEST;
                    }
                    2 => {
                        dir = NORTH;
                    }
                    _ => {
                        dir = SOUTH;
                    }
                }
                let neighbor_coord = removal_update.coord + dir;
                let neighbor_idx = self.cell_at(neighbor_coord.x, neighbor_coord.y);
                let neighbor_patterns = self.cells[neighbor_idx].patterns.clone();
                let neighbor_cell = &mut self.cells[neighbor_idx];

                for pattern in neighbor_patterns {
                    let possible = removal_update
                        .tile
                        .compatible
                        .iter()
                        .any(|c| c.0 == pattern.pattern && c.1 == dir);
                    if !possible {
                        neighbor_cell.remove_tile(pattern.clone(), freq);
                        if neighbor_cell.patterns.len() == 0 {
                            println!("Contradiction!"); // do something
                        }
                        self.entropy_queue.push(CoordEntropy {
                            entropy: MinFloat(neighbor_cell.entropy()),
                            coord: neighbor_coord,
                        });
                        self.tile_removals.push(RemovalUpdate {
                            tile: pattern,
                            coord: neighbor_coord,
                        });
                    }
                }
            }
        }
    }

    #[allow(dead_code)]
    pub fn print_cells(&self) {
        for (i, cell) in self.cells.iter().enumerate() {
            if cell.collapsed == true {
                println!("Cell {} is collapsed!", i);
            }
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
struct MinFloat(f32);

impl Eq for MinFloat {}

impl PartialOrd for MinFloat {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        other.0.partial_cmp(&self.0)
    }
}

impl Ord for MinFloat {
    fn cmp(&self, other: &MinFloat) -> Ordering {
        self.partial_cmp(other).unwrap()
    }
}

#[derive(PartialEq, Eq, Debug, Clone)]
struct CoordEntropy {
    entropy: MinFloat,
    coord: Point,
}

impl PartialOrd for CoordEntropy {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        other.entropy.partial_cmp(&self.entropy)
    }
}

impl Ord for CoordEntropy {
    fn cmp(&self, other: &Self) -> Ordering {
        if self < other {
            return Ordering::Less;
        }
        if self == other {
            return Ordering::Equal;
        }
        return Ordering::Greater;
    }
}

#[derive(Debug, Clone)]
pub struct RemovalUpdate {
    tile: MapTile,
    coord: Point,
}

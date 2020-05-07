use super::{Cell, MapTile, Point};
use crate::utils::directions::*;
use bracket_lib::prelude::RandomNumberGenerator;
use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashMap};

#[derive(Debug, Clone)]
pub struct Wave {
    cells: Vec<Cell>,
    pub uncollapsed_cells: usize,
    maptiles: Vec<MapTile>,
    entropy_queue: BinaryHeap<CoordEntropy>,
    tile_removals: Vec<RemovalUpdate>, // stack
    out_width: i32,
    out_height: i32,
}

#[allow(dead_code)]
impl Wave {
    pub fn new(cells: Vec<Cell>, maptiles: Vec<MapTile>, out_width: i32, out_height: i32) -> Self {
        let cells_len = cells.len(); // or out_width * out_height
        Self {
            cells,
            uncollapsed_cells: cells_len,
            maptiles,
            entropy_queue: BinaryHeap::new(),
            tile_removals: Vec::new(),
            out_height,
            out_width,
        }
    }

    /// Initialized the entropy queue.
    pub fn init_entropy_queue(&mut self) {
        for y in 0..self.out_height {
            for x in 0..self.out_width {
                let idx = self.cell_at(x, y);
                let cell = &self.cells[idx];
                self.entropy_queue.push(CoordEntropy {
                    entropy: MinFloat(cell.entropy()),
                    coord: Point::new(x, y),
                });
            }
        }
    }

    /// Returns the cell at (x, y) on the wave.
    fn cell_at(&self, x: i32, y: i32) -> usize {
        (y as usize * self.out_width as usize) + x as usize
    }

    /// Returns true if (x, y) is in the bounds of the wave; false otherwise.
    fn in_bound(&self, x: i32, y: i32) -> bool {
        x >= 0 && x < self.out_width && y > 0 && y < self.out_height
    }

    /// Given a tile index, returns all the compatible tiles it has.
    pub fn get_compatible_dir(&self, idx: usize, dir: Direction) -> Vec<usize> {
        self.maptiles[idx].get_compatible_dir(dir)
    }

    /// Select the next cell to collapse and return its coordinate in the wave.
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

    /// Collapses a cell at a given point.
    /// That is, remove all the possibilities except the only possible one.
    pub fn collapse_cell_at(
        &mut self,
        pt: Point,
        //freq: &HashMap<Vec<TileType>, f32>,
        freq: &HashMap<usize, f32>,
        rng: &mut RandomNumberGenerator,
    ) {
        let idx = self.cell_at(pt.x, pt.y);
        let mut cell = &mut self.cells[idx];
        let locked_tile = cell.choose_tile(freq, rng);

        cell.collapsed = true;

        let possibles = cell.possible.clone();
        for (idx, _p) in possibles.iter().enumerate() {
            if idx != locked_tile {
                //let t = tile.clone();
                cell.remove_tile(idx, freq);
                self.tile_removals.push(RemovalUpdate {
                    tile: idx,
                    coord: pt,
                });
            }
        }
    }

    /// Keeps propagating consequences until there are none (think like it's a sudoku game).
    pub fn propagate(&mut self, freq: &HashMap<usize, f32>) -> bool {
        while let Some(removal_update) = self.tile_removals.pop() {
            //let curr_possible_tiles = removal_update.tile.compatible;
            println!("NEW REMOVAL");

            // Iterate through each adjacent tile of the the current one.
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
                if !self.in_bound(neighbor_coord.x, neighbor_coord.y) {
                    println!("SKIP (NOT IN BOUNDS)");
                    continue;
                }
                let neighbor_idx = self.cell_at(neighbor_coord.x, neighbor_coord.y);
                if self.cells[neighbor_idx].collapsed {
                    println!("SKIP (COLLAPSED)");
                    continue;
                }
                //let neighbor_patterns = self.cells[neighbor_idx].patterns.clone();
                //let neighbor_cell = &mut self.cells[neighbor_idx];

                let compatible_tiles = self.get_compatible_dir(removal_update.tile, dir);

                for compat in compatible_tiles {
                    let j = opposite_idx(i); // Opposite direction to i
                                             //let j = i;

                    if self.cells[neighbor_idx].enabler_count[compat].by_direction[j] == 1 {
                        println!("ONE!");
                        println!("{:?}", self.cells[neighbor_idx].enabler_count[compat]);
                        /*
                        if self.cells[neighbor_idx].enabler_count[compat].any_zero() {
                            println!("HAS ZERO");
                            self.cells[neighbor_idx].remove_tile(compat, freq);
                        }
                        if self.cells[neighbor_idx].patterns.len() == 0 {
                            println!("Contradiction!"); // do something
                            return false;
                        }
                        */
                        if self.cells[neighbor_idx].possible[compat] {
                            self.cells[neighbor_idx].remove_tile(compat, freq);
                            self.entropy_queue.push(CoordEntropy {
                                entropy: MinFloat(self.cells[neighbor_idx].entropy()),
                                coord: neighbor_coord,
                            });
                            self.tile_removals.push(RemovalUpdate {
                                tile: compat,
                                coord: neighbor_coord,
                            });
                        }
                    }

                    //println!("{:?}", self.cells[neighbor_idx].enabler_count[compatible_tile]);
                    //println!("{}", self.cells[neighbor_idx].enabler_count[compatible_tile].by_direction[j]);
                    self.cells[neighbor_idx].enabler_count[compat].by_direction[j] -= 1;
                }

                /*
                let mut idx = 0;
                for pattern in neighbor_patterns {
                    /*
                    for possible in removal_update.tile.compatible.iter() {
                        if pattern.pattern == possible.0 {
                            println!("Pattern:  {:?}, {:?}", pattern.pattern, dir);
                            println!("Possible: {:?}, {:?}", possible.0, possible.1);
                        }
                    }
                    */
                    let compatible_dirs = removal_update.tile.get_compatible_dir(dir);
                    //println!("Compatibles: {:?}\n", compatible_dirs);
                    let possible = compatible_dirs.iter().any(|c| *c == pattern.pattern);
                    /*
                    let possible = removal_update
                        .tile
                        .compatible
                        .iter()
                        .any(|c| c.0 == pattern.pattern && c.1 == dir);
                    */
                    if !possible {
                        //println!("LEN: {}", neighbor_cell.patterns.len());
                        neighbor_cell.remove_tile(&pattern, freq);
                        // Problems here! is tile compatibility wrong?
                        if neighbor_cell.patterns.len() == 0 {
                            println!("Contradiction!"); // do something
                            return false;
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
                    idx += 1;
                }
                */
            }
        }
        true
    }

    /*
    #[allow(dead_code)]
    pub fn print_collapsed_cells(&self) {
        for (i, cell) in self.cells.iter().enumerate() {
            if cell.collapsed == true {
                println!("Cell {} is collapsed!", i);
                for (j, _pattern) in cell.patterns.iter().enumerate() {
                    println!("\t{}", j);
                }
            }
        }
    }
    */
}

#[derive(Debug, Clone)]
pub struct RemovalUpdate {
    //tile: MapTile,
    tile: usize,
    coord: Point,
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

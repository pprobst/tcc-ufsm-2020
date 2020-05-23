use super::{
    common::{connect_regions, count_neighbor_tile},
    get_all_regions,
    region::Operations,
    Map, Point, Tile, TileType,
};

/*
 *
 * cellular_automata.rs
 * --------------------
 * Cellular Automata cave generation.
 *
 * http://www.roguebasin.com/index.php?title=Cellular_Automata_Method_for_Generating_Random_Cave-Like_Levels
 * https://github.com/vurmux/urizen/blob/master/urizen/generators/dungeons/dungeon_cellular.py
 * https://github.com/SPIGS/Polymorph/blob/master/src/level_generation/cellular_automata.rs
 */

#[allow(dead_code)]
pub struct CellularAutomata {
    n_iterations: u8, // the more iterations we have, the smoother the map will be
    n_walls_rule: u8,
    min_cave_size: usize,
    open_halls: bool,
    dry_caves: bool,
}

#[allow(dead_code)]
impl CellularAutomata {
    pub fn new(
        n_iterations: u8,
        n_walls_rule: u8,
        min_cave_size: usize,
        open_halls: bool,
        dry_caves: bool,
    ) -> Self {
        Self {
            n_iterations,
            n_walls_rule,
            min_cave_size,
            open_halls,
            dry_caves,
        }
    }

    pub fn generate(&mut self, map: &mut Map) {
        let w = map.width - 1;
        let h = map.height - 1;

        // We need to make a clone here because the already replaced cells MUST NOT
        // affect the current cell.
        let mut tiles = map.tiles.clone();

        for _i in 0..self.n_iterations {
            for y in 1..h {
                for x in 1..w {
                    let mut flag = false;
                    let curr_pt = Point::new(x, y);
                    let curr_idx = map.idx(x, y);
                    let wall_counter = count_neighbor_tile(map, curr_pt, TileType::Wall, true);
                    let water_counter =
                        count_neighbor_tile(map, curr_pt, TileType::ShallowWater, true);
                    if wall_counter >= self.n_walls_rule || (wall_counter == 0 && !self.open_halls)
                    {
                        tiles[curr_idx] = Tile::wall();
                        flag = true;
                    }
                    if water_counter > 2 && water_counter < 4 {
                        tiles[curr_idx] = Tile::shallow_water();
                        flag = true;
                    }
                    if water_counter >= 5 {
                        tiles[curr_idx] = Tile::deep_water();
                        flag = true;
                    }
                    if flag == false {
                        tiles[curr_idx] = Tile::floor();
                    }
                }
            }
        }

        map.tiles = tiles.clone();

        let mut main_caves = get_all_regions(map);
        let mut lesser_caves = main_caves.clone();

        // Get caves < min_cave_size.
        lesser_caves.retain(|a| a.len() < self.min_cave_size);

        // Get caves >= min_cave_size
        main_caves.retain(|a| a.len() >= self.min_cave_size);
        main_caves.sort_by(|a, b| b.len().cmp(&a.len()));

        for cave in lesser_caves {
            if self.dry_caves {
                cave.fill_region(map, TileType::Wall);
            } else {
                cave.fill_region(map, TileType::ShallowWater);
            }
        }

        if main_caves.len() > 2 && !self.dry_caves {
            let last_main_cave = main_caves[main_caves.len() - 1].clone();
            last_main_cave.fill_region(map, TileType::ShallowWater);
        }

        //main_caves.sort_by(|a, b| a[0].cmp(&b[0]));
        main_caves.sort_by(|a, b| map.idx_pos(a[0]).x.cmp(&map.idx_pos(b[0]).x));
        connect_regions(map, main_caves, TileType::Floor, true);
        self.smooth_map(map);
    }

    fn smooth_map(&self, map: &mut Map) {
        let mut tiles = map.tiles.clone();

        for _i in 0..self.n_iterations {
            for y in 1..map.height - 1 {
                for x in 1..map.width - 1 {
                    let curr_pt = Point::new(x, y);
                    let curr_idx = map.idx(x, y);
                    if !map.is_water(curr_idx) {
                        let wall_counter = count_neighbor_tile(map, curr_pt, TileType::Wall, false);
                        let water_counter =
                            count_neighbor_tile(map, curr_pt, TileType::ShallowWater, false);
                        let deep_counter =
                            count_neighbor_tile(map, curr_pt, TileType::DeepWater, false);
                        if wall_counter <= 1 {
                            tiles[curr_idx] = Tile::floor();
                        }
                        if water_counter >= 2 || deep_counter >= 1 {
                            tiles[curr_idx] = Tile::shallow_water();
                        }
                    }
                }
            }
        }

        map.tiles = tiles;
    }
}

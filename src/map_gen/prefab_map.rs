use super::{Map, Tile};
use bracket_lib::prelude::XpFile;

/*
 *
 * prefab_map.rs
 * -------------
 * Generates a map based on a prefabricated .xp map.
 * Based on the examples by TheBracket.
 *
 */

pub struct PrefabMap {
    template: &'static str,
    width: usize,
    height: usize,
}

impl PrefabMap {
    pub fn new(template: &'static str, width: usize, height: usize) -> Self {
        Self {
            template,
            width,
            height,
        }
    }

    pub fn generate(&mut self, map: &mut Map) {
        let prefab_map = XpFile::from_resource(self.template).unwrap();
        for layer in &prefab_map.layers {
            for y in 0..layer.height {
                for x in 0..layer.width {
                    let cell = layer.get(x, y).unwrap();
                    if map.in_map_bounds_xy(x as i32, y as i32) {
                        let idx = map.idx(x as i32, y as i32);
                        match (cell.ch as u8) as char {
                            '.' => {
                                map.tiles[idx] = Tile::floor();
                            }
                            ' ' => {
                                map.tiles[idx] = Tile::floor();
                            }
                            '#' => {
                                map.tiles[idx] = Tile::wall();
                            }
                            '~' => {
                                map.tiles[idx] = Tile::shallow_water();
                            }
                            '♣' => {
                                map.tiles[idx] = Tile::tree();
                            }
                            _ => {
                                break;
                            }
                        }
                    }
                }
            }
        }
    }
}

use super::{Map, Tile};
use bracket_lib::prelude::{XpFile, to_char};

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
}

#[allow(dead_code)]
impl PrefabMap {
    pub fn new(template: &'static str) -> Self {
        Self { template }
    }

    pub fn generate(&mut self, map: &mut Map) {
        map.tiles = vec![Tile::floor(); (map.width * map.height) as usize];
        let prefab_map = XpFile::from_resource(self.template).unwrap();

        for layer in &prefab_map.layers {
            for y in 0..layer.height {
                for x in 0..layer.width {
                    let cell = layer.get(x, y).unwrap();
                    //println!("{}", (cell.ch as u8));
                    //if map.in_map_bounds_xy(x as i32, y as i32) {
                    let idx = map.idx(x as i32, y as i32);
                    //map.paint_tile_char(idx, (cell.ch as u8) as char);
                    map.paint_tile_char(idx, to_char(cell.ch as u8));
                }
            }
        }
    }
}

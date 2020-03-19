use bracket_lib::prelude::{RandomNumberGenerator};
use super::{Map, Tile};

// Just a test!
pub fn random_map_gen(map: &mut Map) {
    for x in 0..map.width {
        let idx1 = map.idx(x, 0);
        map.tiles[idx1] = Tile::wall();
        let idx2 = map.idx(x, map.height-1);
        map.tiles[idx2] = Tile::wall();
    }
    for y in 0..map.height {
        let idx1 = map.idx(0, y);
        map.tiles[idx1] = Tile::wall();
        let idx2 = map.idx(map.width-1, y);
        map.tiles[idx2] = Tile::wall();
    }

    let mut rng = RandomNumberGenerator::new();

    for _i in 0..600 {
        let x = rng.roll_dice(1, map.width-1);
        let y = rng.roll_dice(1, map.height-1);
        let idx = map.idx(x, y);
        map.tiles[idx] = Tile::wall();
    }
}

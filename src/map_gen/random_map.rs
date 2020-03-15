use bracket_lib::prelude::{RandomNumberGenerator};
use super::{Map, Tile};

// Just a test!
pub fn random_map_gen(map: &mut Map) {
    for x in 0..80 {
        let idx1 = map.idx(x, 0);
        map.tiles[idx1] = Tile::wall();
        let idx2 = map.idx(x, 59);
        map.tiles[idx2] = Tile::wall();
    }
    for y in 0..60 {
        let idx1 = map.idx(0, y);
        map.tiles[idx1] = Tile::wall();
        let idx2 = map.idx(79, y);
        map.tiles[idx2] = Tile::wall();
    }

    let mut rng = RandomNumberGenerator::new();

    for _i in 0..400 {
        let x = rng.roll_dice(1, 79);
        let y = rng.roll_dice(1, 59);
        let idx = map.idx(x, y);
        map.tiles[idx] = Tile::wall();
    }
}

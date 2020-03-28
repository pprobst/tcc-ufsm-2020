use bracket_lib::prelude::{RandomNumberGenerator};
use super::{Map, Tile};

// Just a test!
pub fn random_map_gen(map: &mut Map) {
    for x in 1..map.width {
        let idx1 = map.idx(x, 1);
        map.tiles[idx1] = Tile::tree();
        let idx2 = map.idx(x, map.height-2);
        map.tiles[idx2] = Tile::wall();
    }
    for y in 1..map.height {
        let idx1 = map.idx(1, y);
        map.tiles[idx1] = Tile::wall();
        let idx2 = map.idx(map.width-2, y);
        map.tiles[idx2] = Tile::tree();
    }

    let mut rng = RandomNumberGenerator::new();

    for _i in 0..1500 {
        let x = rng.roll_dice(1, map.width-1);
        let y = rng.roll_dice(1, map.height-1);
        let idx = map.idx(x, y);
        let chance = rng.range(0, 11);
        if !map.tiles[idx].block {
            if chance == 0 { map.tiles[idx] = Tile::wall(); }
            else if chance == 1 { map.tiles[idx] = Tile::tree(); }
            else if chance == 2 { map.tiles[idx] = Tile::flower(); }
            else if chance > 2 && chance <= 9 { map.tiles[idx] = Tile::grass(); }
            else { map.tiles[idx] = Tile::tallgrass(); }
        }
    }
}

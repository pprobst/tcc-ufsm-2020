use bracket_lib::prelude::*;
//use specs::prelude::*;
mod tile;
use tile::{Tile};
mod random_map;
use random_map::*;

#[derive(Default, Clone)]
pub struct Map {
    pub tiles: Vec<Tile>,
    pub width: i32,
    pub height: i32,
	//pub spawn_point: (i32, i32),
	//pub exit_point: (i32, i32),
}

impl Map {
    pub fn idx(&self, x: i32, y: i32) -> usize {
        (y as usize * self.width as usize) + x as usize
    }
    pub fn new(width: i32, height: i32) -> Map {
        Map{
            tiles: vec![Tile::floor(); (width*height) as usize],
            width,
            height,
        }
    }
}

pub struct MapGenerator {
    pub map: Map
}

impl MapGenerator {
    pub fn new() -> Self {
        Self{
           map: Map::new(80, 60),
        }
    }
    pub fn gen_map(&mut self) {
        random_map_gen(&mut self.map);
        // future: apply_theme(map)
        println!("Map generated!");
    }

    pub fn get_map(&self) -> Map {
        self.map.clone()
    }
}

impl Algorithm2D for Map {
    fn dimensions(&self) -> Point {
        Point::new(self.width, self.height)
    }

}

impl BaseMap for Map {}

pub fn render_map(map: &Map, ctx: &mut BTerm) {
    let mut y = 0;
    let mut x = 0;

    let bg = RGB::from_f32(0., 0., 0.);
    for tile in map.tiles.iter() {
        ctx.set(x, y, tile.fg, bg, tile.glyph);

        x += 1;
        if x > 80 as i32 - 1 {
            x = 0;
            y += 1;
        }
    }
}

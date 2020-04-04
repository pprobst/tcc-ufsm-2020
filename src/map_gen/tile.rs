use bracket_lib::prelude::{RGB, to_cp437, ColorPair, BLACK};
use crate::utils::colors::*;

/*
 *
 * tile.rs
 * -------
 * Basic structure of every map tile.
 *
 */

#[derive(PartialEq, Eq, Hash, Copy, Clone)]
pub enum TileType {
    Empty,
    Wall,
    Floor,
    Grass,
    TallGrass,
    Flower,
    Tree,
    //Mushroom,
    //ShallowWater,
    //DeepWater,
}

impl Default for TileType {
    fn default() -> TileType {
        TileType::Empty
    }
}

#[derive(Copy, Clone, Default)]
pub struct Tile {
    pub ttype: TileType,
    pub block: bool,
    pub visible: bool,
    pub revealed: bool,
    // https://dwarffortresswiki.org/index.php/Character_table
    pub glyph: u8,
    //pub fg: RGB,
    pub color: ColorPair
    //pub entities: Vec<Entity> ! Can't have this because we need Copy, and Vec contains a pointer to
    //                            some variable amount of heap memory.
}

impl Tile {
    /*
    pub fn empty() -> Self {
        Self {
            ttype: TileType::Empty,
            block: false,
            glyph: to_cp437(' '),
            fg: RGB::named(BLACK),
            ..Default::default()
        }
    }
    */

    pub fn wall() -> Self {
        Self {
            ttype: TileType::Wall,
            block: true,
            glyph: to_cp437('█'),
            color: ColorPair::new(to_rgb(WALL_GRAY), RGB::named(BLACK)),
            ..Default::default()
        }
    }

    pub fn floor() -> Self {
        Self {
            ttype: TileType::Floor,
            glyph: to_cp437('.'),
            color: ColorPair::new(to_rgb(FLOOR_GRAY), RGB::named(BLACK)),
            ..Default::default()
        }
    }

    pub fn grass() -> Self {
        Self {
            ttype: TileType::Grass,
            glyph: to_cp437(','),
            color: ColorPair::new(to_rgb(GRASS_GREEN), RGB::named(BLACK)),
            ..Default::default()
        }
    }

    pub fn tallgrass() -> Self {
        Self {
            ttype: TileType::TallGrass,
            glyph: to_cp437('⌠'),
            color: ColorPair::new(to_rgb(GRASS_GREEN_DARKER), RGB::named(BLACK)),
            ..Default::default()
        }
    } 

    pub fn flower() -> Self {
        Self {
            ttype: TileType::Flower,
            glyph: to_cp437('¥'),
            color: ColorPair::new(to_rgb(FLOWER_MAGENTA), RGB::named(BLACK)),
            ..Default::default()
        }

    }

    pub fn tree() -> Self {
        Self {
            ttype: TileType::Tree,
            block: true,
            glyph: to_cp437('♣'),
            color: ColorPair::new(to_rgb(TREE_GREEN), RGB::named(BLACK)),
            ..Default::default()
        }
    }

    pub fn to_color(&mut self, new_fg: RGB) {
        self.color.fg = new_fg;
    }
}

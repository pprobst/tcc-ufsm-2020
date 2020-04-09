use bracket_lib::prelude::{RGB, RGBA, to_cp437, ColorPair, BLACK};
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
    pub glyph: u16,
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
            color: ColorPair::new(RGB::from_hex(WALL_GRAY).unwrap(), RGB::named(BLACK)),
            ..Default::default()
        }
    }

    pub fn floor() -> Self {
        Self {
            ttype: TileType::Floor,
            glyph: to_cp437('.'),
            color: ColorPair::new(RGB::from_hex(FLOOR_GRAY).unwrap(), RGB::named(BLACK)),
            ..Default::default()
        }
    }

    #[allow(dead_code)]
    pub fn grass() -> Self {
        Self {
            ttype: TileType::Grass,
            glyph: to_cp437(','),
            color: ColorPair::new(RGB::from_hex(GRASS_GREEN).unwrap(), RGB::named(BLACK)),
            ..Default::default()
        }
    }

    #[allow(dead_code)]
    pub fn tallgrass() -> Self {
        Self {
            ttype: TileType::TallGrass,
            glyph: to_cp437('⌠'),
            color: ColorPair::new(RGB::from_hex(GRASS_GREEN_DARKER).unwrap(), RGB::named(BLACK)),
            ..Default::default()
        }
    } 

    #[allow(dead_code)]
    pub fn flower() -> Self {
        Self {
            ttype: TileType::Flower,
            glyph: to_cp437('¥'),
            color: ColorPair::new(RGB::from_hex(FLOWER_MAGENTA).unwrap(), RGB::named(BLACK)),
            ..Default::default()
        }

    }

    #[allow(dead_code)]
    pub fn tree() -> Self {
        Self {
            ttype: TileType::Tree,
            block: true,
            glyph: to_cp437('♣'),
            color: ColorPair::new(RGB::from_hex(TREE_GREEN).unwrap(), RGB::named(BLACK)),
            ..Default::default()
        }
    }

    pub fn shadowed(&mut self) {
        self.color.fg = RGBA::from_hex(SHADOW).unwrap();
    }

    pub fn change_glyph(&mut self, newglyph: char) {
        self.glyph = to_cp437(newglyph);
    }
}

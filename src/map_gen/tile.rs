use crate::utils::colors::*;
use bracket_lib::prelude::{to_cp437, ColorPair, BLACK, RGB, RGBA};

/*
 *
 * tile.rs
 * -------
 * Basic structure of every map tile.
 *
 */

#[derive(Debug, PartialEq, Eq, Copy, Clone, Hash)]
pub enum TileType {
    Empty,
    Wall,
    Floor,
    Floor2,
    WoodenFloor,
    Door,
    Grass,
    TallGrass,
    Flower,
    Tree,
    //Mushroom,
    ShallowWater,
    DeepWater,
}

impl Default for TileType {
    fn default() -> TileType {
        TileType::Empty
    }
}

#[derive(Copy, Clone, Default, PartialEq)]
pub struct Tile {
    pub ttype: TileType,
    pub block: bool,
    pub visible: bool,
    pub revealed: bool,
    // https://dwarffortresswiki.org/index.php/Character_table
    pub glyph: u16,
    //pub fg: RGB,
    pub color: ColorPair, //pub entities: Vec<Entity> ! Can't have this because we need Copy, an Vec contains a pointer to
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

    pub fn floor2() -> Self {
        Self {
            ttype: TileType::Floor2,
            glyph: to_cp437('.'),
            color: ColorPair::new(RGB::from_hex(FLOOR_GRAY).unwrap(), RGB::named(BLACK)),
            ..Default::default()
        }
    }

    pub fn woodenfloor() -> Self {
        Self {
            ttype: TileType::WoodenFloor,
            glyph: to_cp437('_'),
            color: ColorPair::new(RGB::from_hex(FLOOR_WOOD).unwrap(), RGB::named(BLACK)),
            ..Default::default()
        }
    }

    pub fn door() -> Self {
        Self {
            ttype: TileType::Door,
            glyph: to_cp437('+'),
            color: ColorPair::new(RGB::from_hex(DOOR_ORANGE).unwrap(), RGB::named(BLACK)),
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
            color: ColorPair::new(
                RGB::from_hex(GRASS_GREEN_DARKER).unwrap(),
                RGB::named(BLACK),
            ),
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

    #[allow(dead_code)]
    pub fn deep_water() -> Self {
        Self {
            ttype: TileType::DeepWater,
            glyph: to_cp437('~'),
            color: ColorPair::new(
                RGB::from_hex(WATER_BLUE).unwrap(),
                RGBA::from_hex(DEEP_BLUE).unwrap(),
            ),
            ..Default::default()
        }
    }

    #[allow(dead_code)]
    pub fn shallow_water() -> Self {
        Self {
            ttype: TileType::ShallowWater,
            glyph: to_cp437('~'),
            color: ColorPair::new(
                RGB::from_hex(WATER_BLUE).unwrap(),
                RGBA::from_hex(SHALLOW_BLUE).unwrap(),
            ),
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

use crate::utils::colors::*;
use bracket_lib::prelude::{to_cp437, ColorPair};

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
    InvisibleWall,
    Floor,
    Floor2,
    WoodenFloor,
    ClosedDoor,
    OpenDoor,
    Grass,
    Grass2,
    TallGrass,
    Flower,
    Tree,
    //Mushroom,
    ShallowWater,
    DeepWater,
    Computer,
    FakeMob,
}

impl Default for TileType {
    fn default() -> TileType {
        TileType::Empty
    }
}

#[derive(Copy, Clone, Default, PartialEq, Debug)]
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

#[allow(dead_code)]
impl Tile {
    pub fn empty() -> Self {
        Self {
            ttype: TileType::Empty,
            block: false,
            glyph: to_cp437(' '),
            color: ColorPair::new(color("Background", 1.0), color("Background", 1.0)),
            ..Default::default()
        }
    }

    pub fn wall() -> Self {
        Self {
            ttype: TileType::Wall,
            block: true,
            glyph: to_cp437('#'),
            color: ColorPair::new(color("White", 1.0), color("Background", 1.0)),
            ..Default::default()
        }
    }

    pub fn invisible_wall() -> Self {
        // Can't believe I'm doing this.
        Self {
            ttype: TileType::InvisibleWall,
            block: true,
            glyph: to_cp437(' '),
            color: ColorPair::new(color("Background", 1.0), color("Background", 1.0)),
            ..Default::default()
        }
    }

    pub fn floor() -> Self {
        Self {
            ttype: TileType::Floor,
            glyph: to_cp437('.'),
            color: ColorPair::new(color("BrightBlack", 1.0), color("Background", 1.0)),
            ..Default::default()
        }
    }

    pub fn floor2() -> Self {
        Self {
            ttype: TileType::Floor2,
            glyph: to_cp437('.'),
            color: ColorPair::new(color("BrightBlack", 1.0), color("Background", 1.0)),
            ..Default::default()
        }
    }

    pub fn woodenfloor() -> Self {
        Self {
            ttype: TileType::WoodenFloor,
            glyph: to_cp437('_'),
            color: ColorPair::new(color("Black", 1.0), color("Background", 1.0)),
            ..Default::default()
        }
    }

    pub fn closed_door() -> Self {
        Self {
            ttype: TileType::ClosedDoor,
            glyph: to_cp437('+'),
            block: true,
            color: ColorPair::new(color("BrightRed", 1.0), color("Background", 1.0)),
            ..Default::default()
        }
    }

    pub fn open_door() -> Self {
        Self {
            ttype: TileType::OpenDoor,
            glyph: to_cp437('/'),
            color: ColorPair::new(color("BrightRed", 1.0), color("Background", 1.0)),
            ..Default::default()
        }
    }

    pub fn grass() -> Self {
        Self {
            ttype: TileType::Grass,
            glyph: to_cp437(','),
            color: ColorPair::new(color("BrightGreen", 1.0), color("Background", 1.0)),
            ..Default::default()
        }
    }

    pub fn grass2() -> Self {
        Self {
            ttype: TileType::Grass2,
            glyph: to_cp437('`'),
            color: ColorPair::new(color("BrightYellow", 1.0), color("Background", 1.0)),
            ..Default::default()
        }
    }

    pub fn tallgrass() -> Self {
        Self {
            ttype: TileType::TallGrass,
            glyph: to_cp437('⌠'),
            color: ColorPair::new(color("Green", 1.0), color("Background", 1.0)),
            ..Default::default()
        }
    }

    pub fn flower() -> Self {
        Self {
            ttype: TileType::Flower,
            glyph: to_cp437('¥'),
            color: ColorPair::new(color("BrightMagenta", 1.0), color("Background", 1.0)),
            ..Default::default()
        }
    }

    pub fn tree() -> Self {
        Self {
            ttype: TileType::Tree,
            block: true,
            glyph: to_cp437('♣'),
            color: ColorPair::new(color("Green", 1.0), color("Background", 1.0)),
            ..Default::default()
        }
    }

    pub fn deep_water() -> Self {
        Self {
            ttype: TileType::DeepWater,
            glyph: to_cp437('≈'),
            color: ColorPair::new(color("Cyan", 1.0), color("Blue", 1.0)),
            ..Default::default()
        }
    }

    pub fn shallow_water() -> Self {
        Self {
            ttype: TileType::ShallowWater,
            glyph: to_cp437('~'),
            color: ColorPair::new(color("Cyan", 1.0), color("BrightBlue", 1.0)),
            ..Default::default()
        }
    }

    pub fn computer() -> Self {
        Self {
            ttype: TileType::Computer,
            block: true,
            glyph: to_cp437('Φ'),
            color: ColorPair::new(color("Cyan", 1.0), color("Background", 1.0)),
            ..Default::default()
        }
    }

    pub fn fakemob() -> Self {
        Self {
            ttype: TileType::FakeMob,
            block: true,
            glyph: to_cp437('g'),
            color: ColorPair::new(color("Red", 1.0), color("Background", 1.0)),
            ..Default::default()
        }
    }

    pub fn shadowed(&mut self) {
        self.color.fg = color("Shadow", 1.0);
    }

    pub fn reload_color(&mut self) {
        match self.ttype {
            TileType::Floor => {
                self.color = ColorPair::new(color("BrightBlack", 1.0), color("Background", 1.0));
            }
            TileType::WoodenFloor => {
                self.color = ColorPair::new(color("BrightBlack", 1.0), color("Background", 1.0));
            }
            TileType::ClosedDoor | TileType::OpenDoor => {
                self.color = ColorPair::new(color("BrightRed", 1.0), color("Background", 1.0));
            }
            TileType::Tree => {
                self.color = ColorPair::new(color("Green", 1.0), color("Background", 1.0));
            }
            TileType::Wall => {
                self.color = ColorPair::new(color("White", 1.0), color("Background", 1.0));
            }
            TileType::ShallowWater => {
                self.color = ColorPair::new(color("Cyan", 1.0), color("BrightBlue", 1.0));
            }
            TileType::DeepWater => {
                self.color = ColorPair::new(color("Cyan", 1.0), color("Blue", 1.0));
            }
            TileType::Grass => {
                self.color = ColorPair::new(color("BrightGreen", 1.0), color("Background", 1.0));
            }
            TileType::Grass2 => {
                self.color = ColorPair::new(color("BrightYellow", 1.0), color("Background", 1.0));
            }
            TileType::TallGrass => {
                self.color = ColorPair::new(color("Green", 1.0), color("Background", 1.0));
            }
            TileType::Flower => {
                self.color = ColorPair::new(color("BrightMagenta", 1.0), color("Background", 1.0));
            }
            TileType::Computer => {
                self.color = ColorPair::new(color("Magenta", 1.0), color("Background", 1.0));
            }
            TileType::FakeMob => {
                self.color = ColorPair::new(color("Red", 1.0), color("Background", 1.0));
            }
            _ => {
                self.color = ColorPair::new(color("Background", 1.0), color("Background", 1.0));
            }
        }
    }

    pub fn change_glyph(&mut self, newglyph: char) {
        self.glyph = to_cp437(newglyph);
    }
}

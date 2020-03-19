use bracket_lib::prelude::{RGB, to_cp437};

#[derive(PartialEq, Eq, Hash, Copy, Clone)]
pub enum TileType {
    Empty,
    Wall,
    Floor,
    Tree,
    Mushroom,
    ShallowWater,
    DeepWater,
}

#[derive(Copy, Clone)]
pub struct Tile {
    pub ttype: TileType,
    pub visible: bool,
    pub revealed: bool,
    pub block: bool,
    pub glyph: u8,
    pub fg: RGB
}

impl Tile {
    pub fn wall() -> Self {
        Self {
            ttype: TileType::Wall,
            block: true,
            visible: false,
            revealed: false,
            glyph: to_cp437('#'),
            fg: RGB::from_hex("#6ABE89").expect("Invalid hex string"),
        }
    }

    pub fn floor() -> Self {
        Self {
            ttype: TileType::Floor,
            block: false,
            visible: false,
            revealed: false,
            glyph: to_cp437('.'),
            fg: RGB::from_hex("#6ABE89").expect("Invalid hex string"),
        }
    }

    pub fn tree() -> Self {
        Self {
            ttype: TileType::Tree,
            block: false,
            visible: false,
            revealed: false,
            glyph: to_cp437('♣'),
            fg: RGB::from_hex("#6ABE89").expect("Invalid hex string"),
        }
    }
}

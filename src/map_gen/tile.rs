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
    pub glyph: u8,
    pub fg: RGB
}

impl Tile {
    pub fn wall() -> Self {
        Self {
            ttype: TileType::Wall,
            block: true,
            glyph: to_cp437('#'),
            fg: RGB::from_hex("#F9DFA7").expect("Invalid hex string"),
            ..Default::default()
        }
    }

    pub fn floor() -> Self {
        Self {
            ttype: TileType::Floor,
            glyph: to_cp437('.'),
            fg: RGB::from_hex("#F8E6C0").expect("Invalid hex string"),
            ..Default::default()
        }
    }

    pub fn tree() -> Self {
        Self {
            ttype: TileType::Tree,
            glyph: to_cp437('♣'),
            fg: RGB::from_hex("#6ABE89").expect("Invalid hex string"),
            ..Default::default()
        }
    }
}

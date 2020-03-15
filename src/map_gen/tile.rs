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

#[derive(Clone)]
pub struct Tile {
    pub ttype: TileType,
    pub block: bool,
    pub glyph: u8,
    pub fg: RGB
}

impl Tile {
    pub fn wall() -> Self {
        Self {
            ttype: TileType::Wall,
            block: true,
            glyph: to_cp437('#'),
            fg: RGB::from_hex("#636363").expect("Invalid hex string"),
        }
    }

    pub fn floor() -> Self {
        Self {
            ttype: TileType::Floor,
            block: false,
            glyph: to_cp437('.'),
            fg: RGB::from_hex("#373737").expect("Invalid hex string"),
        }
    }

    pub fn tree() -> Self {
        Self {
            ttype: TileType::Tree,
            block: false,
            glyph: to_cp437('♣'),
            fg: RGB::from_hex("#4DA25F").expect("Invalid hex string"),
        }
    }
}

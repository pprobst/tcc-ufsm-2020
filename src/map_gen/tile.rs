use bracket_lib::prelude::{RGB, to_cp437, BLACK};

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
    // https://dwarffortresswiki.org/index.php/Character_table
    pub glyph: u8,
    pub fg: RGB
}

impl Tile {
    pub fn empty() -> Self {
        Self {
            ttype: TileType::Empty,
            block: false,
            glyph: to_cp437(' '),
            fg: RGB::named(BLACK),
            ..Default::default()
        }
    }

 
    pub fn wall() -> Self {
        Self {
            ttype: TileType::Wall,
            block: true,
            glyph: to_cp437('█'),
            fg: RGB::from_hex("#C7C7C7").expect("Invalid hex string"),
            ..Default::default()
        }
    }

    pub fn floor() -> Self {
        Self {
            ttype: TileType::Floor,
            glyph: to_cp437('.'),
            fg: RGB::from_hex("#999A9C").expect("Invalid hex string"),
            ..Default::default()
        }
    }

    pub fn tree() -> Self {
        Self {
            ttype: TileType::Tree,
            block: true,
            glyph: to_cp437('♣'),
            fg: RGB::from_hex("#6ABE89").expect("Invalid hex string"),
            ..Default::default()
        }
    }

    pub fn to_color(&mut self, new_fg: RGB) {
        self.fg = new_fg;
    }
}

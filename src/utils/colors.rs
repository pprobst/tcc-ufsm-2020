use crate::raws::*;
use bracket_lib::prelude::{RGB, RGBA};

/*
 *
 * colors.rs
 * ---------
 * Just a file to store my colors in case I don't want to use bracket's RGB::named colors.
 *
 * In the future, this will be substituted by using a JSON/RON of colors.
 *
 */

//pub fn get_color(color: &str) -> String {
pub fn color(color: &str, alpha: f32) -> RGBA {
    RGB::from_hex(RAWS.lock().unwrap().get_curr_colorscheme().colors[color].to_string())
        .unwrap()
        .to_rgba(alpha)
}

pub const SHADOW: &str = "#2f2f4fff";
//pub const SHADOW: &str = &RAWS.lock().unwrap().get_curr_colorscheme().colors["SHADOW"];
pub const SHALLOW_BLUE: &str = "#005fafff";
pub const DEEP_BLUE: &str = "#004d8bff";
pub const WATER_BLUE: &str = "#0069be";
pub const WALL_GRAY: &str = "#949494";
pub const FLOOR_GRAY: &str = "#333333";
pub const FLOOR_WOOD: &str = "#46230F";
pub const DOOR_ORANGE: &str = "#AF5124";
pub const GRASS_GREEN: &str = "#61be67";
pub const GRASS_YELLOW: &str = "#EEB448";
pub const GRASS_GREEN_DARKER: &str = "#3ea346";
pub const TREE_GREEN: &str = "#194F1D";
pub const FLOWER_MAGENTA: &str = "#c074ab";
pub const BLOOD_RED: &str = "#B9281E";

// Targeting
pub const SELECTED_TARGET: &str = "#424242";

// UI
pub const UI_GRAY: &str = "#666666";
pub const UI_CYAN: &str = "#157fa1";

// Items
pub const MED_RED: &str = "#BA3155";
pub const SWORD_GRAY: &str = "#5F7D8B";

// Furniture
pub const CHEST_BROWN: &str = "#653D26";
pub const COMPUTER: &str = "#6F3176";

//use bracket_lib::prelude::RGB;

/*
 *
 * colors.rs
 * ---------
 * Just a file to store my colors in case I don't want to use bracket's RGB::named colors.
 *
 * In the future, this will be substituted by using a JSON/RON of colors.
 *
 */

//pub const SHADOW:              (u8, u8, u8)  =  (99,99,140);     // #63638c
pub const SHADOW:              &str = "#63638cff";
//pub const WALL_GRAY:           (u8, u8, u8)  =  (199,199,199);   // #c7c7c7
pub const WALL_GRAY:           &str = "#c7c7c7";
//pub const FLOOR_GRAY:          (u8, u8, u8)  =  (153,154,156);   // #999a9c
pub const FLOOR_GRAY:          &str = "#999a9c";
//pub const GRASS_GREEN:         (u8, u8, u8)  =  (97,190,103);    // #61be67
pub const GRASS_GREEN:         &str = "#61be67";
//pub const GRASS_GREEN_DARKER:  (u8, u8, u8)  =  (62,163,70);     // #3ea346
pub const GRASS_GREEN_DARKER:  &str = "#3ea346";
//pub const TREE_GREEN:          (u8, u8, u8)  =  (77,147,82);     // #4d9352
pub const TREE_GREEN:          &str = "#4d9352";
//pub const FLOWER_MAGENTA:      (u8, u8, u8)  =  (192,116,171);   // #c074ab
pub const FLOWER_MAGENTA:      &str = "#c074ab";
//pub const BLOOD_RED:           (u8, u8, u8)  =  (214,69,69);     // #d64545
pub const BLOOD_RED:           &str = "#d64545";

// Targeting
//pub const SELECTED_TARGET:     (u8, u8, u8)  =  (66,66,66);     // #424242
pub const SELECTED_TARGET:     &str = "#424242";

// UI
//pub const UI_GRAY:             (u8, u8, u8)  =  (102,102,102);   // #666666
pub const UI_GRAY:             &str = "#666666";
//pub const UI_CYAN:             (u8, u8, u8)  =  (21,127,161);    // #157fa1
pub const UI_CYAN:             &str = "#157fa1";

/*pub fn hex_to_rgb(c: (u8, u8, u8)) -> RGB {
    RGB::from_u8(c.0, c.1, c.2)
}*/

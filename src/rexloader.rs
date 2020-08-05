use bracket_lib::prelude::{embedded_resource, link_resource, EMBED};
//use bracket_lib::prelude::*;

/*
 * rexloader.rs
 * ------------
 * Loads .xp files generated from GridSage's awesome RexPaint program, a powerful and
 * easy-to-use ASCII art editor.
 *
 * See:
 * - https://www.gridsagegames.com/rexpaint/
 * - https://github.com/thebracket/bracket-lib/blob/master/bracket-terminal/examples/rex.rs
 *
 */

embedded_resource!(LEVEL01, "../resources/rex/level01_80x60.xp");
embedded_resource!(DUNGEON01, "../resources/rex/dungeon80x60.xp");
embedded_resource!(DUNGEON02, "../resources/rex/dungeon02_80x60.xp");
embedded_resource!(DUNGEON03, "../resources/rex/dungeon03_60x60.xp");
embedded_resource!(WFC01, "../resources/rex/wfc_20x20.xp");
embedded_resource!(WFC02, "../resources/rex/wfc_20x20_2.xp");
embedded_resource!(WFC03, "../resources/rex/wfc_20x20_3.xp");
embedded_resource!(WFC04, "../resources/rex/wfc_20x20_4.xp");
embedded_resource!(WFC05, "../resources/rex/wfc_6x6.xp");
embedded_resource!(WFC06, "../resources/rex/wfc_9x9.xp");
embedded_resource!(WFC07, "../resources/rex/wfc_20x20_1.xp");
embedded_resource!(WFC08, "../resources/rex/wfc_20x20_5.xp");
embedded_resource!(WFC09, "../resources/rex/wfc_15x15.xp");
embedded_resource!(WFC10, "../resources/rex/wfc_6x6_internal.xp");

pub fn load_dungeons() {
    link_resource!(LEVEL01, "resources/level01_80x60.xp");
    link_resource!(DUNGEON01, "resources/dungeon80x60.xp");
    link_resource!(DUNGEON02, "resources/dungeon02_80x60.xp");
    link_resource!(DUNGEON03, "resources/dungeon03_60x60.xp");
    link_resource!(WFC01, "resources/wfc_20x20.xp");
    link_resource!(WFC02, "resources/wfc_20x20_2.xp");
    link_resource!(WFC03, "resources/wfc_20x20_3.xp");
    link_resource!(WFC04, "resources/wfc_20x20_4.xp");
    link_resource!(WFC05, "resources/wfc_6x6.xp");
    link_resource!(WFC06, "resources/wfc_9x9.xp");
    link_resource!(WFC07, "resources/wfc_20x20_1.xp");
    link_resource!(WFC08, "resources/wfc_20x20_5.xp");
    link_resource!(WFC09, "resources/wfc_15x15.xp");
    link_resource!(WFC10, "resources/wfc_6x6_internal.xp");
}

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

embedded_resource!(DUNGEON01, "../rex_resources/dungeon80x60.xp");
embedded_resource!(DUNGEON02, "../rex_resources/dungeon02_80x60.xp");
embedded_resource!(DUNGEON03, "../rex_resources/dungeon03_60x60.xp");
embedded_resource!(WFC01, "../rex_resources/wfc_20x20.xp");
embedded_resource!(WFC02, "../rex_resources/wfc_20x20_2.xp");
embedded_resource!(WFC03, "../rex_resources/wfc_20x20_3.xp");
embedded_resource!(WFC04, "../rex_resources/wfc_20x20_4.xp");
embedded_resource!(WFC05, "../rex_resources/wfc_6x6.xp");
embedded_resource!(WFC06, "../rex_resources/wfc_9x9.xp");
embedded_resource!(WFC07, "../rex_resources/wfc_20x20_1.xp");
embedded_resource!(WFC08, "../rex_resources/wfc_20x20_5.xp");
embedded_resource!(WFC09, "../rex_resources/wfc_15x15.xp");
embedded_resource!(WFC10, "../rex_resources/wfc_6x6_internal.xp");

pub fn load_dungeons() {
    link_resource!(DUNGEON01, "../rex_resources/dungeon80x60.xp");
    link_resource!(DUNGEON02, "../rex_resources/dungeon02_80x60.xp");
    link_resource!(DUNGEON03, "../rex_resources/dungeon03_60x60.xp");
    link_resource!(WFC01, "../rex_resources/wfc_20x20.xp");
    link_resource!(WFC02, "../rex_resources/wfc_20x20_2.xp");
    link_resource!(WFC03, "../rex_resources/wfc_20x20_3.xp");
    link_resource!(WFC04, "../rex_resources/wfc_20x20_4.xp");
    link_resource!(WFC05, "../rex_resources/wfc_6x6.xp");
    link_resource!(WFC06, "../rex_resources/wfc_9x9.xp");
    link_resource!(WFC07, "../rex_resources/wfc_20x20_1.xp");
    link_resource!(WFC08, "../rex_resources/wfc_20x20_5.xp");
    link_resource!(WFC09, "../rex_resources/wfc_15x15.xp");
    link_resource!(WFC10, "../rex_resources/wfc_6x6_internal.xp");
}

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

pub fn load_dungeons() {
    link_resource!(DUNGEON01, "../rex_resources/dungeon80x60.xp");
    link_resource!(DUNGEON02, "../rex_resources/dungeon02_80x60.xp");
}

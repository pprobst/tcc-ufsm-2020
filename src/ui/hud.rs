use bracket_lib::prelude::*;
use specs::prelude::*;
use crate::utils::colors::*;
use crate::components::{Name, BaseStats};
use super::{WINDOW_WIDTH, WINDOW_HEIGHT, X_OFFSET, Y_OFFSET, Log};

/*
 * 
 * hud.rs
 * ------
 * Responsible for rendering and defining the player's HUD.
 *
 */

const X: i32 = WINDOW_WIDTH;
const Y: i32 = WINDOW_HEIGHT;
const MSG_HEIGHT_MIN: i32 = Y-Y_OFFSET*2; 
const MSG_HEIGHT_MAX: i32 = Y-Y_OFFSET-1; 

/// Renders the UI skeleton.
pub fn boxes(draw_batch: &mut DrawBatch) {
    let black = RGB::named(BLACK);
    let gray = to_rgb(UI_GRAY);

    draw_batch.draw_hollow_box(Rect::with_size(0, 0, X-1, Y-Y_OFFSET-1), ColorPair::new(gray, black)); // Screen borders
    draw_batch.draw_hollow_box(Rect::with_size(0, 0, X_OFFSET, Y-Y_OFFSET-1), ColorPair::new(gray, black)); // Left box
    draw_batch.draw_hollow_box(Rect::with_size(X_OFFSET, Y-Y_OFFSET*2, X-X_OFFSET-1, -Y_OFFSET+1), ColorPair::new(gray, black)); // Bottom box
    draw_batch.set(Point::new(X_OFFSET, Y-Y_OFFSET*2), ColorPair::new(gray, black), to_cp437('├'));
    draw_batch.set(Point::new(X-1, Y-Y_OFFSET*2), ColorPair::new(gray, black), to_cp437('┤'));
    draw_batch.set(Point::new(X_OFFSET, Y-Y_OFFSET-1), ColorPair::new(gray, black), to_cp437('┴'));
    draw_batch.set(Point::new(X_OFFSET, 0), ColorPair::new(gray, black), to_cp437('┬'));
}

/// Renders the player's name and their possible stats.
pub fn name_stats(ecs: &World, draw_batch: &mut DrawBatch) {
    let black = RGB::named(BLACK);
    let white = RGB::named(WHITE);
    let red = to_rgb(BLOOD_RED);
    let cyan = to_rgb(UI_CYAN);
    let player = ecs.fetch::<Entity>();
    let names = ecs.read_storage::<Name>();
    let stats = ecs.read_storage::<BaseStats>();

    let player_name = names.get(*player).unwrap();
    let pname = format!("{}", player_name.name);
    let player_stats = stats.get(*player).unwrap();
    let phealth = format!("{}/{}", player_stats.health.hp, player_stats.health.max_hp);

    let y = 3;
    let bar_end = X_OFFSET-7;

    draw_batch.print_color(Point::new(2, y), pname, ColorPair::new(white, black));
    draw_batch.set(Point::new(1, y), ColorPair::new(cyan, black), to_cp437('>'));
    draw_batch.set(Point::new(1, y+2), ColorPair::new(cyan, black), to_cp437('Ω'));
    draw_batch.bar_horizontal(
        Point::new(2, y+2), 
        bar_end, 
        player_stats.health.hp,
        player_stats.health.max_hp,
        ColorPair::new(red, black)
    );
    draw_batch.print_color(Point::new(bar_end+1, y+2), phealth, ColorPair::new(white, black));
}

/// Renders messages from the log structure.
pub fn game_log(ecs: &World, draw_batch: &mut DrawBatch) {
    let log = ecs.fetch::<Log>(); 
    let mut y = MSG_HEIGHT_MIN+1;

    for &(ref msg, color) in log.messages.iter().rev() {
        //println!("{}", msg);
        //println!("{}, {}", y, Y-Y_OFFSET-2);
        if y < MSG_HEIGHT_MAX {
            draw_batch.print_color(Point::new(X_OFFSET+1, y), msg, 
                                   ColorPair::new(color, RGB::named(BLACK)));
        }
        y += 1;
    }
}

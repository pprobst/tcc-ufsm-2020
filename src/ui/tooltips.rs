use super::{X_OFFSET, Y_OFFSET};
use crate::components::{Description, Name, Position};
use crate::map_gen::Map;
use crate::utils::colors::*;
use bracket_lib::prelude::*;
use specs::prelude::*;

/*
 *
 * tooltips.rs
 * -----------
 * Check information of entities on the game by hovering the mouse over them.
 * Easier than making a keyboard-based (l)ook system, as bracket-lib has mouse support.
 *
 */

struct Tooltip {
    lines: Vec<String>,
}

impl Tooltip {
    fn new() -> Tooltip {
        Tooltip { lines: Vec::new() }
    }

    fn add(&mut self, line: String) {
        if self.lines.len() == 1 {
            self.lines
                .push(format!("{}", "-".repeat(self.lines[0].len())));
        }
        let lines = line.lines();
        for l in lines {
            self.lines.push(l.to_string());
        }
    }

    fn width(&self) -> i32 {
        let mut max = 0;
        for s in self.lines.iter() {
            if s.len() > max {
                max = s.len();
            }
        }
        max as i32 + 2
    }

    fn height(&self) -> i32 {
        self.lines.len() as i32 + 2
    }

    fn render(&self, draw_batch: &mut DrawBatch, x: i32, y: i32) {
        let white = color("White", 1.0);
        let gray = color("BrightBlack", 1.0);
        let black = color("Background", 1.0);
        draw_batch.draw_box(
            Rect::with_size(x, y, self.width() - 1, self.height() - 1),
            ColorPair::new(gray, black),
        );
        draw_batch.fill_region(
            Rect::with_size(x + 1, y + 1, self.width() - 3, self.height() - 3),
            ColorPair::new(black, black),
            ' ' as u16,
        );
        for (i, s) in self.lines.iter().enumerate() {
            let fg = if i < 2 { white } else { gray };
            draw_batch.print_color(
                Point::new(x + 1, y + i as i32 + 1),
                &s,
                ColorPair::new(fg, black),
            );
        }
    }
}

pub fn show_tooltip(
    ecs: &World,
    term: &mut BTerm,
    draw_batch: &mut DrawBatch,
    min_x: i32,
    min_y: i32,
) {
    let map = ecs.fetch::<Map>();
    let mut mouse_pos = term.mouse_pos();
    mouse_pos.0 += min_x - X_OFFSET;
    mouse_pos.1 += min_y + Y_OFFSET;

    let names = ecs.read_storage::<Name>();
    let descriptions = ecs.read_storage::<Description>();
    let positions = ecs.read_storage::<Position>();

    let mut tooltips: Vec<Tooltip> = Vec::new();
    for (name, descr, pos) in (&names, &descriptions, &positions).join() {
        let idx = map.idx(pos.x, pos.y);
        if mouse_pos.0 == pos.x && mouse_pos.1 == pos.y && map.is_visible(idx) {
            let mut ttip = Tooltip::new();
            ttip.add(name.name.to_string());
            ttip.add(descr.descr.to_string());
            tooltips.push(ttip);
        }
    }
    if tooltips.is_empty() {
        return;
    }

    for tooltip in tooltips.iter() {
        let x = term.mouse_pos().0 + 1;
        let y = term.mouse_pos().1 + 1;
        tooltip.render(draw_batch, x, y);
    }
}

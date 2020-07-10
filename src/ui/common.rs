use crate::utils::colors::*;
use bracket_lib::prelude::*;
use std::collections::HashMap;

pub fn draw_list_items(
    items: &HashMap<String, u32>,
    items_vec: &Vec<String>,
    x1: i32,
    y1: i32,
    w: i32,
    draw_batch: &mut DrawBatch,
) {
    let black = color("Background", 1.0);
    let white = color("White", 1.0);
    let mut i = 0;
    let mut y = y1 + 1;
    for item in items_vec.iter() {
        //for item in items.iter() {
        draw_batch.set(
            Point::new(x1 + 1, y),
            ColorPair::new(white, black),
            97 + i as FontCharType,
        );
        draw_batch.print_color(
            Point::new(x1 + 2, y),
            //format!(") {}", &item.0),
            format!(") {}", item),
            ColorPair::new(white, black),
        );
        //let x2 = x1 + (item.0.len() as i32) + 4;
        let x2 = x1 + (item.len() as i32) + 4;
        let ct = (x1 + w) - x2 - 4;
        draw_batch.print_color(
            Point::new(x2, y),
            //format!(" {} x{}", ".".repeat(ct as usize), &item.1),
            format!(
                " {} x{}",
                ".".repeat(ct as usize),
                &items.get(item).unwrap()
            ),
            ColorPair::new(white, black),
        );
        i += 1;
        y += 1;
    }
}

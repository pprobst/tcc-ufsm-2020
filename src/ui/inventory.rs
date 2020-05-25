use super::{Log, WINDOW_HEIGHT, WINDOW_WIDTH, X_OFFSET, Y_OFFSET};
use crate::components::{Name, InBackpack};
use crate::utils::colors::*;
use bracket_lib::prelude::*;
use specs::prelude::*;
use std::collections::HashMap;

const X: i32 = WINDOW_WIDTH;
const Y: i32 = WINDOW_HEIGHT;

#[derive(PartialEq, Copy, Clone)]
pub enum InventoryResult { Cancel, NoResponse, Selected }

pub fn show_inventory(ecs: &World, term: &mut BTerm, draw_batch: &mut DrawBatch) -> InventoryResult {
    let names = ecs.read_storage::<Name>();
    let player = ecs.fetch::<Entity>();
    let backpack = ecs.read_storage::<InBackpack>();

    let black = RGB::named(BLACK);
    let white = RGB::named(WHITE);
    let gray = RGB::from_hex(UI_GRAY).unwrap();

    let x1 = X_OFFSET+5;
    let y1 = 4;
    let w = X-X_OFFSET-10;
    let h = Y-Y_OFFSET-9;

    draw_batch.draw_box(
        Rect::with_size(x1, y1, w, h),
        ColorPair::new(gray, black),
    ); 

    draw_batch.print_color(
        Point::new(w-5, y1),
        "·INVENTORY·",
        ColorPair::new(gray, black),
    );

    let mut items: HashMap<String, u32> = HashMap::new();
    for (_pack, name) in (&backpack, &names).join().filter(|item| item.0.owner == *player ) {
        let item_name = name.name.to_string();
        *items.entry(item_name).or_insert(0) += 1;
    }

    let mut i = 0;
    let mut y = y1+1;

    for item in items {
        draw_batch.set(
            Point::new(x1+1, y),
            ColorPair::new(white, black),
            97+i as FontCharType,
        );
        draw_batch.print_color(
            Point::new(x1+2, y),
            format!(". {}", &item.0),
            ColorPair::new(white, black),
        );
        let x2 = x1+(item.0.len() as i32)+4;
        let ct = (x1+w)-x2-4;
        draw_batch.print_color(
            Point::new(x2, y),
            format!(" {} x{}", ".".repeat(ct as usize), &item.1),
            ColorPair::new(white, black),
        );

        i += 1;
        y += 1;
    }

    match term.key {
        None => InventoryResult::NoResponse,
        Some(key) => {
            match key {
                VirtualKeyCode::Escape => { InventoryResult::Cancel }
                _ => InventoryResult::NoResponse
            }
        }
    }
}

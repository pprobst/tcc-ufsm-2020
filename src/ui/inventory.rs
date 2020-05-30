use super::{WINDOW_HEIGHT, WINDOW_WIDTH, X_OFFSET, Y_OFFSET};
use crate::components::{ConsumeItem, DropItem, InBackpack, Name, SelectedItem};
use crate::utils::colors::*;
use bracket_lib::prelude::*;
use specs::prelude::*;
use std::collections::HashMap;

const X: i32 = WINDOW_WIDTH;
const Y: i32 = WINDOW_HEIGHT;

#[derive(PartialEq, Copy, Clone)]
pub enum InventoryResult {
    Select,
    Cancel,
    Idle,
    DropItem,
    UseItem,
}

pub fn show_inventory(
    ecs: &World,
    term: &mut BTerm,
    draw_batch: &mut DrawBatch,
) -> InventoryResult {
    let names = ecs.read_storage::<Name>();
    let player = ecs.fetch::<Entity>();
    let backpack = ecs.read_storage::<InBackpack>();
    let entities = ecs.entities();

    let black = RGB::named(BLACK);
    let white = RGB::named(WHITE);
    let gray = RGB::from_hex(UI_GRAY).unwrap();

    let x1 = X_OFFSET + 5;
    let y1 = 4;
    let w = X - X_OFFSET - 10;
    let h = Y - Y_OFFSET - 9;

    draw_batch.draw_box(Rect::with_size(x1, y1, w, h), ColorPair::new(gray, black));

    let mut items: HashMap<String, u32> = HashMap::new();
    let mut items_vec: Vec<String> = Vec::new();
    let mut items_ent: Vec<Entity> = Vec::new();

    let mut item_count = 0;
    for (_pack, name, ent) in (&backpack, &names, &entities)
        .join()
        .filter(|item| item.0.owner == *player)
    {
        let item_name = name.name.to_string();
        *items.entry(item_name).or_insert(0) += 1;

        if !items_vec.contains(&name.name.to_string()) {
            items_vec.push(name.name.to_string());
            items_ent.push(ent);
        }
        item_count += 1;
    }

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

    draw_batch.print_color(
        Point::new(w - 5, y1),
        "·INVENTORY·",
        ColorPair::new(gray, black),
    );

    let count_w = if item_count < 10 { w - 2 } else { w - 3 };
    draw_batch.print_color(
        Point::new(count_w, y1 + h),
        format!("({}/26)", item_count),
        ColorPair::new(gray, black),
    );

    let items_len = items.len() as i32;
    match term.key {
        None => InventoryResult::Idle,
        Some(key) => match key {
            VirtualKeyCode::Escape => InventoryResult::Cancel,
            _ => {
                let select = letter_to_option(key);
                if select >= 0 && select < items_len {
                    let mut selected = ecs.write_storage::<SelectedItem>();
                    let selected_item = items_ent[select as usize];
                    selected
                        .insert(
                            selected_item,
                            SelectedItem {
                                item: selected_item,
                            },
                        )
                        .expect("Could not select item.");
                    InventoryResult::Select
                } else {
                    InventoryResult::Idle
                }
            }
        },
    }
}

/// Show the item use menu.
/// Options:
/// -- Use item
/// -- Drop item
pub fn show_use_menu(ecs: &World, term: &mut BTerm, draw_batch: &mut DrawBatch) -> InventoryResult {
    let mut selected_item = ecs.write_storage::<SelectedItem>();
    let names = ecs.read_storage::<Name>();
    let entities = ecs.entities();
    let player_ent = ecs.fetch::<Entity>();

    let item = (&selected_item, &names, &entities)
        .join()
        .collect::<Vec<_>>()[0];

    let black = RGB::named(BLACK);
    let white = RGB::named(WHITE);
    let gray = RGB::from_hex(UI_GRAY).unwrap();

    let x1 = X_OFFSET + 20;
    let y1 = 20;
    //let w = X - X_OFFSET*3;
    let w = item.1.name.len() as i32 + 1;
    let h = 5; // Number of lines + 1

    draw_batch.draw_box(Rect::with_size(x1, y1, w, h), ColorPair::new(gray, black));

    draw_batch.print_color(
        Point::new(x1 + 1, y1 + 1),
        format!("{}", item.1.name),
        ColorPair::new(gray, black),
    );

    draw_batch.print_color(
        Point::new(x1 + 1, y1 + 2),
        format!("{}", "-".repeat(w as usize - 1)),
        ColorPair::new(gray, black),
    );

    draw_batch.set(
        Point::new(x1 + 1, y1 + 3),
        ColorPair::new(white, black),
        101 as FontCharType,
    );

    draw_batch.print_color(
        Point::new(x1 + 2, y1 + 3),
        format!(") Use item."),
        ColorPair::new(white, black),
    );

    draw_batch.set(
        Point::new(x1 + 1, y1 + 4),
        ColorPair::new(white, black),
        100 as FontCharType,
    );

    draw_batch.print_color(
        Point::new(x1 + 2, y1 + 4),
        format!(") Drop item."),
        ColorPair::new(white, black),
    );

    match term.key {
        None => InventoryResult::Idle,
        Some(key) => match key {
            VirtualKeyCode::Escape => InventoryResult::Cancel,
            VirtualKeyCode::D => {
                let mut drop = ecs.write_storage::<DropItem>();
                drop.insert(
                    *player_ent,
                    DropItem {
                        item: item.0.item,
                        dropper: *player_ent,
                    },
                )
                .expect("FAILED to drop item.");
                selected_item.clear();
                InventoryResult::DropItem
            }
            VirtualKeyCode::E => {
                // TODO: check type of item used, because action may change.
                let mut use_item = ecs.write_storage::<ConsumeItem>();
                use_item
                    .insert(
                        *player_ent,
                        ConsumeItem {
                            target: *player_ent,
                            item: item.0.item,
                        },
                    )
                    .expect("FAILED to drop item.");
                selected_item.clear();
                InventoryResult::UseItem
            }
            _ => InventoryResult::Idle,
        },
    }
}

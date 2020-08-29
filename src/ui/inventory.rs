use super::{common::draw_list_items, WINDOW_HEIGHT, WINDOW_WIDTH, X_OFFSET, Y_OFFSET};
use crate::components::{
    ConsumeItem, DropItem, Equipable, Equipment, Inventory, InventoryCapacity, Name, SelectedItem,
    TryEquip,
};
use crate::utils::colors::*;
use bracket_lib::prelude::*;
use specs::prelude::*;
use std::collections::HashMap;

/*
 *
 * inventory.rs
 * ------------
 * UI regarding the inventory screen.
 *
 */

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
    let backpack = ecs.read_storage::<Inventory>();
    let inventory_cap = ecs.read_storage::<InventoryCapacity>();
    let entities = ecs.entities();

    let black = color("Background", 1.0);
    let gray = color("BrightBlack", 1.0);

    let x1 = X_OFFSET + 5;
    let y1 = 10;
    let w = X - X_OFFSET - 10;
    let h = Y - Y_OFFSET - 25;

    draw_batch.draw_box(Rect::with_size(x1, y1, w, h), ColorPair::new(gray, black));
    draw_batch.fill_region(
        Rect::with_size(x1 + 1, y1 + 1, w - 2, h - 2),
        ColorPair::new(black, black),
        ' ' as u16,
    );

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

    items_vec.sort();
    items_ent.sort_by(|a, b| {
        names
            .get(*a)
            .unwrap()
            .name
            .cmp(&names.get(*b).unwrap().name)
    });

    draw_batch.print_color(
        Point::new(w - 5, y1),
        "·INVENTORY·",
        ColorPair::new(gray, black),
    );

    draw_list_items(&items, &items_vec, x1, y1, w, draw_batch);

    let count_w = if item_count < 10 { w - 2 } else { w - 3 };
    draw_batch.print_color(
        Point::new(count_w, y1 + h),
        format!(
            "({}/{})",
            item_count,
            inventory_cap.get(*player).unwrap().max
        ),
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
    let equipable = ecs.read_storage::<Equipable>();

    let item = (&selected_item, &names, &entities)
        .join()
        .collect::<Vec<_>>()[0];

    let is_equip = equipable.get(item.2);

    let black = color("Background", 1.0);
    let white = color("White", 1.0);
    let gray = color("BrightBlack", 1.0);

    let x1 = X_OFFSET + 22;
    let y1 = 20;
    let w = i32::max(15, item.1.name.len() as i32 + 1);
    let h = 5; // Number of lines + 1

    draw_batch.draw_box(Rect::with_size(x1, y1, w, h), ColorPair::new(gray, black));
    draw_batch.fill_region(
        Rect::with_size(x1 + 1, y1 + 1, w - 2, h - 2),
        ColorPair::new(black, black),
        ' ' as u16,
    );

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
        match is_equip {
            None => format!(") Use item."),
            _ => format!(") Equip item."),
        },
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
            VirtualKeyCode::Escape => {
                selected_item.clear();
                InventoryResult::Cancel
            }
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
                match is_equip {
                    None => {
                        let mut use_item = ecs.write_storage::<ConsumeItem>();
                        use_item
                            .insert(
                                *player_ent,
                                ConsumeItem {
                                    target: *player_ent,
                                    item: item.0.item,
                                },
                            )
                            .expect("FAILED to use item.");
                    }
                    _ => {
                        let mut equip_item = ecs.write_storage::<TryEquip>();
                        equip_item
                            .insert(
                                *player_ent,
                                TryEquip {
                                    equipment: {
                                        Equipment {
                                            user: *player_ent,
                                            equip: item.0.item,
                                        }
                                    },
                                },
                            )
                            .expect("FAILED to try to equip item.");
                    }
                };
                selected_item.clear();
                InventoryResult::UseItem
            }
            _ => InventoryResult::Idle,
        },
    }
}

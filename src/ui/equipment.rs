use super::{
    common::draw_list, common::draw_named_box, WINDOW_HEIGHT, WINDOW_WIDTH, X_OFFSET, Y_OFFSET,
};
use crate::components::{
    Ammunition, ConsumeItem, DropItem, Equipable, Equipment, Inventory, InventoryCapacity, Name,
    SelectedItem, TryEquip,
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
pub enum EquipmentResult {
    Select,
    Cancel,
    Idle,
    DropEquip,
    Equip,
    Unequip,
}

pub fn show_equipment(
    ecs: &World,
    term: &mut BTerm,
    draw_batch: &mut DrawBatch,
) -> EquipmentResult {
    let names = ecs.read_storage::<Name>();
    let player = ecs.fetch::<Entity>();
    let equipments = ecs.read_storage::<Equipment>();
    let entities = ecs.entities();

    let black = color("Background", 1.0);
    let gray = color("BrightBlack", 1.0);

    let mut equips_vec: Vec<(String, Entity)> = Vec::new();

    for (_equip, name, ent) in (&equipments, &names, &entities)
        .join()
        .filter(|e| e.0.user == *player)
    {
        let equip_name = name.name.to_string();
        //equips_names_vec.push(equip_name);
        equips_vec.push((equip_name, ent));
    }

    equips_vec.sort();

    let x1 = X_OFFSET + 5;
    let y1 = 10;
    let w = X - X_OFFSET - 10;
    let h = Y - Y_OFFSET - 25;

    draw_named_box("·EQUIPMENT·", x1, y1, w, h, draw_batch);
    let equips_names_vec: Vec<String> = equips_vec.clone().into_iter().map(|x| x.0).collect();
    draw_list(equips_names_vec, x1, y1, w, draw_batch);

    let equips_len = equips_vec.len() as i32;
    match term.key {
        None => EquipmentResult::Idle,
        Some(key) => match key {
            VirtualKeyCode::Escape => EquipmentResult::Cancel,
            _ => {
                let select = letter_to_option(key);
                if select >= 0 && select < equips_len {
                    let mut selected = ecs.write_storage::<SelectedItem>();
                    let selected_equip = equips_vec[select as usize].1;
                    selected
                        .insert(
                            selected_equip,
                            SelectedItem {
                                item: selected_equip,
                            },
                        )
                        .expect("Could not select item.");
                    EquipmentResult::Select
                } else {
                    EquipmentResult::Idle
                }
            }
        },
    }
}

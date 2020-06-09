use super::{common::draw_list_items, WINDOW_HEIGHT, WINDOW_WIDTH, X_OFFSET, Y_OFFSET};
use crate::components::{CollectItem, Contained, Name, SelectedPosition};
use crate::map_gen::Map;
use crate::utils::colors::*;
use bracket_lib::prelude::*;
use specs::prelude::*;
use std::collections::HashMap;

/*
 *
 * containers.rs
 * -------------
 * UI regarding containers of items (e.g. loot, treasure chests, etc.).
 * Very similar to inventory.rs, so this will probably be refactored one day.
 *
 */

const X: i32 = WINDOW_WIDTH;
const Y: i32 = WINDOW_HEIGHT;

#[derive(PartialEq, Copy, Clone)]
pub enum ContainerResult {
    Select,
    Cancel,
    Idle,
}

pub fn show_container(
    ecs: &World,
    term: &mut BTerm,
    draw_batch: &mut DrawBatch,
) -> ContainerResult {
    let names = ecs.read_storage::<Name>();
    let selected_pos = ecs.read_storage::<SelectedPosition>();
    let map = ecs.fetch::<Map>();
    let entities = ecs.entities();

    let black = RGB::named(BLACK);
    let gray = RGB::from_hex(UI_GRAY).unwrap();

    let sel_pos = (&selected_pos).join().collect::<Vec<_>>()[0];
    let idx = map.idx(sel_pos.pos.x, sel_pos.pos.y);
    let container_ent = map.entities[idx];

    if container_ent == None {
        return ContainerResult::Cancel;
    }

    let mut items: HashMap<String, u32> = HashMap::new();
    let mut items_vec: Vec<String> = Vec::new();
    let mut items_ent: Vec<Entity> = Vec::new();

    let mut contained = ecs.write_storage::<Contained>();

    for (_c, name, ent) in (&contained, &names, &entities)
        .join()
        .filter(|item| item.0.container == container_ent.unwrap())
    {
        let item_name = name.name.to_string();
        *items.entry(item_name).or_insert(0) += 1;

        if !items_vec.contains(&name.name.to_string()) {
            items_vec.push(name.name.to_string());
            items_ent.push(ent);
        }
    }

    items_vec.sort();
    items_ent.sort_by(|a, b| {
        names
            .get(*a)
            .unwrap()
            .name
            .cmp(&names.get(*b).unwrap().name)
    });

    let x1 = X_OFFSET + 5;
    let y1 = 4;
    let w = X - X_OFFSET - 10;
    let h = Y - Y_OFFSET - 9;

    draw_batch.draw_box(Rect::with_size(x1, y1, w, h), ColorPair::new(gray, black));

    let container_name = &names
        .get(container_ent.unwrap())
        .unwrap()
        .name
        .to_uppercase();
    draw_batch.print_color(
        Point::new(w - ((container_name.len() as i32 + 2) / 2), y1),
        format!("·{}·", container_name),
        ColorPair::new(gray, black),
    );

    draw_list_items(&items, &items_vec, x1, y1, w, draw_batch);

    match term.key {
        None => ContainerResult::Idle,
        Some(key) => match key {
            VirtualKeyCode::Escape => ContainerResult::Cancel,
            _ => {
                let select = letter_to_option(key);
                if select >= 0 && select < items.len() as i32 {
                    let player = ecs.fetch::<Entity>();
                    let mut collect = ecs.write_storage::<CollectItem>();
                    let selected_item = items_ent[select as usize];
                    //contained.remove(selected_item);
                    CollectItem::add_collect(&mut collect, selected_item, *player);
                    return ContainerResult::Select;
                }
                ContainerResult::Idle
            }
        },
    }
}

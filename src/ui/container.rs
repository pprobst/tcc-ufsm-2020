use super::{WINDOW_HEIGHT, WINDOW_WIDTH, X_OFFSET, Y_OFFSET};
use crate::components::{
    Name, Item, Container, SelectedPosition,
};
use crate::map_gen::Map;
use crate::utils::colors::*;
use bracket_lib::prelude::*;
use specs::prelude::*;
use std::collections::HashMap;

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
    let player = ecs.fetch::<Entity>();
    let selected_pos = ecs.read_storage::<SelectedPosition>();
    let map = ecs.fetch::<Map>();
    let entities = ecs.entities();

    let black = RGB::named(BLACK);
    let white = RGB::named(WHITE);
    let gray = RGB::from_hex(UI_GRAY).unwrap();

    let sel_pos = (&selected_pos).join().collect::<Vec<_>>()[0];
    let idx = map.idx(sel_pos.pos.x, sel_pos.pos.y);
    let ent = map.entities[idx];

    if ent == None {  return ContainerResult::Cancel; }

    let x1 = X_OFFSET + 5;
    let y1 = 4;
    let w = X - X_OFFSET - 10;
    let h = Y - Y_OFFSET - 9;

    draw_batch.draw_box(Rect::with_size(x1, y1, w, h), ColorPair::new(gray, black));

    let mut i = 0;
    let mut y = y1 + 1;

    match term.key {
        None => ContainerResult::Idle,
        Some(key) => match key {
            VirtualKeyCode::Escape => ContainerResult::Cancel,
            _ => { ContainerResult::Idle
                /*
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
                    ContainerResult::Select
                } else {
                    ContainerResult::Idle
                }
                */
            }
        },
    }
}

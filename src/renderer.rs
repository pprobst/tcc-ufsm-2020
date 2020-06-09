use super::{
    map_gen::Map, ui::*, utils::colors::*, Position, Renderable, RunState, Target, WINDOW_HEIGHT,
    WINDOW_WIDTH, X_OFFSET, Y_OFFSET,
};
use bracket_lib::prelude::*;
use specs::prelude::*;

/*
 *
 * renderer.rs
 * -----------
 * Controls the rendering of everything on the screen.
 *
 */

pub struct Renderer<'a> {
    pub ecs: &'a World,
    pub term: &'a mut BTerm,
    pub state: RunState,
}

pub fn render_all(ecs: &World, term: &mut BTerm, state: RunState, show_map: bool) {
    Renderer { ecs, term, state }.render_all(show_map)
}

impl<'a> Renderer<'a> {
    /*pub fn new(ecs: &'a World, term: &'a mut BTerm) -> Self {
        Self { ecs, term }
    }*/

    /// Renders all the elements of the game.
    /// * Map;
    /// * Entities;
    /// * UI.
    pub fn render_all(&mut self, show_map: bool) {
        let (min_x, max_x, min_y, max_y, x_offset, y_offset) = self.screen_bounds();
        let mut draw_batch = DrawBatch::new();

        draw_batch.target(0);
        draw_batch.cls();
        self.render_map(
            &mut draw_batch,
            show_map,
            min_x,
            max_x,
            min_y,
            max_y,
            x_offset,
            y_offset,
        );
        if !show_map {
            self.render_entitites(&mut draw_batch, min_x, min_y, x_offset, y_offset);

            draw_batch.target(1);
            draw_batch.cls();
            self.render_ui(&mut draw_batch);
        }

        draw_batch.submit(0).expect("Batch error");
        render_draw_buffer(self.term).expect("Render error");
    }

    fn screen_bounds(&mut self) -> (i32, i32, i32, i32, i32, i32) {
        // https://www.reddit.com/r/roguelikedev/comments/8exy6o/brand_new_dev_struggling_with_a_scrolling_screen/
        // Player position.
        let ppos = self.ecs.fetch::<Point>();
        //println!("{}, {}", ppos.x, ppos.y);

        // Size of the map portion shown on screen.
        //let (cam_x, cam_y) = self.term.get_char_size();
        let (cam_x, cam_y) = (WINDOW_WIDTH - X_OFFSET, WINDOW_HEIGHT + Y_OFFSET);

        let min_x = ppos.x - (cam_x / 2) as i32;
        let max_x = min_x + cam_x as i32;
        let min_y = ppos.y - (cam_y / 2) as i32;
        let max_y = min_y + cam_y as i32;
        //println!("min_x: {}, max_x: {}, min_y: {}, max_y: {}", min_x, max_x, min_y, max_y);

        let x_offset = X_OFFSET;
        let y_offset = Y_OFFSET;

        (min_x, max_x, min_y, max_y - y_offset, x_offset, -y_offset)
    }

    /// Renders a targeting path between an origin point and a destiny point.
    fn render_line_path(
        &mut self,
        draw_batch: &mut DrawBatch,
        orig: Point,
        dest: Point,
        render: Renderable,
        covered: bool,
    ) {
        let points = line2d_vector(orig, dest);
        //let points = line2d_bresenham(orig, dest);
        if points.len() > 1 {
            for (i, pt) in points.iter().enumerate() {
                if i == points.len() - 1 {
                    draw_batch.set(
                        *pt,
                        ColorPair::new(render.color.fg, RGB::from_hex(SELECTED_TARGET).unwrap()),
                        render.glyph,
                    );
                } else if i != 0 {
                    if !covered {
                        draw_batch.set(
                            *pt,
                            ColorPair::new(RGB::from_hex(BLOOD_RED).unwrap(), RGB::named(BLACK)),
                            to_cp437('∙'),
                        );
                    } else {
                        draw_batch.set(
                            *pt,
                            ColorPair::new(RGB::from_hex(WALL_GRAY).unwrap(), RGB::named(BLACK)),
                            to_cp437('∙'),
                        );
                    }
                }
            }
        }
    }

    fn render_map(
        &mut self,
        draw_batch: &mut DrawBatch,
        show_map: bool,
        min_x: i32,
        max_x: i32,
        min_y: i32,
        max_y: i32,
        x_offset: i32,
        y_offset: i32,
    ) {
        let mut map = self.ecs.fetch_mut::<Map>();

        if show_map {
            let _map = map.clone();
            for (idx, tile) in map.tiles.iter_mut().enumerate() {
                let pos = _map.idx_pos(idx);
                draw_batch.set(Point::new(pos.x, pos.y), tile.color, tile.glyph);
            }
            return;
        }

        for (y, y2) in (min_y..max_y).enumerate() {
            for (x, x2) in (min_x..max_x).enumerate() {
                if map.in_map_bounds_xy(x2, y2) {
                    let idx = map.idx(x2, y2);
                    let mut tile = map.tiles[idx];
                    if !tile.visible {
                        tile.shadowed();
                    }
                    if tile.revealed {
                        draw_batch.set(
                            Point::new(x as i32 + x_offset, y as i32 + y_offset),
                            tile.color,
                            tile.glyph,
                        );
                    }
                } //else { draw_batch.set(Point::new(x as i32 + x_offset, y as i32 + y_offset), ColorPair::new(RGB::from_hex(WALL_GRAY).unwrap(), RGB::named(BLACK)), to_cp437('#')); }
            }
        }
    }

    fn render_entitites(
        &mut self,
        draw_batch: &mut DrawBatch,
        min_x: i32,
        min_y: i32,
        x_offset: i32,
        y_offset: i32,
    ) {
        let map = self.ecs.fetch::<Map>();
        let positions = self.ecs.read_storage::<Position>();
        let renderables = self.ecs.read_storage::<Renderable>();
        let targets = self.ecs.read_storage::<Target>();
        let entities = self.ecs.entities();

        let mut render_data = (&positions, &renderables, &entities)
            .join()
            .collect::<Vec<_>>();

        // Sorting renderables by layer: the renderables with layer 0 will be rendered first, that
        // is, bellow the renderables with layer 1 and so on.
        render_data.sort_by(|&a, &b| a.1.layer.cmp(&b.1.layer));

        for (pos, render, ent) in render_data {
            let idx = map.idx(pos.x, pos.y);
            if map.tiles[idx].visible {
                let ent_x = pos.x - min_x;
                let ent_y = pos.y - min_y;
                if map.in_map_bounds_xy(ent_x, ent_y) {
                    draw_batch.set(
                        Point::new(ent_x + x_offset, ent_y + y_offset),
                        render.color,
                        render.glyph,
                    );
                    let target = targets.get(ent);
                    if let Some(_target) = target {
                        let cover = _target.covered;
                        let pt = self.ecs.fetch::<Point>();
                        let ppos = *pt;
                        self.render_line_path(
                            draw_batch,
                            Point::new(ppos.x - min_x + x_offset, ppos.y - min_y + y_offset),
                            Point::new(ent_x + x_offset, ent_y + y_offset),
                            *render,
                            cover,
                        );
                    }
                }
            }
        }
    }

    fn render_ui(&mut self, draw_batch: &mut DrawBatch) {
        hud::boxes(draw_batch);
        hud::name_stats(self.ecs, draw_batch);
        hud::show_equipped(self.ecs, draw_batch);
        hud::game_log(self.ecs, draw_batch);
        let mut write_state = self.ecs.write_resource::<RunState>();
        match self.state {
            RunState::Inventory => {
                let inventory_result = inventory::show_inventory(self.ecs, self.term, draw_batch);
                if inventory_result == inventory::InventoryResult::Cancel {
                    *write_state = RunState::Running;
                } else if inventory_result == inventory::InventoryResult::Select {
                    *write_state = RunState::ItemUse;
                }
            }
            RunState::ItemUse => {
                let inventory_result = inventory::show_use_menu(self.ecs, self.term, draw_batch);
                if inventory_result == inventory::InventoryResult::Cancel {
                    *write_state = RunState::Running;
                } else if inventory_result == inventory::InventoryResult::DropItem
                    || inventory_result == inventory::InventoryResult::UseItem
                {
                    *write_state = RunState::MobTurn;
                }
            }
            RunState::AccessContainer => {
                let container_result = container::show_container(self.ecs, self.term, draw_batch);
                if container_result == container::ContainerResult::Cancel {
                    *write_state = RunState::MobTurn;
                } else if container_result == container::ContainerResult::Select {
                    *write_state = RunState::AccessContainer;
                }
            }
            _ => {}
        }
    }
}

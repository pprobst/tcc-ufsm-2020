use bracket_lib::prelude::*;
use specs::prelude::*;

use crate::components::{Position, Renderable};
use crate::map_gen::Map;

pub struct Renderer<'a> {
    pub ecs: &'a World,
    pub ctx: &'a mut BTerm
}

pub fn render_all(ecs: &World, ctx: &mut BTerm) {
    Renderer {
        ecs,
        ctx,
    }.render_all()
}

impl<'a> Renderer<'a> {
    /*pub fn new(ecs: &'a World, ctx: &'a mut BTerm) -> Self {
        Self { ecs, ctx }
    }*/
    
    pub fn render_all(&mut self) {
        let (min_x, max_x, min_y, max_y, x_offset, y_offset) = self.screen_bounds();
        self.render_map(min_x, max_x, min_y, max_y, x_offset, y_offset);
        self.render_entitites(min_x, min_y, x_offset, y_offset);
    }

    fn screen_bounds(&mut self) -> (i32, i32, i32, i32, i32, i32) {
        // https://www.reddit.com/r/roguelikedev/comments/8exy6o/brand_new_dev_struggling_with_a_scrolling_screen/
        // Player position.
        let ppos = self.ecs.fetch::<Point>();
        //println!("{}, {}", ppos.x, ppos.y);

        // Size of the map portion shown on screen.
        let (cam_x, cam_y) = self.ctx.get_char_size();
        //let (cam_x, cam_y) = (64, 50);

        let min_x = ppos.x - (cam_x / 2) as i32;
        let max_x = min_x + cam_x as i32;
        let min_y = ppos.y - (cam_y / 2) as i32;
        let max_y = min_y + cam_y as i32;
        //println!("min_x: {}, max_x: {}, min_y: {}, max_y: {}", min_x, max_x, min_y, max_y);

        let x_offset = 15;
        let y_offset = -7;

        (min_x, max_x, min_y, max_y, x_offset, y_offset)
    }

    fn render_map(&mut self, min_x: i32, max_x: i32, min_y: i32, max_y: i32, x_offset: i32, y_offset: i32) {
        let map = self.ecs.fetch::<Map>();
        let bg = RGB::from_f32(0., 0., 0.);

        for (y, y2) in (min_y .. max_y).enumerate() {
            for (x, x2) in (min_x .. max_x).enumerate() { 
                if map.in_map_bounds_xy(x2, y2) {
                    let idx = map.idx(x2, y2);
                    let tile = map.tiles[idx];
                    let mut fg = tile.fg;
                    if !tile.visible { fg = fg.to_greyscale(); }
                    if tile.revealed { self.ctx.set(x as i32 + x_offset, y as i32 + y_offset, fg, bg, tile.glyph); }
                } //else { self.ctx.set(x as i32 + x_offset, y as i32 + y_offset, RGB::named(GRAY), bg, to_cp437('.')); }
            }
        }
        /*
        for (idx, tile) in map.tiles.iter().enumerate() {
            let pos = map.idx_pos(idx);
            let mut fg = tile.fg;
            if !tile.visible { fg = fg.to_greyscale(); }
            if tile.revealed { self.ctx.set(pos.x, pos.y, fg, bg, tile.glyph); }
        }
        */
    }

    fn render_entitites(&mut self, min_x: i32, min_y: i32, x_offset: i32, y_offset: i32) {
        let map = self.ecs.fetch::<Map>();
        let positions = self.ecs.read_storage::<Position>();
        let renderables = self.ecs.read_storage::<Renderable>();

        for (pos, render) in (&positions, &renderables).join() {
            let idx = map.idx(pos.x, pos.y);
            if map.tiles[idx].visible {
                let ent_x = pos.x - min_x;
                let ent_y = pos.y - min_y;
                if map.in_map_bounds_xy(ent_x, ent_y) {
                    self.ctx.set(ent_x + x_offset, ent_y + y_offset, render.fg, render.bg, render.glyph);
                }
            }
        }
    }
}

use bracket_lib::prelude::*;
use specs::prelude::*;
use super::{
    X_OFFSET, Y_OFFSET, Position, Renderable, Target, map_gen::Map, utils::colors::*, ui::*
};

pub struct Renderer<'a> {
    pub ecs: &'a World,
    pub term: &'a mut BTerm
}

pub fn render_all(ecs: &World, term: &mut BTerm) {
    Renderer {
        ecs,
        term,
    }.render_all()
}

impl<'a> Renderer<'a> {
    /*pub fn new(ecs: &'a World, term: &'a mut BTerm) -> Self {
        Self { ecs, term }
    }*/
    
    pub fn render_all(&mut self) {
        let (min_x, max_x, min_y, max_y, x_offset, y_offset) = self.screen_bounds();
        let mut draw_batch = DrawBatch::new(); 

        draw_batch.target(0);
        draw_batch.cls();
        self.render_map(&mut draw_batch, min_x, max_x, min_y, max_y, x_offset, y_offset);
        self.render_entitites(&mut draw_batch, min_x, min_y, x_offset, y_offset);

        //draw_batch.target(1);
        //draw_batch.cls();
        self.render_ui(&mut draw_batch);

        draw_batch.submit(0);
    }

    fn screen_bounds(&mut self) -> (i32, i32, i32, i32, i32, i32) {
        // https://www.reddit.com/r/roguelikedev/comments/8exy6o/brand_new_dev_struggling_with_a_scrolling_screen/
        // Player position.
        let ppos = self.ecs.fetch::<Point>();
        //println!("{}, {}", ppos.x, ppos.y);

        // Size of the map portion shown on screen.
        let (cam_x, cam_y) = self.term.get_char_size();
        //let (cam_x, cam_y) = (64, 50);

        let min_x = ppos.x - (cam_x / 2) as i32;
        let max_x = min_x + cam_x as i32;
        let min_y = ppos.y - (cam_y / 2) as i32;
        let max_y = min_y + cam_y as i32;
        //println!("min_x: {}, max_x: {}, min_y: {}, max_y: {}", min_x, max_x, min_y, max_y);

        let x_offset = X_OFFSET;
        let y_offset = -Y_OFFSET;

        (min_x, max_x, min_y, max_y, x_offset, y_offset)
    }

    fn render_map(&mut self, draw_batch: &mut DrawBatch, min_x: i32, max_x: i32, min_y: i32, max_y: i32, x_offset: i32, y_offset: i32) {
        let map = self.ecs.fetch::<Map>();

        for (y, y2) in (min_y .. max_y).enumerate() {
            for (x, x2) in (min_x .. max_x).enumerate() { 
                if map.in_map_bounds_xy(x2, y2) {
                    let idx = map.idx(x2, y2);
                    let mut tile = map.tiles[idx];
                    let shadow_color = to_rgb(SHADOW);
                    if !tile.visible { tile.to_color(shadow_color); }
                    //if tile.revealed { self.term.set(x as i32 + x_offset, y as i32 + y_offset, tile.color.fg, tile.color.bg, tile.glyph); }
                    if tile.revealed { draw_batch.set(Point::new(x as i32 + x_offset, y as i32 + y_offset), tile.color, tile.glyph); }
                } //else { self.term.set(x as i32 + x_offset, y as i32 + y_offset, RGB::named(GRAY), bg, to_cp437('.')); }
            }
        }
        /*
        for (idx, tile) in map.tiles.iter().enumerate() {
            let pos = map.idx_pos(idx);
            let mut fg = tile.fg;
            if !tile.visible { fg = fg.to_greyscale(); }
            if tile.revealed { self.term.set(pos.x, pos.y, fg, bg, tile.glyph); }
        }
        */
    }

    fn render_path(&mut self, draw_batch: &mut DrawBatch, orig: usize, dest: usize) {
        let map = self.ecs.fetch::<Map>();
        // TODO: don't use A* for this.
        let a_star = a_star_search(orig, dest, &*map);
        for (i, step) in a_star.steps.iter().enumerate() {
            if a_star.steps.len() > 1 {
                if i != 0 && i != a_star.steps.len()-1 {
                    let pt = map.idx_pos(*step);
                    //self.term.set(pt.x, pt.y, to_rgb(BLOOD_RED), RGB::named(BLACK), to_cp437('∙'));
                    draw_batch.set(pt, ColorPair::new(to_rgb(BLOOD_RED), RGB::named(BLACK)), to_cp437('∙'));
                }
            }
        }
    }

    fn render_entitites(&mut self, draw_batch: &mut DrawBatch, min_x: i32, min_y: i32, x_offset: i32, y_offset: i32) {
        let map = self.ecs.fetch::<Map>();
        let positions = self.ecs.read_storage::<Position>();
        let renderables = self.ecs.read_storage::<Renderable>();
        let targets = self.ecs.read_storage::<Target>();
        let entities = self.ecs.entities();
        
        for (pos, render, ent) in (&positions, &renderables, &entities).join() {
            let idx = map.idx(pos.x, pos.y);
            if map.tiles[idx].visible {
                let ent_x = pos.x - min_x;
                let ent_y = pos.y - min_y;
                if map.in_map_bounds_xy(ent_x, ent_y) {
                    //self.term.set(ent_x + x_offset, ent_y + y_offset, render.color.fg, render.color.bg, render.glyph);
                    draw_batch.set(Point::new(ent_x + x_offset, ent_y + y_offset), render.color, render.glyph);
                    if targets.get(ent).is_some() {
                        let pt = self.ecs.fetch::<Point>();
                        let ppos = *pt;
                        self.render_path(draw_batch, map.idx(ent_x + x_offset, ent_y + y_offset), map.idx(ppos.x - min_x + x_offset, ppos.y - min_y + y_offset));
                    }
                }
            }
        }
    }

    fn render_ui(&mut self, draw_batch: &mut DrawBatch) {
       hud::boxes(draw_batch);
       hud::name_stats(self.ecs, draw_batch);
    }
}

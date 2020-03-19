use bracket_lib::prelude::*;
use specs::prelude::*;

use crate::components::{Position, Renderable};
use crate::map_gen::Map;

pub struct Renderer {}

impl<'a> Renderer {
    pub fn new() -> Renderer {
        return Renderer {};
    }
    
    pub fn render_all(&self, ecs: &World, ctx: &mut BTerm) {
        self.render_map(ecs, ctx);
        self.render_entitites(ecs, ctx);
    }

    fn render_map(&self, ecs: &World, ctx: &mut BTerm) {
        let map = &ecs.fetch::<Map>();
        let bg = RGB::from_f32(0., 0., 0.);

        for (idx, tile) in map.tiles.iter().enumerate() {
            let pos = map.idx_pos(idx);
            let mut fg = tile.fg;
            if !tile.visible { fg = fg.to_greyscale(); }
            if tile.revealed { ctx.set(pos.x, pos.y, fg, bg, tile.glyph); }
        }
    }

    fn render_entitites(&self, ecs: &World, ctx: &mut BTerm) {
        let positions = ecs.read_storage::<Position>();
        let renderables = ecs.read_storage::<Renderable>();
        for (pos, render) in (&positions, &renderables).join() {
            ctx.set(pos.x, pos.y, render.fg, render.bg, render.glyph);
        }
    }
}

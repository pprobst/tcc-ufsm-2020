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
        self.render_map(&ecs.fetch::<Map>(), ctx);
        self.render_entitites(ecs, ctx);
    }

    fn render_map(&self, map: &Map, ctx: &mut BTerm) {
        let mut y = 0;
        let mut x = 0;

        let bg = RGB::from_f32(0., 0., 0.);
        for tile in map.tiles.iter() {
            ctx.set(x, y, tile.fg, bg, tile.glyph);

            x += 1;
            if x > 80 as i32 - 1 {
                x = 0;
                y += 1;
            }
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

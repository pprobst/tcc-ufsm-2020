use bracket_lib::prelude::*;
use specs::prelude::*;

use crate::components::*;
use crate::player::*;
use crate::map_gen::*;

#[derive(PartialEq, Copy, Clone)]
pub enum RunState {
    Running,
    Waiting,
    Start,
}

pub struct State {
    pub ecs: World,
    pub runstate: RunState
}

impl State {
  pub fn new(world: World) -> Self {
    Self{ ecs: world, runstate: RunState::Start }
  }

  fn run_systems(&mut self) {
    self.ecs.maintain();
  }

  pub fn generate_map(&mut self) {
      let mut mapgen = MapGenerator::new();
      mapgen.gen_map();
      {
          let mut this_map = self.ecs.write_resource::<Map>();
          *this_map = mapgen.get_map();
      }
  }
}

impl GameState for State {
    fn tick(&mut self, ctx: &mut BTerm) {
        ctx.cls();
        // TODO: Change this to a match!
        if self.runstate == RunState::Start {
            // Do start stuff
            self.runstate = RunState::Running;
        } else if self.runstate == RunState::Running {
            self.run_systems();
            self.runstate = RunState::Waiting;
        } else {
            self.runstate = player_input(self, ctx);
        }

        let positions = self.ecs.read_storage::<Position>();
        let renderables = self.ecs.read_storage::<Renderable>();

        render_map(&self.ecs.fetch::<Map>(), ctx); 

        for (pos, render) in (&positions, &renderables).join() {
            ctx.set(pos.x, pos.y, render.fg, render.bg, render.glyph);
        }
    }
}

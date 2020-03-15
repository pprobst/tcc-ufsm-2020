use bracket_lib::prelude::*;
use specs::prelude::*;

use crate::components::*;
use crate::player::*;

#[derive(PartialEq, Copy, Clone)]
pub enum RunState {
    Running,
    Waiting,
}

pub struct State {
    pub ecs: World,
    pub runstate: RunState
}

impl State {
  pub fn new(world: World) -> Self {
    Self { ecs: world, runstate: RunState::Running }
  }

  fn run_systems(&mut self) {
    self.ecs.maintain();
  }
}

impl GameState for State {
    fn tick(&mut self, ctx: &mut BTerm) {
        ctx.cls();
        //ctx.print(1, 1, "Test");
        //
        if self.runstate == RunState::Running {
            self.run_systems();
            self.runstate = RunState::Waiting;
        } else {
            self.runstate = player_input(self, ctx);
        }

        let positions = self.ecs.read_storage::<Position>();
        let renderables = self.ecs.read_storage::<Renderable>();

        for (pos, render) in (&positions, &renderables).join() {
            ctx.set(pos.x, pos.y, render.fg, render.bg, render.glyph);
        }
    }
}

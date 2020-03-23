use bracket_lib::prelude::*;
use specs::prelude::*;

//use crate::components::*;
use crate::player::*;
use crate::map_gen::*;
use crate::renderer::{render_all};
use crate::systems::{visibility::VisibilitySystem};

#[derive(PartialEq, Copy, Clone)]
pub enum RunState {
    Running,
    Waiting,
    Start,
}

pub struct State {
    pub ecs: World,
    pub runstate: RunState,
    pub burn: bool,
}

impl State {
  pub fn new(world: World, burn: bool) -> Self {
    Self { 
        ecs: world, 
        runstate: RunState::Start, 
        burn 
    }
  }

  fn run_systems(&mut self) {
    let mut vis = VisibilitySystem{};
    vis.run_now(&self.ecs);
    self.ecs.maintain();
  }

  pub fn generate_map(&mut self) -> Map {
      let mut mapgen = MapGenerator::new();
      mapgen.gen_map();
      let mut this_map = self.ecs.write_resource::<Map>();
      *this_map = mapgen.get_map();
      mapgen.get_map()
  }
}

impl GameState for State {
    fn tick(&mut self, ctx: &mut BTerm) {
        ctx.cls();

        // F3 to enable/disable post-processing effects.
        match ctx.key {
            None => {}
            Some(key) => {
                if let VirtualKeyCode::F3 = key {
                    self.burn = !self.burn;
                    ctx.with_post_scanlines(self.burn);
                }
            }
        }

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

        render_all(&self.ecs, ctx);
        //self.renderer.render_all(&self.ecs, ctx);
    }
}

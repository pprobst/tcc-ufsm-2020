use bracket_lib::prelude::*;
use specs::prelude::*;

//use crate::components::*;
use crate::player::*;
use crate::map_gen::*;
use crate::renderer::{render_all};
use crate::systems::{visibility::VisibilitySystem, ai::HostileAISystem, mapping::MappingSystem};

#[derive(PartialEq, Copy, Clone)]
pub enum RunState {
    Running,
    Waiting,
    Start,
    PlayerTurn,
    MobTurn
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

    let mut hostile_ai = HostileAISystem{};
    hostile_ai.run_now(&self.ecs);

    let mut mapping = MappingSystem{};
    mapping.run_now(&self.ecs);

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
    fn tick(&mut self, term: &mut BTerm) {
        term.cls();

        // F3 to enable/disable post-processing effects.
        match term.key {
            None => {}
            Some(key) => {
                if let VirtualKeyCode::F3 = key {
                    self.burn = !self.burn;
                    term.with_post_scanlines(self.burn);
                }
            }
        }

        let mut curr_state;
        {
            let runstate = self.ecs.fetch::<RunState>();
            curr_state = *runstate;
        }

        // State machine.
        match curr_state {
            RunState::Start => {
                curr_state = RunState::Running;
            }
            RunState::Running => {
                self.run_systems();
                curr_state = RunState::Waiting;
            }
            RunState::Waiting => {
                curr_state = player_input(self, term);
            }
            RunState::PlayerTurn => {
                self.run_systems();
                curr_state = RunState::MobTurn;
            }
            RunState::MobTurn => {
                self.run_systems();
                curr_state = RunState::Waiting;
            }
        }

        let mut write_state = self.ecs.write_resource::<RunState>();
        *write_state = curr_state;

        render_all(&self.ecs, term);
    }
}

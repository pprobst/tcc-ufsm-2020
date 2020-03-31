use bracket_lib::prelude::*;
use specs::prelude::*;
use super::{
    input::*,
    map_gen::*,
    renderer::render_all,
    killer::remove_dead_entities,
    systems::{fov::FOVSystem, ai::HostileAISystem, mapping::MappingSystem, 
        melee::MeleeSystem, missile::MissileSystem, damage::DamageSystem}
};

#[derive(Debug, PartialEq, Copy, Clone)]
pub enum RunState {
    Running,
    Waiting,
    Start,
    PlayerTurn,
    MobTurn,
    Targeting
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
    let mut vis = FOVSystem{};
    vis.run_now(&self.ecs);

    let mut hostile_ai = HostileAISystem{};
    hostile_ai.run_now(&self.ecs);

    let mut mapping = MappingSystem{};
    mapping.run_now(&self.ecs);

    let mut melee = MeleeSystem{};
    melee.run_now(&self.ecs);

    let mut missile = MissileSystem{};
    missile.run_now(&self.ecs);

    let mut damage = DamageSystem{};
    damage.run_now(&self.ecs);


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
        //term.cls();

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
        // We need scope because we'll do mutable borrow later.
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
            RunState::Targeting => {
                curr_state = targeting_input(self, term);
            }
        }

        {
            let mut write_state = self.ecs.write_resource::<RunState>();
            *write_state = curr_state;
        }

        remove_dead_entities(&mut self.ecs);
        render_all(&self.ecs, term);
        render_draw_buffer(term);
    }
}

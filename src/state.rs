use super::{
    input::*,
    killer::remove_dead_entities,
    map_gen::*,
    renderer::render_all,
    systems::{
        ai::HostileAISystem, damage::DamageSystem, fov::FOVSystem, inventory::PickupSystem,
        item_drop::ItemDropSystem, mapping::MappingSystem, melee::MeleeSystem, missile::MissileSystem,
    },
};
use bracket_lib::prelude::*;
use specs::prelude::*;

/*
 *
 * state.rs
 * --------
 * Controls the running systems, game states and other main functions at every tick.
 *
 */

const SHOW_MAP: bool = true;

#[derive(Debug, PartialEq, Copy, Clone)]
pub enum RunState {
    Running,
    Waiting,
    Start,
    PlayerTurn,
    MobTurn,
    Targeting,
    Inventory,
    ItemUse,
    Mapgen,
}

pub struct State {
    pub ecs: World,
    pub runstate: RunState,
    pub show_map: bool,
}

impl State {
    pub fn new(world: World) -> Self {
        Self {
            ecs: world,
            runstate: RunState::Start,
            show_map: SHOW_MAP,
        }
    }

    fn run_systems(&mut self) {
        let mut vis = FOVSystem {};
        vis.run_now(&self.ecs);

        let mut hostile_ai = HostileAISystem {};
        hostile_ai.run_now(&self.ecs);

        let mut mapping = MappingSystem {};
        mapping.run_now(&self.ecs);

        let mut melee = MeleeSystem {};
        melee.run_now(&self.ecs);

        let mut missile = MissileSystem {};
        missile.run_now(&self.ecs);

        let mut damage = DamageSystem {};
        damage.run_now(&self.ecs);

        let mut pickup = PickupSystem {};
        pickup.run_now(&self.ecs);

        let mut drop = ItemDropSystem {};
        drop.run_now(&self.ecs);

        self.ecs.maintain();
    }

    pub fn generate_map(&mut self, width: i32, height: i32) -> Map {
        let mut mapgen = MapGenerator::new(width, height);
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
                    term.with_post_scanlines(false);
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
                if self.show_map {
                    curr_state = RunState::Mapgen;
                } else {
                    curr_state = RunState::Running;
                }
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
            RunState::Inventory => {
                curr_state = RunState::Inventory;
                // Will change state on rendering (messy, but sometimes we just need things to work).
            }
            RunState::ItemUse => {
                curr_state = RunState::ItemUse;
            }
            RunState::Mapgen => match term.key {
                None => {}
                Some(key) => {
                    if let VirtualKeyCode::Space = key {
                        self.generate_map(80, 60);
                    }
                    if let VirtualKeyCode::Return = key {
                        self.show_map = false;
                        curr_state = RunState::Running;
                    }
                }
            },
        }

        {
            let mut write_state = self.ecs.write_resource::<RunState>();
            *write_state = curr_state;
        }

        remove_dead_entities(&mut self.ecs);
        render_all(&self.ecs, term, curr_state, self.show_map);
    }
}

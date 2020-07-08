use super::{
    input::*,
    killer::remove_dead_entities,
    map_gen::*,
    raws::*,
    renderer::render_all,
    systems::{
        ai::HostileAISystem, consumable::ConsumableSystem, damage::DamageSystem,
        equipment::EquipmentSystem, fov::FOVSystem, item_collect::ItemCollectSystem,
        item_drop::ItemDropSystem, mapping::MappingSystem, melee::MeleeSystem,
        missile::MissileSystem,
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
    AccessContainer,
    Mapgen,
}

pub struct State {
    pub ecs: World,
    pub runstate: RunState,
    pub show_map: bool,
    pub map_generator: MapGenerator,
}

impl State {
    pub fn new(world: World) -> Self {
        Self {
            ecs: world,
            runstate: RunState::Start,
            show_map: SHOW_MAP,
            map_generator: MapGenerator::new(),
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

        let mut collect_item = ItemCollectSystem {};
        collect_item.run_now(&self.ecs);

        let mut drop_item = ItemDropSystem {};
        drop_item.run_now(&self.ecs);

        let mut consumable = ConsumableSystem {};
        consumable.run_now(&self.ecs);

        let mut equip = EquipmentSystem {};
        equip.run_now(&self.ecs);

        self.ecs.maintain();
    }

    fn run_collect_system(&mut self) {
        let mut collect_item = ItemCollectSystem {};
        collect_item.run_now(&self.ecs);
    }

    pub fn generate_new_map(&mut self, width: i32, height: i32) -> Map {
        self.map_generator.push_map(width, height);
        let idx = self.map_generator.maps.len() - 1;
        self.map_generator.gen_map(idx);
        self.set_curr_map(idx);
        self.map_generator.get_map(idx)
    }

    pub fn set_curr_map(&mut self, idx: usize) {
        let mut curr_map = self.ecs.write_resource::<Map>();
        *curr_map = self.map_generator.get_map(idx);
    }

    pub fn set_colorscheme(&mut self, colorscheme: &str) {
        &RAWS.lock().unwrap().set_curr_colorscheme(colorscheme);
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
                if let VirtualKeyCode::F5 = key {
                    self.set_colorscheme("elemental");
                }
                if let VirtualKeyCode::F6 = key {
                    self.set_colorscheme("ayu");
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
            RunState::AccessContainer => {
                self.run_collect_system();
                curr_state = RunState::AccessContainer;
            }
            RunState::Mapgen => match term.key {
                None => {}
                Some(key) => {
                    if let VirtualKeyCode::Space = key {
                        self.generate_new_map(80, 60);
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

use bracket_lib::prelude::*;
use specs::prelude::*;
use crate::components::{Position, Mob, Fov, MeleeAttack};
use crate::state::{RunState};
use crate::map_gen::Map;

pub struct HostileAISystem {}

impl<'a> System<'a> for HostileAISystem {
    type SystemData = (
        ReadStorage<'a, Mob>,
        ReadExpect<'a, Point>,
        ReadExpect<'a, Entity>,
        WriteExpect<'a, Map>,
        WriteStorage<'a, Fov>,
        WriteStorage<'a, Position>,
        ReadExpect<'a, RunState>,
        Entities<'a>,
        WriteStorage<'a, MeleeAttack>
    );

    fn run(&mut self, data: Self::SystemData) {
        let (mob, pt, player, mut map, mut fov, mut pos, runstate, entities, mut melee_attack) = data;
        let ppos = *pt;
        let map = &mut *map;

        if *runstate != RunState::MobTurn { return; }

        for (_mob, mut fov, mut pos, ent) in (&mob, &mut fov, &mut pos, &entities).join() {
            let d = DistanceAlg::Pythagoras.distance2d(Point::new(pos.x, pos.y), ppos);
            if d < 1.5 {
                println!("I'm touching you!");
                melee_attack.insert(
                    ent,
                    MeleeAttack {
                        target: *player
                    },
                )
                .expect("Melee attack insertion failed");
            }
            // https://github.com/thebracket/bracket-lib/blob/master/bracket-pathfinding/examples/astar/main.rs
            else if fov.visible_pos.contains(&ppos) {
                // TODO: first try missile attack; else chase player.
                let mob_location = map.idx(pos.x, pos.y);
                let player_location = map.idx(ppos.x, ppos.y);
                let a_star = a_star_search(mob_location, player_location, map);

                if a_star.success && a_star.steps.len() > 1 {
                    // Previous position is now unblocked.
                    map.clear_blocker(pos.x, pos.y);
                    pos.x = a_star.steps[1] as i32 % map.width;
                    pos.y = a_star.steps[1] as i32 / map.width;
                    // New position is now blocked.
                    //map.add_blocker(pos.x, pos.y);
                    fov.dirty = true;
                }
            }
        }
    }
}

use bracket_lib::prelude::*;
use specs::prelude::*;
use super::{
    RunState,
    Position, 
    Player, 
    Mob,
    Fov, 
    MeleeAttack,
    MissileAttack,
    Target,
    utils::directions::Direction,
    map_gen::Map,
};
use std::cmp::Ordering;

/*
 *
 * player.rs
 * ---------
 * Contains all the actions performed by the player.
 *
 */

/// Tries to move the player, performing melee attacks if needed.
pub fn move_player(dir: Direction, ecs: &mut World) {
    let mut pos_ = ecs.write_storage::<Position>();
    let mut player_ = ecs.write_storage::<Player>();
    let mut fov = ecs.write_storage::<Fov>();
    let map = ecs.fetch::<Map>();
    //let stats = ecs.read_storage::<BaseStats>();
    let mobs = ecs.read_storage::<Mob>();
    let entities = ecs.entities();
    
    for (_player, pos, fov, entity) in (&mut player_, &mut pos_, &mut fov, &entities).join() {
        let dir_x = dir.delta_x as i32;
        let dir_y = dir.delta_y as i32;
        let dest = map.idx(pos.x + dir_x, pos.y + dir_y);

        // Tries melee if you're trying to move into a occupied tile.
        for ent in map.entities[dest].iter() {
            //let t = stats.get(*ent);
            let t = mobs.get(*ent);
            if let Some(_t) = t {
                println!("Attacking enemy.");
                let mut melee_attack = ecs.write_storage::<MeleeAttack>();
                melee_attack.insert(
                        entity,
                        MeleeAttack {
                            target: *ent,
                        },
                    )
                .expect("Melee attack insertion failed");
            }
        }

        if !map.tiles[dest].block {
            pos.x = pos.x + dir_x; 
            pos.y = pos.y + dir_y; 
            let mut player_pos = ecs.write_resource::<Point>();
            player_pos.x = pos.x;
            player_pos.y = pos.y;
            fov.dirty = true;
        }
    }
}

/// Cycles between the player's visible targets.
pub fn choose_target(ecs: &mut World, up: bool) -> RunState {
    let vis_targets = visible_targets(ecs, true);
    let mut targets = ecs.write_storage::<Target>();
    let entities = ecs.entities();

    // Just return a waiting state if there aren't any visible targets.
    if vis_targets.len() < 1 { return RunState::Waiting }

    let mut curr_target: Option<Entity> = None;

    for (e, _t) in (&entities, &targets).join() {
        curr_target = Some(e);
    }

    targets.clear();

    if let Some(curr_target) = curr_target { // If there's already a target selected...
        let mut idx = 0;
        for (i, target) in vis_targets.iter().enumerate() {
            // Get index from current target.
            if target.0 == curr_target {
                idx = i;
            }
        }

        if !up && idx > 0 {
            let tgt = vis_targets[idx-1];
            targets.insert(tgt.0, Target{ map_idx: tgt.2 }).expect("Insert fail");
        } else {
            if idx+1 > vis_targets.len()-1 { idx = 0; }
            else  { idx += 1; }
            let tgt = vis_targets[idx];
            targets.insert(tgt.0, Target{ map_idx: tgt.2 }).expect("Insert fail");
        } 
    } else { // If there's not a target select already, select the first closest visible.
        let first_target = vis_targets[0];
        targets.insert(first_target.0, Target{ map_idx: first_target.2 }).expect("Insert fail");
    }

    RunState::Targeting
}

/// Performs a missile (ranged) attack to the selected entity.
pub fn missile_attack(ecs: &mut World) {
    // TODO: First check if the player readied weapon is ranged.
    let entities = ecs.entities();
    let mut targets = ecs.write_storage::<Target>();

    let mut curr_target: Option<Entity> = None;

    for (e, _t) in (&entities, &targets).join() {
        curr_target = Some(e);
    }

    targets.clear();

    if let Some(target) = curr_target {
        let player = ecs.fetch::<Entity>();
        let mut missile_attack = ecs.write_storage::<MissileAttack>();
        missile_attack.insert(*player, MissileAttack{ target }).expect("Missile attack insertion failed");
    } 
}

/// Cancels targeting, returning a Waiting state.
pub fn reset_targeting(ecs: &mut World) -> RunState {
    let mut targets = ecs.write_storage::<Target>();
    targets.clear();
    RunState::Waiting
}

/// Returns all the visible and/or hittable targets in the player's FOV ordered by distance to the player (cresc.).
fn visible_targets(ecs: &mut World, hittable: bool) -> Vec<(Entity, f32, usize)> {
    let player = ecs.read_storage::<Player>();
    let fov = ecs.read_storage::<Fov>();
    let map = ecs.fetch::<Map>();
    let mobs = ecs.read_storage::<Mob>();
    let positions = ecs.read_storage::<Position>();
    let player_ent = ecs.fetch::<Entity>();

    let mut visible_targets: Vec<(Entity, f32, usize)> = Vec::new(); // (entity, distance, map_idx)
    for (_player, fov) in (&player, &fov).join() {
       for pos in fov.visible_pos.iter() {
           let idx = map.idx(pos.x, pos.y);
            for ent in map.entities[idx].iter() {
                let t = mobs.get(*ent);
                if let Some(_t) = t {
                    let mobpos = Point::new(pos.x, pos.y);
                    let player_pos = positions.get(*player_ent).unwrap();
                    let ppos =  Point::new(player_pos.x, player_pos.y);
                    let mut can_hit = true;
                    if hittable {
                        let points = line2d_vector(ppos, mobpos);
                        //let points = line2d_bresenham(ppos, mobpos);
                        for pt in points.iter().take(points.len()-1) {
                            let i = map.idx(pt.x, pt.y);
                            // if there's a blocker in the aim line, you can't hit the entity.
                            if map.tiles[i].block { can_hit = false; }
                        }
                    }
                    if can_hit {
                        let dist = DistanceAlg::Pythagoras.distance2d(mobpos, ppos);
                        visible_targets.push((*ent, dist, idx));
                    }
                }
            }
        } 
    }

    visible_targets.sort_by(|a, b| a.1.partial_cmp(&b.1).unwrap_or(Ordering::Equal));
    visible_targets
}

/// Switches between the two readied weapons.
pub fn switch_weapon(ecs: &mut World) {
}

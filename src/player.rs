use bracket_lib::prelude::*;
use specs::prelude::*;
use super::{
    Position, 
    Player, 
    Fov, 
    BaseStats, 
    MeleeAttack,
    utils::directions::Direction,
    map_gen::Map,
};

pub fn move_player(dir: Direction, ecs: &mut World) {
    let mut pos_ = ecs.write_storage::<Position>();
    let mut player_ = ecs.write_storage::<Player>();
    let mut fov = ecs.write_storage::<Fov>();
    let map = ecs.fetch::<Map>();
    let stats = ecs.read_storage::<BaseStats>();
    let mut melee_attack = ecs.write_storage::<MeleeAttack>();
    let entities = ecs.entities();
    
    for (_player, pos, fov, entity) in (&mut player_, &mut pos_, &mut fov, &entities).join() {
        let dir_x = dir.delta_x as i32;
        let dir_y = dir.delta_y as i32;
        let dest = map.idx(pos.x + dir_x, pos.y + dir_y);

        // Tries melee.
        for ent in map.entities[dest].iter() {
            let t = stats.get(*ent);
            if let Some(_t) = t {
                println!("Attacking enemy.");
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

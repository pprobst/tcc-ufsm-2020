use bracket_lib::prelude::*;
use specs::prelude::*;
use crate::state::*;
use crate::map_gen::Map;
use super::{Position, Player, Fov};

struct Direction {
    delta_x: i8,
    delta_y: i8
}

const EAST: Direction = Direction { delta_x: 1, delta_y: 0 };
const WEST: Direction = Direction { delta_x: -1, delta_y: 0 };
const NORTH: Direction = Direction { delta_x: 0, delta_y: -1 };
const SOUTH: Direction = Direction { delta_x: 0, delta_y: 1 };
const NORTHEAST: Direction = Direction { delta_x: 1, delta_y: -1 };
const NORTHWEST: Direction = Direction { delta_x: -1, delta_y: -1 };
const SOUTHEAST: Direction = Direction { delta_x: 1, delta_y: 1 };
const SOUTHWEST: Direction = Direction { delta_x: -1, delta_y: 1 };

fn move_player(dir: Direction, ecs: &mut World) {
    let mut pos_ = ecs.write_storage::<Position>();
    let mut player_ = ecs.write_storage::<Player>();
    let mut fov = ecs.write_storage::<Fov>();
    let map = ecs.fetch::<Map>();
    
    // Increment position for all the entities with components 'Player' and 'Position'.
    for (_player, pos, fov) in (&mut player_, &mut pos_, &mut fov).join() {
        let dir_x = dir.delta_x as i32;
        let dir_y = dir.delta_y as i32;
        let destination = map.idx(pos.x + dir_x, pos.y + dir_y);

        if !map.tiles[destination].block {
            pos.x = pos.x + dir_x; 
            pos.y = pos.y + dir_y; 
            let mut player_pos = ecs.write_resource::<Point>();
            player_pos.x = pos.x;
            player_pos.y = pos.y;
            fov.dirty = true;
        }
    }
}

pub fn player_input(gs: &mut State, term: &mut BTerm) -> RunState {
    match term.key {
        None => { return RunState::Waiting }
        Some(key) => match key {
            // Move East (E).
            VirtualKeyCode::L | VirtualKeyCode::Numpad6 | VirtualKeyCode::Right => move_player(EAST, &mut gs.ecs),
            // Move West (W).
            VirtualKeyCode::H | VirtualKeyCode::Numpad4 | VirtualKeyCode::Left => move_player(WEST, &mut gs.ecs),
            // Move North (N).
            VirtualKeyCode::K | VirtualKeyCode::Numpad8 | VirtualKeyCode::Up => move_player(NORTH, &mut gs.ecs),
            // Move South (S).
            VirtualKeyCode::J | VirtualKeyCode::Numpad2 | VirtualKeyCode::Down => move_player(SOUTH, &mut gs.ecs),
            // Move Northeast (NE).
            VirtualKeyCode::U | VirtualKeyCode::Numpad9 => move_player(NORTHEAST, &mut gs.ecs),
            // Move Northwest (NW).
            VirtualKeyCode::Y | VirtualKeyCode::Numpad7 => move_player(NORTHWEST, &mut gs.ecs),
            // Move Southeast (SE).
            VirtualKeyCode::N | VirtualKeyCode::Numpad3 => move_player(SOUTHEAST, &mut gs.ecs),
            // Move Southwest (SW).
            VirtualKeyCode::B | VirtualKeyCode::Numpad1 => move_player(SOUTHWEST, &mut gs.ecs),

            // Wait (skip turn).
            VirtualKeyCode::Period => { return RunState::PlayerTurn }

            _ => { return RunState::Waiting }

        },
    }
    RunState::PlayerTurn
}

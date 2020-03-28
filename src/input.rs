use bracket_lib::prelude::*;
use super::{
    utils::directions::*,
    state::{State, RunState},
    player::{move_player}
};

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

/*
pub fn menu_input(gs: &mut State, term: &mut BTerm) -> RunState {

}
*/

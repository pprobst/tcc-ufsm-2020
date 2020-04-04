/*
 *
 * directions.rs
 * -------------
 * Making directions constants so it'll be easier to manage player movement etc.
 *
 */

pub struct Direction {
    pub delta_x: i8,
    pub delta_y: i8
}

pub const EAST: Direction = Direction { delta_x: 1, delta_y: 0 };
pub const WEST: Direction = Direction { delta_x: -1, delta_y: 0 };
pub const NORTH: Direction = Direction { delta_x: 0, delta_y: -1 };
pub const SOUTH: Direction = Direction { delta_x: 0, delta_y: 1 };
pub const NORTHEAST: Direction = Direction { delta_x: 1, delta_y: -1 };
pub const NORTHWEST: Direction = Direction { delta_x: -1, delta_y: -1 };
pub const SOUTHEAST: Direction = Direction { delta_x: 1, delta_y: 1 };
pub const SOUTHWEST: Direction = Direction { delta_x: -1, delta_y: 1 };

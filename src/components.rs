use bracket_lib::prelude::{RGB, Point};
use specs::{prelude::*, Component};
//use std::collections::HashSet;

/*#[derive(Component)]
pub struct Position {
    pub x: i32,
    pub y: i32,
}*/

pub type Position = Point;

#[derive(Component)]
pub struct Renderable {
    pub glyph: u8,
    pub fg: RGB,
    pub bg: RGB,
}

#[derive(Component)]
pub struct Player {}

#[derive(Component)]
// Enemies & NPCs.
pub struct Mob {}

#[derive(Component, Debug)]
pub struct Name {
    pub name: String
}

#[derive(Component, PartialEq)]
// An entity's field of view (fov).
pub struct Fov {
    pub range: i32,
    pub visible_pos: Vec<Position>,
    pub dirty: bool
}

#[derive(Component)]
// Entities with this component will "block" movement over them.
// After all, you can't walk over enemies (unless you're flying!).
pub struct Blocker {}

#[derive(Component)]
pub struct Health {
   pub max_hp: i32,
   pub hp: i32
}

#[derive(Component)]
pub struct BaseStats {
    pub health: Health,
    pub defense: i32,
    pub attack: i32,
    pub god: bool // Doesn't die
}

#[derive(Component)]
pub struct SufferDamage {
    pub amount: Vec<(i32, bool)>,
}

impl SufferDamage {
    pub fn new_damage(
        store: &mut WriteStorage<SufferDamage>,
        victim: Entity,
        amount: i32,
        from_player: bool,
    ) {
        if let Some(suffering) = store.get_mut(victim) {
            suffering.amount.push((amount, from_player));
        } else {
            let dmg = SufferDamage {
                amount: vec![(amount, from_player)],
            };
            store.insert(victim, dmg).expect("Unable to insert damage");
        }
    }
}

#[derive(Component)]
pub struct MeleeAttack {
    pub target: Entity
}

#[derive(Component)]
pub struct MissileAttack {
    pub target: Entity
}

#[derive(Component)]
pub struct MeleeWeapon {
    pub base_damage: i32
    // special effect?
}

#[derive(Component)]
pub struct MissileWeapon {
    pub base_damage: i32,
    pub range: i32
    // special effect?
}

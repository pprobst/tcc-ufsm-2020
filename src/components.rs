use crate::utils::directions::Direction;
use bracket_lib::prelude::{to_cp437, ColorPair, Point, RGB};
use specs::{prelude::*, Component};
use std::ops::{Add, AddAssign, Sub};
//use std::collections::HashSet;

/*
 *
 * components.rs
 * -------------
 * Contains all the possible ECS components.
 *
 */

pub type Position = Point;

impl AddAssign<Direction> for Point {
    fn add_assign(&mut self, other: Direction) {
        *self = Self {
            x: self.x + other.delta_x as i32,
            y: self.y + other.delta_y as i32,
        };
    }
}

impl Add<Direction> for Point {
    type Output = Self;

    fn add(self, other: Direction) -> Self {
        Self {
            x: self.x + other.delta_x as i32,
            y: self.y + other.delta_y as i32,
        }
    }
}

impl Sub<Direction> for Point {
    type Output = Self;

    fn sub(self, other: Direction) -> Self {
        Self {
            x: self.x - other.delta_x as i32,
            y: self.y - other.delta_y as i32,
        }
    }
}

#[derive(Component, Copy, Clone, Debug)]
pub struct Renderable {
    pub glyph: u16,
    pub color: ColorPair,
    pub layer: u8,
}

impl Renderable {
    pub fn new(glyph: char, fg: RGB, bg: RGB) -> Self {
        Self {
            glyph: to_cp437(glyph),
            color: ColorPair::new(fg, bg),
            layer: 0,
        }
    }
}

#[derive(Component)]
pub struct Player {}

#[derive(Component)]
// Enemies & NPCs.
pub struct Mob {}

#[derive(Component, Debug)]
pub struct Name {
    pub name: String,
}

#[derive(Component, PartialEq)]
// An entity's field of view (fov).
pub struct Fov {
    pub range: i32,
    pub visible_pos: Vec<Position>,
    pub dirty: bool,
}

#[derive(Component)]
// Entities with this component will "block" movement over them.
// After all, you can't walk over enemies (unless you're flying!).
pub struct Blocker {}

#[derive(Component, Debug)]
pub struct Health {
    pub max_hp: i32,
    pub hp: i32,
}

#[derive(Component, Debug)]
pub struct BaseStats {
    pub health: Health,
    pub defense: i32,
    pub attack: i32,
    pub god: bool, // Doesn't die
}

#[derive(Component)]
pub struct SufferDamage {
    pub amount: Vec<(i32, bool)>,
}

impl SufferDamage {
    pub fn add_damage(
        dmg_store: &mut WriteStorage<SufferDamage>,
        victim: Entity,
        amount: i32,
        from_player: bool,
    ) {
        if let Some(suffering) = dmg_store.get_mut(victim) {
            suffering.amount.push((amount, from_player));
        } else {
            let dmg = SufferDamage {
                amount: vec![(amount, from_player)],
            };
            dmg_store
                .insert(victim, dmg)
                .expect("Unable to insert damage");
        }
    }
}

#[derive(Component)]
pub struct MeleeAttack {
    pub target: Entity,
}

#[derive(Component)]
pub struct MissileAttack {
    pub target: Entity,
}

#[derive(Component)]
pub struct MeleeWeapon {
    pub base_damage: i32, // special effect?
}

pub enum AmmoType {
    Arrow,
    _32,
    _9mm,
}

#[derive(Component)]
pub struct MissileWeapon {
    pub base_damage: i32,
    pub range: i32, // Influence on misses
    pub ammo_type: AmmoType,
    pub charges: i32, // special effect?
}

#[derive(Component)]
pub struct Target {
    pub covered: bool,
}

#[derive(Debug, PartialEq, Copy, Clone)]
pub enum EquipSlot {
    Weapon1,
    Weapon2,
    Head,
    Torso,
    Hands,
    Legs,
    Feet,
    Back,
    Floating,
}

#[derive(Component, Debug, PartialEq)]
pub struct Equipable {
    pub slot: EquipSlot,
}

#[derive(Component, Debug)]
pub struct Equipment {
    pub user: Entity,
    pub equip: Entity,
}

#[derive(Component, Debug)]
pub struct TryEquip {
    pub equipment: Equipment,
}

#[derive(Component, Debug)]
pub struct Item {}

#[derive(Component)]
pub struct Consumable {
    pub heal: i32,
}

#[derive(Component, Debug, Clone)]
pub struct CollectItem {
    pub collector: Entity,
    pub item: Entity,
}

#[derive(Component, Debug, Clone)]
pub struct DropItem {
    pub dropper: Entity,
    pub item: Entity,
}

#[derive(Component, Debug, Clone)]
pub struct ConsumeItem {
    pub target: Entity,
    pub item: Entity,
}

#[derive(Component)]
pub struct InBackpack {
    pub owner: Entity,
}

#[derive(Component, Debug)]
pub struct SelectedItem {
    pub item: Entity,
}

#[derive(Component, Debug)]
pub struct Container {} // Chests and other types of containers.

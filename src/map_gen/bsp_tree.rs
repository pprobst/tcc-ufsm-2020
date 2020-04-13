use bracket_lib::prelude::RandomNumberGenerator;
use super::{Map, Room, common::*};

/*
 *
 * bsp_tree.rs
 * -----------
 * Dungeon generation based on BSP (binary space partition) Trees.
 *
 * Based on the implementation by James Baum (with some nice additions):
 * - https://www.jamesbaum.co.uk/blether/procedural-generation-with-binary-space-partitions-and-rust/
 *
 * See also:
 * - http://www.roguebasin.com/index.php?title=Basic_BSP_Dungeon_generation
 * - https://gamedevelopment.tutsplus.com/tutorials/how-to-use-bsp-trees-to-generate-game-maps--gamedev-12268
 * - https://github.com/vurmux/urizen/blob/master/urizen/generators/dungeons/dungeon_bsp_tree.py
 *
 */

#[allow(dead_code)]
pub struct BSPDungeon { 
    pub rooms: Vec<Room>, // nodes (rooms)
    pub optimal_block_size: i32,
    pub connected: bool,
}

#[allow(dead_code)]
impl BSPDungeon {
    pub fn new(optimal_block_size: i32, connected: bool) -> Self {
        Self {
            rooms: vec![],
            optimal_block_size,
            connected,
        }
    }

    pub fn generate(&mut self, map: &mut Map, rng: &mut RandomNumberGenerator) {
        let w = map.width-1;
        let h = map.height-1;

        let mut root = Node::new(1, 1, w, h, self.optimal_block_size);
        root.gen(rng);
        root.make_rooms(rng, self.connected);

        for node in root.iter() {
            if node.is_leaf() {
                if let Some(room) = node.get_room() {
                    create_room(map, room);
                    self.rooms.push(room);
                }
            }
        }
    }

    pub fn build_tunnels_left(&mut self, map: &mut Map, rng: &mut RandomNumberGenerator) -> Vec<Vec<usize>> {
        self.rooms.sort_by(|a, b| a.x1.cmp(&b.x1));
        self.build_tunnels(map, rng)
    }

    pub fn build_tunnels(&mut self, map: &mut Map, rng: &mut RandomNumberGenerator) -> Vec<Vec<usize>> {
        let mut tunnels = Vec::new();

        for i in 0 .. self.rooms.len()-1 {
            let room = self.rooms[i];
            let other = self.rooms[i+1];
            let room_c = room.center();
            let other_c = other.center();

            match rng.range(0, 3) {
                0 => {
                    if room_c.x <= other_c.x {
                        tunnels.push(create_h_tunnel(map, room_c.x, other_c.x, room_c.y));
                    } else { tunnels.push(create_h_tunnel(map, other_c.x, room_c.x, room_c.y)); }

                    if room_c.y <= other_c.y {
                        tunnels.push(create_v_tunnel(map, room_c.y, other_c.y, other_c.x));
                    } else { tunnels.push(create_v_tunnel(map, other_c.y, room_c.y, other_c.x)); }
                }
                _ => {
                    if room_c.y <= other_c.y {
                        tunnels.push(create_v_tunnel(map, room_c.y, other_c.y, other_c.x));
                    } else { tunnels.push(create_v_tunnel(map, other_c.y, room_c.y, other_c.x)); }

                    if room_c.x <= other_c.x {
                        tunnels.push(create_h_tunnel(map, room_c.x, other_c.x, room_c.y));
                    } else { tunnels.push(create_h_tunnel(map, other_c.x, room_c.x, room_c.y)); }
                }
            }
        }

        tunnels 
    }

    pub fn get_rooms(&self) -> Vec<Room> {
        self.rooms.clone() 
    }

} 

pub struct Node {
    min_size: i32,
    x: i32,
    y: i32,
    w: i32,
    h: i32,
    // Box<> tells Rust to use the heap to allocate child structs.
    left_child: Option<Box<Node>>,
    right_child: Option<Box<Node>>,
    room: Option<Room>,
    //tunnels: Vec<Room>
}

impl Node {
    pub fn new(x: i32, y: i32, w: i32, h: i32, min_size: i32) -> Self {
        Self {
            min_size, x, y, w, h, 
            left_child: None, 
            right_child: None, 
            room: None, 
            //tunnels: vec![]
        }
    }

    fn is_leaf(&self) -> bool {
        match self.left_child {
            None => match self.right_child {
                None => true,
                Some(_) => false,
            },
            Some(_) => false
        }
    }

    /// Tries to split the current node if it's a leaf. If it's successful, proceeds
    /// to split its children.
    fn gen(&mut self, rng: &mut RandomNumberGenerator) {
        if self.is_leaf() {
            if self.split(rng) {
                self.left_child.as_mut().unwrap().gen(rng);
                self.right_child.as_mut().unwrap().gen(rng);
            }
        }
    }

    fn split(&mut self, rng: &mut RandomNumberGenerator) -> bool {
        // If width > 25% height, split vertically.
        // If height > 25% width, split horizontally.
        // Otherwise, random.

        let mut split_h = match rng.range(0, 2) {
            0 => false,
            _ => true
        };

        if self.w > self.h && (self.w as f32 / self.h as f32) >= 1.25 {
            split_h = false;
        } else if self.h > self.w && (self.h as f32 / self.w as f32) >= 1.25 {
            split_h = true;
        }

        let max = match split_h {
            true => self.h - self.min_size,
            false => self.w - self.min_size
        };

        // Stop splitting once it's small enough.
        if max <= self.min_size {
            return false;
        }

        let split_pos = rng.range(self.min_size, max);
        if split_h {
            self.left_child = Some(Box::new(Node::new(self.x, self.y, self.w, split_pos, self.min_size)));
            self.right_child = Some(Box::new(Node::new(self.x, self.y + split_pos, self.w, self.h - split_pos, self.min_size)));
        } else {
            self.left_child = Some(Box::new(Node::new(self.x, self.y, split_pos, self.h, self.min_size)));
            self.right_child = Some(Box::new(Node::new(self.x + split_pos, self.y, self.w - split_pos, self.h, self.min_size)));
        }

        true
    }

    fn make_rooms(&mut self, rng: &mut RandomNumberGenerator, connected: bool) {
        if let Some(ref mut room) = self.left_child {
            room.as_mut().make_rooms(rng, connected);
        };

        if let Some(ref mut room) = self.right_child {
            room.as_mut().make_rooms(rng, connected);
        };

        if self.is_leaf() {
            if connected {  
                self.room = Some(Room::with_size(self.x, self.y, self.w, self.h));
            } else {
                let min_room_width = 4;
                let min_room_height = 4;
                let width = rng.range(min_room_width, self.w);
                let height = rng.range(min_room_height, self.h);
                let x = rng.range(0, self.w-width);
                let y = rng.range(0, self.h-height);
                self.room = Some(Room::with_size(x + self.x, y + self.y, width, height));
            }
        }
    }

    fn get_room(&self) -> Option<Room> {
        if self.is_leaf() {
            return self.room;
        }

        let mut left_room: Option<Room> = None;
        let mut right_room: Option<Room> = None;

        if let Some(ref room) = self.left_child {
            left_room = room.get_room();
        }

        if let Some(ref room) = self.right_child {
            right_room = room.get_room();
        }

        match (left_room, right_room) {
            (None, None) => None,
            (Some(room), _) => Some(room),
            (_, Some(room)) => Some(room),
        }
    }

    fn iter(&self) -> NodeIterator {
        NodeIterator::new(&self)
    }
}

struct NodeIterator<'a> {
    current_node: Option<&'a Node>,
    right_nodes: Vec<&'a Node>
}

impl<'a> NodeIterator<'a> {
    fn new(root: &'a Node) -> NodeIterator<'a> {
        let mut iter = NodeIterator {
            right_nodes: vec![],
            current_node: None
        };

        iter.add_subtrees(root);
        iter
    }

    // Set the current node to the one provided, and add any child leaves 
    // to the node vector.
    fn add_subtrees(&mut self, node: &'a Node) {
        if let Some(ref left) = node.left_child {
            self.right_nodes.push(&*left);
        }
        if let Some(ref right) = node.right_child {
            self.right_nodes.push(&*right);
        }

        self.current_node = Some(node);
    }
}

impl<'a> Iterator for NodeIterator<'a> {
    type Item = &'a Node;

    fn next(&mut self) -> Option<Self::Item> {
        let result = self.current_node.take();
        if let Some(rest) = self.right_nodes.pop() {
            self.add_subtrees(rest);
        }

        match result {
            Some(leaf) => Some(&*leaf),
            None => None
        }
    }
}

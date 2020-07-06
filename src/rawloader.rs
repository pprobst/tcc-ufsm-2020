use bracket_lib::prelude::{embedded_resource, link_resource, EMBED};
use serde::Deserialize;
use std::sync::Mutex;

embedded_resource!(RAW_01, "../raws/items.ron");

lazy_static! {
    pub static ref RAWS: Mutex<RawMaster> = Mutex::new(RawMaster::empty());
}

pub struct Raws {
    //pub spawn_table: Vec<SpawnTableEntry>,
    pub items: Vec<Item>,
}

pub fn load_raws() {
    link_resource!(RAW_01, "../raws/items.ron");
    let raw_data = EMBED.lock().get_resource("../raws/items.ron".to_string()).unwrap();
    let raw_string = std::str::from_utf8(&raw_data).expect("Unable to convert to a valid UTF-8 string.");

    let decoder: Raws = ron::de::from_str(&raw_string).expect("Unable to parse RON");

    RAWS.lock().unwrap().load(decoder);
} 

use bracket_lib::prelude::{embedded_resource, link_resource, EMBED};
use serde::Deserialize;
use std::sync::Mutex;

mod rawmaster;
pub use rawmaster::*;
mod item_structs;
use item_structs::*;
mod color_structs;
pub use color_structs::*;

embedded_resource!(RAW_ITEMS, "../../raws/items.ron");
embedded_resource!(RAW_COLORS, "../../raws/colors.ron");

lazy_static! {
    pub static ref RAWS: Mutex<RawMaster> = Mutex::new(RawMaster::empty());
}

#[derive(Deserialize, Debug)]
pub struct Raws {
    //pub spawn_table: Vec<SpawnTableEntry>,
    pub items: Vec<Item>,
    pub colorschemes: Vec<Colorscheme>,
}

pub fn load_raws() {
    link_resource!(RAW_ITEMS, "../../raws/items.ron");
    link_resource!(RAW_COLORS, "../../raws/colors.ron");

    let raw_string_items = get_raw_string("../../raws/items.ron".to_string());
    let raw_string_colors = get_raw_string("../../raws/colors.ron".to_string());

    let full_string = [
        raw_string_items[..raw_string_items.len() - 3].to_string(),
        raw_string_colors[1..].to_string(),
    ]
    .concat();

    //println!("{:?}", full_string);

    let decoder: Raws = ron::de::from_str(&full_string).expect("Unable to parse RON");

    RAWS.lock().unwrap().load(decoder);

    println!("{:?}", *RAWS);
}

fn get_raw_string(path: String) -> &'static str {
    let raw_data = EMBED.lock().get_resource(path).unwrap();
    let raw_string =
        std::str::from_utf8(&raw_data).expect("Unable to convert to a valid UTF-8 string.");
    raw_string
}

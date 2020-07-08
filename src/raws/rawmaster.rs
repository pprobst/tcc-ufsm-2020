use super::{Colorscheme, Raws};
use std::collections::HashMap;

#[derive(Debug)]
pub struct RawMaster {
    pub raws: Raws,
    item_index: HashMap<String, usize>,
    pub color_index: HashMap<String, usize>,
    curr_colorscheme: usize,
}

impl RawMaster {
    pub fn empty() -> Self {
        RawMaster {
            raws: Raws {
                items: Vec::new(),
                colorschemes: Vec::new(),
            },
            item_index: HashMap::new(),
            color_index: HashMap::new(),
            curr_colorscheme: 0,
        }
    }

    pub fn load(&mut self, raws: Raws) {
        self.raws = raws;
        //self.item_index = HashMap::new();

        for (i, item) in self.raws.items.iter().enumerate() {
            self.item_index.insert(item.name.clone(), i);
        }
        for (i, color) in self.raws.colorschemes.iter().enumerate() {
            self.color_index.insert(color.name.clone(), i);
        }
    }

    pub fn set_curr_colorscheme(&mut self, colorscheme: &str) {
        self.curr_colorscheme = self.color_index[colorscheme];
    }

    pub fn get_curr_colorscheme(&self) -> &Colorscheme {
        &self.raws.colorschemes[self.curr_colorscheme]
    }
}

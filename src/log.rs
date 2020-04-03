use bracket_lib::prelude::{RGB};

// Based on: 
// -> http://tomassedovic.github.io/roguelike-tutorial/part-7-gui.html
// (for now, there's no reason to over-engineer this)

pub struct Log {
    pub messages: Vec<(String, RGB)>
}

impl Log {
    pub fn new() -> Self {
        Self { messages: vec![] }
    }

    /// Add the new message as a tuple, with the text and the color.
    pub fn add<T: Into<String>>(&mut self, message: T, color: RGB) {
        self.messages.push((message.into(), color));
    }

    /*
    /// Create a `DoubleEndedIterator` over the messages.
    pub fn iter(&self) -> impl DoubleEndedIterator<Item = &(String, RGB)> {
        self.messages.iter()
    }
    */
}

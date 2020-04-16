use bracket_lib::prelude::RGB;

/*
 *
 * log.rs
 * ------
 * The basic structure of the game log.
 *
 * Based on http://tomassedovic.github.io/roguelike-tutorial/part-7-gui.html
 */

pub struct Log {
    pub messages: Vec<(String, RGB)>,
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

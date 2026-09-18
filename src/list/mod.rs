use crossterm::event::{Event, KeyEvent};

use crate::Item;

#[derive(Default)]
pub struct List {
    items: Vec<Box<dyn Item>>,
}

impl List {
    /// Adds an item to the list.
    pub fn add_item(mut self, item: Box<dyn Item>) -> Self {
        self.items.push(item);
        self
    }

    /// Shows the list.
    pub fn show(mut self) -> std::io::Result<()> {
        crossterm::terminal::enable_raw_mode()?;
        self.render();

        loop {
            match crossterm::event::read()? {
                Event::Key(event) => {}
                Event::Mouse(event) => {}
                _ => (),
            }

            let should_close = self.items.iter_mut().any(|item| item.process());

            if should_close {
                break;
            }
        }

        self.clear();
        crossterm::terminal::disable_raw_mode()?;

        Ok(())
    }

    /// Renders the list.
    fn render(&self) {
        for item in &self.items {
            item.render();
        }
    }

    /// Handles the keyboard input.
    /// Returns `true` if the list should be closed.
    fn handle_key(&mut self, event: KeyEvent) {}

    /// Clears the list.
    fn clear(&self) {}
}

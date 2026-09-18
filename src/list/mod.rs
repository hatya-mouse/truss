mod raw_guard;

use crate::{Item, list::raw_guard::RawGuard};
use crossterm::event::{Event, KeyEvent};

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
        let raw_mode = RawGuard::new()?;
        self.render();

        loop {
            match crossterm::event::read()? {
                Event::Key(event) => {
                    if self.handle_key(event) {
                        break;
                    }
                }
                Event::Mouse(event) => {}
                _ => (),
            }
        }

        self.clear();
        drop(raw_mode);

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
    fn handle_key(&mut self, event: KeyEvent) -> bool {
        false
    }

    /// Clears the list.
    fn clear(&self) {}
}

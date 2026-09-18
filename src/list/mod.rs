mod raw_guard;
mod render_area;

pub use render_area::RenderArea;

use crate::{Item, list::raw_guard::RawGuard};
use crossterm::event::{Event, KeyCode, KeyEvent};

/// A list of items that can be displayed in the terminal.
#[derive(Default)]
pub struct List {
    /// Items in the list.
    items: Vec<Box<dyn Item>>,
    /// The currently rendered area on the terminal.
    render_area: RenderArea,
    /// The index of the currently selected item.
    selected_index: usize,
    /// Whether it's currently input mode.
    input_mode: bool,
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
        self.render()?;

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

            self.render()?;
        }

        self.clear()?;
        drop(raw_mode);

        Ok(())
    }

    /// Renders the list.
    fn render(&mut self) -> std::io::Result<()> {
        self.clear()?;

        for item in &self.items {
            item.render(&mut self.render_area);
        }

        Ok(())
    }

    /// Handles the keyboard input.
    /// Returns `true` if the list should be closed.
    fn handle_key(&mut self, event: KeyEvent) -> bool {
        match &event.code {
            KeyCode::Up | KeyCode::PageUp => {
                self.selected_index = self.selected_index.saturating_sub(1);
            }
            KeyCode::Down | KeyCode::PageDown => {
                self.selected_index = self
                    .selected_index
                    .saturating_add(1)
                    .min(self.items.len() - 1);
            }
            KeyCode::Enter => {
                self.input_mode = true;
            }
            KeyCode::Esc => {
                self.input_mode = false;
            }
            _ => (),
        }

        false
    }

    /// Clears the list.
    fn clear(&mut self) -> std::io::Result<()> {
        self.render_area.clear()
    }
}

mod raw_guard;
mod render_area;

use std::io::stdout;

pub use render_area::RenderArea;

use crate::{Item, list::raw_guard::RawGuard};
use crossterm::{
    cursor::MoveToColumn,
    event::{Event, KeyCode, KeyEvent},
    execute,
};

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
    pub fn add_item<T>(mut self, item: T) -> Self
    where
        T: Item + 'static,
    {
        self.items.push(Box::new(item));
        self
    }

    /// Shows the list.
    pub fn show(mut self) -> std::io::Result<()> {
        let raw_mode = RawGuard::new()?;
        self.render_area.reset();
        self.render()?;

        loop {
            if let Event::Key(event) = crossterm::event::read()?
                && self.handle_key(event)
            {
                break;
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

        for (index, item) in self.items.iter().enumerate() {
            let item_style = self.resolve_style(item);
            item.render(item_style, &mut self.render_area);
            execute!(stdout(), MoveToColumn(0))?;
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

mod raw_guard;
mod render_area;

pub use render_area::RenderArea;

use crate::{Item, ItemStyle, list::raw_guard::RawGuard};
use crossterm::{
    cursor::MoveToColumn,
    event::{Event, KeyCode, KeyEvent},
    execute,
};
use std::io::stdout;

/// A list of items that can be displayed in the terminal.
pub struct List {
    /// Items in the list.
    items: Vec<Box<dyn Item>>,

    // --- STYLES ---
    /// The style for the selected items.
    selected_style: ItemStyle,
    /// The style for the normal items.
    item_style: ItemStyle,

    // --- STATES ---
    /// The currently rendered area on the terminal.
    render_area: RenderArea,
    /// The index of the currently selected item.
    selected_index: usize,
}

impl Default for List {
    fn default() -> Self {
        Self {
            items: Vec::new(),
            selected_style: ItemStyle::default_selected(),
            item_style: ItemStyle::default_item(),
            render_area: RenderArea::default(),
            selected_index: 0,
        }
    }
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
            let item_style = self.resolve_style(index).clone();
            item.render(&mut self.render_area, item_style);
            execute!(stdout(), MoveToColumn(0))?;
        }

        Ok(())
    }

    /// Returns the item style for the given item index.
    pub fn resolve_style(&self, item_index: usize) -> &ItemStyle {
        if self.selected_index == item_index {
            &self.selected_style
        } else {
            &self.item_style
        }
    }

    /// Handles the keyboard input.
    /// Returns `true` if the list should be closed.
    fn handle_key(&mut self, event: KeyEvent) -> bool {
        match &event.code {
            KeyCode::Up | KeyCode::PageUp => {
                self.selected_index = self.selected_index.saturating_sub(1);
                false
            }
            KeyCode::Down | KeyCode::PageDown => {
                self.selected_index = self
                    .selected_index
                    .saturating_add(1)
                    .min(self.items.len() - 1);
                false
            }
            _ => {
                if let Some(item) = self.items.get_mut(self.selected_index) {
                    item.handle_key(event)
                } else {
                    false
                }
            }
        }
    }

    /// Clears the list.
    fn clear(&mut self) -> std::io::Result<()> {
        self.render_area.clear()
    }
}

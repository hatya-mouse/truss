mod chain;
mod raw_guard;
mod render_area;

pub use render_area::RenderArea;

use crate::{Item, ItemStyle, list::raw_guard::RawGuard};
use crossterm::{
    cursor::{MoveToColumn, SetCursorStyle},
    event::{Event, KeyCode, KeyEvent, KeyModifiers},
    execute,
};
use std::io::stdout;

pub struct ListKeyBinds {
    /// Press any of these keys to move the selection up.
    up: Vec<KeyCode>,
    /// Press any of these keys to move the selection down.
    down: Vec<KeyCode>,
}

impl Default for ListKeyBinds {
    fn default() -> Self {
        Self {
            up: vec![KeyCode::Up, KeyCode::PageUp],
            down: vec![KeyCode::Down, KeyCode::PageDown],
        }
    }
}

/// A list of items that can be displayed in the terminal.
pub struct List<'a> {
    /// Items in the list.
    items: Vec<Box<dyn Item + 'a>>,

    // --- STYLES ---
    /// The style for the selected items.
    selected_style: ItemStyle,
    /// The style for the normal items.
    item_style: ItemStyle,
    /// Whether the list should be cleared after it is closed.
    clear_on_close: bool,

    // --- KEY BINDS ---
    /// The key bindings for the list.
    key_binds: ListKeyBinds,

    // --- STATES ---
    /// The currently rendered area on the terminal.
    render_area: RenderArea,
    /// The index of the currently selected item.
    selected_index: Option<usize>,
}

impl Default for List<'_> {
    fn default() -> Self {
        Self {
            items: Vec::new(),
            selected_style: ItemStyle::default_selected(),
            item_style: ItemStyle::default_item(),
            clear_on_close: true,
            key_binds: ListKeyBinds::default(),
            render_area: RenderArea::default(),
            selected_index: Some(0),
        }
    }
}

impl<'a> List<'a> {
    /// Adds an item to the list.
    pub fn add_item<T>(mut self, item: T) -> Self
    where
        T: Item + 'a,
    {
        self.items.push(Box::new(item));
        self
    }

    /// Adds multiple items to the list.
    pub fn add_items<T>(mut self, items: Vec<T>) -> Self
    where
        T: Item + 'a,
    {
        for item in items {
            self = self.add_item(item);
        }
        self
    }

    /// Shows the list.
    pub fn show(mut self) -> std::io::Result<()> {
        let raw_mode = RawGuard::new()?;
        self.render_area.reset();
        self.render()?;

        loop {
            if let Event::Key(event) = crossterm::event::read()?
                && self.handle_key(event)?
            {
                break;
            }

            self.render()?;
        }

        self.selected_index = None;
        if self.clear_on_close {
            self.clear()?;
        } else {
            self.render()?;
        }
        drop(raw_mode);

        Ok(())
    }

    /// Renders the list.
    fn render(&mut self) -> std::io::Result<()> {
        self.clear()?;

        for (index, item) in self.items.iter().enumerate() {
            let item_style = self.resolve_style(index).clone();
            item.render(&mut self.render_area, item_style)?;
            execute!(stdout(), MoveToColumn(0))?;
        }

        Ok(())
    }

    /// Returns the item style for the given item index.
    pub fn resolve_style(&self, item_index: usize) -> &ItemStyle {
        if self.selected_index == Some(item_index) {
            &self.selected_style
        } else {
            &self.item_style
        }
    }

    /// Handles the keyboard input.
    /// Returns `true` if the list should be closed.
    fn handle_key(&mut self, event: KeyEvent) -> std::io::Result<bool> {
        // Handle Ctrl+C to close the list
        if event.modifiers.contains(KeyModifiers::CONTROL) && event.code == KeyCode::Char('c') {
            execute!(stdout(), SetCursorStyle::DefaultUserShape)?;
            return Ok(true);
        }

        let mut should_close = false;
        if let Some(selected_index) = self.selected_index
            && let Some(item) = self.items.get_mut(selected_index)
        {
            let event_handled;
            (should_close, event_handled) = item.handle_key(event)?;
            if event_handled {
                return Ok(should_close);
            }
        }

        for key_code in self.key_binds.up.iter() {
            if event.code == *key_code {
                self.selected_index =
                    Some(self.selected_index.unwrap_or_default().saturating_sub(1));
                return Ok(should_close);
            }
        }

        for key_code in self.key_binds.down.iter() {
            if event.code == *key_code {
                self.selected_index = Some(
                    self.selected_index
                        .unwrap_or_default()
                        .saturating_add(1)
                        .min(self.items.len() - 1),
                );
                return Ok(should_close);
            }
        }

        Ok(should_close)
    }

    /// Clears the list.
    fn clear(&mut self) -> std::io::Result<()> {
        self.render_area.clear()
    }
}

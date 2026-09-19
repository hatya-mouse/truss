use crate::{
    List,
    item::{CheckItem, FieldItem, LabelItem},
    list::ListKeyBinds,
};
use crossterm::event::KeyCode;

impl<'a> List<'a> {
    /// Sets the key bindings for moving the selection up in the list.
    pub fn up_key_binds(self, key_binds: Vec<KeyCode>) -> Self {
        Self {
            key_binds: ListKeyBinds {
                up: key_binds,
                down: self.key_binds.down,
            },
            ..self
        }
    }

    /// Sets the key bindings for moving the selection down in the list.
    pub fn down_key_binds(self, key_binds: Vec<KeyCode>) -> Self {
        Self {
            key_binds: ListKeyBinds {
                up: self.key_binds.up,
                down: key_binds,
            },
            ..self
        }
    }

    /// Sets whether the list should be cleared after it is closed.
    pub fn clear_on_close(mut self, should_clear: bool) -> Self {
        self.clear_on_close = should_clear;
        self
    }

    // --- ITEM ADDITION ---

    /// Adds an label item to the list.
    pub fn label(self, label: impl Into<String>) -> Self {
        self.add_item(LabelItem::new(label))
    }

    /// Adds an check item to the list.
    pub fn check(self, label: impl Into<String>, checked: &'a mut bool) -> Self {
        self.add_item(CheckItem::new(label, checked))
    }

    /// Adds an field item to the list.
    pub fn field(self, label: impl Into<String>, content: &'a mut String) -> Self {
        self.add_item(FieldItem::new(
            label,
            content,
            Box::new(|str| Ok(str.to_string())),
        ))
    }
}

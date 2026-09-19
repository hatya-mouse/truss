use crate::{Item, ItemStyle, RenderArea, text_editor::TextEditor};
use crossterm::{
    cursor::MoveToColumn,
    event::{KeyCode, KeyEvent},
    execute,
};
use std::{fmt::Display, io::stdout};

pub type FieldValidator<T> = Box<dyn Fn(&str) -> Result<T, String>>;

pub struct FieldItem<'a, T: Display> {
    // --- CONTENT ---
    /// The label of the field.
    label: String,
    /// The content being edited in the field.
    content: &'a mut T,
    /// A validator that converts and validates the content of the field.
    validator: FieldValidator<T>,

    // --- RENDERING ---
    /// The text editor used to edit the content of the field.
    editor: TextEditor,
    /// The last validation error message.
    validator_error: Option<String>,

    // --- STYLE ---
    /// The item style for the validator error.
    /// If None, the default item style of the list is used.
    error_style: Option<ItemStyle>,
    /// Whether to use the mode-based editing or not.
    enable_edit_mode: bool,
}

impl<'a, T: Display> FieldItem<'a, T> {
    pub fn new(label: impl Into<String>, content: &'a mut T, validator: FieldValidator<T>) -> Self {
        let content_string = content.to_string();
        Self {
            label: label.into(),
            content,
            validator,
            editor: TextEditor::new(content_string, false),
            validator_error: None,
            error_style: None,
            enable_edit_mode: false,
        }
    }
}

impl<T: Display> Item for FieldItem<'_, T> {
    fn render(
        &mut self,
        render_area: &mut RenderArea,
        item_style: ItemStyle,
        _is_selected: bool,
    ) -> std::io::Result<()> {
        if let Some(validator_error) = self.validator_error.as_ref() {
            let default_style = ItemStyle::default();
            let error_style = self.error_style.as_ref().unwrap_or(&default_style);
            let error_text = error_style.apply(validator_error);
            println!("{}", error_text);
            render_area.advance_by(error_text.lines().try_into().unwrap_or_default());

            execute!(stdout(), MoveToColumn(0))?;
        }

        let label_text = item_style.apply(&self.label);
        print!("{} ", label_text);
        let label_lines: u16 = label_text.lines().try_into().unwrap_or_default();
        render_area.advance_by(label_lines.saturating_sub(1));

        self.editor.render(render_area)
    }

    fn post_render(&self, is_selected: bool) -> std::io::Result<()> {
        self.editor.post_render(is_selected)
    }

    fn handle_key(&mut self, event: KeyEvent) -> std::io::Result<(bool, bool)> {
        if !self.enable_edit_mode {
            self.editor.set_edit_mode(true)?;
        }

        let mut event_handled = self.editor.handle_key(event)?;

        if !event_handled {
            match event.code {
                KeyCode::Enter => {
                    if (self.editor.multiline || self.enable_edit_mode) && !self.editor.edit_mode()
                    {
                        self.editor.set_edit_mode(true)?;
                        event_handled = true;
                    }
                }
                KeyCode::Esc => {
                    if (self.editor.multiline || self.enable_edit_mode) && self.editor.edit_mode() {
                        self.editor.set_edit_mode(false)?;
                        event_handled = true;
                    }
                }
                _ => match (self.validator)(self.editor.get_text()) {
                    Ok(t) => {
                        *self.content = t;
                        self.validator_error = None;

                        if (self.editor.multiline || self.enable_edit_mode)
                            && self.editor.edit_mode()
                        {
                            // Prevent switching to other items when in edit mode
                            event_handled = true;
                        }
                    }
                    Err(err) => {
                        // Return true to prevent switching to other items in the list
                        self.validator_error = Some(err);
                        event_handled = true;
                    }
                },
            }
        }

        Ok((false, event_handled))
    }
}

impl<T: Display> FieldItem<'_, T> {
    /// Turns on or off multi-line editing.
    pub fn multiline(mut self, multiline: bool) -> Self {
        self.editor.multiline = multiline;
        self
    }

    /// Whether to use the mode-based editing or not. If false, the editor will accept key input and insert it to the text without entering the edit mode.
    /// Edit mode is always enabled in multi-line editing regardless of `enable_edit_mode`.
    pub fn enable_edit_mode(mut self, enable_edit_mode: bool) -> Self {
        self.enable_edit_mode = enable_edit_mode;
        self
    }
}

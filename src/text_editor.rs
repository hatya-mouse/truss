use crate::RenderArea;
use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};
use std::range::Range;

pub(super) struct TextEditor<'a> {
    /// The text being edited.
    text: &'a mut String,
    /// The current range of the cursor.
    cursor: Range<usize>,
    /// Whether the editor is currently active.
    edit_mode: bool,
}

impl<'a> TextEditor<'a> {
    pub(super) fn new(text: &'a mut String) -> Self {
        Self {
            text,
            cursor: Range::default(),
            edit_mode: false,
        }
    }

    pub(super) fn render(&self, render_area: &mut RenderArea) {
        print!("{}", self.text);
        let text_lines: u16 = self.text.lines().count().try_into().unwrap_or_default();
        render_area.advance_by(text_lines - 1);
    }

    pub(super) fn handle_key(&mut self, event: KeyEvent) -> bool {
        if self.edit_mode {
            match event.code {
                KeyCode::Enter => {
                    self.enter_char('\n');
                    true
                }
                KeyCode::Esc => {
                    self.edit_mode = false;
                    true
                }
                KeyCode::Home => {
                    self.cursor.start = 0;
                    self.cursor.end = 0;
                    true
                }
                KeyCode::End => {
                    self.cursor.start = self.text.len();
                    self.cursor.end = self.text.len();
                    true
                }
                KeyCode::Left => {
                    if self.cursor.start != self.cursor.end {
                        self.cursor.end = self.cursor.start;
                    } else if self.cursor.start > 0 {
                        self.cursor.start -= 1;
                        self.cursor.end = self.cursor.start;
                    }
                    true
                }
                KeyCode::Right => {
                    if self.cursor.start != self.cursor.end {
                        self.cursor.end = self.cursor.end.min(self.text.len());
                        self.cursor.start = self.cursor.end;
                    } else if self.cursor.start < self.text.len() {
                        self.cursor.start += 1;
                        self.cursor.end = self.cursor.start;
                    }
                    true
                }
                KeyCode::Char(c) => {
                    if event.modifiers.contains(KeyModifiers::SHIFT) {
                        self.enter_char(c.to_ascii_uppercase());
                    } else {
                        self.enter_char(c);
                    }
                    true
                }
                _ => false,
            }
        } else {
            match event.code {
                KeyCode::Enter | KeyCode::Char(' ') => {
                    self.edit_mode = true;
                    true
                }
                _ => false,
            }
        }
    }

    fn enter_char(&mut self, c: char) {
        // Replace the selection if there is one
        if self.cursor.end - self.cursor.start > 0 {
            let replace_range = Range {
                start: self.cursor.start.min(self.text.len()),
                end: self.cursor.end.min(self.text.len()),
            };
            self.text.replace_range(replace_range, "");
            self.cursor.end = self.cursor.start;
        }

        // Insert the character at the cursor position
        let char_pos = self.cursor.start.min(self.text.len());
        self.text.insert(char_pos, c);
        // Advance the cursor position after inserting the character
        self.cursor.start = char_pos + 1;
        self.cursor.end = char_pos + 1;
    }
}

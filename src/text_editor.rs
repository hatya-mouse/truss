use crate::RenderArea;
use crossterm::{
    cursor::{MoveTo, MoveToColumn, SetCursorStyle},
    event::{KeyCode, KeyEvent},
    execute,
};
use std::{io::stdout, range::Range};
use unicode_width::UnicodeWidthStr;

pub(super) struct TextEditor<'a> {
    /// The text being edited.
    text: &'a mut String,
    /// The current range of the cursor.
    cursor: Range<usize>,
    /// Whether the editor is currently active.
    edit_mode: bool,
    /// The origin of the text editor in the terminal.
    origin: (u16, u16),
}

impl<'a> TextEditor<'a> {
    pub(super) fn new(text: &'a mut String) -> Self {
        Self {
            text,
            cursor: Range::default(),
            edit_mode: false,
            origin: (0, 0),
        }
    }

    pub(super) fn render(&mut self, render_area: &mut RenderArea) -> std::io::Result<()> {
        self.origin = crossterm::cursor::position()?;

        for line in self.text.lines() {
            println!("{}", line);
            execute!(stdout(), MoveToColumn(self.origin.0))?;
            render_area.advance_by(1);
        }

        Ok(())
    }

    pub(super) fn post_render(&self) -> std::io::Result<()> {
        // Move the cursor to the current position in the text
        let chars_before_cursor = self.text.get(..self.cursor.start).unwrap_or("");
        let lines: u16 = chars_before_cursor
            .chars()
            .filter(|c| *c == '\n')
            .count()
            .try_into()
            .unwrap_or_default();
        let last_line_len: u16 = chars_before_cursor
            .split('\n')
            .next_back()
            .and_then(|line| line.width().try_into().ok())
            .unwrap_or_default();
        let col = self.origin.0 + last_line_len;
        let row = self.origin.1 + lines;
        execute!(stdout(), MoveTo(col, row))
    }

    pub(super) fn handle_key(&mut self, event: KeyEvent) -> std::io::Result<bool> {
        if self.edit_mode {
            match event.code {
                KeyCode::Enter => {
                    self.enter_char('\n');
                    Ok(true)
                }
                KeyCode::Esc => {
                    self.edit_mode = false;
                    execute!(stdout(), SetCursorStyle::DefaultUserShape)?;
                    Ok(true)
                }
                KeyCode::Backspace | KeyCode::Delete => {
                    if self.cursor.start != self.cursor.end {
                        let replace_range = Range {
                            start: self.cursor.start.min(self.text.len()),
                            end: self.cursor.end.min(self.text.len()),
                        };
                        self.text.replace_range(replace_range, "");
                        self.cursor.end = self.cursor.start;
                    } else if self.cursor.start > 0 {
                        let prev_char_idx = self
                            .text
                            .char_indices()
                            .rev()
                            .find_map(|(idx, _)| {
                                if idx < self.cursor.start {
                                    Some(idx)
                                } else {
                                    None
                                }
                            })
                            .unwrap_or(0);
                        self.text
                            .replace_range(prev_char_idx..self.cursor.start, "");
                        self.cursor.start = prev_char_idx;
                        self.cursor.end = prev_char_idx;
                    }
                    Ok(true)
                }
                KeyCode::Home => {
                    self.cursor.start = 0;
                    self.cursor.end = 0;
                    Ok(true)
                }
                KeyCode::End => {
                    self.cursor.start = self.text.len();
                    self.cursor.end = self.text.len();
                    Ok(true)
                }
                KeyCode::Left => {
                    if self.cursor.start != self.cursor.end {
                        self.cursor.end = self.cursor.start;
                    } else if self.cursor.start > 0 {
                        self.move_cursor(-1);
                    }
                    Ok(true)
                }
                KeyCode::Right => {
                    if self.cursor.start != self.cursor.end {
                        self.cursor.end = self.cursor.end.min(self.text.len());
                        self.cursor.start = self.cursor.end;
                    } else if self.cursor.start < self.text.len() {
                        self.move_cursor(1);
                    }
                    Ok(true)
                }
                KeyCode::Char(c) => {
                    self.enter_char(c);
                    Ok(true)
                }
                _ => Ok(true),
            }
        } else {
            match event.code {
                KeyCode::Enter | KeyCode::Char(' ') => {
                    self.edit_mode = true;
                    execute!(stdout(), SetCursorStyle::BlinkingBar)?;
                    Ok(true)
                }
                _ => Ok(false),
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
        self.cursor.start = char_pos + c.len_utf8();
        self.cursor.end = char_pos + c.len_utf8();
    }

    fn move_cursor(&mut self, offset: isize) {
        if offset > 0
            && let Some(bytes) = self.text.get(self.cursor.start..).map(|str| {
                str.chars()
                    .take(offset.unsigned_abs())
                    .map(|c| c.len_utf8())
                    .sum::<usize>()
            })
        {
            self.cursor.start += bytes;
            self.cursor.end = self.cursor.start;
        } else if offset < 0
            && let Some(bytes) = self.text.get(..self.cursor.start).map(|str| {
                str.chars()
                    .rev()
                    .take(offset.unsigned_abs())
                    .map(|c| c.len_utf8())
                    .sum::<usize>()
            })
        {
            self.cursor.start -= bytes;
            self.cursor.end = self.cursor.start;
        }
    }
}

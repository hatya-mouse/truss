use crossterm::{
    cursor::{MoveTo, MoveUp},
    execute,
    terminal::{Clear, ClearType},
};
use std::io::stdout;

#[derive(Default)]
pub struct RenderArea {
    origin: (u16, u16),
    height: u16,
}

impl RenderArea {
    /// Reset the origin and height of the render area.
    pub fn reset(&mut self) {
        self.origin = crossterm::cursor::position().unwrap_or((0, 0));
        self.height = 0;
    }

    pub fn advance_by(&mut self, rows: u16) {
        self.height = self.height.saturating_add(rows);
    }

    /// Clear the all rendered area and reset the height to zero.
    pub fn clear(&mut self) -> std::io::Result<()> {
        execute!(stdout(), MoveTo(self.origin.0, self.origin.1 + self.height))?;
        for _ in 0..self.height {
            execute!(stdout(), MoveUp(1), Clear(ClearType::CurrentLine))?;
        }
        self.reset();

        Ok(())
    }
}

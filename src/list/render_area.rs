use crossterm::{
    cursor::MoveUp,
    execute,
    terminal::{Clear, ClearType},
};
use std::io::stdout;

#[derive(Default)]
pub struct RenderArea {
    height: u16,
}

impl RenderArea {
    /// Reset the origin and height of the render area.
    pub fn reset(&mut self) {
        self.height = 0;
    }

    pub fn advance_by(&mut self, rows: u16) {
        self.height = self.height.saturating_add(rows);
    }

    /// Clear the all rendered area and reset the height to zero.
    pub fn clear(&mut self) -> std::io::Result<()> {
        for _ in 0..self.height {
            execute!(stdout(), MoveUp(1), Clear(ClearType::CurrentLine))?;
        }
        self.reset();

        Ok(())
    }
}

use crossterm::{
    execute,
    terminal::{Clear, ClearType},
};
use std::io::stdout;

#[derive(Default)]
pub struct RenderArea {
    row: u16,
}

impl RenderArea {
    pub fn advance_by(&mut self, rows: u16) {
        self.row = self.row.saturating_add(rows);
    }

    pub fn clear(&mut self) -> std::io::Result<()> {
        for _ in 0..self.row {
            self.row = self.row.saturating_sub(1);
            execute!(stdout(), Clear(ClearType::CurrentLine))?;
        }

        Ok(())
    }
}

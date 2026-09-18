/// Enabled raw mode for terminal on creation and ensures that raw mode is disabled when the guard is dropped.
pub(super) struct RawGuard;

impl RawGuard {
    /// Creates a new `RawGuard` and enables raw mode for the terminal.
    pub(super) fn new() -> std::io::Result<Self> {
        crossterm::terminal::enable_raw_mode()?;
        Ok(RawGuard)
    }
}

impl Drop for RawGuard {
    fn drop(&mut self) {
        crossterm::terminal::disable_raw_mode().ok();
    }
}

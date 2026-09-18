use crate::RenderArea;
use std::fmt::Display;

pub trait Item: Display {
    /// Renders the item to the console.
    fn render(&self, render_area: &mut RenderArea);

    /// Handles the keyboard input for this item.
    /// Should return `true` if the list should be closed after rendering this item, or `false` if it should remain shown.
    fn handle_key(&mut self) -> bool;
}

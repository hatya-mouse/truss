mod label;

pub use label::LabelItem;

use crate::RenderArea;

pub trait Item {
    /// Renders the item to the console.
    fn render(&self, render_area: &mut RenderArea);

    /// Handles the keyboard input for this item.
    /// Should return `true` if the list should be closed after rendering this item, or `false` if it should remain shown.
    fn handle_key(&mut self) -> bool;

    /// Returns the default style of the item.
    fn default_style(&self) -> ItemStyle;
}

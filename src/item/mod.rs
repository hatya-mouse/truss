mod check;
mod label;

pub use check::CheckItem;
pub use label::LabelItem;

use crate::{ItemStyle, RenderArea};
use crossterm::event::KeyEvent;

pub trait Item {
    /// Renders the item to the console.
    fn render(&self, render_area: &mut RenderArea, item_style: ItemStyle);

    /// Handles the keyboard input for this item.
    /// Should return `true` if the list should be closed after rendering this item, or `false` if it should remain shown.
    fn handle_key(&mut self, event: KeyEvent) -> bool;
}

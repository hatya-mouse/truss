mod check;
mod field;
mod label;

pub use check::CheckItem;
pub use field::FieldItem;
pub use label::LabelItem;

use crate::{ItemStyle, RenderArea};
use crossterm::event::KeyEvent;

pub trait Item {
    /// Renders the item to the console.
    fn render(&self, render_area: &mut RenderArea, item_style: ItemStyle) -> std::io::Result<()>;

    /// Handles the keyboard input for this item.
    /// The first boolean should be `true` if the list should be closed after rendering this item, or `false` if it should remain shown.
    /// The second boolean represents whether the key event was handled by the item.
    fn handle_key(&mut self, event: KeyEvent) -> std::io::Result<(bool, bool)>;
}

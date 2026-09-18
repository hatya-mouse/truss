pub mod item;
mod list;
mod style;

pub use item::Item;
pub use list::{List, RenderArea};
pub use style::{ItemStyle, RichText};

use crate::item::LabelItem;

fn main() {
    List::default()
        .add_item(LabelItem::new("first"))
        .add_item(LabelItem::new("second"))
        .add_item(LabelItem::new("third"))
        .show()
        .expect("An error occured");
}

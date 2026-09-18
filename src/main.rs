pub mod item;
mod list;
mod style;

pub use item::Item;
pub use list::{List, RenderArea};
pub use style::{ItemStyle, RichText};

use crate::item::{CheckItem, LabelItem};

fn main() {
    let mut flag = false;
    List::default()
        .add_item(LabelItem::new("first"))
        .add_item(LabelItem::new("second"))
        .add_item(LabelItem::new("third"))
        .add_item(CheckItem::new("check", &mut flag))
        .show()
        .expect("An error occured");
}

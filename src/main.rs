pub mod item;
mod list;
mod style;
mod text_editor;

pub use item::Item;
pub use list::{List, RenderArea};
pub use style::{ItemStyle, RichText};

use crate::item::FieldItem;

fn main() {
    let mut flag = false;
    let mut test = "editor test\nmaomao".to_string();
    let mut test2 = "editor test\nmaomao".to_string();
    List::default()
        .label("first")
        .label("second")
        .label("third")
        .check("check", &mut flag)
        .field("field", &mut test)
        .add_item(
            FieldItem::new(
                "field",
                &mut test2,
                Box::new(|str| {
                    if str.len() < 10 {
                        Ok(str.to_string())
                    } else {
                        Err("This is wrong".to_string())
                    }
                }),
            )
            .multiline(true)
            .enable_edit_mode(true),
        )
        .clear_on_close(false)
        .show()
        .expect("An error occured");
}

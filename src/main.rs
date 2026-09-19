pub mod item;
mod list;
mod style;
mod text_editor;

pub use item::Item;
pub use list::{List, RenderArea};
pub use style::{ItemStyle, RichText};

fn main() {
    let mut flag = false;
    let mut test = "editor test\nmaomao".to_string();
    List::default()
        .label("first")
        .label("second")
        .label("third")
        .check("check", &mut flag)
        .field("field", &mut test)
        .clear_on_close(false)
        .show()
        .expect("An error occured");
}

pub mod item;
mod list;
mod style;

pub use item::Item;
pub use list::{List, RenderArea};
pub use style::{ItemStyle, RichText};

fn main() {
    let mut flag = false;
    List::default()
        .label("first")
        .label("second")
        .label("third")
        .check("check", &mut flag)
        .clear_on_close(false)
        .show()
        .expect("An error occured");
}

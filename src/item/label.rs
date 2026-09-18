use crate::{Item, ItemStyle, RenderArea};
use crossterm::event::KeyEvent;

pub struct LabelItem {
    text: String,
}

impl LabelItem {
    pub fn new(text: impl Into<String>) -> Self {
        Self { text: text.into() }
    }
}

impl Item for LabelItem {
    fn render(&self, render_area: &mut RenderArea, item_style: ItemStyle) {
        println!("{}", item_style.apply(&self.text));
        render_area.advance_by(self.text.lines().count().try_into().unwrap_or_default());
    }

    fn handle_key(&mut self, _event: KeyEvent) -> bool {
        false
    }
}

use crate::{Item, RenderArea, RichText};

pub struct LabelItem {
    text: RichText,
}

impl LabelItem {
    pub fn new(text: impl Into<RichText>) -> Self {
        Self { text: text.into() }
    }
}

impl Item for LabelItem {
    fn render(&self, render_area: &mut RenderArea) {
        println!("{}", self.text);
        render_area.advance_by(self.text.lines().try_into().unwrap_or_default());
    }

    fn handle_key(&mut self) -> bool {
        false
    }
}

use crate::{Item, ItemStyle, RenderArea};

pub struct LabelItem {
    text: String,
}

impl LabelItem {
    pub fn new(text: impl Into<String>) -> Self {
        Self { text: text.into() }
    }
}

impl Item for LabelItem {
    fn render(
        &mut self,
        render_area: &mut RenderArea,
        item_style: ItemStyle,
        _is_selected: bool,
    ) -> std::io::Result<()> {
        println!("{}", item_style.apply(&self.text));
        render_area.advance_by(self.text.lines().count().try_into().unwrap_or_default());
        Ok(())
    }
}

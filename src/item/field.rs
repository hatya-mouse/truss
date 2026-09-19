use crate::{Item, ItemStyle, RenderArea, text_editor::TextEditor};
use crossterm::event::KeyEvent;

pub struct FieldItem<'a> {
    label: String,
    editor: TextEditor<'a>,
}

impl<'a> FieldItem<'a> {
    pub fn new(label: impl Into<String>, content: &'a mut String) -> Self {
        Self {
            label: label.into(),
            editor: TextEditor::new(content),
        }
    }
}

impl Item for FieldItem<'_> {
    fn render(&self, render_area: &mut RenderArea, item_style: ItemStyle) -> std::io::Result<()> {
        print!("{}", item_style.apply(&self.label));
        let label_lines: u16 = self.label.lines().count().try_into().unwrap_or_default();
        render_area.advance_by(label_lines.saturating_sub(1));

        self.editor.render(render_area)?;

        println!();
        render_area.advance_by(1);

        Ok(())
    }

    fn handle_key(&mut self, event: KeyEvent) -> std::io::Result<(bool, bool)> {
        Ok((false, self.editor.handle_key(event)?))
    }
}

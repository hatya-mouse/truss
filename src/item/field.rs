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
    fn render(
        &mut self,
        render_area: &mut RenderArea,
        item_style: ItemStyle,
    ) -> std::io::Result<()> {
        print!("{}", item_style.apply(&self.label));
        let label_lines: u16 = self.label.lines().count().try_into().unwrap_or_default();
        render_area.advance_by(label_lines.saturating_sub(1));

        self.editor.render(render_area)
    }

    fn post_render(&self) -> std::io::Result<()> {
        self.editor.post_render()
    }

    fn handle_key(&mut self, event: KeyEvent) -> std::io::Result<(bool, bool)> {
        self.editor
            .handle_key(event)
            .map(|event_handled| (false, event_handled))
    }
}

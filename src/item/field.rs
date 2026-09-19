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
            editor: TextEditor::new(content, false, false),
        }
    }
}

impl Item for FieldItem<'_> {
    fn render(
        &mut self,
        render_area: &mut RenderArea,
        item_style: ItemStyle,
        _is_selected: bool,
    ) -> std::io::Result<()> {
        print!("{}", item_style.apply(&self.label));
        let label_lines: u16 = self.label.lines().count().try_into().unwrap_or_default();
        render_area.advance_by(label_lines.saturating_sub(1));

        self.editor.render(render_area)
    }

    fn post_render(&self, is_selected: bool) -> std::io::Result<()> {
        self.editor.post_render(is_selected)
    }

    fn handle_key(&mut self, event: KeyEvent) -> std::io::Result<(bool, bool)> {
        self.editor
            .handle_key(event)
            .map(|event_handled| (false, event_handled))
    }
}

impl FieldItem<'_> {
    /// Turns on or off multi-line editing for the field item.
    pub fn multiline(mut self, multiline: bool) -> Self {
        self.editor.multiline = multiline;
        self
    }

    pub fn enable_edit_mode(mut self, enable_edit_mode: bool) -> Self {
        self.editor.enable_edit_mode = enable_edit_mode;
        self
    }
}

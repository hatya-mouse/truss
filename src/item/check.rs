use crate::{Item, ItemStyle, RenderArea};
use crossterm::{
    event::{KeyCode, KeyEvent},
    style::Stylize,
};

pub struct CheckItem<'a> {
    label: String,
    checked: &'a mut bool,
}

impl<'a> CheckItem<'a> {
    pub fn new(label: impl Into<String>, checked: &'a mut bool) -> Self {
        Self {
            label: label.into(),
            checked,
        }
    }
}

impl Item for CheckItem<'_> {
    fn render(&self, render_area: &mut RenderArea, item_style: ItemStyle) -> std::io::Result<()> {
        let checkmark = if *self.checked {
            " \u{2713}".green()
        } else {
            "".reset()
        };
        println!("{}", item_style.apply(&self.label).join(checkmark));
        render_area.advance_by(self.label.lines().count().try_into().unwrap_or_default());

        Ok(())
    }

    fn handle_key(&mut self, event: KeyEvent) -> std::io::Result<(bool, bool)> {
        match event.code {
            crossterm::event::KeyCode::Char(' ') | KeyCode::Enter => {
                *self.checked = !*self.checked;
                return Ok((false, true));
            }
            _ => (),
        }

        Ok((false, false))
    }
}

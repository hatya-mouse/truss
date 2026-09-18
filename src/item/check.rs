use crate::{Item, ItemStyle, RenderArea};
use crossterm::{
    event::{KeyCode, KeyEvent},
    style::Stylize,
};

pub struct CheckItem<'a> {
    name: String,
    checked: &'a mut bool,
}

impl<'a> CheckItem<'a> {
    pub fn new(name: impl Into<String>, checked: &'a mut bool) -> Self {
        Self {
            name: name.into(),
            checked,
        }
    }
}

impl Item for CheckItem<'_> {
    fn render(&self, render_area: &mut RenderArea, item_style: ItemStyle) {
        let checkmark = if *self.checked {
            " \u{2713}".green()
        } else {
            "".reset()
        };
        println!("{}", item_style.apply(&self.name).join(checkmark));
        render_area.advance_by(self.name.lines().count().try_into().unwrap_or_default());
    }

    fn handle_key(&mut self, event: KeyEvent) -> bool {
        match event.code {
            crossterm::event::KeyCode::Char(' ') | KeyCode::Enter => {
                *self.checked = !*self.checked;
            }
            _ => (),
        }

        false
    }
}

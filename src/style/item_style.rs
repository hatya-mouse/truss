use crate::RichText;
use crossterm::style::{Attribute, ContentStyle, StyledContent, Stylize};

/// The style of the item in the list.
#[derive(Clone)]
pub struct ItemStyle {
    /// The prefix text of the item.
    pub prefix: RichText,
    /// The style of the item.
    pub style: ContentStyle,
}

impl ItemStyle {
    /// Creates a new `ItemStyle` with the given prefix, suffix, and style.
    pub fn new(prefix: RichText, style: ContentStyle) -> Self {
        Self { prefix, style }
    }

    /// Returns the default `ItemStyle` for selected items.
    pub fn default_selected() -> Self {
        Self {
            prefix: RichText::default().join("> ".green()),
            style: ContentStyle::new().attribute(Attribute::Bold),
        }
    }

    /// Returns the default `ItemStyle` for unselected items.
    pub fn default_item() -> Self {
        Self {
            prefix: RichText::default().join("  "),
            style: ContentStyle::new(),
        }
    }

    /// Generates a `RichText` for the given content with the current style, prefix, and suffix.
    pub fn apply(&self, content: impl Into<String>) -> RichText {
        RichText::default()
            .join(self.prefix.clone())
            .join(StyledContent::new(self.style, content.into()))
    }
}

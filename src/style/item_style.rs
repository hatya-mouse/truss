use crate::RichText;

/// The style of the item in the list.
pub struct ItemStyle {
    /// The prefix text of the item.
    pub prefix: RichText,
    /// The suffix text of the item.
    pub suffix: RichText,
    /// The style of the item.
    pub style: RichStyle,
}

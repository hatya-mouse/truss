use crossterm::style::{ContentStyle, StyledContent};
use std::fmt::Display;

#[derive(Debug, Default)]
pub struct RichText(Vec<StyledContent<String>>);

impl RichText {
    /// Returns a RichText with the given unstyled string joined at the end.
    pub fn join_str(mut self, string: impl Into<String>) -> Self {
        self.push(StyledContent::new(ContentStyle::default(), string.into()));
        self
    }

    /// Returns a RichText with the given styled content joined at the end.
    pub fn join(mut self, styled_content: impl Into<StyledContent<String>>) -> Self {
        self.push(styled_content);
        self
    }

    /// Appends the given styled content to the end of the RichText.
    pub fn push(&mut self, styled_content: impl Into<StyledContent<String>>) {
        self.0.push(styled_content.into());
    }

    /// Returns the number of lines in the RichText.
    pub fn lines(&self) -> usize {
        self.0
            .iter()
            .map(|styled_content| styled_content.content().lines().count())
            .sum()
    }
}

impl Display for RichText {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for styled_content in &self.0 {
            write!(f, "{}", styled_content)?;
        }
        Ok(())
    }
}

impl Into<RichText> for &str {
    fn into(self) -> RichText {
        RichText(vec![StyledContent::new(
            ContentStyle::default(),
            self.to_string(),
        )])
    }
}

impl Into<RichText> for String {
    fn into(self) -> RichText {
        RichText(vec![StyledContent::new(ContentStyle::default(), self)])
    }
}

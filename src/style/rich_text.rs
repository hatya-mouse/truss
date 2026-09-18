use crossterm::style::{ContentStyle, StyledContent};
use std::fmt::Display;

#[derive(Debug, Default, Clone)]
pub struct RichText(Vec<StyledContent<String>>);

impl RichText {
    /// Returns a RichText with the given unstyled string joined at the end.
    pub fn join_str(mut self, string: impl Into<String>) -> Self {
        self.push(StyledContent::new(ContentStyle::default(), string.into()));
        self
    }

    /// Returns a RichText with the given styled content joined at the end.
    pub fn join(mut self, styled_content: impl Into<RichText>) -> Self {
        self.push(styled_content);
        self
    }

    /// Appends the given styled content to the end of the RichText.
    pub fn push(&mut self, styled_content: impl Into<RichText>) {
        self.0.extend(styled_content.into().0);
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

impl From<&str> for RichText {
    fn from(s: &str) -> Self {
        RichText(vec![StyledContent::new(
            ContentStyle::default(),
            s.to_string(),
        )])
    }
}

impl From<String> for RichText {
    fn from(s: String) -> Self {
        RichText(vec![StyledContent::new(ContentStyle::default(), s)])
    }
}

impl From<StyledContent<&str>> for RichText {
    fn from(styled_content: StyledContent<&str>) -> Self {
        RichText(vec![StyledContent::new(
            *styled_content.style(),
            styled_content.content().to_string(),
        )])
    }
}

impl From<StyledContent<String>> for RichText {
    fn from(styled_content: StyledContent<String>) -> Self {
        RichText(vec![styled_content])
    }
}

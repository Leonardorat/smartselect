use super::{TextContext, TextRange};

pub struct SelectionEngine;

impl SelectionEngine {
    pub fn new() -> Self {
        Self
    }

    pub fn select(
        &self,
        context: &TextContext,
    ) -> Option<TextRange> {
        let caret = usize::try_from(context.caret).ok()?;

        let chars: Vec<char> =
            context.text.chars().collect();

        if chars.is_empty() {
            return None;
        }

        let index = caret.min(chars.len().saturating_sub(1));

        if !Self::is_identifier_char(chars[index]) {
            return None;
        }

        let mut start = index;
        let mut end = index + 1;

        while start > 0
            && Self::is_identifier_char(chars[start - 1])
        {
            start -= 1;
        }

        while end < chars.len()
            && Self::is_identifier_char(chars[end])
        {
            end += 1;
        }

        Some(TextRange {
            start: start as i32,
            end: end as i32,
        })
    }

    fn is_identifier_char(ch: char) -> bool {
        ch.is_alphanumeric()
            || matches!(
                ch,
                '_'
                    | '-'
                    | '.'
                    | '/'
                    | '\\'
                    | ':'
                    | '@'
            )
    }
}
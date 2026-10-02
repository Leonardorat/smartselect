#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct TextRange {
    pub start: i32,
    pub end: i32,
}

#[derive(Debug, Clone)]
pub struct TextContext {
    pub text: String,

    pub caret: i32,

    pub selection: Option<TextRange>,
}
use crate::config::Config;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Selection {
    pub start: usize,
    pub end: usize,
}

pub struct SelectionEngine {
    config: Config,
}

impl SelectionEngine {
    pub fn new(config: Config) -> Self {
        Self { config }
    }

    pub fn select(
        &self,
        text: &str,
        cursor: usize,
    ) -> Option<Selection> 
}
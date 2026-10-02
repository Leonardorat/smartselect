use anyhow::Result;
use async_trait::async_trait;

use crate::core::{
    TextContext,
    TextRange,
};

pub mod atspi;

#[async_trait]
pub trait TextAdapter {
    async fn context(
        &self,
    ) -> Result<Option<TextContext>>;

    async fn apply_selection(
        &self,
        range: TextRange,
    ) -> Result<()>;
}
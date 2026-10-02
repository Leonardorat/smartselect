use anyhow::{
    anyhow,
    Result,
};

use atspi::{
    AccessibilityConnection,
    ObjectRefOwned,
};

use atspi::proxy::text::TextProxy;

use crate::core::{
    TextContext,
    TextRange,
};

pub struct AtspiAdapter {
    connection: AccessibilityConnection,
}

impl AtspiAdapter {
    pub async fn connect() -> Result<Self> {
        let connection =
            AccessibilityConnection::new().await?;

        Ok(Self {
            connection,
        })
    }

    pub fn connection(
        &self,
    ) -> &AccessibilityConnection {
        &self.connection
    }

    async fn text_proxy<'a>(
        &'a self,
        object: &ObjectRefOwned,
    ) -> Result<TextProxy<'a>> {
        let bus_name = object
            .name_as_str()
            .ok_or_else(|| {
                anyhow!("AT-SPI object has no bus name")
            })?;

        let path = object.path_as_str();

        let proxy =
            TextProxy::builder(
                self.connection.connection()
            )
            .destination(bus_name.to_owned())?
            .path(path.to_owned())?
            .build()
            .await?;

        Ok(proxy)
    }

    pub async fn read_context(
        &self,
        object: &ObjectRefOwned,
    ) -> Result<TextContext> {
        let text_proxy =
            self.text_proxy(object).await?;

        let character_count =
            text_proxy.character_count().await?;

        let text =
            text_proxy
                .get_text(
                    0,
                    character_count,
                )
                .await?;

        let caret =
            text_proxy.caret_offset().await?;

        let selection =
            if text_proxy
                .get_n_selections()
                .await?
                > 0
            {
                let (start, end) =
                    text_proxy
                        .get_selection(0)
                        .await?;

                Some(TextRange {
                    start,
                    end,
                })
            } else {
                None
            };

        Ok(TextContext {
            text,
            caret,
            selection,
        })
    }

    pub async fn apply_selection(
        &self,
        object: &ObjectRefOwned,
        range: TextRange,
    ) -> Result<()> {
        let text_proxy =
            self.text_proxy(object).await?;

        let number_of_selections =
            text_proxy
                .get_n_selections()
                .await?;

        let successful =
            if number_of_selections > 0 {
                text_proxy
                    .set_selection(
                        0,
                        range.start,
                        range.end,
                    )
                    .await?
            } else {
                text_proxy
                    .add_selection(
                        range.start,
                        range.end,
                    )
                    .await?
            };

        if !successful {
            return Err(anyhow!(
                "Application refused selection change"
            ));
        }

        Ok(())
    }
}
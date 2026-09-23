use crate::{
    engine::SelectionEngine,
    linux::LinuxAccessibility,
};

pub struct SmartSelectService {
    engine: SelectionEngine,
    accessibility: LinuxAccessibility,
}

impl SmartSelectService {
    pub fn new(
        engine: SelectionEngine,
        accessibility: LinuxAccessibility,
    ) -> Self {
        Self {
            engine,
            accessibility,
        }
    }

    pub async fn run(
        &mut self,
    ) -> Result<(), String> {
        loop {
            self.accessibility
                .wait_for_event()
                .await?;

            self.handle_selection()
                .await?;
        }
    }

    async fn handle_selection(
        &self,
    ) -> Result<(), String> {
        let Some(text) =
            self.accessibility
                .current_text()
                .await?
        else {
            return Ok(());
        };

        let Some(cursor) =
            self.accessibility
                .current_cursor()
                .await?
        else {
            return Ok(());
        };

        let Some(selection) =
            self.engine.select(
                &text,
                cursor,
            )
        else {
            return Ok(());
        };

        self.accessibility
            .set_selection(
                selection.start,
                selection.end,
            )
            .await?;

        Ok(())
    }
}
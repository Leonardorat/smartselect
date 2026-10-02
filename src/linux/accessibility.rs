use atspi::AccessibilityConnection;

pub struct LinuxAccessibility {
    connection: AccessibilityConnection,
}

impl LinuxAccessibility {
    pub async fn connect() -> Result<Self, atspi::AtspiError> 
    {
        let connection =
            AccessibilityConnection::new().await?;

        Ok(Self {
            connection,
        })
    },

    pub async fn current_text(
        &self,
    ) -> Result<Option<String>, String> 

    pub async fn current_cursor(
        &self,
    ) -> Result<Option<usize>, String> 

    pub async fn set_selection(
        &self,
        start: usize,
        end: usize,
    ) -> Result<(), String> 

    pub async fn wait_for_event(
        &self,
    ) -> Result<(), String> 
}
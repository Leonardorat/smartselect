#[derive(Debug, Clone)]
pub struct Config {
    pub include_underscore: bool,
    pub include_dot: bool,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            include_underscore: true,
            include_dot: true,
        }
    }
}
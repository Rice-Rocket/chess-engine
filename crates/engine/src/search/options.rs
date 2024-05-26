#[derive(Clone, Copy)]
pub struct SearchOptions {
    /// Movetime in milliseconds
    pub movetime: Option<u32>,
    /// Depth to search to
    pub depth: Option<u16>,
}

impl Default for SearchOptions {
    fn default() -> Self {
        Self {
            movetime: Some(1000),
            depth: None,
        }
    }
}

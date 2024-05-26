#[derive(Clone, Copy, Default)]
pub struct SearchOptions {
    /// Movetime in milliseconds
    pub movetime: Option<u32>,
    /// Depth to search to
    pub depth: Option<u16>,
}

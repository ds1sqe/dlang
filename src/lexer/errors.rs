#[derive(Debug)]
pub struct LexerError {
    pub pos_start: usize,
    pub pos_end: usize,
    pub reason: String,
}

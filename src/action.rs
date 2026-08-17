// Advance (For now, Enter)
// Quit

#[derive(Debug, Clone, Copy)]
pub enum Action {
    Advance,
    ScrollUp,
    ScrollDown,
    Quit,
}
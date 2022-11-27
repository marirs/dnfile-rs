use serde::Serialize;

#[derive(Debug, Clone, Serialize)]
pub struct Argument {
    index: usize,
}

impl Argument {
    pub fn new(index: usize) -> Self {
        Self { index }
    }
}

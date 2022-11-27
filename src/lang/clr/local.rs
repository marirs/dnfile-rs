use serde::Serialize;

#[derive(Debug, Clone, Serialize)]
pub struct Local {
    index: usize,
}

impl Local {
    pub fn new(index: usize) -> Self {
        Self { index }
    }
}

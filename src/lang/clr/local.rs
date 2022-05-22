#[derive(Debug, Clone, serde::Serialize)]
pub struct Local {
    index: usize,
}

impl Local {
    pub fn new(index: usize) -> Self {
        Self { index }
    }
}

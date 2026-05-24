use serde::Serialize;

#[derive(Debug, Default, Clone, Serialize)]
pub struct Local {
    index: usize,
}

impl Local {
    #[must_use]
    pub fn new(index: usize) -> Self {
        Self { index }
    }
}

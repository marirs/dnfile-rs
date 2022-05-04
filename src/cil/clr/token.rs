

pub const RID_MASK: usize = 0x00FFFFFF;
pub const RID_MAX: usize = RID_MASK;
pub const TABLE_SHIFT: usize = 24;

#[derive(Debug, Clone, serde::Serialize)]
pub struct Token{
    pub value: usize
}

impl Token{
    pub fn new(value: usize) -> Self{
        Self{
            value
        }
    }
    pub fn rid(&self) -> usize{
        self.value & RID_MASK
    }
    pub fn table(&self) -> usize{
        self.value >> TABLE_SHIFT
    }
}

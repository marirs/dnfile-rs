
#[derive(Debug)]
pub enum Operand{
    Token(super::super::clr::token::Token),
    Local(super::super::clr::local::Local),
    Argument(super::super::clr::argument::Argument),
    Arguments(Vec<Operand>),
    Int(i64),
    Float(f64),
    None
}

#[derive(Debug)]
pub struct Instruction{
    pub offset: usize,
    pub opcode: super::opcode::OpCode,
    pub opcode_bytes: Vec<u8>,
    pub operand: Operand,
    pub operand_bytes: Vec<u8>
}

impl Instruction{
}

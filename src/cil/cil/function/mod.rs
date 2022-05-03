mod reader;
mod flags;

use super::super::clr::token::Token;

#[derive(Debug, Clone)]
pub struct Function {
    offset: u64,
    header_size: usize,
    flags: flags::CilMethodBodyFlags,
    max_stack: usize,
    code_size: usize,
    local_var_sig_tok: Option<Token>,
    size: usize,
    raw_bytes: Vec<u8>,
    exception_handlers_size: usize,
    instructions: Vec<super::instruction::Instruction>,
    exception_handlers: Vec<super::exception::ExceptionHandler>,
}

impl Function {}

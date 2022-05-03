use super::enums::ExceptionHandlerType;


pub const TINY_SIZE: usize = 12;
pub const FAT_SIZE: usize = 24;

#[derive(Debug, Default)]
pub struct ExceptionHandler{
    exception_type: usize,
    try_start: Option<usize>,
    try_end: Option<usize>,
    filter_start: Option<usize>,
    handler_start: Option<usize>,
    handler_end: Option<usize>,
    catch_type: Option<super::super::clr::token::Token>
}

impl ExceptionHandler{
    pub fn new(exception_type: usize) -> Self{
        Self{
            exception_type,
            ..Default::default()
        }
    }

    pub fn is_catch(&self) -> bool{
        self.exception_type & 7 == ExceptionHandlerType::Catch as usize
    }

    pub fn is_filter(&self) -> bool{
        self.exception_type & ExceptionHandlerType::Filter as usize != 0
    }

    pub fn is_finally(&self) -> bool{
        self.exception_type & ExceptionHandlerType::Finally as usize != 0
    }

    pub fn is_fault(&self) -> bool{
        self.exception_type & ExceptionHandlerType::Fault as usize != 0
    }
}

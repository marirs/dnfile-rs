use super::enums::ExceptionHandlerType;


pub const TINY_SIZE: usize = 12;
pub const FAT_SIZE: usize = 24;

#[derive(Debug, Default, Clone, serde::Serialize)]
pub struct ExceptionHandler{
    pub exception_type: usize,
    pub try_start: i64,
    pub try_end: i64,
    pub filter_start: i64,
    pub handler_start: i64,
    pub handler_end: i64,
    pub catch_type: Option<super::super::clr::token::Token>
}

impl ExceptionHandler{
    pub fn new(exception_type: usize) -> Self{
        Self{
            exception_type,
            try_start: -1,
            try_end: -1,
            filter_start: -1,
            handler_start: -1,
            handler_end: -1,
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

use crate::Result;

pub mod reader;
pub mod flags;

use super::super::clr::token::Token;
use super::enums::*;

#[derive(Debug, Clone, serde::Serialize)]
pub struct Function {
    offset: usize,
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

impl Function {
    pub fn new(reader: &mut reader::Reader) -> Result<Self>{
        let mut res = Self{
            offset: reader.tell()?,
            header_size: 0,
            flags: flags::CilMethodBodyFlags::new(0),
            max_stack: 0,
            code_size: 0,
            local_var_sig_tok: None,
            size: 0,
            raw_bytes: vec![],
            exception_handlers_size: 0,
            instructions: vec![],
            exception_handlers: vec![],
        };
        res.parse_header(reader)?;
        res.parse_instructions(reader)?;
        res.parse_exception_handlers(reader)?;
        Ok(res)
    }

    pub fn parse_header(&mut self, reader: &mut reader::Reader) -> Result<()>{
        let header_byte = reader.read_u8()? as usize;
        if vec![CorILMethod::TinyFormat as usize, CorILMethod::TinyFormat1 as usize].contains(&(header_byte & CorILMethod::FormatMask as usize)){
            self.flags = flags::CilMethodBodyFlags::new(header_byte as usize & CorILMethod::FormatMask as usize);
            self.header_size = 1;
            self.max_stack = 8;
            self.code_size = header_byte as usize >> 2;
            self.local_var_sig_tok = None;
        } else if vec![CorILMethod::FatFormat as usize].contains(&(header_byte as usize & CorILMethod::FormatMask as usize)){
            self.flags = flags::CilMethodBodyFlags::new(((reader.read_u8()? as usize) << 8) | header_byte as usize);
            self.header_size = self.flags.flags >> 12;
            self.max_stack = reader.read_u16()? as usize;
            self.code_size = reader.read_u32()? as usize;
            let local_var_sig_tok = reader.read_u32()? as usize;
            if local_var_sig_tok == 0{
                self.local_var_sig_tok = None;
            } else {
                self.local_var_sig_tok = Some(Token::new(local_var_sig_tok));
            }
            let pos = reader.tell()? - 12 + self.header_size * 4;
            reader.seek(pos)?;
            if self.header_size < 3{
                self.flags.flags &= 0xFFF7;
            }
            self.header_size *= 4
        } else {
            return Err(crate::error::Error::MethodBodyFormatError(format!("bad header format {:02x}", header_byte as usize & CorILMethod::FormatMask as usize)))
        }
        Ok(())
    }

    pub fn parse_instructions(&mut self, reader: &mut reader::Reader) -> Result<()>{
        let mut current_offset = self.offset + self.header_size;
        let code_end_offset = reader.tell()? + self.code_size;
        while reader.tell()? < code_end_offset{
            let insn = reader.read_instruction(current_offset)?;
            current_offset += insn.size();
            self.instructions.push(insn);
        }
        Ok(())
    }

    pub fn parse_exception_handlers(&mut self, reader: &mut reader::Reader) -> Result<()>{
        if !self.flags.more_sects(){
            self.size = reader.tell()? - self.offset;
            return Ok(())
        }
        let pos = (reader.tell()? + 3) & !3;
        reader.seek(pos)?;
        let header_byte = reader.read_u8()?;
        if header_byte as usize & CorILMethodSect::KindMask as usize != 1{
            self.size = reader.tell()? - self.offset;
            return Ok(())
        }
        if header_byte as usize & CorILMethodSect::FatFormat as usize != 0{
            self.parse_fat_exception_handlers(reader)?;
        } else {
            self.parse_tiny_exception_handlers(reader)?;
        }
        self.size = reader.tell()? - self.offset;
        Ok(())
    }

    pub fn parse_fat_exception_handlers(&mut self, reader: &mut reader::Reader) -> Result<()>{
        let pos = reader.tell()? - 1;
        reader.seek(pos)?;
        let total_size = reader.read_u32()? >> 8;
        let num_exceptions = total_size; // ExceptionHandler.FAT_SIZE
        for _ in 0..num_exceptions{
            let mut eh = super::exception::ExceptionHandler::new(reader.read_u32()? as usize);
            eh.try_start = reader.read_i32()?;
            eh.try_end = eh.try_start + reader.read_i32()?;
            eh.handler_start = reader.read_i32()?;
            eh.handler_end = eh.handler_start + reader.read_i32()?;
            if eh.is_catch(){
                eh.catch_type = Some(Token::new(reader.read_u32()? as usize));
            } else if eh.is_filter(){
                eh.filter_start = reader.read_u32()? as i32;
            } else {
                reader.read_u32()?;
            }
            self.exception_handlers.push(eh);
        }
        Ok(())
    }

    pub fn parse_tiny_exception_handlers(&mut self, reader: &mut reader::Reader) -> Result<()>{
        let num_exceptions = reader.read_u8()? as usize;
        let pos = reader.tell()? + 2;
        reader.seek(pos)?;
        for _ in 0..num_exceptions{
            let mut eh = super::exception::ExceptionHandler::new(reader.read_u16()? as usize);
            eh.try_start = reader.read_u16()? as i32;
            eh.try_end = eh.try_start + reader.read_u8()? as i32;
            eh.handler_start = reader.read_u16()? as i32;
            eh.handler_end = eh.handler_start + reader.read_u8()? as i32;
            if eh.is_catch(){
                eh.catch_type = Some(Token::new(reader.read_u32()? as usize));
            } else if eh.is_filter(){
                eh.filter_start = reader.read_u32()? as i32;
            } else {
                reader.read_u32()?;
            }
            self.exception_handlers.push(eh);
        }
        Ok(())
    }
}

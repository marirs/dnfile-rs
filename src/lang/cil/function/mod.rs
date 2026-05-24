use crate::{Result, error::Error};

pub mod flags;
pub mod reader;

use super::super::clr::token::Token;
use super::enums::*;

use serde::Serialize;

#[derive(Debug, Clone, Serialize)]
pub struct Function {
    pub offset: usize,
    header_size: usize,
    flags: flags::CilMethodBodyFlags,
    max_stack: usize,
    code_size: usize,
    local_var_sig_tok: Option<Token>,
    size: usize,
    exception_handlers_size: usize,
    pub instructions: Vec<super::instruction::Instruction>,
    exception_handlers: Vec<super::exception::ExceptionHandler>,
}

impl Function {
    pub fn new(reader: &mut reader::Reader<'_>) -> Result<Self> {
        let mut res = Self {
            offset: reader.tell()?,
            header_size: 0,
            flags: flags::CilMethodBodyFlags::new(0),
            max_stack: 0,
            code_size: 0,
            local_var_sig_tok: None,
            size: 0,
            exception_handlers_size: 0,
            instructions: vec![],
            exception_handlers: vec![],
        };
        res.parse_header(reader)?;
        res.parse_instructions(reader)?;
        res.parse_exception_handlers(reader)?;
        Ok(res)
    }

    pub fn parse_header(&mut self, reader: &mut reader::Reader<'_>) -> Result<()> {
        let header_byte = reader.read_u8()? as usize;
        if [
            CorILMethod::TinyFormat as usize,
            CorILMethod::TinyFormat1 as usize,
        ]
        .contains(&(header_byte & CorILMethod::FormatMask as usize))
        {
            self.flags =
                flags::CilMethodBodyFlags::new(header_byte & CorILMethod::FormatMask as usize);
            self.header_size = 1;
            self.max_stack = 8;
            self.code_size = header_byte >> 2;
            self.local_var_sig_tok = None;
        } else if [CorILMethod::FatFormat as usize]
            .contains(&(header_byte & CorILMethod::FormatMask as usize))
        {
            self.flags =
                flags::CilMethodBodyFlags::new(((reader.read_u8()? as usize) << 8) | header_byte);
            self.header_size = self.flags.flags >> 12;
            self.max_stack = reader.read_u16()? as usize;
            self.code_size = reader.read_u32()? as usize;
            let local_var_sig_tok = reader.read_u32()? as usize;
            if local_var_sig_tok == 0 {
                self.local_var_sig_tok = None;
            } else {
                self.local_var_sig_tok = Some(Token::new(local_var_sig_tok));
            }
            let pos = reader.tell()? - 12 + self.header_size * 4;
            reader.seek(pos)?;
            if self.header_size < 3 {
                self.flags.flags &= 0xFFF7;
            }
            self.header_size *= 4
        } else {
            return Err(Error::MethodBodyFormatError(format!(
                "bad header format {:02x}",
                header_byte & CorILMethod::FormatMask as usize
            )));
        }
        Ok(())
    }

    pub fn parse_instructions(&mut self, reader: &mut reader::Reader<'_>) -> Result<()> {
        let mut current_offset = self
            .offset
            .checked_add(self.header_size)
            .ok_or(Error::MethodBodyFormatError(
                "method offset+header_size overflow".to_string(),
            ))?;
        let code_end_offset = reader
            .tell()?
            .checked_add(self.code_size)
            .ok_or(Error::MethodBodyFormatError(
                "method code_size overflow".to_string(),
            ))?;
        while reader.tell()? < code_end_offset {
            let insn = reader.read_instruction(current_offset)?;
            // Defensive: a zero-size instruction would loop forever. The
            // current opcode table never produces one, but a future bug
            // shouldn't be able to hang the parser.
            let isize = insn.size();
            if isize == 0 {
                return Err(Error::MethodBodyFormatError(
                    "zero-size instruction".to_string(),
                ));
            }
            current_offset = current_offset.checked_add(isize).ok_or(
                Error::MethodBodyFormatError("instruction offset overflow".to_string()),
            )?;
            self.instructions.push(insn);
        }
        Ok(())
    }

    pub fn parse_exception_handlers(&mut self, reader: &mut reader::Reader<'_>) -> Result<()> {
        if !self.flags.more_sects() {
            self.size = reader.tell()? - self.offset;
            return Ok(());
        }
        let pos = (reader.tell()? + 3) & !3;
        reader.seek(pos)?;
        let header_byte = reader.read_u8()?;
        if header_byte as usize & CorILMethodSect::KindMask as usize != 1 {
            self.size = reader.tell()? - self.offset;
            return Ok(());
        }
        if header_byte as usize & CorILMethodSect::FatFormat as usize != 0 {
            self.parse_fat_exception_handlers(reader)?;
        } else {
            self.parse_tiny_exception_handlers(reader)?;
        }
        self.size = reader.tell()? - self.offset;
        Ok(())
    }

    pub fn parse_fat_exception_handlers(&mut self, reader: &mut reader::Reader<'_>) -> Result<()> {
        let pos = reader
            .tell()?
            .checked_sub(1)
            .ok_or(Error::MethodBodyFormatError(
                "fat EH header out of bounds".to_string(),
            ))?;
        reader.seek(pos)?;
        let total_size = (reader.read_u32()? >> 8) as usize;
        // Per ECMA-335 II.25.4.6: total_size is the byte length of the
        // section including the 4-byte header; each fat clause is 24 bytes.
        // Use saturating math + clamp against remaining buffer so a crafted
        // `total_size` near 2^24 doesn't drive a ~640 MB Vec allocation.
        let count_from_header = total_size.saturating_sub(4) / 24;
        let remaining = reader.stream_remaining() / 24;
        let num_exceptions = count_from_header.min(remaining);
        for _ in 0..num_exceptions {
            let mut eh = super::exception::ExceptionHandler::new(reader.read_u32()? as usize);
            eh.try_start = reader.read_i32()? as i64;
            let bb = reader.read_i32()?;
            eh.try_end = eh.try_start + bb as i64;
            eh.handler_start = reader.read_i32()? as i64;
            eh.handler_end = eh.handler_start + reader.read_i32()? as i64;
            if eh.is_catch() {
                eh.catch_type = Some(Token::new(reader.read_u32()? as usize));
            } else if eh.is_filter() {
                eh.filter_start = reader.read_u32()? as i64;
            } else {
                reader.read_u32()?;
            }
            self.exception_handlers.push(eh);
        }
        Ok(())
    }

    pub fn parse_tiny_exception_handlers(&mut self, reader: &mut reader::Reader<'_>) -> Result<()> {
        let num_exceptions = reader.read_u8()? as usize;
        let pos = reader.tell()? + 2;
        reader.seek(pos)?;
        for _ in 0..num_exceptions {
            let mut eh = super::exception::ExceptionHandler::new(reader.read_u16()? as usize);
            eh.try_start = reader.read_u16()? as i64;
            eh.try_end = eh.try_start + reader.read_u8()? as i64;
            eh.handler_start = reader.read_u16()? as i64;
            eh.handler_end = eh.handler_start + reader.read_u8()? as i64;
            if eh.is_catch() {
                eh.catch_type = Some(Token::new(reader.read_u32()? as usize));
            } else if eh.is_filter() {
                eh.filter_start = reader.read_u32()? as i64;
            } else {
                reader.read_u32()?;
            }
            self.exception_handlers.push(eh);
        }
        Ok(())
    }
}

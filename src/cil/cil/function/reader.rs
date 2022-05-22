use super::super::super::clr::{argument::Argument, local::Local, token::Token};
use super::super::instruction::{Instruction, Operand};
use super::super::{enums::*, opcode::*};
use crate::Result;
use byteorder::ReadBytesExt;

use std::io::Seek;

pub struct Reader {
    cil_opcodes: OpCodes,
    stream: std::io::BufReader<std::io::Cursor<Vec<u8>>>,
}

impl Reader {
    pub fn new(bytes: &[u8]) -> Self {
        Self {
            cil_opcodes: OpCodes::new(),
            stream: std::io::BufReader::new(std::io::Cursor::new(bytes.to_vec())),
        }
    }

    pub fn tell(&mut self) -> Result<usize> {
        Ok(self.stream.stream_position()? as usize)
    }

    pub fn seek(&mut self, pos: usize) -> Result<usize> {
        Ok(self.stream.seek(std::io::SeekFrom::Start(pos as u64))? as usize)
    }

    pub fn is_arg_operand_instruction(&mut self, insn: &Instruction) -> bool {
        return vec![
            OpCodeValue::Ldarg,
            OpCodeValue::Ldarg_S,
            OpCodeValue::Ldarga,
            OpCodeValue::Ldarga_S,
            OpCodeValue::Starg,
            OpCodeValue::Starg_S,
        ]
        .contains(&insn.opcode.value);
    }

    pub fn read_u8(&mut self) -> Result<u8> {
        Ok(self.stream.read_u8()?)
    }

    pub fn read_i8(&mut self) -> Result<i8> {
        Ok(self.stream.read_i8()?)
    }

    pub fn read_u16(&mut self) -> Result<u16> {
        Ok(self.stream.read_u16::<byteorder::LittleEndian>()?)
    }

    pub fn read_i16(&mut self) -> Result<i16> {
        Ok(self.stream.read_i16::<byteorder::LittleEndian>()?)
    }

    pub fn read_u32(&mut self) -> Result<u32> {
        Ok(self.stream.read_u32::<byteorder::LittleEndian>()?)
    }

    pub fn read_i32(&mut self) -> Result<i32> {
        Ok(self.stream.read_i32::<byteorder::LittleEndian>()?)
    }

    pub fn read_u64(&mut self) -> Result<u64> {
        Ok(self.stream.read_u64::<byteorder::LittleEndian>()?)
    }

    pub fn read_i64(&mut self) -> Result<i64> {
        Ok(self.stream.read_i64::<byteorder::LittleEndian>()?)
    }

    pub fn read_f32(&mut self) -> Result<f32> {
        Ok(self.stream.read_f32::<byteorder::LittleEndian>()?)
    }

    pub fn read_f64(&mut self) -> Result<f64> {
        Ok(self.stream.read_f64::<byteorder::LittleEndian>()?)
    }

    pub fn read_inline_br_target(&mut self, insn: &Instruction) -> Result<Operand> {
        let branch_offset = self.read_u32()? as usize;
        Ok(Operand::Int(
            (insn.offset + insn.size() + branch_offset) as i64,
        ))
    }

    pub fn read_inline_field(&mut self, _insn: &Instruction) -> Result<Operand> {
        let token_value = self.read_u32()? as usize;
        Ok(Operand::Token(Token::new(token_value)))
    }

    pub fn read_inline_i(&mut self, _insn: &Instruction) -> Result<Operand> {
        let v = self.read_u32()? as i64;
        Ok(Operand::Int(v))
    }

    pub fn read_inline_i8(&mut self, _insn: &Instruction) -> Result<Operand> {
        let v = self.read_i64()?;
        Ok(Operand::Int(v))
    }

    pub fn read_inline_method(&mut self, _insn: &Instruction) -> Result<Operand> {
        let token_value = self.read_u32()? as usize;
        Ok(Operand::Token(Token::new(token_value)))
    }

    pub fn read_inline_none(&mut self, _insn: &Instruction) -> Result<Operand> {
        Ok(Operand::None)
    }

    pub fn read_inline_phi(&mut self, _insn: &Instruction) -> Result<Operand> {
        Ok(Operand::None)
    }

    pub fn read_inline_r(&mut self, _insn: &Instruction) -> Result<Operand> {
        let v = self.read_f64()?;
        Ok(Operand::Float(v))
    }

    pub fn read_inline_sig(&mut self, _insn: &Instruction) -> Result<Operand> {
        let token_value = self.read_u32()? as usize;
        Ok(Operand::Token(Token::new(token_value)))
    }

    pub fn read_inline_string(&mut self, _insn: &Instruction) -> Result<Operand> {
        let token_value = self.read_u32()? as usize;
        Ok(Operand::StringToken(Token::new(token_value)))
    }

    pub fn read_inline_switch(&mut self, insn: &Instruction) -> Result<Operand> {
        let num_branches = self.read_u32()? as usize;
        let offset_after_insn = insn.offset + insn.opcode.size() + 4 + num_branches * 4;
        let mut branches = vec![];
        for _ in 0..num_branches {
            let branch_offset = self.read_u32()? as usize;
            branches.push(Operand::Int((offset_after_insn + branch_offset) as i64));
        }
        Ok(Operand::Arguments(branches))
    }

    pub fn read_inline_tok(&mut self, _insn: &Instruction) -> Result<Operand> {
        let token_value = self.read_u32()? as usize;
        Ok(Operand::Token(Token::new(token_value)))
    }

    pub fn read_inline_type(&mut self, _insn: &Instruction) -> Result<Operand> {
        let token_value = self.read_u32()? as usize;
        Ok(Operand::Token(Token::new(token_value)))
    }

    pub fn read_inline_var(&mut self, insn: &Instruction) -> Result<Operand> {
        let var_value = self.read_u16()?;
        if self.is_arg_operand_instruction(insn) {
            Ok(Operand::Argument(Argument::new(var_value as usize)))
        } else {
            Ok(Operand::Local(Local::new(var_value as usize)))
        }
    }

    pub fn read_short_inline_br_target(&mut self, insn: &Instruction) -> Result<Operand> {
        let branch_offset = self.read_u8()? as usize;
        Ok(Operand::Int(
            (insn.offset + insn.size() + branch_offset) as i64,
        ))
    }

    pub fn read_short_inline_i(&mut self, insn: &Instruction) -> Result<Operand> {
        if insn.opcode.value == OpCodeValue::Ldc_I4_S {
            let val = self.read_i8()?;
            Ok(Operand::Int(val as i64))
        } else {
            let val = self.read_u8()?;
            Ok(Operand::Int(val as i64))
        }
    }

    pub fn read_short_inline_r(&mut self, _insn: &Instruction) -> Result<Operand> {
        Ok(Operand::Float(self.read_f32()? as f64))
    }

    pub fn read_short_inline_var(&mut self, insn: &Instruction) -> Result<Operand> {
        let var_value = self.read_u8()?;
        if self.is_arg_operand_instruction(insn) {
            Ok(Operand::Argument(Argument::new(var_value as usize)))
        } else {
            Ok(Operand::Local(Local::new(var_value as usize)))
        }
    }

    pub fn read_instruction(&mut self, off: usize) -> Result<Instruction> {
        let mut insn = Instruction::new();
        insn.offset = off;
        insn.opcode = self.read_opcode()?;
        insn.operand = self.read_operand(&insn)?;
        Ok(insn)
    }

    pub fn read_opcode(&mut self) -> Result<OpCode> {
        let op_value_first = self.read_u8()? as usize;
        if op_value_first == 0xFE {
            let op_value_second = self.read_u8()? as usize;
            Ok(self.cil_opcodes.two_byte_op_codes[op_value_second].clone())
        } else {
            Ok(self.cil_opcodes.two_byte_op_codes[op_value_first].clone())
        }
    }

    pub fn read_operand(&mut self, insn: &Instruction) -> Result<Operand> {
        match insn.opcode.operand_type {
            OperandType::InlineBrTarget => self.read_inline_br_target(insn),
            OperandType::InlineField => self.read_inline_field(insn),
            OperandType::InlineI => self.read_inline_i(insn),
            OperandType::InlineI8 => self.read_inline_i8(insn),
            OperandType::InlineMethod => self.read_inline_method(insn),
            OperandType::InlineNone => self.read_inline_none(insn),
            OperandType::InlinePhi => self.read_inline_phi(insn),
            OperandType::InlineR => self.read_inline_r(insn),
            OperandType::InlineSig => self.read_inline_sig(insn),
            OperandType::InlineString => self.read_inline_string(insn),
            OperandType::InlineSwitch => self.read_inline_switch(insn),
            OperandType::InlineTok => self.read_inline_tok(insn),
            OperandType::InlineType => self.read_inline_type(insn),
            OperandType::InlineVar => self.read_inline_var(insn),
            OperandType::ShortInlineBrTarget => self.read_short_inline_br_target(insn),
            OperandType::ShortInlineI => self.read_short_inline_i(insn),
            OperandType::ShortInlineR => self.read_short_inline_r(insn),
            OperandType::ShortInlineVar => self.read_short_inline_var(insn),
            _ => Err(crate::error::Error::UndefinedOperandType(
                insn.opcode.operand_type.clone(),
            )),
        }
    }
}

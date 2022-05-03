use crate::Result;
use std::io::Read;
use byteorder::ReadBytesExt;

pub struct Reader{
    stream: std::io::BufReader<std::io::Cursor<Vec<u8>>>
}

impl Reader{
    pub fn new(bytes: &[u8]) -> Self{
        Self{
            stream: std::io::BufReader::new(std::io::Cursor::new(bytes.to_vec()))
        }
    }

    pub fn read_u8(&mut self) -> Result<u8>{
        Ok(self.stream.read_u8()?)
    }

    pub fn read_i8(&mut self) -> Result<i8>{
        Ok(self.stream.read_i8()?)
    }

    pub fn read_u16(&mut self) -> Result<u16>{
        Ok(self.stream.read_u16::<byteorder::LittleEndian>()?)
    }

    pub fn read_i16(&mut self) -> Result<i16>{
        Ok(self.stream.read_i16::<byteorder::LittleEndian>()?)
    }
}

use crate::Result;


pub fn read_usize(data: &[u8]) -> Result<usize>{
    match data.len(){
        1 => Ok(data[0] as usize),
        2 => Ok(u16::from_le_bytes(data[..].try_into()?) as usize),
        4 => Ok(u32::from_le_bytes(data[..].try_into()?) as usize),
        8 => Ok(u64::from_le_bytes(data[..].try_into()?) as usize),
        _ => Err(crate::error::Error::CantReadUsizeFromBytesLen(data.len()))
    }
}


pub fn read_compressed_usize(data: &[u8]) -> Result<(usize, usize)>{
    if data[0] & 0x80 == 0{
        Ok((data[0] as usize, 1))
    } else if data[0] & 0x40 == 0{
        let mut value = (data[0] as usize & 0x7F) << 8;
        value |= data[1] as usize;
        Ok((value, 2))
    } else if data[0] & 0x20 == 0 {
        let mut value = (data[0] as usize & 0x3F) << 24;
        value |= (data[1] as usize) << 16;
        value |= (data[2] as usize) << 8;
        value |= data[3] as usize;
        Ok((value, 4))
    } else {
        Err(crate::error::Error::ReadCompressedUsize)
    }
}

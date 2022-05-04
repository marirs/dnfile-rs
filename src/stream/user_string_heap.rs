use crate::Result;

#[derive(Debug, Clone, serde::Serialize)]
pub struct UserStringHeap{
    #[serde(skip_serializing)]
    data: Vec<u8>
}

impl UserStringHeap{
    pub fn get(&self, index: usize) -> Result<Vec<u8>>{
        if index >= self.data.len(){
            return Err(crate::error::Error::UserStringHeapReadOutOfBound(index, self.data.len()));
        }
        let (data_length, length_size) = crate::utils::read_compressed_usize(&self.data[index..index+4])?;
        if index+length_size+data_length >= self.data.len()+1{
            return Err(crate::error::Error::UserStringHeapReadOutOfBound(index+data_length+length_size, self.data.len()));
        }
        Ok(self.data[index+length_size..index+length_size+data_length].to_vec())
    }

    pub fn get_us(&self, index: usize) -> Result<String>{
        let data = self.get(index)?;
        let utf16: Vec<u16> = data.chunks_exact(2).map(|c| (c[0] as u16) << 8 | c[1] as u16).collect();
        Ok(String::from_utf16(&utf16)?)
    }
}


impl crate::DnPe<'_>{
    pub fn new_user_string_heap(&self,
                                _metadata_rva: &u32,
                                _stream_offset: &u32,
                                _stream_size: &usize,
                                _stream_name: &str,
                                stream_data: Vec<u8>) -> Result<super::Stream>{
        Ok(super::Stream::UserStringHeap(UserStringHeap{
            data: stream_data
       }))
    }
}

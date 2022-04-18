use crate::Result;

#[derive(Debug, Clone)]
pub struct StringHeap{
    data: Vec<u8>
}

impl crate::DnPe<'_>{
    pub fn new_string_heap(&self,
                           _metadata_rva: &u32,
                           _stream_offset: &u32,
                           _stream_size: &usize,
                           _stream_name: &str,
                           stream_data: Vec<u8>) -> Result<super::Stream>{
        Ok(super::Stream::StringHeap(StringHeap{
            data: stream_data
        }))
    }
}

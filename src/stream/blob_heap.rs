use crate::Result;

#[derive(Debug, Clone)]
pub struct BlobHeap{
}

impl crate::DnPe<'_>{
    pub fn new_blob_heap(&self,
                         metadata_rva: &u32,
                         stream_offset: &u32,
                         stream_size: &usize,
                         stream_name: &str,
                         stream_data: Vec<u8>) -> Result<super::Stream>{
        Ok(super::Stream::BlobHeap(BlobHeap{
        }))
    }
}

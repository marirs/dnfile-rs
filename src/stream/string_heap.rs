use crate::{error::Error, Result};

#[derive(Debug, Clone, serde::Serialize)]
pub struct StringHeap {
    #[serde(skip_serializing)]
    data: Vec<u8>,
}

impl StringHeap {
    pub fn get(&self, index: usize) -> Result<String> {
        if index >= self.data.len() {
            return Err(Error::StringHeapReadOutOfBound(index, self.data.len()));
        }
        let mut res_buf = vec![];
        for i in index..self.data.len() {
            if self.data[i] != 0 {
                res_buf.push(self.data[i]);
            } else {
                break;
            }
        }
        Ok(String::from_utf8_lossy(&res_buf).to_string())
    }
}

impl crate::DnPe {
    pub fn new_string_heap(
        &self,
        _metadata_rva: &u32,
        _stream_offset: &u32,
        _stream_size: &usize,
        _stream_name: &str,
        stream_data: Vec<u8>,
    ) -> Result<super::Stream> {
        Ok(super::Stream::StringHeap(StringHeap { data: stream_data }))
    }
}

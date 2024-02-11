use crate::{error::Error, Result};

#[derive(Debug, Clone, serde::Serialize)]
pub struct UserStringHeap {
    #[serde(skip_serializing)]
    data: Vec<u8>,
}

impl UserStringHeap {
    pub fn get(&self, index: usize) -> Result<Vec<u8>> {
        if index >= self.data.len() {
            return Err(Error::UserStringHeapReadOutOfBound(index, self.data.len()));
        }

        let (data_length, length_size) = crate::utils::read_compressed_usize(
            self.data.get(index..).ok_or_else(|| Error::UserStringHeapReadOutOfBound(index, self.data.len()))?
        )?;

        let end_index = index.checked_add(length_size).and_then(|i| i.checked_add(data_length));
        match end_index {
            Some(end) if end <= self.data.len() => Ok(self.data[index + length_size..end].to_vec()),
            _ => Err(Error::UserStringHeapReadOutOfBound(index, self.data.len())),
        }
    }

    pub fn get_us(&self, index: usize) -> Result<String> {
        let data = self.get(index)?;
        let utf16: Vec<u16> = data.chunks_exact(2)
            .map(|chunk| u16::from_le_bytes([chunk[0], chunk[1]]))
            .collect();
        Ok(String::from_utf16(&utf16)?)
    }
}

impl crate::DnPe {
    pub fn new_user_string_heap(
        &self,
        _metadata_rva: &u32,
        _stream_offset: &u32,
        _stream_size: &usize,
        _stream_name: &str,
        stream_data: Vec<u8>,
    ) -> Result<super::Stream> {
        Ok(super::Stream::UserStringHeap(UserStringHeap {
            data: stream_data,
        }))
    }
}

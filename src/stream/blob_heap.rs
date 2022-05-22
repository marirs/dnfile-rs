use crate::Result;

#[derive(Debug, Clone, serde::Serialize)]
pub struct BlobHeap {
    #[serde(skip_serializing)]
    data: Vec<u8>,
}

impl BlobHeap {
    pub fn get(&self, index: usize) -> Result<Vec<u8>> {
        if index >= self.data.len() {
            return Err(crate::error::Error::BlobHeapReadOutOfBound(
                index,
                self.data.len(),
            ));
        }
        let (data_length, length_size) =
            crate::utils::read_compressed_usize(&self.data[index..index + 4])?;
        if index + length_size + data_length >= self.data.len() + 1 {
            return Err(crate::error::Error::BlobHeapReadOutOfBound(
                index + data_length + length_size,
                self.data.len(),
            ));
        }
        Ok(self.data[index + length_size..index + length_size + data_length].to_vec())
    }
}

impl crate::DnPe {
    pub fn new_blob_heap(
        &self,
        _metadata_rva: &u32,
        _stream_offset: &u32,
        _stream_size: &usize,
        _stream_name: &str,
        stream_data: Vec<u8>,
    ) -> Result<super::Stream> {
        Ok(super::Stream::BlobHeap(BlobHeap { data: stream_data }))
    }
}

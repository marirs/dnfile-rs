use crate::{Result, error::Error};

#[derive(Debug, Clone, serde::Serialize)]
pub struct BlobHeap<'a> {
    #[serde(skip_serializing)]
    data: &'a [u8],
}

impl<'a> BlobHeap<'a> {
    /// Construct from a borrowed slice into the parent file buffer.
    #[must_use]
    pub fn new(data: &'a [u8]) -> Self {
        Self { data }
    }

    /// Returns a copy of the blob at `index` (length-prefixed in #Blob format).
    pub fn get(&self, index: usize) -> Result<Vec<u8>> {
        Ok(self.get_ref(index)?.to_vec())
    }

    /// Returns a borrowed slice into the blob at `index` — zero-copy.
    pub fn get_ref(&self, index: usize) -> Result<&'a [u8]> {
        if index >= self.data.len() {
            return Err(Error::BlobHeapReadOutOfBound(index, self.data.len()));
        }
        let (data_length, length_size) =
            crate::utils::read_compressed_usize(&self.data[index..index + 4])?;
        if index + length_size + data_length > self.data.len() {
            return Err(Error::BlobHeapReadOutOfBound(
                index + data_length + length_size,
                self.data.len(),
            ));
        }
        Ok(&self.data[index + length_size..index + length_size + data_length])
    }
}

impl<'a> crate::DnPe<'a> {
    pub fn new_blob_heap(
        &self,
        _metadata_rva: &u32,
        _stream_offset: &u32,
        _stream_size: &usize,
        _stream_name: &str,
        stream_data: &'a [u8],
    ) -> Result<super::Stream<'a>> {
        Ok(super::Stream::BlobHeap(BlobHeap::new(stream_data)))
    }
}

use crate::{Result, error::Error};

#[derive(Debug, Clone, serde::Serialize)]
pub struct GuidHeap<'a> {
    #[serde(skip_serializing)]
    data: &'a [u8],
}

impl<'a> GuidHeap<'a> {
    #[must_use]
    pub fn new(data: &'a [u8]) -> Self {
        Self { data }
    }

    pub fn get(&self, index: usize) -> Result<uuid::Uuid> {
        let size = 16;
        if index < 1 {
            return Ok(uuid::Uuid::default());
        }
        let offset = (index - 1) * size;
        if offset + size > self.data.len() {
            return Err(Error::GuidHeapReadOutOfBound(index, self.data.len()));
        }
        let guid_buf = &self.data[offset..offset + size];
        Ok(uuid::Uuid::from_slice(guid_buf)?)
    }
}

impl<'a> crate::DnPe<'a> {
    pub fn new_guid_heap(
        &self,
        _metadata_rva: &u32,
        _stream_offset: &u32,
        _stream_size: &usize,
        _stream_name: &str,
        stream_data: &'a [u8],
    ) -> Result<super::Stream<'a>> {
        Ok(super::Stream::GuidHeap(GuidHeap::new(stream_data)))
    }
}

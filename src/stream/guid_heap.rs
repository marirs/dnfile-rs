use crate::Result;

#[derive(Debug, Clone, serde::Serialize)]
pub struct GuidHeap {
    #[serde(skip_serializing)]
    data: Vec<u8>,
}

impl GuidHeap {
    pub fn get(&self, index: usize) -> Result<uuid::Uuid> {
        let size = 16;
        if index < 1 {
            return Ok(uuid::Uuid::default());
        }
        let offset = (index - 1) * size;
        if offset + size > self.data.len() {
            return Err(crate::error::Error::GuidHeapReadOutOfBound(
                index,
                self.data.len(),
            ));
        }
        let guid_buf = &self.data[offset..offset + size];
        Ok(uuid::Uuid::from_slice(guid_buf)?)
    }
}

impl crate::DnPe {
    pub fn new_guid_heap(
        &self,
        _metadata_rva: &u32,
        _stream_offset: &u32,
        _stream_size: &usize,
        _stream_name: &str,
        stream_data: Vec<u8>,
    ) -> Result<super::Stream> {
        Ok(super::Stream::GuidHeap(GuidHeap { data: stream_data }))
    }
}

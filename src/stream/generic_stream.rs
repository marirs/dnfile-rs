use crate::Result;

#[derive(Debug, Clone, serde::Serialize)]
pub struct GenericStream{
    #[serde(skip_serializing)]
    data: Vec<u8>
}

impl crate::DnPe<'_>{
    pub fn new_generic_stream(&self,
                              _metadata_rva: &u32,
                              _stream_offset: &u32,
                              _stream_size: &usize,
                              _stream_name: &str,
                              stream_data: Vec<u8>) -> Result<super::Stream>{
        Ok(super::Stream::GenericStream(GenericStream{
            data: stream_data
        }))
    }
}

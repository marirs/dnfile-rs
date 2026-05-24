use crate::Result;

#[derive(Debug, Default, Clone, serde::Serialize)]
pub struct GenericStream {}

impl crate::DnPe {
    #[allow(clippy::trivially_copy_pass_by_ref)]
    pub fn new_generic_stream(
        &self,
        _metadata_rva: &u32,
        _stream_offset: &u32,
        _stream_size: &usize,
        _stream_name: &str,
        _stream_data: Vec<u8>,
    ) -> Result<super::Stream> {
        Ok(super::Stream::GenericStream(GenericStream::default()))
    }
}

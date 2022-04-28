use crate::Result;

pub mod blob_heap;
pub mod generic_stream;
pub mod guid_heap;
pub mod meta_data_tables;
pub mod string_heap;
pub mod user_string_heap;

#[derive(Debug, Clone, serde::Serialize)]
pub enum Stream {
    GenericStream(generic_stream::GenericStream),
    MetaDataTables(meta_data_tables::MetaDataTable),
    StringHeap(string_heap::StringHeap),
    BlobHeap(blob_heap::BlobHeap),
    GuidHeap(guid_heap::GuidHeap),
    UserStringHeap(user_string_heap::UserStringHeap),
}

#[derive(Debug, Clone, serde::Serialize)]
pub struct ClrStream {
    pub name: String,
    pub rva: u32,
    pub size: usize,
    pub stream_table_entry_size: usize,
    pub stream: Stream,
}

impl ClrStream {
    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn get_string(&self, index: &[u8]) -> Result<String> {
        if let Stream::StringHeap(s) = &self.stream {
            let index = crate::utils::read_usize(index)?;
            s.get(index)
        } else {
            Err(crate::error::Error::TryReadStringFromNotStringHeap)
        }
    }

    pub fn get_blob(&self, index: &[u8]) -> Result<Vec<u8>> {
        if let Stream::BlobHeap(s) = &self.stream {
            let index = crate::utils::read_usize(index)?;
            s.get(index)
        } else {
            Err(crate::error::Error::TryReadStringFromNotStringHeap)
        }
    }

    pub fn get_guid(&self, index: &[u8]) -> Result<uuid::Uuid> {
        if let Stream::GuidHeap(s) = &self.stream {
            let index = crate::utils::read_usize(index)?;
            s.get(index)
        } else {
            Err(crate::error::Error::TryReadGuidFromNotGuidHeap)
        }
    }
}

impl crate::DnPe<'_> {
    pub fn nnew_clr_stream(
        &self,
        metadata_rva: &u32,
        stream_offset: &u32,
        stream_size: &usize,
        stream_name: &str,
        stream_data: Vec<u8>,
    ) -> Result<ClrStream> {
        Ok(ClrStream {
            name: stream_name.to_string(),
            rva: metadata_rva + stream_offset,
            size: stream_data.len(),
            stream_table_entry_size: stream_name.len() + (4 - stream_name.len() % 4) + 8,
            stream: match stream_name {
                "#~" | "#-" => self.new_meta_data_table(
                    metadata_rva,
                    stream_offset,
                    stream_size,
                    stream_name,
                    stream_data,
                )?,
                "#Strings" => self.new_string_heap(
                    metadata_rva,
                    stream_offset,
                    stream_size,
                    stream_name,
                    stream_data,
                )?,
                "#GUID" => self.new_guid_heap(
                    metadata_rva,
                    stream_offset,
                    stream_size,
                    stream_name,
                    stream_data,
                )?,
                "#Blob" => self.new_blob_heap(
                    metadata_rva,
                    stream_offset,
                    stream_size,
                    stream_name,
                    stream_data,
                )?,
                "#US" => self.new_user_string_heap(
                    metadata_rva,
                    stream_offset,
                    stream_size,
                    stream_name,
                    stream_data,
                )?,
                "_" => self.new_generic_stream(
                    metadata_rva,
                    stream_offset,
                    stream_size,
                    stream_name,
                    stream_data,
                )?,
                &_ => return Err(crate::error::Error::UndefinedStream),
            },
        })
    }

    pub fn parse_clr_stream(
        &self,
        stream: &ClrStream,
        stream_map: &std::collections::HashMap<String, ClrStream>,
    ) -> Result<ClrStream> {
        let mut res = stream.clone();
        match &mut res.stream {
            Stream::MetaDataTables(m) => {
                m.tables = self.parse_meta_data_tables(m, stream_map)?;
            }
            Stream::GenericStream(_) => {}
            Stream::StringHeap(_) => {}
            Stream::BlobHeap(_) => {}
            Stream::GuidHeap(_) => {}
            Stream::UserStringHeap(_) => {}
        }
        Ok(res)
    }
}

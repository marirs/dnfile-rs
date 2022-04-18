use crate::Result;

pub mod generic_stream;
pub mod meta_data_tables;
pub mod string_heap;
pub mod blob_heap;
pub mod guid_heap;
pub mod user_string_heap;

#[derive(Debug)]
pub enum Stream{
    GenericStream(generic_stream::GenericStream),
    MetaDataTables(meta_data_tables::MetaDataTable),
    StringHeap(string_heap::StringHeap),
    BlobHeap(blob_heap::BlobHeap),
    GuidHeap(guid_heap::GuidHeap),
    UserStringHeap(user_string_heap::UserStringHeap)
}

#[derive(Debug)]
pub struct ClrStream{
    pub name: String,
    pub rva: u32,
    pub size: usize,
    pub stream_table_entry_size: usize,
    pub stream: Stream
}

impl ClrStream{
    pub fn name(&self) -> &str{
        &self.name
    }
}

impl crate::DnPe<'_>{
    pub fn nnew_clr_stream(&self,
                           metadata_rva: &u32,
                           stream_offset: &u32,
                           stream_size: &usize,
                           stream_name: &str,
                           stream_data: Vec<u8>) -> Result<ClrStream>{
        Ok(ClrStream{
            name: stream_name.to_string(),
            rva: metadata_rva+stream_offset,
            size: stream_data.len(),
            stream_table_entry_size: stream_name.len() + (4 - stream_name.len()%4) + 8,
            stream: match stream_name{
                "#~" | "#-"=> self.new_meta_data_table(metadata_rva, stream_offset, stream_size, stream_name, stream_data)?,
                "#Strings" => self.new_string_heap(metadata_rva, stream_offset, stream_size, stream_name, stream_data)?,
                "#GUID" => self.new_guid_heap(metadata_rva, stream_offset, stream_size, stream_name, stream_data)?,
                "#Blob" => self.new_blob_heap(metadata_rva, stream_offset, stream_size, stream_name, stream_data)?,
                "#US" => self.new_user_string_heap(metadata_rva, stream_offset, stream_size, stream_name, stream_data)?,
                "_" => self.new_generic_stream(metadata_rva, stream_offset, stream_size, stream_name, stream_data)?,
                &_ => return Err(crate::error::Error::UndefinedStream)
            }
        })
    }
}

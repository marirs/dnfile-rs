use crate::Result;

#[derive(Debug)]
pub struct MetaDataTable{
}

impl crate::DnPe<'_>{
    pub fn new_meta_data_table(&self,
                               metadata_rva: &u32,
                               stream_offset: &u32,
                               stream_size: &usize,
                               stream_name: &str,
                               stream_data: Vec<u8>) -> Result<super::Stream>{
        Ok(super::Stream::MetaDataTables(MetaDataTable{
        }))
    }
}

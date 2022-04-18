use crate::Result;

#[derive(Debug, Clone)]
pub struct MetaDataTable{
    data: Vec<u8>
}

impl crate::DnPe<'_>{
    pub fn new_meta_data_table(&self,
                               _metadata_rva: &u32,
                               _stream_offset: &u32,
                               _stream_size: &usize,
                               _stream_name: &str,
                               stream_data: Vec<u8>) -> Result<super::Stream>{



        Ok(super::Stream::MetaDataTables(MetaDataTable{
            data: stream_data
        }))
    }

    pub fn parse_meta_data_tables(&self,
                                  s: &mut MetaDataTable,
                                  stream_map: &std::collections::HashMap<String, super::ClrStream>) -> Result<()>{

        Ok(())
    }
}

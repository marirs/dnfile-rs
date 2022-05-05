use serde::{Deserialize, Serialize};

pub mod error;
pub mod stream;
pub mod utils;
pub mod cil;

use crate::stream::meta_data_tables::mdtables::{*, enums::*};

pub type Result<T> = std::result::Result<T, error::Error>;

#[derive(Debug, Serialize)]
pub struct DnPe{
    name: String,
    #[serde(skip_serializing)]
    data: Vec<u8>,
    net: Option<ClrData>
}

impl DnPe{
    pub fn net(&self) -> Result<&ClrData>{
        match &self.net{
            Some(s) => Ok(s),
            None => Err(crate::error::Error::NotImplementedError)
        }
    }

    pub fn pe<'a>(&'a self) -> Result<goblin::pe::PE<'a>>{
        match goblin::Object::parse(&self.data)?{
            goblin::Object::PE(pe) => Ok(pe),
            _ => return Err(error::Error::UnsupportedBinaryFormat("main"))
        }
    }

    pub fn new(name: &str) -> Result<DnPe>{
        let mut res = DnPe{
            name: name.to_string(),
            data: std::fs::read(name)?,
            net: None
        };
        let opt_header = match res.pe()?.header.optional_header{
            Some(oh) => oh,
            None => return Err(error::Error::UnsupportedBinaryFormat("optional header absence"))
        };
        let clr_directory = match opt_header.data_directories.get_clr_runtime_header(){
            Some(oh) => oh,
            None => return Err(error::Error::UnsupportedBinaryFormat("ClR runtime header absence"))
        };
        let clr_struct: ClrStruct = res.get_data(&clr_directory.virtual_address, &(clr_directory.size as usize))?;
        res.net = Some(res.new_clrdata(clr_struct)?);
        Ok(res)
    }

    fn offset(&self, rva: u32) -> Result<usize>{
        match goblin::pe::utils::find_offset(rva as usize, &self.pe()?.sections, self.pe()?.header.optional_header.unwrap().windows_fields.file_alignment, &goblin::pe::options::ParseOptions{resolve_rva: true}){
            Some(s) => Ok(s),
            None => return Err(crate::error::Error::UnresolvedRvaError(rva))
        }
    }

    fn get_data<'a, T>(&'a self, rva: &'a u32, size: &'a usize) -> Result<T>
    where T: scroll::ctx::TryFromCtx<'a, goblin::container::Endian, Error = scroll::Error>{
        Ok(goblin::pe::utils::get_data(&self.data, &self.pe()?.sections, goblin::pe::data_directories::DataDirectory{
            virtual_address: *rva,
            size: *size as u32
        }, self.pe()?.header.optional_header.unwrap().windows_fields.file_alignment)?)
    }

    fn get_nullterminated_string(&self, rva: &u32) -> Result<String>{
        let mut res_buf = vec![];
        let mut rrva = *rva;
        let mut c = self.data[self.offset(rrva)?];
        while c != 0{
            res_buf.push(c);
            rrva += 1;
            c = self.data[self.offset(rrva)?];
        }
        Ok(String::from_utf8(res_buf)?)
    }

    fn get_vec(&self, rva: &u32, size: &usize) -> Result<Vec<u8>>{
        let offset = self.offset(*rva)?;
        Ok(self.data[offset .. offset+size].to_vec())
    }

    fn get_dword_at_rva(&self, rva: &u32) -> Result<u32>{
        self.get_data(rva, &4)
    }

    fn new_clrdata(&self, clr_struct: ClrStruct) -> Result<ClrData>{
        let metadata_struct: MetaDataStruct = self.get_data(&clr_struct.meta_data_rva, &(clr_struct.meta_data_size as usize))?;
        let metadata = self.new_metadata(&clr_struct.meta_data_rva, metadata_struct)?;
        let flags = ClrHeaderFlags::new(clr_struct.flags as usize);
        let functions = self.parse_functions(&metadata)?;
        Ok(ClrData{
            //clr_struct,
            metadata,
            flags,
            functions
        })
    }

    fn parse_functions(&self, metadata: &MetaData) -> Result<Vec<cil::cil::function::Function>>{
        let mut res = vec![];
        let method_def_table = metadata.md_table("MethodDef")?;
        for i in 0..method_def_table.row_count(){
            let row = method_def_table.row::<MethodDef>(i)?;
            if !row.impl_flags.contains(&ClrMethodImpl::MethodCodeType(CorMethodCodeType::IL))
                || row.flags.contains(&ClrMethodAttr::AttrFlag(CorMethodAttrFlag::Abstract))
                || row.flags.contains(&ClrMethodAttr::AttrFlag(CorMethodAttrFlag::PinvokeImpl)){
                    continue;
                }
            res.push(self.parse_function(row)?);
        }
        Ok(res)
    }
    fn parse_function(&self, row: &MethodDef) -> Result<cil::cil::function::Function>{
        let mut reader =  cil::cil::function::reader::Reader::new(&self.data);
        reader.seek(self.offset(row.rva)?)?;
        cil::cil::function::Function::new(&mut reader)
    }

    fn new_metadata(&self, metadata_rva: &u32, metadata_struct: MetaDataStruct) -> Result<MetaData>{
        let version_offset =  self.offset(metadata_rva+16)?;
        let version = self.data[version_offset .. version_offset + metadata_struct.version_length.clone() as usize].to_vec();
        let flags: u16 = self.get_data(&(metadata_rva+16+metadata_struct.version_length.clone()), &2)?;
        let number_of_streams: u16 = self.get_data(&(metadata_rva+16+metadata_struct.version_length.clone()+2), &2)?;
        let struct_size = 16+metadata_struct.version_length.clone()+2+2;
        let mut streams = std::collections::HashMap::new();
        if number_of_streams > 0{
            let streams_table_rva = metadata_rva+struct_size;
            streams = self.new_streams(metadata_rva, &streams_table_rva, &(number_of_streams as usize))?;
        }
        Ok(MetaData{
            _version: String::from_utf8(version)?,
            flags,
            streams
        })
    }

    fn new_streams(&self, metadata_rva: &u32, streams_table_rva: &u32, number_of_streams: &usize) -> Result<std::collections::HashMap<String, stream::ClrStream>>{
        let mut res = std::collections::HashMap::new();
        let mut stream_entry_rva = *streams_table_rva;
        for _i in 0..*number_of_streams{
            let stream = self.new_clr_stream(&stream_entry_rva, metadata_rva)?;
            stream_entry_rva+= &(stream.stream_table_entry_size as u32);
            res.insert(stream.name().to_string(), stream);
        }
        let mut rres = std::collections::HashMap::new();
        for (n, s) in &res{
            rres.insert(n.to_string(), self.parse_clr_stream(s, &res)?);
        }
        Ok(rres)
    }

    fn new_clr_stream(&self, stream_table_entry_rva: &u32, metadata_rva: &u32) -> Result<stream::ClrStream>{
        let stream_offset: u32 = self.get_data(stream_table_entry_rva, &4)?;
        let stream_size: u32 = self.get_data(&(stream_table_entry_rva+4), &4)?;
        let stream_name = self.get_nullterminated_string(&(stream_table_entry_rva+8))?;
        let stream_data = self.get_vec(&(metadata_rva+stream_offset), &(stream_size as usize))?;
        self.nnew_clr_stream(metadata_rva, &stream_offset, &(stream_size as usize), &stream_name, stream_data)
    }
}

#[repr(C)]
#[derive(Deserialize, Debug, scroll::Pread)]
pub struct ClrStruct{
    cb: u32,
    majorr_runtime_version: u16,
    minor_runtime_version: u16,
    meta_data_rva: u32,
    meta_data_size: u32,
    flags: u32,
    entry_point_token_or_rva: u32,
    resources_rva: u32,
    resources_size: u32,
    strong_name_signature_rva: u32,
    strong_name_signature_size: u32,
    code_manager_table_rva: u32,
    code_manager_table_size: u32,
    v_table_fixups_rva: u32,
    v_table_fixups_size: u32,
    export_address_table_jumps_rva: u32,
    export_address_table_jumps_size: u32,
    managed_native_header_rva: u32,
    managed_native_header_size: u32,
}

#[derive(Debug, Serialize, PartialOrd, Ord, PartialEq, Eq)]
pub enum ClrHeaderFlags{
    IlOnly,
    BitRequired32,
    IlLibrary,
    StrongNamesSigned,
    NativeEntryPiont,
    TrackDebugData,
    Prefer32Bit
}

impl ClrHeaderFlags{
    pub fn new(value: usize) -> std::collections::BTreeSet<Self>{
        let mut res = std::collections::BTreeSet::new();
        if value & 1 != 0 {
            res.insert(Self::IlOnly);
        }
        if value & 2 != 0 {
            res.insert(Self::BitRequired32);
        }
        if value & 4 != 0 {
            res.insert(Self::IlLibrary);
        }
        if value & 8 != 0 {
            res.insert(Self::StrongNamesSigned);
        }
        if value & 0x10 != 0 {
            res.insert(Self::NativeEntryPiont);
        }
        if value & 0x10000 != 0 {
            res.insert(Self::TrackDebugData);
        }
        if value & 0x20000 != 0 {
            res.insert(Self::Prefer32Bit);
        }
        res
    }
}


#[derive(Debug, Serialize)]
pub struct ClrData{
    #[serde(skip_serializing)]
//    clr_struct: ClrStruct,
    pub metadata: MetaData,
//    strings: Option<StringsHeap>,
//    user_strings: Option<UserStringHeap>,
//    guids: Option<GuidHeap>,
 //   blobs: Option<BlobHeap>,
//    mdtables: Option<MetaDataTables>,
    pub flags: std::collections::BTreeSet<ClrHeaderFlags>,
    pub functions: Vec<cil::cil::function::Function>
}

impl ClrData{
    pub fn md_table(&self, name: &'static str) -> Result<&stream::meta_data_tables::mdtables::MetaDataTable>{
        self.metadata.md_table(name)
    }

    pub fn resolve_coded_index<T>(&self, index: &dyn stream::meta_data_tables::mdtables::codedindex::CodedIndex) -> Result<&T>
    where T: stream::meta_data_tables::mdtables::MDTableRowTrait + 'static{
        let table = self.md_table(index.table())?;
        Ok(table.row(index.row_index())?)
    }

    pub fn functions(&self) -> &Vec<cil::cil::function::Function>{
        &self.functions
    }

    pub fn get_us(&self, rid: usize) -> Result<String>{
        self.metadata.get_us(rid)
    }
}

#[repr(C)]
#[derive(Deserialize, Debug, Clone, scroll::Pread)]
pub struct MetaDataStruct{
    signature: u32,
    major_version: u16,
    minor_version: u16,
    reserved: u32,
    version_length: u32,
//    version: u32,
//    flags: u32,
//    number_of_streams: u32
}

#[derive(Debug, Serialize)]
pub struct MetaData{
    #[serde(skip_serializing)]
    _version: String,
    flags: u16,
    pub streams: std::collections::HashMap<String, stream::ClrStream>
}

impl MetaData{
    pub fn md_table(&self, name: &'static str) -> Result<&stream::meta_data_tables::mdtables::MetaDataTable>{
        for (_, s) in &self.streams{
            if let stream::Stream::MetaDataTables(mt) = &s.stream{
                match mt.tables.get(&stream::meta_data_tables::mdtables::table_name_2_index(name)?){
                    Some(s) => return Ok(s),
                    None => return Err(crate::error::Error::UndefinedMetaDataTableName(name))
                }
            }
        }
        Err(crate::error::Error::UndefinedMetaDataTableName(name))
    }

    pub fn get_us(&self, rid: usize) -> Result<String>{
       for (_, s) in &self.streams{
           if let stream::Stream::UserStringHeap(us) = &s.stream{
               return us.get_us(rid);
           }
        }
        Err(crate::error::Error::UndefinedMetaDataTableName("US"))
    }
}

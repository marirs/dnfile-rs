use crate::Result;

pub mod codedindex;
pub mod enums;

#[derive(Debug, Clone)]
pub enum MDTable{
    Module(Vec<Module>),
    TypeRef(Vec<TypeRef>),
    TypeDef(Vec<TypeDef>),
    FieldPtr(Vec<FieldPtr>),
    Field(Vec<Field>),
    MethodPtr(Vec<MethodPtr>),
    MethodDef(Vec<MethodDef>),
    ParamPtr(Vec<ParamPtr>),
    Param(Vec<Param>),
    InterfaceImpl(Vec<InterfaceImpl>),
    MemberRef(Vec<MemberRef>),
    Constant(Vec<Constant>),
    CustomAttribute(Vec<CustomAttribute>),
    FieldMarshal(Vec<FieldMarshal>),
    DeclSecurity(Vec<DeclSecurity>),
    ClassLayout(Vec<ClassLayout>),
    FieldLayout(Vec<FieldLayout>),
    StandAloneSig(Vec<StandAloneSig>),
    EventMap(Vec<EventMap>),
    EventPtr(Vec<EventPtr>),
    Event(Vec<Event>),
    PropertyMap(Vec<PropertyMap>),
    PropertyPtr(Vec<PropertyPtr>),
    Property(Vec<Property>),
    MethodSemantics(Vec<MethodSemantics>),
    MethodImpl(Vec<MethodImpl>),
    ModuleRef(Vec<ModuleRef>),
    TypeSpec(Vec<TypeSpec>),
    ImplMap(Vec<ImplMap>),
    FieldRva(Vec<FieldRva>),
    EncLog(Vec<EncLog>),
    EncMap(Vec<EncMap>),
    Assembly(Vec<Assembly>),
    AssemblyProcessor(Vec<AssemblyProcessor>),
    AssemblyOS(Vec<AssemblyOS>),
    AssemblyRef(Vec<AssemblyRef>),
    AssemblyRefProcessor(Vec<AssemblyRefProcessor>),
    AssemblyRefOS(Vec<AssemblyRefOS>),
    File(Vec<File>),
    ExportedType(Vec<ExportedType>),
    ManifestResource(Vec<ManifestResource>),
    NestedClass(Vec<NestedClass>),
    GenericParam(Vec<GenericParam>),
    GenericMethod(Vec<GenericMethod>),
    GenericParamConstraint(Vec<GenericParamConstraint>),
    Unused(Vec<Unused>),
    MaxTable(Vec<MaxTable>)
}

impl MDTable{
    pub fn row_size(&self,
                    strings_offset_size: usize,
                    guids_offset_size: usize,
                    blobs_offset_size: usize) -> usize{
        match self{
            MDTable::Module(_) => 0,
            MDTable::TypeRef(_) => 0,
            MDTable::TypeDef(_) => 0,
            MDTable::FieldPtr(_) => 0,
            MDTable::Field(_) => 0,
            MDTable::MethodPtr(_) => 0,
            MDTable::MethodDef(_) => 0,
            MDTable::ParamPtr(_) => 0,
            MDTable::Param(_) => 0,
            MDTable::InterfaceImpl(_) => 0,
            MDTable::MemberRef(_) => 0,
            MDTable::Constant(_) => 0,
            MDTable::CustomAttribute(_) => 0,
            MDTable::FieldMarshal(_) => 0,
            MDTable::DeclSecurity(_) => 0,
            MDTable::ClassLayout(_) => 0,
            MDTable::FieldLayout(_) => 0,
            MDTable::StandAloneSig(_) => 0,
            MDTable::EventMap(_) => 0,
            MDTable::EventPtr(_) => 0,
            MDTable::Event(_) => 0,
            MDTable::PropertyMap(_) => 0,
            MDTable::PropertyPtr(_) => 0,
            MDTable::Property(_) => 0,
            MDTable::MethodSemantics(_) => 0,
            MDTable::MethodImpl(_) => 0,
            MDTable::ModuleRef(_) => 0,
            MDTable::TypeSpec(_) => 0,
            MDTable::ImplMap(_) => 0,
            MDTable::FieldRva(_) => 0,
            MDTable::EncLog(_) => 0,
            MDTable::EncMap(_) => 0,
            MDTable::Assembly(_) => 0,
            MDTable::AssemblyProcessor(_) => 0,
            MDTable::AssemblyOS(_) => 0,
            MDTable::AssemblyRef(_) => 0,
            MDTable::AssemblyRefProcessor(_) => 0,
            MDTable::AssemblyRefOS(_) => 0,
            MDTable::File(_) => 0,
            MDTable::ExportedType(_) => 0,
            MDTable::ManifestResource(_) => 0,
            MDTable::NestedClass(_) => 0,
            MDTable::GenericParam(_) => 0,
            MDTable::GenericMethod(_) => 0,
            MDTable::GenericParamConstraint(_) => 0,
            MDTable::Unused(_) => 0,
            MDTable::MaxTable(_) => 0
        }
    }
}


#[derive(Debug, Clone, Default)]
pub struct Module{
    generation: u16,
    name: String,
    mvid: uuid::Uuid,
    enc_id: uuid::Uuid,
    enc_base_id: uuid::Uuid
}

#[derive(Debug, Clone, Default)]
pub struct TypeRef{
    resolution_scope: codedindex::ResolutionScope,
    type_name: String,
    type_namespace: String
}

#[derive(Debug, Clone, Default)]
pub struct TypeDef{
    flags: Option<enums::ClrTypeAttr>,
    type_name: String,
    type_namespace: String,
    extends: codedindex::TypeDefOrRef,
    field_list: Vec<Field>,
    method_list: Vec<MethodDef>
}

#[derive(Debug, Clone, Default)]
pub struct FieldPtr{
    field: Field
}

#[derive(Debug, Clone, Default)]
pub struct Field{
    flags: Option<enums::ClrFieldAttr>,
    name: String,
    Signature: Vec<u8>
}

#[derive(Debug, Clone, Default)]
pub struct MethodPtr{
    field: MethodDef
}

#[derive(Debug, Clone, Default)]
pub struct MethodDef{
    rva: u32,
    impl_flags: Option<enums::ClrMethodImpl>,
    flags: Option<enums::ClrMethodAttr>,
    name: String,
    signature: Vec<u8>,
    param_list: Vec<Param>
}

#[derive(Debug, Clone, Default)]
pub struct ParamPtr{
    field: Param
}

#[derive(Debug, Clone, Default)]
pub struct Param{
    flags: Option<enums::ClrParamAttr>,
    sequence: u32,
    name: String
}

#[derive(Debug, Clone, Default)]
pub struct InterfaceImpl{
    class: TypeDef,
    interface: codedindex::TypeDefOrRef
}

#[derive(Debug, Clone, Default)]
pub struct MemberRef{
    class: codedindex::MemberRefParent,
    name: String,
    signature: Vec<u8>
}

#[derive(Debug, Clone, Default)]
pub struct Constant{
    _type: u32,
    padding: u32,
    parent: codedindex::HasConstant,
    value: Vec<u8>
}

#[derive(Debug, Clone, Default)]
pub struct CustomAttribute{
    parent: codedindex::HasCustomAttribute,
    _type: codedindex::CustomAttributeType,
    value: Vec<u8>
}

#[derive(Debug, Clone, Default)]
pub struct FieldMarshal{
    parent: codedindex::HasFieldMarshall,
    native_type: Vec<u8>
}

#[derive(Debug, Clone, Default)]
pub struct DeclSecurity{
    action: u32,
    parent: codedindex::HasDeclSecurity,
    permission_set: Vec<u8>
}

#[derive(Debug, Clone, Default)]
pub struct ClassLayout{
    packing_size: usize,
    class_size: usize,
    parent: TypeDef
}

#[derive(Debug, Clone, Default)]
pub struct FieldLayout{
    offset: u32,
    field: Field
}

#[derive(Debug, Clone, Default)]
pub struct StandAloneSig{
    signature: Vec<u8>
}

#[derive(Debug, Clone, Default)]
pub struct EventMap{
    parent: TypeDef,
    event_list: Vec<Event>
}

#[derive(Debug, Clone, Default)]
pub struct EventPtr{
}

#[derive(Debug, Clone, Default)]
pub struct Event{
    event_flags: Option<enums::ClrEventAttr>,
    name: String,
    event_type: codedindex::TypeDefOrRef
}

#[derive(Debug, Clone, Default)]
pub struct PropertyMap{
    parent: TypeDef,
    property_list: Vec<Property>
}

#[derive(Debug, Clone, Default)]
pub struct PropertyPtr{
}

#[derive(Debug, Clone, Default)]
pub struct Property{
    flags: Option<enums::ClrPropertyAttr>,
    name: String,
    _type: Vec<u8>
}

#[derive(Debug, Clone, Default)]
pub struct MethodSemantics{
    semantics: Option<enums::ClrMethodSemanticsAttr>,
    method: MethodDef,
    association: codedindex::HasSemantics
}

#[derive(Debug, Clone, Default)]
pub struct MethodImpl{
    class: TypeDef,
    method_body: codedindex::MethodDefOrRef,
    method_declaration: codedindex::MethodDefOrRef
}

#[derive(Debug, Clone, Default)]
pub struct ModuleRef{
    name: String
}

#[derive(Debug, Clone, Default)]
pub struct TypeSpec{
    signature: Vec<u8>
}

#[derive(Debug, Clone, Default)]
pub struct ImplMap{
    mapping_flags: Option<enums::ClrPinvokeMap>,
    member_forwarded: codedindex::MemberForwarded,
    import_name: String,
    import_scope: ModuleRef
}

#[derive(Debug, Clone, Default)]
pub struct FieldRva{
    rva: u32,
    field: Field
}

#[derive(Debug, Clone, Default)]
pub struct EncLog{
    token: u32,
    func_code: u32
}

#[derive(Debug, Clone, Default)]
pub struct EncMap{
    token: u32,
}

#[derive(Debug, Clone, Default)]
pub struct Assembly{
    hash_alg_id: Option<enums::AssemblyHashAlgorithm>,
    major_version: u32,
    minor_version: u32,
    build_number: u32,
    revision_number: u32,
    flags: Option<enums::ClrAssemblyFlags>,
    public_key: Vec<u8>,
    name: String,
    culture: String
}

#[derive(Debug, Clone, Default)]
pub struct AssemblyProcessor{
    processor: u32,
}

#[derive(Debug, Clone, Default)]
pub struct AssemblyOS{
    os_platform_id: u32,
    os_major_version: u32,
    os_minor_version: u32
}

#[derive(Debug, Clone, Default)]
pub struct AssemblyRef{
    major_version: u32,
    minor_version: u32,
    build_number: u32,
    revision_number: u32,
    flags: Option<enums::ClrAssemblyFlags>,
    public_key: Vec<u8>,
    name: String,
    culture: String,
    hash_value: Vec<u8>
}

#[derive(Debug, Clone, Default)]
pub struct AssemblyRefProcessor{
    processor: u32,
    assembly_ref: AssemblyRef
}

#[derive(Debug, Clone, Default)]
pub struct AssemblyRefOS{
    os_platform_id: u32,
    os_major_version: u32,
    os_minor_version: u32,
    assembly_ref: AssemblyRef
}

#[derive(Debug, Clone, Default)]
pub struct File{
    flags: Option<enums::ClrFileFlags>,
    name: String,
    hash_value: Vec<u8>
}

#[derive(Debug, Clone, Default)]
pub struct ExportedType{
    flags: Option<enums::ClrTypeAttr>,
    type_def_id: u32,
    type_name: String,
    type_namespace: String,
    implementation: codedindex::Implementation
}

#[derive(Debug, Clone, Default)]
pub struct ManifestResource{
    offset: u32,
    flags: Option<enums::ClrManifestResourceFlags>,
    name: String,
    implementation: codedindex::Implementation
}

#[derive(Debug, Clone, Default)]
pub struct NestedClass{
    nested_class: TypeDef,
    enclosing_class: TypeDef
}

#[derive(Debug, Clone, Default)]
pub struct GenericParam{
    number: u32,
    flags: Option<enums::ClrGenericParamAttr>,
    owner: codedindex::TypeOrMethodDef,
    name: String
}

#[derive(Debug, Clone, Default)]
pub struct GenericMethod{
    unknown1: codedindex::MethodDefOrRef,
    unknown2: Vec<u8>
}

#[derive(Debug, Clone, Default)]
pub struct GenericParamConstraint{
    owner: GenericParam,
    constraint: codedindex::TypeDefOrRef
}

#[derive(Debug, Clone, Default)]
pub struct Unused{
}

#[derive(Debug, Clone, Default)]
pub struct MaxTable{
}


#[derive(Debug, Clone)]
pub struct MetaDataTable{
    number: usize,
    is_sorted: bool,
    pub row_size: usize,
    pub num_rows: usize,
    pub rva: u32,
    table: MDTable
}

impl crate::DnPe<'_>{
    pub fn create_md_table(&self,
                           i: &usize,
                           table_rowcounts: &Vec<usize>,
                           is_sorted: bool,
                           strings_offset_size: usize,
                           guids_offset_size: usize,
                           blobs_offset_size: usize) -> Result<MetaDataTable>{
        let num_rows = table_rowcounts[*i as usize];
        let table = self.new_mdtable(*i, &num_rows)?;
        let mut table = MetaDataTable{
            number: *i,
            is_sorted,
            row_size: table.row_size(strings_offset_size, guids_offset_size, blobs_offset_size),
            num_rows,
            rva: 0,
            table
        };
        Ok(table)
    }

    pub fn new_table<T>(&self, num_rows: &usize) -> Result<Vec<T>>
    where T: Default + Clone{
        Ok(vec![T::default(); *num_rows])
    }

    pub fn new_mdtable(&self, i: usize,
                       num_rows: &usize) -> Result<MDTable>{
        match i{
            0 => Ok(MDTable::Module(self.new_table::<Module>(num_rows)?)),
            1 => Ok(MDTable::TypeRef(self.new_table::<TypeRef>(num_rows)?)),
            2 => Ok(MDTable::TypeDef(self.new_table::<TypeDef>(num_rows)?)),
            3 => Ok(MDTable::FieldPtr(self.new_table::<FieldPtr>(num_rows)?)),
            4 => Ok(MDTable::Field(self.new_table::<Field>(num_rows)?)),
            5 => Ok(MDTable::MethodPtr(self.new_table::<MethodPtr>(num_rows)?)),
            6 => Ok(MDTable::MethodDef(self.new_table::<MethodDef>(num_rows)?)),
            7 => Ok(MDTable::ParamPtr(self.new_table::<ParamPtr>(num_rows)?)),
            8 => Ok(MDTable::Param(self.new_table::<Param>(num_rows)?)),
            9 => Ok(MDTable::InterfaceImpl(self.new_table::<InterfaceImpl>(num_rows)?)),
            10 => Ok(MDTable::MemberRef(self.new_table::<MemberRef>(num_rows)?)),
            11 => Ok(MDTable::Constant(self.new_table::<Constant>(num_rows)?)),
            12 => Ok(MDTable::CustomAttribute(self.new_table::<CustomAttribute>(num_rows)?)),
            13 => Ok(MDTable::FieldMarshal(self.new_table::<FieldMarshal>(num_rows)?)),
            14 => Ok(MDTable::DeclSecurity(self.new_table::<DeclSecurity>(num_rows)?)),
            15 => Ok(MDTable::ClassLayout(self.new_table::<ClassLayout>(num_rows)?)),
            16 => Ok(MDTable::FieldLayout(self.new_table::<FieldLayout>(num_rows)?)),
            17 => Ok(MDTable::StandAloneSig(self.new_table::<StandAloneSig>(num_rows)?)),
            18 => Ok(MDTable::EventMap(self.new_table::<EventMap>(num_rows)?)),
            19 => Ok(MDTable::EventPtr(self.new_table::<EventPtr>(num_rows)?)),
            20 => Ok(MDTable::Event(self.new_table::<Event>(num_rows)?)),
            21 => Ok(MDTable::PropertyMap(self.new_table::<PropertyMap>(num_rows)?)),
            22 => Ok(MDTable::PropertyPtr(self.new_table::<PropertyPtr>(num_rows)?)),
            23 => Ok(MDTable::Property(self.new_table::<Property>(num_rows)?)),
            24 => Ok(MDTable::MethodSemantics(self.new_table::<MethodSemantics>(num_rows)?)),
            25 => Ok(MDTable::MethodImpl(self.new_table::<MethodImpl>(num_rows)?)),
            26 => Ok(MDTable::ModuleRef(self.new_table::<ModuleRef>(num_rows)?)),
            27 => Ok(MDTable::TypeSpec(self.new_table::<TypeSpec>(num_rows)?)),
            28 => Ok(MDTable::ImplMap(self.new_table::<ImplMap>(num_rows)?)),
            29 => Ok(MDTable::FieldRva(self.new_table::<FieldRva>(num_rows)?)),
            30 => Ok(MDTable::EncLog(self.new_table::<EncLog>(num_rows)?)),
            31 => Ok(MDTable::EncMap(self.new_table::<EncMap>(num_rows)?)),
            32 => Ok(MDTable::Assembly(self.new_table::<Assembly>(num_rows)?)),
            33 => Ok(MDTable::AssemblyProcessor(self.new_table::<AssemblyProcessor>(num_rows)?)),
            34 => Ok(MDTable::AssemblyOS(self.new_table::<AssemblyOS>(num_rows)?)),
            35 => Ok(MDTable::AssemblyRef(self.new_table::<AssemblyRef>(num_rows)?)),
            36 => Ok(MDTable::AssemblyRefProcessor(self.new_table::<AssemblyRefProcessor>(num_rows)?)),
            37 => Ok(MDTable::AssemblyRefOS(self.new_table::<AssemblyRefOS>(num_rows)?)),
            38 => Ok(MDTable::File(self.new_table::<File>(num_rows)?)),
            39 => Ok(MDTable::ExportedType(self.new_table::<ExportedType>(num_rows)?)),
            40 => Ok(MDTable::ManifestResource(self.new_table::<ManifestResource>(num_rows)?)),
            41 => Ok(MDTable::NestedClass(self.new_table::<NestedClass>(num_rows)?)),
            42 => Ok(MDTable::GenericParam(self.new_table::<GenericParam>(num_rows)?)),
            43 => Ok(MDTable::GenericMethod(self.new_table::<GenericMethod>(num_rows)?)),
            44 => Ok(MDTable::GenericParamConstraint(self.new_table::<GenericParamConstraint>(num_rows)?)),
            // 45 through 63 are not used
            62 => Ok(MDTable::Unused(self.new_table::<Unused>(num_rows)?)),
            63 => Ok(MDTable::MaxTable(self.new_table::<MaxTable>(num_rows)?)),
            _ => Err(crate::error::Error::UndefinedMetaDataTableIndex(i as u32))
        }
    }

    pub fn parse_rows(&self, table: &MetaDataTable, rva: &u32, table_data: Vec<u8>) -> Result<MetaDataTable>{
        Ok(table.clone())
    }

    pub fn parse_table(&self, table: &MetaDataTable, ttables: &std::collections::HashMap<usize, MetaDataTable>) -> Result<MetaDataTable>{
        Ok(table.clone())
    }
}

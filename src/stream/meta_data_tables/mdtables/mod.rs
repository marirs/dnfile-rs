use crate::Result;

pub mod codedindex;
pub mod enums;

#[derive(Debug, Clone)]
pub enum MDTable{
    Module(Module),
    TypeRef(TypeRef),
    TypeDef(TypeDef),
    FieldPtr(FieldPtr),
    Field(Field),
    MethodPtr(MethodPtr),
    MethodDef(MethodDef),
    ParamPtr(ParamPtr),
    Param(Param),
    InterfaceImpl(InterfaceImpl),
    MemberRef(MemberRef),
    Constant(Constant),
    CustomAttribute(CustomAttribute),
    FieldMarshal(FieldMarshal),
    DeclSecurity(DeclSecurity),
    ClassLayout(ClassLayout),
    FieldLayout(FieldLayout),
    StandAloneSig(StandAloneSig),
    EventMap(EventMap),
    EventPtr(EventPtr),
    Event(Event),
    PropertyMap(PropertyMap),
    PropertyPtr(PropertyPtr),
    Property(Property),
    MethodSemantics(MethodSemantics),
    MethodImpl(MethodImpl),
    ModuleRef(ModuleRef),
    TypeSpec(TypeSpec),
    ImplMap(ImplMap),
    FieldRva(FieldRva),
    EncLog(EncLog),
    EncMap(EncMap),
    Assembly(Assembly),
    AssemblyProcessor(AssemblyProcessor),
    AssemblyOS(AssemblyOS),
    AssemblyRef(AssemblyRef),
    AssemblyRefProcessor(AssemblyRefProcessor),
    AssemblyRefOS(AssemblyRefOS),
    File(File),
    ExportedType(ExportedType),
    ManifestResource(ManifestResource),
    NestedClass(NestedClass),
    GenericParam(GenericParam),
    GenericMethod(GenericMethod),
    GenericParamConstraint(GenericParamConstraint),
    Unused(Unused),
    MaxTable(MaxTable)
}

#[derive(Debug, Clone, Default)]
pub struct ModuleRow{
    generation: u16,
    name: String,
    mvid: uuid::Uuid,
    enc_id: uuid::Uuid,
    enc_base_id: uuid::Uuid
}

#[derive(Debug, Clone)]
pub struct Module{
    rows: Vec<ModuleRow>,
}

#[derive(Debug, Clone, Default)]
pub struct TypeRefRow{
    resolution_scope: codedindex::ResolutionScope,
    type_name: String,
    type_namespace: String
}

#[derive(Debug, Clone)]
pub struct TypeRef{
    rows: Vec<TypeRefRow>,
}

#[derive(Debug, Clone, Default)]
pub struct TypeDefRow{
    flags: enums::ClrTypeAttr,
    type_name: String,
    type_namespace: String,
    extends: codedindex::TypeDefOrRef,
    field_list: Vec<FieldRow>,
    method_list: Vec<MethodDefRow>
}

#[derive(Debug, Clone)]
pub struct TypeDef{
    rows: Vec<TypeDefRow>,
}

#[derive(Debug, Clone, Default)]
pub struct FieldPtrRow{
    field: FieldRow
}

#[derive(Debug, Clone)]
pub struct FieldPtr{
    rows: Vec<FieldPtrRow>,
}

#[derive(Debug, Clone, Default)]
pub struct FieldRow{
    flags: enums::ClrFieldAttr,
    name: String,
    Signature: Vec<u8>
}

#[derive(Debug, Clone)]
pub struct Field{
    rows: Vec<FieldRow>,
}

#[derive(Debug, Clone, Default)]
pub struct MethodPtrRow{
    field: MethodDefRow
}

#[derive(Debug, Clone)]
pub struct MethodPtr{
    rows: Vec<MethodPtrRow>,
}

#[derive(Debug, Clone, Default)]
pub struct MethodDefRow{
    rva: u32,
    impl_flags: enums::ClrMethodImpl,
    flags: enums::ClrMethodAttr,
    name: String,
    signature: Vec<u8>,
    param_list: Vec<ParamRow>
}

#[derive(Debug, Clone)]
pub struct MethodDef{
    rows: Vec<MethodDefRow>,
}

#[derive(Debug, Clone, Default)]
pub struct ParamPtrRow{
    field: ParamRow
}

#[derive(Debug, Clone)]
pub struct ParamPtr{
    rows: Vec<ParamPtrRow>,
}

#[derive(Debug, Clone, Default)]
pub struct ParamRow{
    flags: enums::ClrParamAttr,
    sequence: u32,
    name: String
}

#[derive(Debug, Clone)]
pub struct Param{
    rows: Vec<ParamRow>,
}

#[derive(Debug, Clone, Default)]
pub struct InterfaceImplRow{
    class: TypeDefRow,
    interface: codedindex::TypeDefOrRef
}

#[derive(Debug, Clone)]
pub struct InterfaceImpl{
    rows: Vec<InterfaceImplRow>,
}

#[derive(Debug, Clone, Default)]
pub struct MemberRefRow{
    class: codedindex::MemberRefParent,
    name: String,
    signature: Vec<u8>
}

#[derive(Debug, Clone)]
pub struct MemberRef{
    rows: Vec<MemberRefRow>,
}

#[derive(Debug, Clone, Default)]
pub struct ConstantRow{
    _type: u32,
    padding: u32,
    parent: codedindex::HasConstant,
    value: Vec<u8>
}

#[derive(Debug, Clone)]
pub struct Constant{
    rows: Vec<ConstantRow>,
}

#[derive(Debug, Clone, Default)]
pub struct CustomAttributeRow{
    parent: codedindex::HasCustomAttribute,
    _type: codedindex::CustomAttributeType,
    value: Vec<u8>
}

#[derive(Debug, Clone)]
pub struct CustomAttribute{
    rows: Vec<CustomAttributeRow>,
}

#[derive(Debug, Clone, Default)]
pub struct FieldMarshalRow{
    parent: codedindex::HasFieldMarshall,
    native_type: Vec<u8>
}

#[derive(Debug, Clone)]
pub struct FieldMarshal{
    rows: Vec<FieldMarshalRow>,
}

#[derive(Debug, Clone, Default)]
pub struct DeclSecurityRow{
    action: u32,
    parent: codedindex::HasDeclSecurity,
    permission_set: Vec<u8>
}

#[derive(Debug, Clone)]
pub struct DeclSecurity{
    rows: Vec<DeclSecurityRow>,
}

#[derive(Debug, Clone, Default)]
pub struct ClassLayoutRow{
    packing_size: usize,
    class_size: usize,
    parent: TypeDefRow
}

#[derive(Debug, Clone)]
pub struct ClassLayout{
    rows: Vec<ClassLayoutRow>,
}

#[derive(Debug, Clone, Default)]
pub struct FieldLayoutRow{
    offset: u32,
    field: FieldRow
}

#[derive(Debug, Clone)]
pub struct FieldLayout{
    rows: Vec<FieldLayoutRow>,
}

#[derive(Debug, Clone, Default)]
pub struct StandAloneSigRow{
    signature: Vec<u8>
}

#[derive(Debug, Clone)]
pub struct StandAloneSig{
    rows: Vec<StandAloneSigRow>,
}

#[derive(Debug, Clone, Default)]
pub struct EventMapRow{
    parent: TypeDefRow,
    event_list: Vec<EventRow>
}

#[derive(Debug, Clone)]
pub struct EventMap{
    rows: Vec<EventMapRow>,
}

#[derive(Debug, Clone, Default)]
pub struct EventPtrRow{
}

#[derive(Debug, Clone)]
pub struct EventPtr{
    rows: Vec<EventPtrRow>,
}

#[derive(Debug, Clone, Default)]
pub struct EventRow{
    event_flags: enums::ClrEventAttr,
    name: String,
    event_type: codedindex::TypeDefOrRef
}

#[derive(Debug, Clone)]
pub struct Event{
    rows: Vec<EventRow>,
}

#[derive(Debug, Clone, Default)]
pub struct PropertyMapRow{
    parent: TypeDefRow,
    property_list: Vec<PropertyRow>
}

#[derive(Debug, Clone)]
pub struct PropertyMap{
    rows: Vec<PropertyMapRow>,
}

#[derive(Debug, Clone, Default)]
pub struct PropertyPtrRow{
}

#[derive(Debug, Clone)]
pub struct PropertyPtr{
    rows: Vec<PropertyPtrRow>,
}

#[derive(Debug, Clone, Default)]
pub struct PropertyRow{
    flags: enums::ClrPropertyAttr,
    name: String,
    _type: Vec<u8>
}

#[derive(Debug, Clone)]
pub struct Property{
    rows: Vec<PropertyRow>,
}

#[derive(Debug, Clone, Default)]
pub struct MethodSemanticsRow{
    semantics: enums::ClrMethodSemanticsAttr,
    method: MethodDefRow,
    association: codedindex::HasSemantics
}

#[derive(Debug, Clone)]
pub struct MethodSemantics{
    rows: Vec<MethodSemanticsRow>,
}

#[derive(Debug, Clone, Default)]
pub struct MethodImplRow{
    class: TypeDefRow,
    method_body: codedindex::MethodDefOrRef,
    method_declaration: codedindex::MethodDefOrRef
}

#[derive(Debug, Clone)]
pub struct MethodImpl{
    rows: Vec<MethodImplRow>,
}

#[derive(Debug, Clone, Default)]
pub struct ModuleRefRow{
    name: String
}

#[derive(Debug, Clone)]
pub struct ModuleRef{
    rows: Vec<ModuleRefRow>,
}

#[derive(Debug, Clone, Default)]
pub struct TypeSpecRow{
    signature: Vec<u8>
}

#[derive(Debug, Clone)]
pub struct TypeSpec{
    rows: Vec<TypeSpecRow>,
}

#[derive(Debug, Clone, Default)]
pub struct ImplMapRow{
    mapping_flags: enums::ClrPinvokeMap,
    member_forwarded: codedindex::MemberForwarded,
    import_name: String,
    import_scope: ModuleRefRow
}

#[derive(Debug, Clone)]
pub struct ImplMap{
    rows: Vec<ImplMapRow>,
}

#[derive(Debug, Clone, Default)]
pub struct FieldRvaRow{
    rva: u32,
    field: FieldRow
}

#[derive(Debug, Clone)]
pub struct FieldRva{
    rows: Vec<FieldRvaRow>,
}

#[derive(Debug, Clone, Default)]
pub struct EncLogRow{
    token: u32,
    func_code: u32
}

#[derive(Debug, Clone)]
pub struct EncLog{
    rows: Vec<EncLogRow>,
}

#[derive(Debug, Clone, Default)]
pub struct EncMapRow{
    token: u32,
}

#[derive(Debug, Clone)]
pub struct EncMap{
    rows: Vec<EncMapRow>,
}

#[derive(Debug, Clone, Default)]
pub struct AssemblyRow{
    hash_alg_id: enums::AssemblyHashAlgorithm,
    major_version: u32,
    minor_version: u32,
    build_number: u32,
    revision_number: u32,
    flags: enums::ClrAssemblyFlags,
    public_key: Vec<u8>,
    name: String,
    culture: String
}

#[derive(Debug, Clone)]
pub struct Assembly{
    rows: Vec<AssemblyRow>,
}

#[derive(Debug, Clone, Default)]
pub struct AssemblyProcessorRow{
    processor: u32,
}

#[derive(Debug, Clone)]
pub struct AssemblyProcessor{
    rows: Vec<AssemblyProcessorRow>,
}

#[derive(Debug, Clone, Default)]
pub struct AssemblyOSRow{
    OSPlatformID: u32,
    OSMajorVersion: u32,
    OSMinorVersion: u32
}

#[derive(Debug, Clone)]
pub struct AssemblyOS{
    rows: Vec<AssemblyOSRow>,
}

#[derive(Debug, Clone, Default)]
pub struct AssemblyRefRow{
    major_version: u32,
    minor_version: u32,
    build_number: u32,
    revision_number: u32,
    flags: enums::ClrAssemblyFlags,
    public_key: Vec<u8>,
    name: String,
    culture: String,
    hash_value: Vec<u8>
}

#[derive(Debug, Clone)]
pub struct AssemblyRef{
    rows: Vec<AssemblyRefRow>,
}

#[derive(Debug, Clone, Default)]
pub struct AssemblyRefProcessorRow{
    processor: u32,
    assembly_ref: AssemblyRefRow
}

#[derive(Debug, Clone)]
pub struct AssemblyRefProcessor{
    rows: Vec<AssemblyRefProcessorRow>,
}

#[derive(Debug, Clone, Default)]
pub struct AssemblyRefOSRow{
    os_platform_id: u32,
    os_major_version: u32,
    os_minor_version: u32,
    assembly_ref: AssemblyRefRow
}

#[derive(Debug, Clone)]
pub struct AssemblyRefOS{
    rows: Vec<AssemblyRefOSRow>,
}

#[derive(Debug, Clone, Default)]
pub struct FileRow{
    flags: enums::ClrFileFlags,
    name: String,
    hash_value: Vec<u8>
}

#[derive(Debug, Clone)]
pub struct File{
    rows: Vec<FileRow>,
}

#[derive(Debug, Clone, Default)]
pub struct ExportedTypeRow{
    flags: enums::ClrTypeAttr,
    type_def_id: u32,
    type_name: String,
    type_namespace: String,
    implementation: codedindex::Implementation
}

#[derive(Debug, Clone)]
pub struct ExportedType{
    rows: Vec<ExportedTypeRow>,
}

#[derive(Debug, Clone, Default)]
pub struct ManifestResourceRow{
    offset: u32,
    flags: enums::ClrManifestResourceFlags,
    name: String,
    implementation: codedindex::Implementation
}

#[derive(Debug, Clone)]
pub struct ManifestResource{
    rows: Vec<ManifestResourceRow>,
}

#[derive(Debug, Clone, Default)]
pub struct NestedClassRow{
    nested_class: TypeDefRow,
    enclosing_class: TypeDefRow
}

#[derive(Debug, Clone)]
pub struct NestedClass{
    rows: Vec<NestedClassRow>,
}

#[derive(Debug, Clone, Default)]
pub struct GenericParamRow{
    number: u32,
    flags: enums::ClrGenericParamAttr,
    owner: codedindex::TypeOrMethodDef,
    name: String
}

#[derive(Debug, Clone)]
pub struct GenericParam{
    rows: Vec<GenericParamRow>,
}

#[derive(Debug, Clone, Default)]
pub struct GenericMethodRow{
    unknown1: codedindex::MethodDefOrRef,
    unknown2: Vec<u8>
}

#[derive(Debug, Clone)]
pub struct GenericMethod{
    rows: Vec<GenericMethodRow>,
}

#[derive(Debug, Clone, Default)]
pub struct GenericParamConstraintRow{
    owner: GenericParamRow,
    constraint: codedindex::TypeDefOrRef
}

#[derive(Debug, Clone)]
pub struct GenericParamConstraint{
    rows: Vec<GenericParamConstraintRow>,
}

#[derive(Debug, Clone, Default)]
pub struct UnusedRow{
}

#[derive(Debug, Clone)]
pub struct Unused{
    rows: Vec<UnusedRow>,
}

#[derive(Debug, Clone, Default)]
pub struct MaxTableRow{
}

#[derive(Debug, Clone)]
pub struct MaxTable{
    rows: Vec<MaxTableRow>,
}


#[derive(Debug, Clone)]
pub struct MetaDataTable{
    number: u32,
    is_sorted: bool,
    table: MDTable
}

impl crate::DnPe<'_>{
    pub fn create_md_table(&self,
                           i: &u32,
                           table_rowcounts: &Vec<usize>,
                           is_sorted: bool) -> Result<MetaDataTable>{
        let num_rows = table_rowcounts[*i as usize];
        let table = self.new_mdtable(*i, &num_rows)?;
        let mut table = MetaDataTable{
            number: *i,
            is_sorted,
            table
        };
        Ok(table)
    }

    pub fn new_module(&self, num_rows: &usize) -> Result<Module>{
        Ok(Module{
            rows: vec![ModuleRow::default(); *num_rows]
        })
    }

    pub fn new_type_ref(&self, num_rows: &usize) -> Result<TypeRef>{
        Ok(TypeRef{
            rows: vec![TypeRefRow::default(); *num_rows]
        })
    }

    pub fn new_mdtable(&self, i: u32,
                       num_rows: &usize) -> Result<MDTable>{
        match i{
            0 => Ok(MDTable::Module(self.new_module(num_rows)?)),
            1 => Ok(MDTable::TypeRef(self.new_type_ref(num_rows)?)),
            2 => Ok(MDTable::TypeDef(self.new_type_def(num_rows)?)),
            3 => Ok(MDTable::FieldPtr(self.new_field_ptr(num_rows)?)),
            4 => Ok(MDTable::Field(self.new_field(num_rows)?)),
            5 => Ok(MDTable::MethodPtr(self.new_method_ptr(num_rows)?)),
            6 => Ok(MDTable::MethodDef(self.new_method_def(num_rows)?)),
            7 => Ok(MDTable::ParamPtr(self.new_param_ptr(num_rows)?)),
            8 => Ok(MDTable::Param(self.new_param(num_rows)?)),
            9 => Ok(MDTable::InterfaceImpl(self.new_interface_impl(num_rows)?)),
            10 => Ok(MDTable::MemberRef(self.new_member_ref(num_rows)?)),
            11 => Ok(MDTable::Constant(self.new_constant(num_rows)?)),
            12 => Ok(MDTable::CustomAttribute(self.new_custom_attribute(num_rows)?)),
            13 => Ok(MDTable::FieldMarshal(self.new_field_marshal(num_rows)?)),
            14 => Ok(MDTable::DeclSecurity(self.new_decl_security(num_rows)?)),
            15 => Ok(MDTable::ClassLayout(self.new_class_layout(num_rows)?)),
            16 => Ok(MDTable::FieldLayout(self.new_field_layout(num_rows)?)),
            17 => Ok(MDTable::StandAloneSig(self.new_stand_alone_sig(num_rows)?)),
            18 => Ok(MDTable::EventMap(self.new_event_map(num_rows)?)),
            19 => Ok(MDTable::EventPtr(self.new_event_ptr(num_rows)?)),
            20 => Ok(MDTable::Event(self.new_event(num_rows)?)),
            21 => Ok(MDTable::PropertyMap(self.new_property_map(num_rows)?)),
            22 => Ok(MDTable::PropertyPtr(self.new_property_ptr(num_rows)?)),
            23 => Ok(MDTable::Property(self.new_property(num_rows)?)),
            24 => Ok(MDTable::MethodSemantics(self.new_method_semantics(num_rows)?)),
            25 => Ok(MDTable::MethodImpl(self.new_method_impl(num_rows)?)),
            26 => Ok(MDTable::ModuleRef(self.new_module_ref(num_rows)?)),
            27 => Ok(MDTable::TypeSpec(self.new_type_spec(num_rows)?)),
            28 => Ok(MDTable::ImplMap(self.new_impl_map(num_rows)?)),
            29 => Ok(MDTable::FieldRva(self.new_field_rva(num_rows)?)),
            30 => Ok(MDTable::EncLog(self.new_enc_log(num_rows)?)),
            31 => Ok(MDTable::EncMap(self.new_enc_map(num_rows)?)),
            32 => Ok(MDTable::Assembly(self.new_assembly(num_rows)?)),
            33 => Ok(MDTable::AssemblyProcessor(self.new_assembly_processor(num_rows)?)),
            34 => Ok(MDTable::AssemblyOS(self.new_assembly_os(num_rows)?)),
            35 => Ok(MDTable::AssemblyRef(self.new_assembly_ref(num_rows)?)),
            36 => Ok(MDTable::AssemblyRefProcessor(self.new_assembly_ref_processor(num_rows)?)),
            37 => Ok(MDTable::AssemblyRefOS(self.new_assembly_ref_os(num_rows)?)),
            38 => Ok(MDTable::File(self.new_file(num_rows)?)),
            39 => Ok(MDTable::ExportedType(self.new_exported_type(num_rows)?)),
            40 => Ok(MDTable::ManifestResource(self.new_manifest_resource(num_rows)?)),
            41 => Ok(MDTable::NestedClass(self.new_nested_class(num_rows)?)),
            42 => Ok(MDTable::GenericParam(self.new_generic_param(num_rows)?)),
            43 => Ok(MDTable::GenericMethod(self.new_generic_method(num_rows)?)),
            44 => Ok(MDTable::GenericParamConstraint(self.new_generic_param_constraint(num_rows)?)),
            // 45 through 63 are not used
            62 => Ok(MDTable::Unused(self.new_unused(num_rows)?)),
            63 => Ok(MDTable::MaxTable(self.new_max_table(num_rows)?)),
            _ => Err(crate::error::Error::UndefinedMetaDataTableIndex(i))
        }
    }
}

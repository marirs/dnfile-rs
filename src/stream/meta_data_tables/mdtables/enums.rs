use crate::Result;

#[derive(Debug, Clone)]
pub enum CorTypeVisibility{
    NotPublic,
    Public,
    NestedPublic,
    NestedPrivate,
    NestedFamily,
    NestedAssembly,
    NestedFamANDAssem,
    NestedFamORAssem
}

impl CorTypeVisibility{
    pub fn new(val: usize) -> Self{
        match val & 0x7{
            1 => Self::Public,
            2 => Self::NestedPublic,
            3 => Self::NestedPrivate,
            4 => Self::NestedFamily,
            5 => Self::NestedAssembly,
            6 => Self::NestedFamANDAssem,
            7 => Self::NestedFamORAssem,
            _ => Self::NotPublic,
        }
    }
}

impl Default for CorTypeVisibility{
    fn default() -> Self{
        Self::NotPublic
    }
}

#[derive(Debug, Clone)]
pub enum CorTypeLayout{
    AutoLayout,
    SequentialLayout,
    ExplicitLayout
}

impl CorTypeLayout{
    pub fn new(val: usize) -> Self{
        match val & 0x18{
            0x8 => Self::SequentialLayout,
            0x10 => Self::ExplicitLayout,
            _ => Self::AutoLayout,
        }
    }
}

impl Default for CorTypeLayout{
    fn default() -> Self{
        Self::AutoLayout
    }
}

#[derive(Debug, Clone)]
pub enum CorTypeSemantics{
    Class,
    Interface
}

impl CorTypeSemantics{
    pub fn new(val: usize) -> Self{
        match val & 0x20{
            0x20 => Self::Interface,
            _ => Self::Class,
        }
    }
}

impl Default for CorTypeSemantics{
    fn default() -> Self{
        Self::Class
    }
}

#[derive(Debug, Clone)]
pub enum CorTypeAttrFlags{
    Abstract,
    Sealed,
    SpecialName,
    RTSpecialName,
    Import,
    Serializable,
    WindowsRuntime,
    HasSecurity,
    BeforeFieldInit,
    Forwarder
}

impl CorTypeAttrFlags{
    pub fn new(val: usize) -> Vec<Self>{
        let mut res = vec![];
        if val & 0x00000080 != 0 {
            res.push(Self::Abstract);
        }
        if val & 0x00000100 != 0 {
            res.push(Self::Sealed);
        }
        if val & 0x00000400 != 0 {
            res.push(Self::SpecialName);
        }
        if val & 0x00000800 != 0 {
            res.push(Self::RTSpecialName);
        }
        if val & 0x00001000 != 0 {
            res.push(Self::Import);
        }
        if val & 0x00002000 != 0 {
            res.push(Self::Serializable);
        }
        if val & 0x00004000 != 0 {
            res.push(Self::WindowsRuntime);
        }
        if val & 0x00040000 != 0 {
            res.push(Self::HasSecurity);
        }
        if val & 0x00100000 != 0 {
            res.push(Self::BeforeFieldInit);
        }
        if val & 0x00200000 != 0 {
            res.push(Self::Forwarder);
        }
        res
    }
}

#[derive(Debug, Clone)]
pub enum CorTypeStringFormat{
    AnsiClass,
    UnicodeClass,
    AutoClass,
    CustomFormatClass
}

impl CorTypeStringFormat{
    pub fn new(val: usize) -> Self{
        match val & 0x00030000{
            0x00010000 => Self::UnicodeClass,
            0x00020000 => Self::AutoClass,
            0x00030000 => Self::CustomFormatClass,
            _ => Self::AnsiClass,
        }
    }
}

impl Default for CorTypeStringFormat{
    fn default() -> Self{
        Self::AnsiClass
    }
}

#[derive(Debug, Clone)]
pub enum CorMethodCodeType{
    IL,
    Native,
    OPTIL,
    Runtime
}

impl CorMethodCodeType{
    pub fn new(value: usize) -> Self{
        match value & 0x3{
            1 => Self::Native,
            2 => Self::OPTIL,
            3 => Self::Runtime,
            _ => Self::IL,
        }
    }
}

#[derive(Debug, Clone)]
pub enum CorMethodManaged{
    Unmanaged,
    Managed
}

impl CorMethodManaged{
    pub fn new(value: usize) -> Self{
        match value & 0x4{
            4 => Self::Unmanaged,
            _ => Self::Managed,
        }
    }
}

#[derive(Debug, Clone)]
pub enum ClrMethodImpl{
    MethodCodeType(CorMethodCodeType),
    MethodManaged(CorMethodManaged),
    ForwardRef,
    PreserveSig,
    InternalCall,
    Synchronized,
    NoInlining,
    MaxMethodImplVal
}
impl ClrMethodImpl{
    pub fn new(value: usize) -> Vec<Self>{
        let mut res = vec![];
        if value & 0x10 != 0{
            res.push(Self::ForwardRef);
        }
        if value & 0x080 != 0{
            res.push(Self::PreserveSig);
        }
        if value & 0x1000 != 0{
            res.push(Self::InternalCall);
        }
        if value & 0x20 != 0{
            res.push(Self::NoInlining);
        }
        if value & 0x8 != 0{
            res.push(Self::MaxMethodImplVal);
        }
        res.push(Self::MethodCodeType(CorMethodCodeType::new(value)));
        res.push(Self::MethodManaged(CorMethodManaged::new(value)));
        res
    }
}

#[derive(Debug, Clone, Default)]
pub struct ClrTypeAttr{
    visibility: CorTypeVisibility,
    layout: CorTypeLayout,
    class_semantics:  CorTypeSemantics,
    flags: Vec<CorTypeAttrFlags>,
    string_format: CorTypeStringFormat
}

impl ClrTypeAttr{
    pub fn set(&mut self, data: &[u8]) -> Result<()>{
        if data.len() != 4{
            return Err(crate::error::Error::FormatError(format!("CtrlTypeAttr incorrect size {} {}", data.len(), 4)));
        }
        let val = crate::utils::read_usize(data)?;
        self.visibility = CorTypeVisibility::new(val);
        self.layout = CorTypeLayout::new(val);
        self.class_semantics = CorTypeSemantics::new(val);
        self.flags = CorTypeAttrFlags::new(val);
        self.string_format = CorTypeStringFormat::new(val);
        Ok(())
    }
}

#[derive(Debug, Clone)]
pub enum CorFieldAccess{
    PrivateScope,
    Private,
    FamANDAssem,
    Assembly,
    Family,
    FamORAssem,
    Public,
    Unknown1
}

impl CorFieldAccess{
    pub fn new(value: usize) -> Self{
        match value & 7{
            1 => Self::Private,
            2 => Self::FamANDAssem,
            3 => Self::Assembly,
            4 => Self::Family,
            5 => Self::FamORAssem,
            6 => Self::Public,
            7 => Self::Unknown1,
            _ => Self::PrivateScope
        }
    }
}

#[derive(Debug, Clone)]
pub enum ClrFieldAttr{
    FieldAccess(CorFieldAccess),
    Static,
    InitOnly,
    Literal,
    NotSerialized,
    SpecialName,
    PinvokeImpl,
    RTSpecialName,
    HasFieldMarshal,
    HasDefault,
    HasFieldRVA
}

impl ClrFieldAttr{
    pub fn new(value: usize) -> Vec<Self>{
        let mut res = vec![];
        res.push(Self::FieldAccess(CorFieldAccess::new(value)));
        if value & 0x10 != 0{
            res.push(Self::Static);
        }
        if value & 0x20 != 0{
            res.push(Self::InitOnly);
        }
        if value & 0x40 != 0{
            res.push(Self::Literal);
        }
        if value & 0x80 != 0{
            res.push(Self::NotSerialized);
        }
        if value & 0x200 != 0{
            res.push(Self::SpecialName);
        }
        if value & 0x2000 != 0{
            res.push(Self::PinvokeImpl);
        }
        if value & 0x100 != 0{
            res.push(Self::HasFieldRVA);
        }
        if value & 0x400 != 0{
            res.push(Self::RTSpecialName);
        }
        if value & 0x1000 != 0{
            res.push(Self::HasFieldMarshal);
        }
        if value & 0x8000 != 0{
            res.push(Self::HasDefault);
        }
        res
    }
}

#[derive(Debug, Clone)]
pub enum CorMethodMemberAccess{
    PrivateScope,
    Private,
    FamANDAssem,
    Assem,
    Family,
    FamORAssem,
    Public,
    Unknown1
}

impl CorMethodMemberAccess{
    pub fn new(value: usize) -> Self{
        match value & 0x7{
            1 => Self::Private,
            2 => Self::FamANDAssem,
            3 => Self::Assem,
            4 => Self::Family,
            5 => Self::FamORAssem,
            6 => Self::Public,
            7 => Self::Unknown1,
            _ => Self::PrivateScope
        }
    }
}

#[derive(Debug, Clone)]
pub enum CorMethodAttrFlag{
    Static,
    Final,
    Virtual,
    HideBySig,
    CheckAccessOnOverride,
    Abstract,
    SpecialName,
    PinvokeImpl,
    UnmanagedExport,
    RTSpecialName,
    HasSecurity,
    RequireSecObject
}

impl CorMethodAttrFlag{
    pub fn new(value: usize) -> Vec<Self>{
        let mut res = vec![];
        if value & 0x10 != 0{
            res.push(Self::Static);
        }
        if value & 0x20 != 0{
            res.push(Self::Final);
        }
        if value & 0x40 != 0{
            res.push(Self::Virtual);
        }
        if value & 0x80 != 0{
            res.push(Self::HideBySig);
        }
        if value & 0x200 != 0{
            res.push(Self::CheckAccessOnOverride);
        }
        if value & 0x400 != 0{
            res.push(Self::Abstract);
        }
        if value & 0x800 != 0{
            res.push(Self::SpecialName);
        }
        if value & 0x2000 != 0{
            res.push(Self::PinvokeImpl);
        }
        if value & 0x8 != 0{
            res.push(Self::UnmanagedExport);
        }
        if value & 0x1000 != 0{
            res.push(Self::RTSpecialName);
        }
        if value & 0x4000 != 0{
            res.push(Self::HasSecurity);
        }
        if value & 0x8000 != 0{
            res.push(Self::RTSpecialName);
        }
        res
    }
}

#[derive(Debug, Clone)]
pub enum CorMethodVtableLayout{
    ReuseSlot,
    NewSlot
}

impl CorMethodVtableLayout{
    pub fn new(value: usize) -> Self{
        match value & 0x100 {
            0x100 => Self::NewSlot,
            _ => Self::ReuseSlot
        }
    }
}

#[derive(Debug, Clone)]
pub enum ClrMethodAttr{
    MemberAccess(CorMethodMemberAccess),
    AttrFlag(Vec<CorMethodAttrFlag>),
    VtableLayout(CorMethodVtableLayout)
}

impl ClrMethodAttr{
    pub fn new(value: usize) -> Vec<Self>{
        let mut res = vec![];
        res.push(Self::MemberAccess(CorMethodMemberAccess::new(value)));
        res.push(Self::AttrFlag(CorMethodAttrFlag::new(value)));
        res.push(Self::VtableLayout(CorMethodVtableLayout::new(value)));
        res
    }
}

#[derive(Debug, Clone)]
pub enum ClrParamAttr{
    In,
    Out,
    Optional,
    HasDefault,
    HasFieldMarshal
}

impl ClrParamAttr{
    pub fn new(value: usize) -> Vec<Self>{
        let mut res = vec![];
        if value & 1 != 0{
            res.push(Self::In);
        }
        if value & 2 != 0{
            res.push(Self::Out);
        }
        if value & 0x10 != 0{
            res.push(Self::Optional);
        }
        if value & 0x1000 != 0{
            res.push(Self::HasDefault);
        }
        if value & 0x2000 != 0{
            res.push(Self::HasFieldMarshal);
        }
        res
    }
}

#[derive(Debug, Clone)]
pub enum ClrEventAttr{}

#[derive(Debug, Clone)]
pub enum ClrPropertyAttr{
    SpecialName,
    RTSpecialName,
    HasDefault
}

impl ClrPropertyAttr{
    pub fn new(value: usize) -> Vec<Self>{
        let mut res = vec![];
        if value & 0x200 != 0{
            res.push(Self::SpecialName);
        }
        if value & 0x400 != 0{
            res.push(Self::RTSpecialName);
        }
        if value & 0x1000 != 0{
            res.push(Self::HasDefault);
        }
        res
    }
}

#[derive(Debug, Clone)]
pub enum ClrMethodSemanticsAttr{
    Setter,
    Getter,
    Other,
    AddOn,
    RemoveOn,
    Fire
}

impl ClrMethodSemanticsAttr{
    pub fn new(value: usize) -> Vec<Self>{
        let mut res = vec![];
        if value & 0x1 != 0{
            res.push(Self::Setter);
        }
        if value & 0x2 != 0{
            res.push(Self::Getter);
        }
        if value & 0x4 != 0{
            res.push(Self::Other);
        }
        if value & 0x8 != 0{
            res.push(Self::AddOn);
        }
        if value & 0x10 != 0{
            res.push(Self::RemoveOn);
        }
        if value & 0x20 != 0{
            res.push(Self::Fire);
        }
        res
    }
}


#[derive(Debug, Clone)]
pub enum CorPinvokeMapCharSet{
    NotSpec,
    Ansi,
    Unicode,
    Auto
}

impl CorPinvokeMapCharSet{
    pub fn new(value: usize) -> Self{
        match value & 6{
            2 => Self::Ansi,
            4 => Self::Unicode,
            6 => Self::Auto,
            _ => Self::NotSpec
        }
    }
}

#[derive(Debug, Clone)]
pub enum  CorPinvokeBestFit{
    UseAssem,
    Enabled,
    Disabled
}

impl CorPinvokeBestFit{
    pub fn new(value: usize) -> Self{
        match value & 0x30{
            0x10 => Self::Enabled,
            0x20 => Self::Disabled,
            _ => Self::UseAssem
        }
    }
}


#[derive(Debug, Clone)]
pub enum CorPinvokeThrowOnUnmappableChar{
    UseAssem,
    Enabled,
    Disabled
}

impl CorPinvokeThrowOnUnmappableChar{
    pub fn new(value: usize) -> Self{
        match value & 0x3000{
            0x1000 => Self::Enabled,
            0x2000 => Self::Disabled,
            _ => Self::UseAssem
        }
    }
}

#[derive(Debug, Clone)]
pub enum CorPinvokeCallConv{
    None,
    Winapi,
    Cdecl,
    Stdcall,
    Thiscall,
    Fastcall,
    Unknown1,
    Unknown2
}

impl CorPinvokeCallConv{
    pub fn new(value: usize) -> Self{
        match value & 0x700{
            0x100 => Self::Winapi,
            0x200 => Self::Cdecl,
            0x300 => Self::Stdcall,
            0x400 => Self::Thiscall,
            0x500 => Self::Fastcall,
            0x600 => Self::Unknown1,
            0x700 => Self::Unknown2,
            _ => Self::None
        }
    }
}

#[derive(Debug, Clone)]
pub enum ClrPinvokeMap{
    CharSet(CorPinvokeMapCharSet),
    BestFit(CorPinvokeBestFit),
    ThrowOnUnmappableChar(CorPinvokeThrowOnUnmappableChar),
    CallConv(CorPinvokeCallConv),
    NoMangle,
    SupportsLastError
}

impl ClrPinvokeMap{
    pub fn new(value: usize) -> Vec<Self>{
        let mut res = vec![];
        res.push(Self::CharSet(CorPinvokeMapCharSet::new(value)));
        res.push(Self::BestFit(CorPinvokeBestFit::new(value)));
        res.push(Self::ThrowOnUnmappableChar(CorPinvokeThrowOnUnmappableChar::new(value)));
        res.push(Self::CallConv(CorPinvokeCallConv::new(value)));
        if value & 1 != 0{
            res.push(Self::NoMangle);
        }
        if value & 0x40 != 0{
            res.push(Self::SupportsLastError);
        }
        res
    }
}

#[derive(Debug, Clone)]
pub enum AssemblyHashAlgorithm{
    None,
    Md5,
    Sha1,
    Sha256,
    Sha384,
    Sha512
}

impl AssemblyHashAlgorithm{
    pub fn new(val: usize) -> AssemblyHashAlgorithm{
        match val{
            0x8003 => Self::Md5,
            0x8004 => Self::Sha1,
            0x800c => Self::Sha256,
            0x800d => Self::Sha384,
            0x800e => Self::Sha512,
            _ => Self::None
        }
    }
}

impl Default for AssemblyHashAlgorithm{
    fn default() -> Self{
        Self::None
    }
}

#[derive(Debug, Clone)]
pub enum CorAssemblyFlagsPA{
    PaNone,
    PaMSIL,
    PaX86,
    PaIA64,
    PaAMD64,
    PaUnknown1,
    PaUnknown2,
    PaUnknown3
}

impl CorAssemblyFlagsPA{
    pub fn new(value: usize) -> Self{
        match value & 0x70{
            0x0010 => Self::PaMSIL,
            0x0020 => Self::PaX86,
            0x0030 => Self::PaIA64,
            0x0040 => Self::PaAMD64,
            0x0050 => Self::PaUnknown1,
            0x0060 => Self::PaUnknown2,
            0x0070 => Self::PaUnknown3,
            _ => Self::PaNone
        }
    }
}

#[derive(Debug, Clone)]
pub enum ClrAssemblyFlags{
    PublicKey,
    PA(CorAssemblyFlagsPA),
    PaSpecified,
    EnableJITcompileTracking,
    DisableJITcompileOptimizer,
    Retargetable
}

impl ClrAssemblyFlags{
    pub fn new(value: usize) -> Vec<ClrAssemblyFlags>{
        let mut res = vec![];
        if value & 1 != 0{
            res.push(ClrAssemblyFlags::PublicKey);
        }
        if value & 0x0100 != 0{
            res.push(ClrAssemblyFlags::Retargetable);
        }
        if value & 0x4000 != 0{
            res.push(ClrAssemblyFlags::DisableJITcompileOptimizer);
        }
        if value & 0x8000 != 0{
            res.push(ClrAssemblyFlags::EnableJITcompileTracking);
        }
        if value & 0x0080 != 0{
            res.push(ClrAssemblyFlags::PaSpecified);
            res.push(ClrAssemblyFlags::PA(CorAssemblyFlagsPA::new(value)));
        }
        res
    }
}

#[derive(Debug, Clone)]
pub enum ClrFileFlags{}

#[derive(Debug, Clone)]
pub enum ClrManifestResourceFlags{}

#[derive(Debug, Clone)]
pub enum ClrGenericParamAttr{}

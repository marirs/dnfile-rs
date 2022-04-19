use crate::Result;

#[derive(Debug, Clone, Default)]
pub struct ResolutionScope{}

#[derive(Debug, Clone, Default)]
pub struct TypeDefOrRef{}

#[derive(Debug, Clone, Default)]
pub struct MemberRefParent{}

#[derive(Debug, Clone, Default)]
pub struct HasConstant{}

#[derive(Debug, Clone, Default)]
pub struct HasCustomAttribute{}

#[derive(Debug, Clone, Default)]
pub struct CustomAttributeType{}

#[derive(Debug, Clone, Default)]
pub struct HasFieldMarshall{}

#[derive(Debug, Clone, Default)]
pub struct HasCustomMarshall{}

#[derive(Debug, Clone, Default)]
pub struct HasDeclSecurity{}

#[derive(Debug, Clone, Default)]
pub struct HasSemantics{}

#[derive(Debug, Clone, Default)]
pub struct MethodDefOrRef{}

#[derive(Debug, Clone, Default)]
pub struct MemberForwarded{}

#[derive(Debug, Clone, Default)]
pub struct Implementation{}

#[derive(Debug, Clone, Default)]
pub struct TypeOrMethodDef{}

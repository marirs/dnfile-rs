use crate::{error::Error, Result};

pub fn clr_coded_index_struct_size(
    tag_bits: usize,
    table_names: &[&'static str],
    tables_row_counts: &[usize],
) -> usize {
    let mut max_index = 0;
    for name in table_names {
        let table_index = if let Ok(s) = super::table_name_2_index(name) {
            s
        } else {
            0
        };
        let table_rowcnt = tables_row_counts[table_index];
        max_index = std::cmp::max(max_index, table_rowcnt);
    }
    if max_index <= 1 << (16 - tag_bits) {
        2
    } else {
        4
    }
}

pub trait CodedIndex {
    fn set_row_index(&mut self, value: usize);
    fn set_table(&mut self, value: String);
    fn get_table_name(&self, index: usize) -> Result<&'static str>;
    fn get_tag_bits(&self) -> usize;
    fn set(
        &mut self,
        value: &[u8],
        tables: &std::collections::BTreeMap<usize, super::MetaDataTable>,
    ) -> Result<()> {
        let value = crate::utils::read_usize(value)?;
        let table_name = self.get_table_name(value & ((1 << self.get_tag_bits()) - 1))?;
        self.set_row_index(value >> self.get_tag_bits());
        for t in tables.values() {
            if t.table.name() != table_name {
                continue;
            }
            self.set_table(table_name.to_string());
            return Ok(());
        }
        Err(Error::CodedIndexWithUndefinedTable(table_name.to_string()))
    }
}

#[derive(Debug, Clone)]
pub struct SimpleCodedIndex {
    pub tag_bits: usize,
    pub table_names: Vec<&'static str>,
    pub row_index: usize,
    pub table: String,
}

impl SimpleCodedIndex {
    pub fn new(
        table_names: Vec<&'static str>,
        tag_bits: usize,
        value: &[u8],
        tables: &std::collections::BTreeMap<usize, super::MetaDataTable>,
    ) -> Result<SimpleCodedIndex> {
        let mut res = SimpleCodedIndex {
            tag_bits,
            table_names,
            row_index: 0,
            table: "".to_string(),
        };
        res.set(value, tables)?;
        Ok(res)
    }
}

impl CodedIndex for SimpleCodedIndex {
    fn set_row_index(&mut self, value: usize) {
        self.row_index = value;
    }
    fn set_table(&mut self, value: String) {
        self.table = value;
    }
    fn get_table_name(&self, index: usize) -> Result<&'static str> {
        Ok(self.table_names[index])
    }
    fn get_tag_bits(&self) -> usize {
        self.tag_bits
    }
}

impl Default for SimpleCodedIndex {
    fn default() -> Self {
        Self {
            tag_bits: 0,
            table_names: vec![],
            row_index: 0,
            table: "".to_string(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct ResolutionScope {
    pub tag_bits: usize,
    pub table_names: Vec<&'static str>,
    pub row_index: usize,
    pub table: String,
}

impl CodedIndex for ResolutionScope {
    fn set_row_index(&mut self, value: usize) {
        self.row_index = value;
    }
    fn set_table(&mut self, value: String) {
        self.table = value;
    }
    fn get_table_name(&self, index: usize) -> Result<&'static str> {
        Ok(self.table_names[index])
    }
    fn get_tag_bits(&self) -> usize {
        self.tag_bits
    }
}

impl Default for ResolutionScope {
    fn default() -> Self {
        Self {
            tag_bits: 2,
            table_names: vec!["Module", "ModuleRef", "AssemblyRef", "TypeRef"],
            row_index: 0,
            table: "".to_string(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct TypeDefOrRef {
    pub tag_bits: usize,
    pub table_names: Vec<&'static str>,
    pub row_index: usize,
    pub table: String,
}

impl CodedIndex for TypeDefOrRef {
    fn set_row_index(&mut self, value: usize) {
        self.row_index = value;
    }
    fn set_table(&mut self, value: String) {
        self.table = value;
    }
    fn get_table_name(&self, index: usize) -> Result<&'static str> {
        Ok(self.table_names[index])
    }
    fn get_tag_bits(&self) -> usize {
        self.tag_bits
    }
}

impl Default for TypeDefOrRef {
    fn default() -> Self {
        Self {
            tag_bits: 2,
            table_names: vec!["TypeDef", "TypeRef", "TypeSpec"],
            row_index: 0,
            table: "".to_string(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct MemberRefParent {
    pub tag_bits: usize,
    pub table_names: Vec<&'static str>,
    pub row_index: usize,
    pub table: String,
}

impl CodedIndex for MemberRefParent {
    fn set_row_index(&mut self, value: usize) {
        self.row_index = value;
    }
    fn set_table(&mut self, value: String) {
        self.table = value;
    }
    fn get_table_name(&self, index: usize) -> Result<&'static str> {
        Ok(self.table_names[index])
    }
    fn get_tag_bits(&self) -> usize {
        self.tag_bits
    }
}

impl Default for MemberRefParent {
    fn default() -> Self {
        Self {
            tag_bits: 3,
            table_names: vec!["TypeDef", "TypeRef", "ModuleRef", "MethodDef", "TypeSpec"],
            row_index: 0,
            table: "".to_string(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct HasConstant {
    pub tag_bits: usize,
    pub table_names: Vec<&'static str>,
    pub row_index: usize,
    pub table: String,
}

impl CodedIndex for HasConstant {
    fn set_row_index(&mut self, value: usize) {
        self.row_index = value;
    }
    fn set_table(&mut self, value: String) {
        self.table = value;
    }
    fn get_table_name(&self, index: usize) -> Result<&'static str> {
        Ok(self.table_names[index])
    }
    fn get_tag_bits(&self) -> usize {
        self.tag_bits
    }
}

impl Default for HasConstant {
    fn default() -> Self {
        Self {
            tag_bits: 2,
            table_names: vec!["Field", "Param", "Property"],
            row_index: 0,
            table: "".to_string(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct HasCustomAttribute {
    pub tag_bits: usize,
    pub table_names: Vec<&'static str>,
    pub row_index: usize,
    pub table: String,
}

impl CodedIndex for HasCustomAttribute {
    fn set_row_index(&mut self, value: usize) {
        self.row_index = value;
    }
    fn set_table(&mut self, value: String) {
        self.table = value;
    }
    fn get_table_name(&self, index: usize) -> Result<&'static str> {
        Ok(self.table_names[index])
    }
    fn get_tag_bits(&self) -> usize {
        self.tag_bits
    }
}

impl Default for HasCustomAttribute {
    fn default() -> Self {
        Self {
            tag_bits: 5,
            table_names: vec![
                "MethodDef",
                "Field",
                "TypeRef",
                "TypeDef",
                "Param",
                "InterfaceImpl",
                "MemberRef",
                "Module",
                "DeclSecurity",
                "Property",
                "Event",
                "StandAloneSig",
                "ModuleRef",
                "TypeSpec",
                "Assembly",
                "AssemblyRef",
                "File",
                "ExportedType",
                "ManifestResource",
                "GenericParam",
                "GenericParamConstraint",
            ],
            row_index: 0,
            table: "".to_string(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct CustomAttributeType {
    pub tag_bits: usize,
    pub table_names: Vec<&'static str>,
    pub row_index: usize,
    pub table: String,
}

impl CodedIndex for CustomAttributeType {
    fn set_row_index(&mut self, value: usize) {
        self.row_index = value;
    }
    fn set_table(&mut self, value: String) {
        self.table = value;
    }
    fn get_table_name(&self, index: usize) -> Result<&'static str> {
        Ok(self.table_names[index])
    }
    fn get_tag_bits(&self) -> usize {
        self.tag_bits
    }
}

impl Default for CustomAttributeType {
    fn default() -> Self {
        Self {
            tag_bits: 3,
            table_names: vec!["Unused", "Unused", "MethodDef", "MemberRef", "Unused"],
            row_index: 0,
            table: "".to_string(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct HasFieldMarshall {
    pub tag_bits: usize,
    pub table_names: Vec<&'static str>,
    pub row_index: usize,
    pub table: String,
}

impl CodedIndex for HasFieldMarshall {
    fn set_row_index(&mut self, value: usize) {
        self.row_index = value;
    }
    fn set_table(&mut self, value: String) {
        self.table = value;
    }
    fn get_table_name(&self, index: usize) -> Result<&'static str> {
        Ok(self.table_names[index])
    }
    fn get_tag_bits(&self) -> usize {
        self.tag_bits
    }
}

impl Default for HasFieldMarshall {
    fn default() -> Self {
        Self {
            tag_bits: 1,
            table_names: vec!["Field", "Param"],
            row_index: 0,
            table: "".to_string(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct HasDeclSecurity {
    pub tag_bits: usize,
    pub table_names: Vec<&'static str>,
    pub row_index: usize,
    pub table: String,
}

impl CodedIndex for HasDeclSecurity {
    fn set_row_index(&mut self, value: usize) {
        self.row_index = value;
    }
    fn set_table(&mut self, value: String) {
        self.table = value;
    }
    fn get_table_name(&self, index: usize) -> Result<&'static str> {
        Ok(self.table_names[index])
    }
    fn get_tag_bits(&self) -> usize {
        self.tag_bits
    }
}

impl Default for HasDeclSecurity {
    fn default() -> Self {
        Self {
            tag_bits: 2,
            table_names: vec!["TypeDef", "MethodDef", "Assembly"],
            row_index: 0,
            table: "".to_string(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct HasSemantics {
    pub tag_bits: usize,
    pub table_names: Vec<&'static str>,
    pub row_index: usize,
    pub table: String,
}

impl CodedIndex for HasSemantics {
    fn set_row_index(&mut self, value: usize) {
        self.row_index = value;
    }
    fn set_table(&mut self, value: String) {
        self.table = value;
    }
    fn get_table_name(&self, index: usize) -> Result<&'static str> {
        Ok(self.table_names[index])
    }
    fn get_tag_bits(&self) -> usize {
        self.tag_bits
    }
}

impl Default for HasSemantics {
    fn default() -> Self {
        Self {
            tag_bits: 1,
            table_names: vec!["Event", "Property"],
            row_index: 0,
            table: "".to_string(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct MethodDefOrRef {
    pub tag_bits: usize,
    pub table_names: Vec<&'static str>,
    pub row_index: usize,
    pub table: String,
}

impl CodedIndex for MethodDefOrRef {
    fn set_row_index(&mut self, value: usize) {
        self.row_index = value;
    }
    fn set_table(&mut self, value: String) {
        self.table = value;
    }
    fn get_table_name(&self, index: usize) -> Result<&'static str> {
        Ok(self.table_names[index])
    }
    fn get_tag_bits(&self) -> usize {
        self.tag_bits
    }
}

impl Default for MethodDefOrRef {
    fn default() -> Self {
        Self {
            tag_bits: 1,
            table_names: vec!["MethodDef", "MemberRef"],
            row_index: 0,
            table: "".to_string(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct MemberForwarded {
    pub tag_bits: usize,
    pub table_names: Vec<&'static str>,
    pub row_index: usize,
    pub table: String,
}

impl CodedIndex for MemberForwarded {
    fn set_row_index(&mut self, value: usize) {
        self.row_index = value;
    }
    fn set_table(&mut self, value: String) {
        self.table = value;
    }
    fn get_table_name(&self, index: usize) -> Result<&'static str> {
        Ok(self.table_names[index])
    }
    fn get_tag_bits(&self) -> usize {
        self.tag_bits
    }
}

impl Default for MemberForwarded {
    fn default() -> Self {
        Self {
            tag_bits: 1,
            table_names: vec!["Field", "MethodDef"],
            row_index: 0,
            table: "".to_string(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct Implementation {
    pub tag_bits: usize,
    pub table_names: Vec<&'static str>,
    pub row_index: usize,
    pub table: String,
}

impl CodedIndex for Implementation {
    fn set_row_index(&mut self, value: usize) {
        self.row_index = value;
    }
    fn set_table(&mut self, value: String) {
        self.table = value;
    }
    fn get_table_name(&self, index: usize) -> Result<&'static str> {
        Ok(self.table_names[index])
    }
    fn get_tag_bits(&self) -> usize {
        self.tag_bits
    }
}

impl Default for Implementation {
    fn default() -> Self {
        Self {
            tag_bits: 2,
            table_names: vec!["File", "AssemblyRef", "ExportedType"],
            row_index: 0,
            table: "".to_string(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct TypeOrMethodDef {
    pub tag_bits: usize,
    pub table_names: Vec<&'static str>,
    pub row_index: usize,
    pub table: String,
}

impl CodedIndex for TypeOrMethodDef {
    fn set_row_index(&mut self, value: usize) {
        self.row_index = value;
    }
    fn set_table(&mut self, value: String) {
        self.table = value;
    }
    fn get_table_name(&self, index: usize) -> Result<&'static str> {
        Ok(self.table_names[index])
    }
    fn get_tag_bits(&self) -> usize {
        self.tag_bits
    }
}

impl Default for TypeOrMethodDef {
    fn default() -> Self {
        Self {
            tag_bits: 1,
            table_names: vec!["TypeDef", "MethodDef"],
            row_index: 0,
            table: "".to_string(),
        }
    }
}

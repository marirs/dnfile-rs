use crate::Result;

pub fn clr_coded_index_struct_size(tag_bits: usize, table_names: &Vec<&'static str>, tables_row_counts: &Vec<usize>) -> usize{
    let mut max_index = 0;
    for name in table_names{
        let table_index = if let Ok(s) = super::table_name_2_index(name){
            s
        } else {
            0
        };
        let table_rowcnt = tables_row_counts[table_index];
        max_index = std::cmp::max(max_index, table_rowcnt);
    }
    if max_index <= 1<<(16 - tag_bits){
        2
    } else {
        4
    }
}

#[derive(Debug, Clone)]
pub struct ResolutionScope{
    pub tag_bits: usize,
    pub table_names: Vec<&'static str>
}

impl Default for ResolutionScope{
    fn default() -> Self{
        Self{
            tag_bits: 2,
            table_names: vec!["Module", "ModuleRef", "AssemblyRef", "TypeRef"]
        }
    }
}

#[derive(Debug, Clone)]
pub struct TypeDefOrRef{
    pub tag_bits: usize,
    pub table_names: Vec<&'static str>
}

impl Default for TypeDefOrRef{
    fn default() -> Self{
        Self{
            tag_bits: 2,
            table_names: vec!["TypeDef", "TypeRef", "TypeSpec"]
        }
    }
}

#[derive(Debug, Clone)]
pub struct MemberRefParent{
    pub tag_bits: usize,
    pub table_names: Vec<&'static str>
}

impl Default for MemberRefParent{
    fn default() -> Self{
        Self{
            tag_bits: 3,
            table_names: vec!["TypeDef", "TypeRef", "ModuleRef", "MethodDef", "TypeSpec"]
        }
    }
}

#[derive(Debug, Clone)]
pub struct HasConstant{
    pub tag_bits: usize,
    pub table_names: Vec<&'static str>
}

impl Default for HasConstant{
    fn default() -> Self{
        Self{
            tag_bits: 2,
            table_names: vec!["Field", "Param", "Property"]
        }
    }
}

#[derive(Debug, Clone)]
pub struct HasCustomAttribute{
    pub tag_bits: usize,
    pub table_names: Vec<&'static str>
}

impl Default for HasCustomAttribute{
    fn default() -> Self{
        Self{
            tag_bits: 5,
            table_names: vec!["MethodDef",
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
                              "GenericParamConstraint"]
        }
    }
}

#[derive(Debug, Clone)]
pub struct CustomAttributeType{
    pub tag_bits: usize,
    pub table_names: Vec<&'static str>
}

impl Default for CustomAttributeType{
    fn default() -> Self{
        Self{
            tag_bits: 3,
            table_names: vec!["Unused", "Unused", "MethodDef", "MemberRef", "Unused"]
        }
    }
}

#[derive(Debug, Clone)]
pub struct HasFieldMarshall{
    pub tag_bits: usize,
    pub table_names: Vec<&'static str>
}

impl Default for HasFieldMarshall{
    fn default() -> Self{
        Self{
            tag_bits: 1,
            table_names: vec!["Field", "Param"]
        }
    }
}

#[derive(Debug, Clone)]
pub struct HasDeclSecurity{
    pub tag_bits: usize,
    pub table_names: Vec<&'static str>
}

impl Default for HasDeclSecurity{
    fn default() -> Self{
        Self{
            tag_bits: 2,
            table_names: vec!["TypeDef", "MethodDef", "Assembly"]
        }
    }
}

#[derive(Debug, Clone)]
pub struct HasSemantics{
    pub tag_bits: usize,
    pub table_names: Vec<&'static str>
}

impl Default for HasSemantics{
    fn default() -> Self{
        Self{
            tag_bits: 1,
            table_names: vec!["Event", "Property"]
        }
    }
}

#[derive(Debug, Clone)]
pub struct MethodDefOrRef{
    pub tag_bits: usize,
    pub table_names: Vec<&'static str>
}

impl Default for MethodDefOrRef{
    fn default() -> Self{
        Self{
            tag_bits: 1,
            table_names: vec!["MethodDef", "MemberRef"]
        }
    }
}

#[derive(Debug, Clone)]
pub struct MemberForwarded{
    pub tag_bits: usize,
    pub table_names: Vec<&'static str>
}

impl Default for MemberForwarded{
    fn default() -> Self{
        Self{
            tag_bits: 1,
            table_names: vec!["Field", "MethodDef"]
        }
    }
}

#[derive(Debug, Clone)]
pub struct Implementation{
    pub tag_bits: usize,
    pub table_names: Vec<&'static str>
}

impl Default for Implementation{
    fn default() -> Self{
        Self{
            tag_bits: 2,
            table_names: vec!["File", "AssemblyRef", "ExportedType"]
        }
    }
}

#[derive(Debug, Clone)]
pub struct TypeOrMethodDef{
    pub tag_bits: usize,
    pub table_names: Vec<&'static str>
}

impl Default for TypeOrMethodDef{
    fn default() -> Self{
        Self{
            tag_bits: 1,
            table_names: vec!["TypeDef", "MethodDef"]
        }
    }
}

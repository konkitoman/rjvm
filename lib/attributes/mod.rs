mod bootstrap_methods;
mod code;
mod exeptions;
mod inner_classes;
mod line_number_table;
mod local_variabile_table;
mod local_variabile_type_table;
mod method_perameters;
mod module;
mod module_package;
mod nest_members;
mod primitted_subclasses;
mod record;
mod runtime_invisible_annotations;
mod runtime_invisible_parameter_annotations;
mod runtime_invisible_type_annotations;
mod runtime_visible_annotations;
mod runtime_visible_parameters_annotations;
mod runtime_visible_type_annotations;
mod stack_map_table;

pub use bootstrap_methods::AttributeBootstrapMethods;
pub use code::AttributeCode;
pub use exeptions::AttributeExeptions;
pub use inner_classes::AttributeInnerClasses;
pub use line_number_table::AttributeLineNumberTable;
pub use local_variabile_table::AttributeLocalVariabileTable;
pub use local_variabile_type_table::AttributeLocalVariabileTypeTable;
pub use method_perameters::AttributeMethodParameters;
pub use module::AttributeModule;
pub use module_package::AttributeModulePackage;
pub use nest_members::AttributeNestMembers;
pub use primitted_subclasses::AttributePrimittedSubclasses;
pub use record::AttributeRecord;
pub use runtime_invisible_annotations::AttributeRuntimeInvisibleAnnotations;
pub use runtime_invisible_parameter_annotations::AttributeRuntimeInvisibleParameterAnnotations;
pub use runtime_invisible_type_annotations::AttributeRuntimeInvisibleTypeAnnotations;
pub use runtime_visible_annotations::AttributeRuntimeVisibleAnnotations;
pub use runtime_visible_parameters_annotations::AttributeRuntimeVisibleParameterAnnotations;
pub use runtime_visible_type_annotations::AttributeRuntimeVisibleTypeAnnotations;
pub use stack_map_table::AttributeStackMapTable;

use crate::{
    constant_info::Constant,
    error::Error,
    stream::{ReadStream, Stream},
};

use self::runtime_visible_annotations::ElementValue;

#[derive(Debug, Clone)]
pub enum Attribute {
    ConstantValue(u16),
    Code(AttributeCode),
    Exeptions(AttributeExeptions),
    SourceFile(u16),
    LineNumberTable(AttributeLineNumberTable),
    LocalVariabileTable(AttributeLocalVariabileTable),
    InnerClasses(AttributeInnerClasses),
    Synthetic,
    Depricated,
    EnclosingMethod(u16, u16),
    Signature(u16),
    SourceDebugExtension(Vec<u8>),
    LocalVariabileTypeTable(AttributeLocalVariabileTypeTable),
    RuntimeVisibleAnnotations(AttributeRuntimeVisibleAnnotations),
    RuntimeInvisibleAnnotations(AttributeRuntimeInvisibleAnnotations),
    RuntimeVisibleParameterAnnotations(AttributeRuntimeVisibleParameterAnnotations),
    RuntimeInvisibleParameterAnnotations(AttributeRuntimeInvisibleParameterAnnotations),
    AnnotationDefault(ElementValue),
    StackMapTable(AttributeStackMapTable),
    BootstrapMethods(AttributeBootstrapMethods),
    RuntimeVisibleTypeAnnotations(AttributeRuntimeVisibleTypeAnnotations),
    RuntimeInvisibleTypeAnnotations(AttributeRuntimeInvisibleTypeAnnotations),
    MethodParameters(AttributeMethodParameters),
    Module(AttributeModule),
    ModulePackages(AttributeModulePackage),
    ModuleMainClass(u16),
    NestHost(u16),
    NestMembers(AttributeNestMembers),
    Record(AttributeRecord),
    PermittedSubClass(AttributePrimittedSubclasses),
    Row(Vec<u8>, String),
}

impl Attribute {
    pub fn new(
        stream: &mut Stream,
        constant_pool: &[Constant],
        name_index: u16,
        length: u32,
    ) -> Result<Self, Error> {
        if let Constant::Utf8(name) = constant_pool[name_index as usize - 1].clone() {
            Ok(match name.trim() {
                "ConstantValue" => Self::ConstantValue(stream.read_u16()?),
                "Code" => Self::Code(AttributeCode::new(stream, constant_pool)?),
                "Exceptions" => Self::Exeptions(AttributeExeptions::new(stream)?),
                "SourceFile" => Self::SourceFile(stream.read_u16()?),
                "LineNumberTable" => Self::LineNumberTable(AttributeLineNumberTable::new(stream)?),
                "LocalVariableTable" => {
                    Self::LocalVariabileTable(AttributeLocalVariabileTable::new(stream)?)
                }
                "InnerClasses" => Self::InnerClasses(AttributeInnerClasses::new(stream)?),
                "Synthetic" => Self::Synthetic,
                "Deprecated" => Self::Depricated,
                "EnclosingMethod" => Self::EnclosingMethod(stream.read_u16()?, stream.read_u16()?),
                "Signature" => Self::Signature(stream.read_u16()?),
                "SourceDebugExtension" => {
                    let mut extensions = Vec::with_capacity(length as usize);
                    for _ in 0..length {
                        extensions.push(stream.read_u8()?)
                    }
                    Self::SourceDebugExtension(extensions)
                }
                "LocalVariableTypeTable" => {
                    Self::LocalVariabileTypeTable(AttributeLocalVariabileTypeTable::new(stream)?)
                }
                "RuntimeVisibleAnnotations" => Self::RuntimeVisibleAnnotations(
                    AttributeRuntimeVisibleAnnotations::new(stream)?,
                ),
                "RuntimeInvisibleAnnotations" => Self::RuntimeInvisibleAnnotations(
                    AttributeRuntimeInvisibleAnnotations::new(stream)?,
                ),
                "RuntimeVisibleParameterAnnotations" => Self::RuntimeVisibleParameterAnnotations(
                    AttributeRuntimeVisibleParameterAnnotations::new(stream)?,
                ),
                "RuntimeInvisibleParameterAnnotations" => {
                    Self::RuntimeInvisibleParameterAnnotations(
                        AttributeRuntimeInvisibleParameterAnnotations::new(stream)?,
                    )
                }
                "AnnotationDefault" => Self::AnnotationDefault(ElementValue::new(stream)?),
                "StackMapTable" => Self::StackMapTable(AttributeStackMapTable::new(stream)?),
                "BootstrapMethods" => {
                    Self::BootstrapMethods(AttributeBootstrapMethods::new(stream)?)
                }
                "RuntimeVisibleTypeAnnotations" => Self::RuntimeVisibleTypeAnnotations(
                    AttributeRuntimeVisibleTypeAnnotations::new(stream)?,
                ),
                "RuntimeInvisibleTypeAnnotations" => Self::RuntimeInvisibleTypeAnnotations(
                    AttributeRuntimeInvisibleTypeAnnotations::new(stream)?,
                ),
                "MethodParameters" => {
                    Self::MethodParameters(AttributeMethodParameters::new(stream)?)
                }
                "Module" => Self::Module(AttributeModule::new(stream)?),
                "ModulePackages" => Self::ModulePackages(AttributeModulePackage::new(stream)?),
                "ModuleMainClass" => Self::ModuleMainClass(stream.read_u16()?),
                "NestHost" => Self::NestHost(stream.read_u16()?),
                "NestMembers" => Self::NestMembers(AttributeNestMembers::new(stream)?),
                "Record" => Self::Record(AttributeRecord::new(stream, constant_pool)?),
                "PermittedSubclasses" => {
                    Self::PermittedSubClass(AttributePrimittedSubclasses::new(stream)?)
                }
                _ => {
                    eprintln!("Unimplemented Attribute name: {}", name);
                    let mut row = Vec::with_capacity(length as usize);
                    for _ in 0..length {
                        row.push(stream.read_u8()?)
                    }
                    Self::Row(row, name.clone())
                }
            })
        } else {
            return Err(Error::AttributeDontHaveName);
        }
    }
}

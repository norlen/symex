//! Runner for the symbolic execution engine.
//!
//!
#![warn(missing_docs, rust_2018_idioms)]
use x0001e::{
    ir::{types::NamedStructDef, Type},
    Project,
};

mod display_impls;
mod run;

pub use run::{run, run_project};

/// Result for a single path of execution.
///
/// This contains which path it was, if it succeded or not. If it failed the error will have a
/// stack trace to where the error occured.
///
/// All input variables and variables used in `symbolic` calls will also have solutions available.
#[derive(Debug)]
pub struct PathResult {
    /// Which path this is.
    pub path: usize,

    /// The final value from the path.
    ///
    /// If the path failed the reason vill be in the error. Otherwise there will be a value
    /// unless the analyzed function returned void.
    pub result: PathStatus,

    /// Input variables passed to the analyzed function.
    pub inputs: Vec<Variable>,

    /// Variables explicitly marked as symbolic.
    pub symbolics: Vec<Variable>,
}

/// Status of the path.
///
/// If the path succeeded the return value (if any) is contained in that variant. Otherwise,
/// the reason for failure is contained in the `Failure` variant.
#[derive(Debug, Clone, PartialEq)]
pub enum PathStatus {
    /// The path finished successfully.
    Ok(Option<Variable>),

    /// The path failed.
    Failed(ErrorReason),
}

/// Detailed description of why a run failed.
///
/// Contains the error message, where the error happend and the stack trace from the point of failure.
#[derive(Debug, Clone, PartialEq)]
pub struct ErrorReason {
    /// Error message from the received error.
    pub error_message: String,

    /// For which line in the execution the error was encountered.
    pub error_location: Option<String>,

    /// The stack trace to where the error was encountered.
    ///
    /// The stack trace is in the order of the innermost call to the outermost.
    pub stack_trace: Vec<LineTrace>,
}

/// One line in the stack trace. Contains the name of the function and the line where it occurred.
#[derive(Debug, Clone, PartialEq)]
pub struct LineTrace {
    /// Name of the function.
    pub function_name: String,

    /// Source location if available.
    pub line: Option<String>,
}

/// A concrete solution to a symbol.
#[derive(Debug, Clone, PartialEq)]
pub struct Variable {
    /// `name` is the source name of the variable, if it exists.
    pub name: Option<String>,

    /// The concrete value of the symbol.
    pub value: ConcreteValue,
}

/// A concrete value from a symbol.
///
///
#[derive(Debug, Clone, PartialEq)]
pub enum ConcreteValue {
    /// Integer value of size.
    ///
    /// Note that values are always treated as unsigned integers.
    Value {
        /// The integer value.
        value: u64,
        /// Size in bits of the integer value.
        bits: u32,
    },

    /// Array or vector of values
    Array(Vec<ConcreteValue>),

    /// Structure with fields.
    Struct(Vec<ConcreteValue>),
}

impl ConcreteValue {
    /// Converts a binary string to a usable value.
    ///
    /// Requires the LLVM type to generate fields for structs, and iterate of arrays and vectors.
    fn from_binary_str(binary_str: &str, ty: &Type, project: &Project) -> Self {
        use Type::*;

        match ty {
            IntegerType { bits } => Self::Value {
                value: u64::from_str_radix(binary_str, 2).unwrap(),
                bits: *bits,
            },

            PointerType { .. } => Self::Value {
                value: u64::from_str_radix(binary_str, 2).unwrap(),
                bits: project.ptr_size,
            },

            VectorType {
                element_type,
                num_elements,
                ..
            }
            | ArrayType {
                element_type,
                num_elements,
            } => {
                let el_size = project.bit_size(&element_type).unwrap() as usize;

                let mut elements = Vec::new();
                for i in 0..*num_elements {
                    let start = i * el_size;
                    let end = (i + 1) * el_size;
                    let s = &binary_str[start..end];
                    let element = Self::from_binary_str(s, element_type, project);
                    elements.push(element);
                }

                Self::Array(elements)
            }

            StructType { element_types, .. } => {
                let mut fields = Vec::new();
                let mut current_offset = 0;
                for el_ty in element_types {
                    let size = project.bit_size(&el_ty).unwrap() as usize;
                    let end = current_offset + size;
                    let s = &binary_str[current_offset..end];

                    fields.push(Self::from_binary_str(s, el_ty, project));
                    current_offset = end;
                }

                Self::Struct(fields)
            }

            NamedStructType { name } => match project.get_named_struct(name) {
                Some(named_struct) => match named_struct {
                    NamedStructDef::Opaque => todo!(),
                    NamedStructDef::Defined(ty) => Self::from_binary_str(binary_str, ty, project),
                },
                None => todo!(),
            },

            // TODO
            FPType(_) => todo!(),
            FuncType { .. } => todo!(),
            VoidType => todo!(),
            X86_MMXType => todo!(),
            X86_AMXType => todo!(),
            MetadataType => todo!(),
            LabelType => todo!(),
            TokenType => todo!(),
        }
    }
}

use anyhow::Result;
use llvm_ir::{types::NamedStructDef, Terminator, Type};
use std::path::Path;
use x0001e::{
    common::SolutionVariable, solver::SolutionGenerator, Project, ReturnValue, VMError, VM,
};

/// A concrete value from a symbol.
///
///
#[derive(Debug, Clone, PartialEq)]
pub enum ConcreteValue {
    /// Integer value of size.
    Value { value: u64, bits: u32 },

    /// Array or vector of values
    Array { elements: Vec<ConcreteValue> },

    /// Structure with fields.
    Struct { fields: Vec<ConcreteValue> },
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

                Self::Array { elements: elements }
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

                Self::Struct { fields }
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

    pub fn as_u64(&self) -> u64 {
        match self {
            ConcreteValue::Value { value, .. } => *value,
            ConcreteValue::Array { .. } => panic!("Expected u64, got {self:?}"),
            ConcreteValue::Struct { .. } => panic!("Expected u64, got {self:?}"),
        }
    }
}

/// Helper to generate solutions from a list of `SolutionVariable`s.
fn generate_solutions<'a>(
    symbols: impl Iterator<Item = &'a SolutionVariable>,
    cache: &mut SolutionGenerator,
    project: &Project,
) -> Result<Vec<ConcreteValue>> {
    let mut values = Vec::new();

    for symbol in symbols {
        let value = cache.get_solution(&symbol.value)?;
        let value = ConcreteValue::from_binary_str(value.as_01x_str(), symbol.ty.as_ref(), project);
        values.push(value);
    }

    Ok(values)
}

pub struct PathResult {
    pub result: Result<Option<ConcreteValue>, VMError>,

    pub inputs: Vec<ConcreteValue>,

    pub symbolics: Vec<ConcreteValue>,
}

pub fn run(path: impl AsRef<Path>, function: &str) -> Result<Vec<PathResult>> {
    let _ = env_logger::builder().is_test(true).try_init();

    let project = Project::from_path(path)?;
    let mut vm = VM::new(function, &project)?;
    let mut results = Vec::new();

    // Go through all paths.
    while let Some(path_result) = vm.run() {
        // Cache for solutions.
        //
        // Solutions cannot be cached between paths, so instantiate a new one for each path.
        let mut cache = SolutionGenerator::new(vm.solver.clone())?;

        let inputs = generate_solutions(vm.parameters.iter(), &mut cache, &project)?;
        let symbolics = generate_solutions(vm.state.symbols.iter(), &mut cache, &project)?;

        let result = match path_result {
            Ok(return_value) => {
                let return_value = match return_value {
                    ReturnValue::Value(return_value) => {
                        // Try to reconstruct the type based on the last executed instruction.
                        let terminator = &vm.state.current_loc.block.term;
                        let ty = match terminator {
                            Terminator::Ret(instr) => match &instr.return_operand {
                                Some(op) => Some(vm.state.type_of(op)),
                                None => None,
                            },
                            _ => None,
                        };

                        if let Some(ty) = ty {
                            // Solve the return value.
                            let value = cache.get_solution(&return_value)?;
                            let value = ConcreteValue::from_binary_str(
                                value.as_01x_str(),
                                ty.as_ref(),
                                &project,
                            );
                            Some(value)
                        } else {
                            None
                        }
                    }
                    ReturnValue::Void => None,
                };

                Ok(return_value)
            }
            Err(error) => Err(error),
        };

        let path_result = PathResult {
            result,
            inputs,
            symbolics,
        };
        results.push(path_result);
    }

    Ok(results)
}

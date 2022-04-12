//#![allow(unused_variables)]

use std::error::Error;

use anyhow::{anyhow, Result};
use llvm_ir::constant::{self, GetElementPtr};
use llvm_ir::types::{FPType, NamedStructDef, Typed};
use llvm_ir::{Constant, ConstantRef, Operand, Type, TypeRef};
use log::trace;

use crate::memory::BITS_IN_BYTE;
use crate::project::Project;
use crate::vm::{self, State};
use crate::BV;

pub use self::util::{const_to_bv, operand_to_bv};

mod util;

fn to_bytes(size: u64) -> Result<u64> {
    if size % BITS_IN_BYTE as u64 != 0 {
        Err(anyhow!("invalid number of bits"))
    } else {
        Ok(size / 8)
    }
}

pub trait Size {
    fn size(&self, project: &Project) -> Option<u64>;

    fn size_in_bytes(&self, project: &Project) -> Result<Option<u64>, anyhow::Error>;

    fn offset(&self, index: u64, project: &Project) -> Option<(u64, TypeRef)>;

    fn offset_in_bytes(
        &self,
        index: u64,
        project: &Project,
    ) -> Result<Option<(u64, TypeRef)>, anyhow::Error>;

    fn inner_ty(&self, project: &Project) -> Option<TypeRef>;
}

impl Size for Type {
    fn size(&self, project: &Project) -> Option<u64> {
        util::size_in_bits(self, project)
    }

    fn offset(&self, index: u64, project: &Project) -> Option<(u64, TypeRef)> {
        util::get_offset(self, index, project)
    }

    fn inner_ty(&self, project: &Project) -> Option<TypeRef> {
        util::get_inner_type(self, project)
    }

    fn size_in_bytes(&self, project: &Project) -> Result<Option<u64>, anyhow::Error> {
        self.size(project).map(to_bytes).transpose()
    }

    fn offset_in_bytes(
        &self,
        index: u64,
        project: &Project,
    ) -> Result<Option<(u64, TypeRef)>, anyhow::Error> {
        match self.offset(index, project) {
            None => Ok(None),
            Some((size, ty)) => to_bytes(size).map(|s| Some((s, ty))),
        }
    }
}

impl Size for TypeRef {
    fn size(&self, project: &Project) -> Option<u64> {
        util::size_in_bits(self, project)
    }

    fn offset(&self, index: u64, project: &Project) -> Option<(u64, TypeRef)> {
        util::get_offset(self, index, project)
    }

    fn inner_ty(&self, project: &Project) -> Option<TypeRef> {
        util::get_inner_type(self, project)
    }

    fn size_in_bytes(&self, project: &Project) -> Result<Option<u64>, anyhow::Error> {
        self.size(project).map(to_bytes).transpose()
    }

    fn offset_in_bytes(
        &self,
        index: u64,
        project: &Project,
    ) -> Result<Option<(u64, TypeRef)>, anyhow::Error> {
        match self.offset(index, project) {
            None => Ok(None),
            Some((size, ty)) => to_bytes(size).map(|s| Some((s, ty))),
        }
    }
}

// // -----------------------------------------------------------------------------
// // INTO BV
// // -----------------------------------------------------------------------------

// pub trait ToBV {
//     fn to_bv(&self, state: &mut State<'_>) -> Result<BV, anyhow::Error>;
// }

// impl ToBV for Operand {
//     fn to_bv(&self, state: &mut State<'_>) -> Result<BV, anyhow::Error> {
//         operand_to_bv(self, state)
//     }
// }

// impl ToBV for ConstantRef {
//     fn to_bv(&self, state: &mut State<'_>) -> Result<BV, anyhow::Error> {
//         const_to_bv(self, state)
//     }
// }

// // impl ToBV for Constant {
// //     fn to_bv(&self, state: &mut State<'_>) -> Result<BV, anyhow::Error> {
// //         const_to_bv(self, state)
// //     }
// // }

// -----------------------------------------------------------------------------
// INTO CONCRETE
// -----------------------------------------------------------------------------

pub trait AsConcrete<T> {
    fn as_concrete(&self) -> Result<T, vm::VMError>;
}

impl AsConcrete<u64> for Operand {
    fn as_concrete(&self) -> Result<u64, vm::VMError> {
        match self {
            Operand::LocalOperand { .. } => todo!(),
            Operand::ConstantOperand(constant) => constant.as_concrete(),
            Operand::MetadataOperand => todo!(),
        }
    }
}

impl AsConcrete<u64> for ConstantRef {
    fn as_concrete(&self) -> Result<u64, vm::VMError> {
        self.as_ref().as_concrete()
    }
}

impl AsConcrete<u64> for Constant {
    fn as_concrete(&self) -> Result<u64, vm::VMError> {
        util::constant_as_u64(self)
    }
}

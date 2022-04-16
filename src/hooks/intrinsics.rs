//! LLVM support a large number of [intrinsic functions][1] these are not implemented in bitcode.
//! Thus, these all have to be hooks that are implmented in the system.
//!
//! # Status of supported intrinsics
//!
//! ## Standard C/C++ instrinsics
//!
//! - [ ] `llvm.abs.*`
//! - [ ] `llvm.smax.*`
//! - [ ] `llvm.smin.*`
//! - [ ] `llvm.umax.*`
//! - [ ] `llvm.umin.*`
//! - [x] `llvm.memcpy`
//! - [ ] `llvm.memcpy.inline`
//! - [ ] `llvm.memmove`
//! - [ ] `llvm.memset`
//! - [ ] `llvm.sqrt.*`
//! - [ ] `llvm.powi.*`
//! - [ ] `llvm.sin.*`
//! - [ ] `llvm.cos.*`
//! - [ ] `llvm.pow.*`
//! - [ ] `llvm.exp.*`
//! - [ ] `llvm.exp2.*`
//! - [ ] `llvm.log.*`
//! - [ ] `llvm.log10.*`
//! - [ ] `llvm.log2.*`
//! - [ ] `llvm.fma.*`
//! - [ ] `llvm.fabs.*`
//! - [ ] `llvm.minnum.*`
//! - [ ] `llvm.maxnum.*`
//! - [ ] `llvm.minimum.*`
//! - [ ] `llvm.maximum.*`
//! - [ ] `llvm.copysign.*`
//! - [ ] `llvm.floor.*`
//! - [ ] `llvm.ceil.*`
//! - [ ] `llvm.trunc.*`
//! - [ ] `llvm.rint.*`
//! - [ ] `llvm.nearbyint.*`
//! - [ ] `llvm.round.*`
//! - [ ] `llvm.roundeven.*`
//! - [ ] `llvm.lround.*`
//! - [ ] `llvm.llround.*`
//! - [ ] `llvm.lrint.*`
//! - [ ] `llvm.llrint.*`
//!
//! ## Arithmetic with overflow intrinsics
//!
//! - [x] `llvm.sadd.with.overflow.*`
//! - [x] `llvm.uadd.with.overflow.*`
//! - [x] `llvm.ssub.with.overflow.*`
//! - [x] `llvm.usub.with.overflow.*`
//! - [x] `llvm.smul.with.overflow.*`
//! - [x] `llvm.umul.with.overflow.*`
//!
//! ## Saturation arithmetic intrinsics
//!
//! - [x] `llvm.sadd.sat.*`
//! - [x] `llvm.uadd.sat.*`
//! - [ ] `llvm.ssub.sat.*`
//! - [ ] `llvm.usub.sat.*`
//! - [ ] `llvm.sshl.sat.*`
//! - [ ] `llvm.ushl.sat.*`
//!
//! ## General intrinsincs (non-exhaustive)
//!
//! - [ ] `llvm.expect`
//! - [ ] `llvm.expect.with.probability`
//! - [ ] `llvm.assume`
//!
//! [1]: https://llvm.org/docs/LangRef.html#intrinsic-functions
use llvm_ir::Operand;
use radix_trie::Trie;
use std::collections::HashMap;

use crate::{
    hooks::{FnInfo, Hook},
    memory::BITS_IN_BYTE,
    traits::ToValue,
    vm::{Result, ReturnValue, VMError, VM},
};

/// Check if the given name is an LLVM intrinsic.
///
/// Currently it checks that the name starts with `llvm.` which seems like a good approximation.
pub(super) fn is_instrinsic(name: &str) -> bool {
    name.starts_with("llvm.")
}

/// Intrinsic hook storage.
///
/// Keeps track of intrinsics that have only one version such as `llvm.va_start` and those with
/// multiple versions such as `llvm.abs.*` which is valid for multiple bit lengths.
///
/// Internally fixed length name intrinsincs use a `[std::collections::HashMap]` so all lookups are
/// constant time. Variable intrinsic names use a `[radix_trie::Trie]` so lookups are linear time of
/// the retrieved name.
pub(super) struct Intrinsics {
    /// Fixed length intrinsic values, e.g. `llvm.va_start`.
    fixed: HashMap<String, Hook>,

    /// Intrinsics with a suffix such as `llvm.abs.*`.
    ///
    /// Note that the field does not care what the suffix is, it only finds the closest ancestor
    /// (if any).
    variable: Trie<String, Hook>,
}

impl Intrinsics {
    /// Creates a new intrinsic hook storage with all the default intrinsics enabled.
    pub(super) fn new_with_defaults() -> Self {
        let mut s = Self {
            fixed: HashMap::new(),
            variable: Trie::new(),
        };

        // Add fixed intrinsics.

        // Add variable intrinsics.
        s.add_variable("llvm.memcpy.", llvm_memcpy);

        s.add_variable("llvm.sadd.with.overflow.", llvm_sadd_with_overflow);
        s.add_variable("llvm.uadd.with.overflow.", llvm_uadd_with_overflow);
        s.add_variable("llvm.ssub.with.overflow.", llvm_ssub_with_overflow);
        s.add_variable("llvm.usub.with.overflow.", llvm_usub_with_overflow);
        s.add_variable("llvm.smul.with.overflow.", llvm_smul_with_overflow);
        s.add_variable("llvm.umul.with.overflow.", llvm_umul_with_overflow);

        s.add_variable("llvm.sadd.sat.", llvm_sadd_sat);
        s.add_variable("llvm.uadd.sat.", llvm_uadd_sat);

        s.add_variable("llvm.expect.", llvm_expect);

        s
    }

    // /// Add a fixed length intrinsic.
    // fn add_fixed(&mut self, name: impl Into<String>, hook: Hook) {
    //     self.fixed.insert(name.into(), hook);
    // }

    /// Add a variable length intrinsic, e.g. if they support any kind of bidwidth such as
    /// `llvm.abs.*`.
    ///
    /// Note that it matches for the entire prefix, so to add `llvm.abs.*` the name should only be
    /// `llvm.abs.`
    fn add_variable(&mut self, name: impl Into<String>, hook: Hook) {
        self.variable.insert(name.into(), hook);
    }

    /// Returns a reference to the hook of the given name. If the hook cannot be found `None` is
    /// returned.
    ///
    /// It first checks the fixed length names, and if such a name cannot be found it checks the
    /// variable length names.
    pub(super) fn get(&self, name: &str) -> Option<&Hook> {
        self.fixed
            .get(name)
            .or_else(|| self.variable.get_ancestor_value(name))
    }
}

// -------------------------------------------------------------------------------------------------
// Standard C/C++ instrinsics
// -------------------------------------------------------------------------------------------------

/// Copy a block of memory from the source to the destination.
pub fn llvm_memcpy(vm: &mut VM<'_>, f: FnInfo) -> Result<ReturnValue> {
    // Arguments:
    // 1. Pointer to destination.
    // 2. Pointer to source.
    // 3. Integer, number of bytes to copy.
    // 4. Bool, indicates volatile access.
    // The first two arguments can have an optional alignment.
    //
    // The source and destination must either be equal or nonoverlapping. If the length is not a
    // well-defined value the behavior is undefined. Pointers to source and destination should be
    // well-defined is the length is not zero.
    //
    // TODO: What is a `well-defined` value?
    // TODO: Check the isvolatile and the details of volatile operations.
    assert_eq!(f.arguments.len(), 4);

    let (dst, _) = &f.arguments[0];
    let (src, _) = &f.arguments[1];
    let (size, _) = &f.arguments[2];

    let dst = vm.state.get_var(dst)?;
    let src = vm.state.get_var(src)?;
    match size {
        Operand::LocalOperand { name: _, ty: _ } => {
            todo!("Symbolic operand in memcpy not supported yet")
        }
        Operand::ConstantOperand(constant) => {
            let size = constant.to_value()? as u32 * BITS_IN_BYTE;
            println!("memcpy size={}", size);
            let value = vm.state.mem.read(&src, size)?;
            vm.state.mem.write(&dst, value)?;

            Ok(ReturnValue::Void)
        }
        Operand::MetadataOperand => Err(VMError::MalformedInstruction),
    }
}

// -------------------------------------------------------------------------------------------------
// Arithmetic with overflow intrinsics
// -------------------------------------------------------------------------------------------------

/// All the binary operations that check for overflow.
enum BinaryOpOverflow {
    SAdd,
    UAdd,
    SSub,
    USub,
    SMul,
    UMul,
}

fn binary_op_overflow(vm: &mut VM<'_>, f: FnInfo, op: BinaryOpOverflow) -> Result<ReturnValue> {
    assert_eq!(f.arguments.len(), 2);

    let (a0, _) = f.arguments.get(0).unwrap();
    let (a1, _) = f.arguments.get(1).unwrap();

    let a0 = vm.state.get_var(a0)?;
    let a1 = vm.state.get_var(a1)?;

    let (result, overflow) = match op {
        BinaryOpOverflow::SAdd => (a0.add(&a1), a0.saddo(&a1)),
        BinaryOpOverflow::UAdd => (a0.add(&a1), a0.uaddo(&a1)),
        BinaryOpOverflow::SSub => (a0.sub(&a1), a0.ssubo(&a1)),
        BinaryOpOverflow::USub => (a0.sub(&a1), a0.usubo(&a1)),
        BinaryOpOverflow::SMul => (a0.mul(&a1), a0.smulo(&a1)),
        BinaryOpOverflow::UMul => (a0.mul(&a1), a0.umulo(&a1)),
    };
    assert_eq!(overflow.len(), 1);

    let result_with_overflow = overflow.concat(&result);
    Ok(ReturnValue::Value(result_with_overflow))
}

/// Signed addition on any bitwidth, performs a signed addition and indicates whether an overflow
/// occured.
pub fn llvm_sadd_with_overflow(vm: &mut VM<'_>, f: FnInfo) -> Result<ReturnValue> {
    binary_op_overflow(vm, f, BinaryOpOverflow::SAdd)
}

pub fn llvm_uadd_with_overflow(vm: &mut VM<'_>, f: FnInfo) -> Result<ReturnValue> {
    binary_op_overflow(vm, f, BinaryOpOverflow::UAdd)
}

/// Signed subtraction on any bitwidth, performs a signed subtraction and indicates whether an
/// overflow occured.
pub fn llvm_ssub_with_overflow(vm: &mut VM<'_>, f: FnInfo) -> Result<ReturnValue> {
    binary_op_overflow(vm, f, BinaryOpOverflow::SSub)
}

pub fn llvm_usub_with_overflow(vm: &mut VM<'_>, f: FnInfo) -> Result<ReturnValue> {
    binary_op_overflow(vm, f, BinaryOpOverflow::USub)
}

/// Signed multiplication on any bitwidth, performs a signed multiplication and indicates whether
/// an overflow occured.
pub fn llvm_smul_with_overflow(vm: &mut VM<'_>, f: FnInfo) -> Result<ReturnValue> {
    binary_op_overflow(vm, f, BinaryOpOverflow::SMul)
}

pub fn llvm_umul_with_overflow(vm: &mut VM<'_>, f: FnInfo) -> Result<ReturnValue> {
    binary_op_overflow(vm, f, BinaryOpOverflow::UMul)
}

// -------------------------------------------------------------------------------------------------
// Saturation arithmetic intrinsics
// -------------------------------------------------------------------------------------------------

enum BinaryOpSaturate {
    SAdd,
    UAdd,
}

fn binary_op_saturate(vm: &mut VM<'_>, f: FnInfo, op: BinaryOpSaturate) -> Result<ReturnValue> {
    assert_eq!(f.arguments.len(), 2);

    let (a0, _) = f.arguments.get(0).unwrap();
    let (a1, _) = f.arguments.get(1).unwrap();

    let a0 = vm.state.get_var(a0)?;
    let a1 = vm.state.get_var(a1)?;

    let result = match op {
        BinaryOpSaturate::SAdd => a0.uadds(&a1),
        BinaryOpSaturate::UAdd => a0.sadds(&a1),
    };
    Ok(ReturnValue::Value(result))
}

pub fn llvm_uadd_sat(vm: &mut VM<'_>, f: FnInfo) -> Result<ReturnValue> {
    binary_op_saturate(vm, f, BinaryOpSaturate::UAdd)
}
pub fn llvm_sadd_sat(vm: &mut VM<'_>, f: FnInfo) -> Result<ReturnValue> {
    binary_op_saturate(vm, f, BinaryOpSaturate::SAdd)
}

// -------------------------------------------------------------------------------------------------
// General intrinsincs
// -------------------------------------------------------------------------------------------------

pub fn llvm_expect(vm: &mut VM<'_>, f: FnInfo) -> Result<ReturnValue> {
    assert_eq!(f.arguments.len(), 2);
    let (a0, _) = f.arguments.get(0).unwrap();
    let val = vm.state.get_var(a0).unwrap();

    Ok(ReturnValue::Value(val))
}

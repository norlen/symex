//! LLVM support a large number of [intrinsic functions][1] these are not implemented in bitcode.
//! Thus, these all have to be hooks that are implemented in the system.
//!
//! [1]: https://llvm.org/docs/LangRef.html#intrinsic-functions
use llvm_ir::{Type, Value};
use radix_trie::Trie;
use std::collections::HashMap;
use tracing::{debug, trace, warn};

use crate::memory::BITS_IN_BYTE;
use crate::smt::{DExpr, Solutions};
use crate::vm::{binop, bit_size, LLVMExecutorError, PathResult, Result};
use crate::MAX_INTRINSIC_CONCRETIZATIONS;

use super::LLVMExecutor;

/// Check if the given name is an LLVM intrinsic.
///
/// Currently it checks that the name starts with `llvm.` which seems like a good approximation.
pub fn is_intrinsic(name: &str) -> bool {
    name.starts_with("llvm.")
}

pub type Intrinsic = fn(&mut LLVMExecutor<'_>, &[Value]) -> Result<PathResult>;

/// Intrinsic hook storage.
///
/// Keeps track of intrinsics that have only one version such as `llvm.va_start` and those with
/// multiple versions such as `llvm.abs.*` which is valid for multiple bit lengths.
///
/// Internally fixed length name intrinsics use a `[std::collections::HashMap]` so all lookups are
/// constant time. Variable intrinsic names use a `[radix_trie::Trie]` so lookups are linear time of
/// the retrieved name.
#[derive(Clone)]
pub struct Intrinsics {
    /// Fixed length intrinsic values, e.g. `llvm.va_start`.
    fixed: HashMap<String, Intrinsic>,

    /// Intrinsics with a suffix such as `llvm.abs.*`.
    ///
    /// Note that the field does not care what the suffix is, it only finds the closest ancestor
    /// (if any).
    variable: Trie<String, Intrinsic>,
}

impl Intrinsics {
    /// Creates a new intrinsic hook storage with all the default intrinsics enabled.
    pub fn new_with_defaults() -> Self {
        let mut s = Self {
            fixed: HashMap::new(),
            variable: Trie::new(),
        };

        // Add fixed intrinsics.
        s.add_fixed("llvm.assume", llvm_assume);

        // Add variable intrinsics.
        s.add_variable("llvm.memcpy.", llvm_memcpy);
        s.add_variable("llvm.memmove.", llvm_memmove);
        s.add_variable("llvm.memset.", llvm_memset);
        s.add_variable("llvm.umax.", llvm_umax);

        s.add_variable("llvm.sadd.with.overflow.", llvm_sadd_with_overflow);
        s.add_variable("llvm.uadd.with.overflow.", llvm_uadd_with_overflow);
        s.add_variable("llvm.ssub.with.overflow.", llvm_ssub_with_overflow);
        s.add_variable("llvm.usub.with.overflow.", llvm_usub_with_overflow);
        s.add_variable("llvm.smul.with.overflow.", llvm_smul_with_overflow);
        s.add_variable("llvm.umul.with.overflow.", llvm_umul_with_overflow);

        s.add_variable("llvm.sadd.sat.", llvm_sadd_sat);
        s.add_variable("llvm.uadd.sat.", llvm_uadd_sat);
        s.add_variable("llvm.ssub.sat.", llvm_ssub_sat);
        s.add_variable("llvm.usub.sat.", llvm_usub_sat);

        s.add_variable("llvm.expect.", llvm_expect);

        // Temporary.
        s.add_variable("llvm.dbg", noop);
        s.add_variable("llvm.lifetime", noop);
        s.add_variable("llvm.experimental", noop);

        s
    }

    /// Add a fixed length intrinsic.
    fn add_fixed(&mut self, name: impl Into<String>, hook: Intrinsic) {
        self.fixed.insert(name.into(), hook);
    }

    /// Add a variable length intrinsic, e.g. if they support any kind of bit width such as
    /// `llvm.abs.*`.
    ///
    /// Note that it matches for the entire prefix, so to add `llvm.abs.*` the name should only be
    /// `llvm.abs.`
    fn add_variable(&mut self, name: impl Into<String>, hook: Intrinsic) {
        self.variable.insert(name.into(), hook);
    }

    /// Returns a reference to the hook of the given name. If the hook cannot be found `None` is
    /// returned.
    ///
    /// It first checks the fixed length names, and if such a name cannot be found it checks the
    /// variable length names.
    pub fn get(&self, name: &str) -> Option<&Intrinsic> {
        self.fixed
            .get(name)
            .or_else(|| self.variable.get_ancestor_value(name))
    }
}

pub fn noop(_vm: &mut LLVMExecutor<'_>, _args: &[Value]) -> Result<PathResult> {
    Ok(PathResult::Success(None))
}

// -------------------------------------------------------------------------------------------------
// Standard C/C++ intrinsics
// -------------------------------------------------------------------------------------------------

/// Copy a block of memory from the source to the destination.
///
/// Requires that source and destination do not overlap.
pub fn llvm_memcpy(vm: &mut LLVMExecutor<'_>, args: &[Value]) -> Result<PathResult> {
    assert_eq!(args.len(), 4);
    debug!("llvm_memcpy");

    // Arguments: ptr <dest>, ptr <src>, i32/i64 <len>, i1 <isvolatile>
    // 1. Pointer to destination.
    // 2. Pointer to source.
    // 3. Integer, number of bytes to copy.
    // 4. Bool, indicates volatile access.
    let dst = vm.state.get_expr(&args[0])?;
    let src = vm.state.get_expr(&args[1])?;
    let len = vm.state.get_expr(&args[2])?;

    if let Some(len) = len.get_constant() {
        if len > 0 {
            let len = len as u32 * BITS_IN_BYTE;
            let value = vm.state.memory.read(&src, len)?;
            vm.state.memory.write(&dst, value)?;
        } else {
            warn!("memcpy with size 0");
        }
    } else {
        todo!("symbolic length in llvm.memcpy.*");
    }

    Ok(PathResult::Success(None))
}

pub fn llvm_memset(vm: &mut LLVMExecutor<'_>, args: &[Value]) -> Result<PathResult> {
    assert_eq!(args.len(), 4);
    trace!("llvm_memset");

    // Arguments: ptr <dest>, i8 <val>, i32/i64 <len>, i1 <isvolatile>
    // - <dest> Pointer to address to fill.
    // - <val> Byte to to fill with.
    // - <len> Number of bytes to fill.
    // - <isvolatile> Indicates volatile access.
    let dst = vm.state.get_expr(&args[0])?;
    let val = vm.state.get_expr(&args[1])?;
    let len = vm.state.get_expr(&args[2])?;

    assert_eq!(val.len(), BITS_IN_BYTE);
    if let Some(len) = len.get_constant() {
        for byte in 0..len {
            let offset = vm.state.ctx.from_u64(byte, vm.project.ptr_size);
            let addr = dst.add(&offset);

            vm.state.memory.write(&addr, val.clone())?;
        }
    } else {
        todo!("symbolic length in llvm.memset.*");
    }

    Ok(PathResult::Success(None))
}

/// Intrisic to move memory from source to destination.
///
/// Similar to `llvm_memcpy` but `llvm_memmove` allows the two memory locations to overlap.
pub fn llvm_memmove(vm: &mut LLVMExecutor<'_>, args: &[Value]) -> Result<PathResult> {
    assert_eq!(args.len(), 4);
    trace!("llvm_memmove");

    // Arguments: ptr <dest>, ptr <src>, i32/i64 <len>, i1 <isvolatile>
    let dst = vm.state.get_expr(&args[0])?;
    let src = vm.state.get_expr(&args[1])?;
    let len = vm.state.get_expr(&args[2])?;

    // TODO: Not sure about the exact semantics when the locations overlap. So copy the bytes
    // one by one for now.

    let len = match len.get_constant() {
        Some(len) => len,
        None => {
            warn!("symbolic length in llvm.memmove.* is experimental");
            let concretizations = MAX_INTRINSIC_CONCRETIZATIONS;
            let solutions = vm.state.constraints.get_values(&len, concretizations)?;
            let solutions = match solutions {
                Solutions::Exactly(v) => v,
                Solutions::AtLeast(v) => {
                    warn!(
                        "More than {} solutions found for length in memmove",
                        concretizations
                    );
                    v
                }
            };

            let (solution, others) = solutions.split_first().unwrap();

            // Fork other paths.
            for solution in others.iter() {
                let constraint = len._eq(&solution);
                vm.fork(constraint)?;
            }

            // OK, lets go.
            let constraint = len._eq(&solution);
            vm.state.constraints.assert(&constraint);
            solution.get_constant().unwrap() // Know this is constant.
        }
    };

    for i in 0..len {
        let increment = vm.state.ctx.from_u64(i, vm.project.ptr_size);
        let src_addr = src.add(&increment);
        let dst_addr = dst.add(&increment);

        let value = vm.state.memory.read(&src_addr, BITS_IN_BYTE)?;
        vm.state.memory.write(&dst_addr, value)?;
    }

    Ok(PathResult::Success(None))
}

pub fn llvm_umax(vm: &mut LLVMExecutor<'_>, args: &[Value]) -> Result<PathResult> {
    assert_eq!(args.len(), 2);
    trace!("llvm_umax");

    let lhs = &args[0];
    let rhs = &args[1];

    let operation = |lhs: &DExpr, rhs: &DExpr| {
        let condition = lhs.ugt(rhs);
        condition.ite(lhs, rhs)
    };
    let result = binop(&mut vm.state, lhs, rhs, operation)?;

    Ok(PathResult::Success(Some(result)))
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

/// Binary operations that indicate whether an overflow occurred or not.
fn binary_op_overflow(
    vm: &mut LLVMExecutor<'_>,
    args: &[Value],
    op: BinaryOpOverflow,
) -> Result<PathResult> {
    assert_eq!(args.len(), 2);
    let lhs_value = &args[0];
    let rhs_value = &args[1];

    let lhs = vm.state.get_expr(&lhs_value)?;
    let rhs = vm.state.get_expr(&rhs_value)?;

    let operation = |lhs: DExpr, rhs: DExpr| {
        let (result, overflow) = match op {
            BinaryOpOverflow::SAdd => (lhs.add(&rhs), lhs.saddo(&rhs)),
            BinaryOpOverflow::UAdd => (lhs.add(&rhs), lhs.uaddo(&rhs)),
            BinaryOpOverflow::SSub => (lhs.sub(&rhs), lhs.ssubo(&rhs)),
            BinaryOpOverflow::USub => (lhs.sub(&rhs), lhs.usubo(&rhs)),
            BinaryOpOverflow::SMul => (lhs.mul(&rhs), lhs.smulo(&rhs)),
            BinaryOpOverflow::UMul => (lhs.mul(&rhs), lhs.umulo(&rhs)),
        };
        assert_eq!(overflow.len(), 1);
        trace!("result: {result:?}, overflow: {overflow:?}");

        (result, overflow)
    };

    // This has to be special cased a bit compared to regular binary operations.
    //
    // The result type is a struct so {result, overflow} and for vectors this means {<iX res>, <i1>}
    // so the results and overflows have to be appended separately until the final return. Which the
    // regular `binop` does not handle.
    let result = match (lhs_value.ty(), rhs_value.ty()) {
        // For integers just perform the operation.
        (Type::Integer(_), Type::Integer(_)) => {
            let (result, overflow) = operation(lhs, rhs);
            Ok(overflow.concat(&result))
        }

        // For vectors each operation has to be done independently, and the return should be in
        // the format of {results, overflows}.
        (Type::Vector(lhs_ty), Type::Vector(rhs_ty))
            if !(lhs_ty.is_scalable() || rhs_ty.is_scalable()) =>
        {
            assert_eq!(lhs_ty, rhs_ty);
            assert_eq!(lhs_ty.num_elements(), rhs_ty.num_elements());

            let num_elements = lhs_ty.num_elements();
            let bits = bit_size(&lhs_ty.element_type(), vm.project.ptr_size)?;

            // Perform the operation per element and concatenate the result.
            let (results, overflows) = (0..num_elements)
                .map(|i| {
                    let low = i * bits;
                    let high = (i + 1) * bits - 1;
                    let lhs = lhs.slice(low, high);
                    let rhs = rhs.slice(low, high);
                    operation(lhs, rhs)
                })
                .reduce(|(res_acc, overflow_acc), (res, overflow)| {
                    (res.concat(&res_acc), overflow.concat(&overflow_acc))
                })
                .ok_or(LLVMExecutorError::MalformedInstruction)?;

            trace!("results: {results:?}, overflows: {overflows:?}");
            Ok(overflows.concat(&results))
        }

        // These types should not appear in a binary operation.
        _ => Err(LLVMExecutorError::MalformedInstruction),
    }?;

    Ok(PathResult::Success(Some(result)))
}

/// Signed addition on any bit width, performs a signed addition and indicates whether an overflow
/// occurred.
pub fn llvm_sadd_with_overflow(vm: &mut LLVMExecutor<'_>, args: &[Value]) -> Result<PathResult> {
    binary_op_overflow(vm, args, BinaryOpOverflow::SAdd)
}

/// Unsigned addition on any bit width, performs an unsigned addition and indicates whether an
/// overflow occurred.
pub fn llvm_uadd_with_overflow(vm: &mut LLVMExecutor<'_>, args: &[Value]) -> Result<PathResult> {
    binary_op_overflow(vm, args, BinaryOpOverflow::UAdd)
}

/// Signed subtraction on any bit width, performs a signed subtraction and indicates whether an
/// overflow occurred.
pub fn llvm_ssub_with_overflow(vm: &mut LLVMExecutor<'_>, args: &[Value]) -> Result<PathResult> {
    binary_op_overflow(vm, args, BinaryOpOverflow::SSub)
}

/// Unsigned subtraction on any bit width, performs an unsigned subtraction and indicates whether an
/// overflow occurred.
pub fn llvm_usub_with_overflow(vm: &mut LLVMExecutor<'_>, args: &[Value]) -> Result<PathResult> {
    binary_op_overflow(vm, args, BinaryOpOverflow::USub)
}

/// Signed multiplication on any bit width, performs a signed multiplication and indicates whether
/// an overflow occurred.
pub fn llvm_smul_with_overflow(vm: &mut LLVMExecutor<'_>, args: &[Value]) -> Result<PathResult> {
    binary_op_overflow(vm, args, BinaryOpOverflow::SMul)
}

/// Unsigned multiplication on any bit width, performs an unsigned multiplication and indicates
/// whether an overflow occurred.
pub fn llvm_umul_with_overflow(vm: &mut LLVMExecutor<'_>, args: &[Value]) -> Result<PathResult> {
    binary_op_overflow(vm, args, BinaryOpOverflow::UMul)
}

// -------------------------------------------------------------------------------------------------
// Saturation arithmetic intrinsics
// -------------------------------------------------------------------------------------------------

enum BinaryOpSaturate {
    SAdd,
    UAdd,
    SSub,
    USub,
}

fn binary_op_saturate(
    vm: &mut LLVMExecutor<'_>,
    args: &[Value],
    op: BinaryOpSaturate,
) -> Result<PathResult> {
    assert_eq!(args.len(), 2);

    let lhs = &args[0];
    let rhs = &args[1];

    let operation = |lhs: &DExpr, rhs: &DExpr| match op {
        BinaryOpSaturate::UAdd => lhs.uadds(&rhs),
        BinaryOpSaturate::SAdd => lhs.sadds(&rhs),
        BinaryOpSaturate::USub => lhs.usubs(&rhs),
        BinaryOpSaturate::SSub => lhs.ssubs(&rhs),
    };
    let result = binop(&mut vm.state, lhs, rhs, operation)?;

    Ok(PathResult::Success(Some(result)))
}

pub fn llvm_uadd_sat(vm: &mut LLVMExecutor<'_>, args: &[Value]) -> Result<PathResult> {
    binary_op_saturate(vm, args, BinaryOpSaturate::UAdd)
}

pub fn llvm_sadd_sat(vm: &mut LLVMExecutor<'_>, args: &[Value]) -> Result<PathResult> {
    binary_op_saturate(vm, args, BinaryOpSaturate::SAdd)
}

/// Unsigned saturating subtraction on two values.
pub fn llvm_usub_sat(vm: &mut LLVMExecutor<'_>, args: &[Value]) -> Result<PathResult> {
    binary_op_saturate(vm, args, BinaryOpSaturate::USub)
}

/// Signed saturating subtraction on two values.
pub fn llvm_ssub_sat(vm: &mut LLVMExecutor<'_>, args: &[Value]) -> Result<PathResult> {
    binary_op_saturate(vm, args, BinaryOpSaturate::SSub)
}

// -------------------------------------------------------------------------------------------------
// General intrinsics
// -------------------------------------------------------------------------------------------------

pub fn llvm_expect(vm: &mut LLVMExecutor<'_>, args: &[Value]) -> Result<PathResult> {
    assert_eq!(args.len(), 2);
    let val = vm.state.get_expr(&args[0])?;
    Ok(PathResult::Success(Some(val)))
}

pub fn llvm_assume(vm: &mut LLVMExecutor<'_>, args: &[Value]) -> Result<PathResult> {
    assert_eq!(args.len(), 1);

    let condition = vm.state.get_expr(&args[0])?;
    vm.state.constraints.assert(&condition);

    Ok(PathResult::Success(None))
}

#[cfg(test)]
mod tests {
    use crate::{
        smt::DContext,
        vm::{PathResult, Project, VM},
    };

    fn run(fn_name: &str) -> Vec<Option<i64>> {
        // let subscriber = tracing_subscriber::FmtSubscriber::builder()
        //     .with_max_level(tracing::Level::TRACE)
        //     .finish();
        // tracing::subscriber::set_global_default(subscriber)
        //     .expect("setting default subscriber failed");

        let path = format!("tests/unit_tests/intrinsics.bc");
        let project = Box::new(Project::from_path(&path).expect("Failed to created project"));
        let project = Box::leak(project);

        let context = Box::new(DContext::new());
        let context = Box::leak(context);
        let mut vm = VM::new(project, context, fn_name).expect("Failed to create VM");

        let mut path_results = Vec::new();
        while let Some((path_result, state)) = vm.run().expect("Failed to run path") {
            let result = match path_result {
                PathResult::Success(Some(value)) => {
                    let value = state
                        .constraints
                        .get_value(&value)
                        .expect("Failed to get concrete value");
                    let binary_str = value.to_binary_string();
                    Some(u128::from_str_radix(&binary_str, 2).unwrap() as i64)
                }
                PathResult::Success(None) => None,
                PathResult::Failure(error) => {
                    panic!("Did not expect any paths to fail, reason: {error:?}")
                }
                PathResult::Suppress => panic!("Did not expect any paths to be suppressed"),
                PathResult::AssumptionUnsat => panic!("Did not expect any paths to be unsat"),
            };
            path_results.push(result);
        }

        eprintln!("{path_results:x?}");
        path_results
    }

    #[test]
    fn test_memcpy() {
        let res = run("test_memcpy");
        assert_eq!(res.len(), 1);
        assert_eq!(res[0], Some(0x6543fe671234abcd));
    }

    #[test]
    fn test_memmove() {
        let res = run("test_memmove");
        assert_eq!(res.len(), 1);
        assert_eq!(res[0], Some(0x6543fe671234abcd));
    }

    #[test]
    fn test_memmove_overlapping() {
        let res = run("test_memmove_overlapping");
        assert_eq!(res.len(), 1);
        assert_eq!(res[0], Some(0xabcd34abcd34abcd_u64 as i64));
    }

    #[test]
    fn test_memmove_symbolic_len() {
        let res = run("test_memmove_symbolic_len");
        assert_eq!(res.len(), 2);
        let possible_res0 = vec![
            Some(0x00000003_0034abcd_u64 as i64),
            Some(0x00000004_1234abcd_u64 as i64),
        ];
        let possible_res1 = vec![
            Some(0x00000004_1234abcd_u64 as i64),
            Some(0x00000003_0034abcd_u64 as i64),
        ];
        assert!(res == possible_res0 || res == possible_res1);
    }

    #[test]
    fn test_memset() {
        let res = run("test_memset");
        assert_eq!(res.len(), 1);
        assert_eq!(res[0], Some(-6076574517859464245i64)); // 0xababababcbcbcbcb
    }

    #[test]
    fn test_umax() {
        let res = run("test_umax");
        assert_eq!(res.len(), 1);
        assert_eq!(res[0], Some(0xbcef));
    }

    #[test]
    fn test_umax_vec() {
        let res = run("test_umax_vec");
        assert_eq!(res.len(), 1);
        assert_eq!(res[0], Some(0x000043210000bcef));
    }

    #[test]
    fn test_sadd_sat0() {
        let res = run("test_sadd_sat0");
        assert_eq!(res.len(), 1);
        assert_eq!(res[0], Some(3));
    }

    #[test]
    fn test_sadd_sat1() {
        let res = run("test_sadd_sat1");
        assert_eq!(res.len(), 1);
        assert_eq!(res[0], Some(7));
    }

    #[test]
    fn test_sadd_sat2() {
        let res = run("test_sadd_sat2");
        assert_eq!(res.len(), 1);
        assert_eq!(res[0], Some(-2));
    }

    #[test]
    fn test_sadd_sat3() {
        let res = run("test_sadd_sat3");
        assert_eq!(res.len(), 1);
        assert_eq!(res[0], Some(-8));
    }

    #[test]
    fn test_uadd_sat0() {
        let res = run("test_uadd_sat0");
        assert_eq!(res.len(), 1);
        assert_eq!(res[0], Some(3));
    }

    #[test]
    fn test_uadd_sat1() {
        let res = run("test_uadd_sat1");
        assert_eq!(res.len(), 1);
        assert_eq!(res[0], Some(11));
    }

    #[test]
    fn test_uadd_sat2() {
        let res = run("test_uadd_sat2");
        assert_eq!(res.len(), 1);
        assert_eq!(res[0], Some(15));
    }

    #[test]
    fn test_uadd_sat_vec() {
        let res = run("test_uadd_sat_vec");
        assert_eq!(res.len(), 1);
        assert_eq!(res[0], Some(0xfb3));
    }

    #[test]
    fn test_ssub_sat0() {
        let res = run("test_ssub_sat0");
        assert_eq!(res.len(), 1);
        assert_eq!(res[0], Some(1));
    }

    #[test]
    fn test_ssub_sat1() {
        let res = run("test_ssub_sat1");
        assert_eq!(res.len(), 1);
        assert_eq!(res[0], Some(-4));
    }

    #[test]
    fn test_ssub_sat2() {
        let res = run("test_ssub_sat2");
        assert_eq!(res.len(), 1);
        assert_eq!(res[0], Some(-8));
    }

    #[test]
    fn test_ssub_sat3() {
        let res = run("test_ssub_sat3");
        assert_eq!(res.len(), 1);
        assert_eq!(res[0], Some(7));
    }

    #[test]
    fn test_usub_sat0() {
        let res = run("test_usub_sat0");
        assert_eq!(res.len(), 1);
        assert_eq!(res[0], Some(1));
    }

    #[test]
    fn test_usub_sat1() {
        let res = run("test_usub_sat1");
        assert_eq!(res.len(), 1);
        assert_eq!(res[0], Some(0));
    }

    #[test]
    fn test_sadd_with_overflow0() {
        let res = run("test_sadd_with_overflow0");
        assert_eq!(res.len(), 1);
        assert_eq!(res[0], Some(0x0182));
    }

    #[test]
    fn test_sadd_with_overflow1() {
        let res = run("test_sadd_with_overflow1");
        assert_eq!(res.len(), 1);
        assert_eq!(res[0], Some(0x6e));
    }

    #[test]
    fn test_sadd_with_overflow_vec() {
        let res = run("test_sadd_with_overflow_vec");
        assert_eq!(res.len(), 1);
        assert_eq!(res[0], Some(0x5b07e6e82));
    }

    #[test]
    fn test_uadd_with_overflow0() {
        let res = run("test_uadd_with_overflow0");
        assert_eq!(res.len(), 1);
        assert_eq!(res[0], Some(0x0104));
    }

    #[test]
    fn test_uadd_with_overflow1() {
        let res = run("test_uadd_with_overflow1");
        assert_eq!(res.len(), 1);
        assert_eq!(res[0], Some(0x00fa));
    }

    #[test]
    fn test_expect() {
        let res = run("test_expect");
        assert_eq!(res.len(), 1);
        assert_eq!(res[0], Some(100));
    }

    #[test]
    fn test_assume() {
        let res = run("test_assume");
        assert_eq!(res.len(), 1);
        assert_eq!(res[0], Some(5));
    }
}

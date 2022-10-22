mod any;

pub use any::{any, Any};
pub use valid_derive::Validate;

/// Assume the condition.
///
/// Adds a constraint that the passed condition must be true. If the condition can never be true,
/// this will lead to an `Unsat` error.
///
/// # Example
///
/// ```rust
/// # use symex_lib::assume;
/// fn foo(var: i32) -> i32 {
///     // Will add a constraint to the solver for the passed condition.
///     assume(var >= 0);
///     if var < 0 {
///         unreachable!();
///     }
///     var
/// }
/// ```
#[inline(never)]
pub fn assume(condition: bool) {
    unsafe { symex_assume(condition as u8) }
}

/// Creates a new symbolic value for `value`. This removes all constraints.
///
/// This creates a new symbolic variable and assigns overwrites the passed `value`. This must be
/// performed since constraints added to the solver cannot be removed, and the previous value may
/// have constraints associated with it.
///
/// # Example
///
/// ```rust
/// # use symex_lib::symbolic;
/// fn foo() {
///     // This will create a symbol with the constraint that x is 0.
///     let mut x = 0;
///     // Removes all constraints from `x`, letting it be an unconstrained symbol
///     // that can be anything.
///     symbolic(&mut x);
///     if x != 0 {
///         // This path will be found
///     }
/// }
/// ```
#[inline(never)]
pub fn symbolic<T>(value: &mut T) {
    unsafe {
        let size = std::mem::size_of_val(value);
        let ptr = std::mem::transmute(value);
        symex_symbolic(ptr, size as u64);
    }
}

/// Assume the passed value contains a valid representation.
///
/// # Example
///
/// ```rust
/// # use symex_lib::{Validate, valid, symbolic};
/// #[derive(Validate)]
/// enum E {
///     A,
///     B,
/// }
///
/// fn assume_valid() {
///     let mut e = E::A;
///     symbolic(&mut e); // `e` does not necessarily contain a valid discriminant.
///     valid(&e); // only allows `e` to contain valid discriminants.
/// }
/// ```
pub fn valid<T: Valid>(value: &T) {
    assume(value.is_valid());
}

pub trait Valid {
    fn is_valid(&self) -> bool {
        true
    }
}

impl<T> Valid for &T {
    fn is_valid(&self) -> bool {
        true
    }
}

/// Suppresses this path from the executor.
///
/// Note that this affects the completeness of the analysis and can prevent certain errors from
/// being found.
#[inline(never)]
pub fn ignore_path() -> ! {
    unsafe { std::hint::unreachable_unchecked() }
}

// These are implemented as hooks.
extern "C" {
    fn symex_assume(condition: u8);

    fn symex_symbolic(ptr: *mut std::ffi::c_void, size: u64);
}

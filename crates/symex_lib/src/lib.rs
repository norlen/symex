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

// #[inline(never)]
pub fn symbolic<T: Valid>(value: &mut T) {
    symbolic_raw(value);
    assume(value.is_valid());
}

// internal
pub fn symbolic_raw<T>(value: &mut T) {
    unsafe {
        let size = std::mem::size_of_val(value);
        let ptr = std::mem::transmute(value);
        symex_symbolic(ptr, size as u64);
    }
}

pub trait Valid {
    fn is_valid(&self) -> bool {
        true
    }
}

impl<T> Valid for &T where T: Valid {}

impl Valid for bool {}
impl Valid for u8 {}
impl Valid for u16 {}
impl Valid for u32 {}
impl Valid for u64 {}
impl Valid for i8 {}
impl Valid for i16 {}
impl Valid for i32 {}
impl Valid for i64 {}

// These are implemented as hooks.
extern "C" {
    #![allow(dead_code)]

    fn symex_assume(condition: u8);

    fn symex_symbolic(ptr: *mut std::ffi::c_void, size: u64);

    // // Implemented as hook `hooks::assume`.
    // fn symex_assume(condition: bool);

    // // Implemented as hook `hooks::symbolic`.
    // fn symex_symbolic(ptr: *mut std::ffi::c_void, size: usize);
}

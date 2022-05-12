/// Assume the condition.
///
/// Adds a constraint that the passed condition must be true. If the condition can never be true,
/// this will lead to an `Unsat` error.
///
/// # Example
///
/// ```rust
/// # use x0001e::assume;
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
#[allow(unused_variables)]
pub fn assume(condition: bool) {
    // unsafe { x0001e_assume(condition) }
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
/// # use x0001e::symbolic;
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
#[allow(unused_variables)]
pub fn symbolic<T>(value: &mut T) {
    // unsafe {
    //     let size = std::mem::size_of_val(value);
    //     let ptr = std::mem::transmute(value);
    //     x0001e_symbolic(ptr, size);
    // }
}

// These are implemented as hooks.
extern "C" {
    #![allow(dead_code)]
    
    // Implemented as hook `hooks::assume`.
    fn x0001e_assume(condition: bool);

    // Implemented as hook `hooks::symbolic`.
    fn x0001e_symbolic(ptr: *mut std::ffi::c_void, size: usize);
}

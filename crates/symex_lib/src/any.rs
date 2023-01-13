use super::symbolic;

/// Provides the [`any`] operation on the implementor
/// ---
///
/// The any operation should return a new [`symbolic`] value with no prior [`assumptions`](crate::assume) placed on the value
///
/// ### Example implementation
/// ```no_run
/// impl Any for u8 {
///     /// Generates a symbolic value a without any assumption of the value.
///     fn any() -> Self{
///         unsafe{
///             let mut a = core::mem::MaybeUninit::uninit();
///             symex_lib::symbolic(&mut a);
///             a.assume_init()
///         }
///     }
/// }
/// ```
pub trait Any {
    fn any() -> Self;
}

/// Creates a new [`symbolic`] value with no prior assumptions.
/// ---
///
/// The symbolic value returned should have no [`assumptions`](crate::assume`) placed on it after the function returns.
#[inline(never)]
pub fn any<T: Any>() -> T {
    T::any()
}

/// Implements the [`Any`] trait for the specified types.
///
/// The blanket implementation provides a method [`any`] that creates a [`uninit`](core::mem::MaybeUninit) value
/// that is converted to a [`symbolic`] value which is then assumed to be initialized to make the compiler happy.
macro_rules! impl_primitive {
    ( $($type: ty),+) => {
        $(
            impl Any for $type {
                /// Generates a new symbolic value from uninitialized memory
                #[inline(always)]
                fn any() -> Self {
                    unsafe {
                        let mut a = core::mem::MaybeUninit::uninit();
                        symbolic(&mut a);
                        a.assume_init()
                    }
                }
            }
        )+
    };
}

impl_primitive!(bool, u8, u16, u32, u64, u128, usize, i8, i16, i32, i64, i128, isize);

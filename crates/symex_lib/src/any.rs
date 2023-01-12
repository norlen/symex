use super::symbolic;

pub trait Any {
    fn any() -> Self;
}

#[inline(never)]
pub fn any<T: Any>() -> T {
    T::any()
}

macro_rules! blanket_impl {
    ( $type: ty) => {
        impl Any for $type {
            #[inline(always)]
            fn any() -> Self {
                internal_any()
            }
        }
    };
}

blanket_impl!(bool);

blanket_impl!(u8);
blanket_impl!(u16);
blanket_impl!(u32);
blanket_impl!(u64);
blanket_impl!(u128);
blanket_impl!(usize);

blanket_impl!(i8);
blanket_impl!(i16);
blanket_impl!(i32);
blanket_impl!(i64);
blanket_impl!(i128);
blanket_impl!(isize);

fn internal_any<T: Any>() -> T {
    unsafe {
        let mut a = core::mem::MaybeUninit::uninit();
        symbolic(&mut a);
        a.assume_init()
    }
}

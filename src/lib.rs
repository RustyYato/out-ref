#![forbid(missing_docs)]
// #![cfg_attr(not(feature = "std"), no_std)]
#![cfg_attr(not(feature = "std"), no_std)]
#![cfg_attr(feature = "nightly", feature(core_intrinsics, coerce_unsized, unsize, maybe_uninit))]

/*!
 * This crate brings a safe out reference to Rust
 * 
 *  See readme for more info
 */

#[cfg(not(feature = "std"))]
extern crate core as std;

#[cfg(test)]
mod tests;

#[cfg(feature = "nightly")]
use std::mem::MaybeUninit;
use std::marker::PhantomData as PD;
/**
 * An Out Reference, you can only write to this reference using the `set` method
 * and reborrow this reference with the `borrow` method
 */
#[derive(Debug)]
#[repr(transparent)]
pub struct Out<'a, T: ?Sized>(*mut T, PD<&'a mut T>);

impl<'a, T> Out<'a, T> {
    /// To allow writing to the value inside the MaybeUninit
    #[cfg(feature = "nightly")]
    pub fn from_maybe_uninit(maybe_uninit: &mut MaybeUninit<T>) -> Out<'_, T> {
        Out(maybe_uninit.as_mut_ptr(), PD)
    }
}

impl<'a, T: ?Sized> Out<'a, T> {
    /// Create Out from raw ptr
    /// 
    /// Note: will panic if the raw pointer is null
    pub fn from_raw(ptr: *mut T) -> Out<'a, T> {
        if ptr.is_null() {
            panic!("Tried to create a Out reference from a raw pointer")
        }
        Out(ptr, PD)
    }
    /// Create Out from raw ptr
    /// 
    /// Note: the raw pointer must not be null
    pub unsafe fn from_raw_unchecked(ptr: *mut T) -> Out<'a, T> {
        Out(ptr, PD)
    }

    /// Reborrows the `Out` reference
    pub fn borrow(&mut self) -> Out<'_, T> { Out(self.0, PD) }

    /// Convert this `Out` reference into a raw pointer
    pub fn into_raw(self) -> *mut T { self.0 }
}

impl<'a, T> Out<'a, T> {
    /// Set the value behind the `Out` reference without dropping the old value 
    pub fn set(&mut self, value: T) { unsafe { std::ptr::write(self.0, value) } }
}

/// Used to create an Out reference, for all types
pub trait OutMethod {
    /// creates an Out ref
    #[inline(always)]
    fn out(&mut self) -> Out<'_, Self> { Out(self, PD) }

    /// creates an LinearOut ref
    #[inline(always)]
    #[cfg(any(feature = "std", feature = "nightly"))]
    fn linear_out(&mut self) -> LinearOut<'_, Self> { OutMethod::out(self).into_linear() }
}

impl<T: ?Sized> OutMethod for T {}

impl<'a, T: ?Sized> From<&'a mut T> for Out<'a, T> {
    fn from(ptr: &'a mut T) -> Self {
        Out(ptr, PD)
    }
}

#[cfg(any(feature = "std", feature = "nightly"))]
pub use self::linear::*;

#[cfg(any(feature = "std", feature = "nightly"))]
mod linear {
    use super::*;

    /// `LienarOut` represents a linear type that must be written to
    /// exactly once, if this type is not written to it will ***abort the process***
    pub struct LinearOut<'a, T: ?Sized>(Out<'a, T>);

    impl<'a, T> LinearOut<'a, T> {
        /// Set a value to the linear type
        pub fn set(mut self, value: T) {
            self.0.set(value);
            std::mem::forget(self)
        }
    }

    impl<'a, T: ?Sized> Drop for LinearOut<'a, T> {
        fn drop(&mut self) {
            #[cfg(feature = "std")]
            std::process::abort();

            #[cfg(all(
                feature = "nightly",
                not(feature = "std")
            ))]
            unsafe { std::intrinsics::abort(); }
        }
    }

    impl<'a, T: ?Sized> Out<'a, T> {
        /// Convert an `Out` reference into a `LinearOut` reference
        pub fn into_linear(self) -> LinearOut<'a, T> {
            LinearOut(self)
        }
    }

    #[cfg(feature = "nightly")]
    pub use self::nightly::*;

    #[cfg(feature = "nightly")]
    mod nightly {
        use std::ops::CoerceUnsized;
        use std::marker::Unsize;
        use super::*;

        impl<'a, T: Unsize<U>, U: ?Sized> CoerceUnsized<Out<'a, U>> for Out<'a, T> {}
    }
}
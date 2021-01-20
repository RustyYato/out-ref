#![forbid(missing_docs)]
#![cfg_attr(not(feature = "std"), no_std)]
#![cfg_attr(feature = "nightly", feature(coerce_unsized, unsize))]

//! This crate brings out references to Rust, this crate has `no_std` support
//! Out reference *never* read values behind the reference
//!
//! ```rust
//! use out_reference::*;
//!
//! let mut x = 0;
//!
//! let mut out_x: Out<'_, u32> = x.out();
//! out_x.set(10);
//!
//! assert_eq!(x, 10);
//! ```
//!
//! Note that setting a value does not drop the old value,
//! as that would require at least 1 read of the value behind the pointer
//!
//! So, the code below leaks the vector
//! ```rust
//! use out_reference::*;
//!
//! let mut x = vec![0, 1, 2];
//!
//! let mut out_x: Out<'_, Vec<u32>> = x.out();
//! out_x.set(vec![]);
//!
//! assert_eq!(x, vec![]);
//! ```

#[cfg(test)]
mod tests;

use core::mem::MaybeUninit;
use core::marker::PhantomData;

/// An Out Reference, you can only write to this reference using the `set` method
/// and reborrow this reference with the `borrow` method. It isn't safe to read from
/// an `Out` pointer.
#[derive(Debug)]
#[repr(transparent)]
pub struct Out<'a, T: ?Sized>(*mut T, PhantomData<&'a mut T>);

/// Writes a value to the reference without dropping the old value
#[inline(always)]
pub fn write<T>(ptr: &mut T, value: T) {
    Out::from_mut(ptr).set(value)
}

impl<'a, T> Out<'a, T> {
    /// To allow writing to the value inside the MaybeUninit
    #[inline(always)]
    pub fn from_maybe_uninit(maybe_uninit: &mut MaybeUninit<T>) -> Out<'_, T> {
        Out(maybe_uninit.as_mut_ptr(), PhantomData)
    }
}

impl<'a, T: ?Sized> Out<'a, T> {
    /// Create `Out` from exclusive reference
    #[inline(always)]
    pub fn from_mut(value: &'a mut T) -> Self {
        unsafe { Self::from_raw(value) }
    }

    /// Create `Out` from raw pointer
    #[inline(always)]
    pub unsafe fn from_raw(ptr: *mut T) -> Out<'a, T> {
        Self(ptr, PhantomData)
    }

    /// Reborrows the `Out` reference
    #[inline(always)]
    pub fn borrow(&mut self) -> Out<'_, T> { Out(self.0, PhantomData) }

    /// Convert this `Out` reference into a raw pointer
    /// 
    /// see `as_mut_ptr` for safety documentation of the this pointer.
    #[inline(always)]
    pub fn into_raw(self) -> *mut T { self.0 }

    /// Get a raw pointer to the `Out`, it is only safe to write to this pointer
    /// unless specified otherwise by the creator of this `Out` reference
    /// 
    /// i.e. it's safe to read to an `Out<'_, T>` that was created from a `&mut T`
    /// and it's safe to read from a `Out<'_, T>` that was created from a
    /// `&mut MaybeUninit<T>` after it has been initialized.
    #[inline(always)]
    pub fn as_mut_ptr(&mut self) -> *mut T {
        self.0
    }
}

impl<'a, T> Out<'a, T> {
    /// Set the value behind the `Out` reference *without dropping the old value *
    pub fn set(&mut self, value: T) { unsafe { std::ptr::write(self.0, value) } }
}

/// Used to create an Out reference, for all types
pub trait OutMethod {
    /// creates an Out ref
    #[inline(always)]
    fn out(&mut self) -> Out<'_, Self> { Out::from_mut(self) }
}

impl<T: ?Sized> OutMethod for T {}

impl<'a, T: ?Sized> From<&'a mut T> for Out<'a, T> {
    #[inline(always)]
    fn from(ptr: &'a mut T) -> Self {
        Self::from_mut(ptr)
    }
}

impl<'a, T> From<&'a mut MaybeUninit<T>> for Out<'a, T> {
    #[inline(always)]
    fn from(ptr: &'a mut MaybeUninit<T>) -> Self {
        Self::from_maybe_uninit(ptr)
    }
}

#[cfg(feature = "nightly")]
mod nightly {
    use std::ops::CoerceUnsized;
    use std::marker::Unsize;
    use super::*;

    impl<'a, T: Unsize<U>, U: ?Sized> CoerceUnsized<Out<'a, U>> for Out<'a, T> {}
}
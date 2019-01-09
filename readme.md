# out-ref

This crate brings out references to Rust,
this crate has `no_std` support

Out reference *never* read values behind the reference

```rust
use out_ref::*;

let mut x = 0;

let mut out_x = x.out();
out_x.set(10);

assert_eq!(x, 10);
```

But do note that setting a value does not drop the old value, as that would require at least 1 read of the value behind the pointer

So, the code below leaks the vector
```rust
use out_ref::*;

let mut x = vec![0, 1, 2];

let mut out_x = x.out();
out_x.set(vec![]);

assert_eq!(x, vec![]);
```

## Feature flags

### `std`

This is the only default feature flag, and if you turn this off you can only use `LinearOut` with nightly

### `nightly`

This turns enables `LinearOut` and Unsizing Corecions for `Out`, as well as from `MaybeUninit`.

```rust
#![feature(maybe_uninit)]

use std::mem::MaybeUninit;
use out_ref::Out;

let mut x = MaybeUninit::uninitialized();
let mut out_x = Out::from_maybe_uninit(&mut x);

out_x.set(10);

let x = unsafe { x.into_inner() };
assert_eq!(x, 10);
```
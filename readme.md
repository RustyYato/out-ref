# out-ref

This crate brings out references to Rust,
this crate has `no_std` support

Out reference *never* read values behind the reference

```rust
use out_ref::*;

let mut x = 0;

let mut out_x: Out<'_, u32> = x.out();
out_x.set(10);

assert_eq!(x, 10);
```

But do note that setting a value does not drop the old value, as that would require at least 1 read of the value behind the pointer

So, the code below leaks the vector
```rust
use out_ref::*;

let mut x = vec![0, 1, 2];

let mut out_x: Out<'_, Vec<u32>> = x.out();
out_x.set(vec![]);

assert_eq!(x, vec![]);
```

This crate also introduces `LinearOut`, this type must be set before it drops, or else it ***aborts the process***.

```rust
let mut x = 10;
let mut lout_x: LinearOut<'_, u32> = x.linear_out();

lout_x.set(20);

assert_eq!(x, 20);
```

This code will abort the process because `lout_x` was not set
```rust
let mut x = 10;
let mut lout_x: LinearOut<'_, u32> = x.linear_out();
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
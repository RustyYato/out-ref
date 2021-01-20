# out-ref

This crate brings out references to Rust, this crate has `no_std` support
Out reference *never* read values behind the reference

```rust
use out_reference::*;

let mut x = 0;

let mut out_x: Out<'_, u32> = x.out();
out_x.set(10);

assert_eq!(x, 10);
```

Note that setting a value does not drop the old value,
as that would require at least 1 read of the value behind the pointer

So, the code below leaks the vector
```rust
use out_reference::*;

let mut x = vec![0, 1, 2];

let mut out_x: Out<'_, Vec<u32>> = x.out();
out_x.set(vec![]);

assert_eq!(x, vec![]);
```

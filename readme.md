# compile-type-eq

This crate help you check type equality at compile time.

Example of usage:

```rust
use compile_type_eq::*;

assert_types_eq::<i32, i32>();
assert_types_not_eq::<i32, u32, _>();
```

This will fail:
```rust
use compile_type_eq::*;

assert_types_eq::<i32, u32>();
assert_types_not_eq::<i32, i32, _>();
```
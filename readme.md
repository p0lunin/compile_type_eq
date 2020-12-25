# compile-type-eq
<a href="https://docs.rs/compile_type_eq">
    <img src="https://docs.rs/compile_type_eq/badge.svg">
</a>
<a href="https://crates.io/crates/compile_type_eq">
    <img src="https://img.shields.io/crates/v/compile_type_eq.svg">
</a>

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

For more complicated examples see [examples folder](https://github.com/p0lunin/compile_type_eq/tree/master/examples)
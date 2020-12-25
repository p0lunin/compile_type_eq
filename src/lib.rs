//! This crate help you check type equality at compile time.
//!
//! Usage:
//!
//! ```rust
//! use compile_type_eq::*;
//!
//! assert_types_eq::<i32, i32>();
//! assert_types_not_eq::<i32, u32, _>();
//! ```
//!
//! ```compile_fail
//! use compile_type_eq::*;
//!
//! assert_types_eq::<i32, u32>();
//! assert_types_not_eq::<i32, i32, _>();
//! ```
//!
//! For more complicated examples see [examples folder](https://github.com/p0lunin/compile_type_eq/tree/master/examples)

pub use structs::{CompileEqResult, TypesEq};

pub mod how_it_works;
mod structs;

use structs::TypesNotEq;

/// Trait for compile-time equality checking
pub trait CompileEq<U, Result: CompileEqResult> {}

impl<T> CompileEq<T, TypesEq> for T {}
impl<T, U> CompileEq<U, TypesNotEq> for T {}

/// Checks that the types are eq.
///
/// Examples:
/// ```rust
/// use compile_type_eq::*;
///
/// assert_types_eq::<i32, i32>();
/// ```
///
/// ```compile_fail
/// use compile_type_eq::*;
///
/// assert_types_eq::<i32, u32>();
/// ```
pub fn assert_types_eq<T, U>()
where
    T: CompileEq<U, TypesEq>,
{
}

/// Checks that the types are not eq.
///
/// Examples:
/// ```rust
/// use compile_type_eq::*;
///
/// assert_types_not_eq::<i32, u32, _>();
/// ```
///
/// ```compile_fail
/// use compile_type_eq::*;
///
/// assert_types_not_eq::<i32, i32, _>();
/// ```
pub fn assert_types_not_eq<T, U, BecauseTypesMustBeNotEq>()
where
    T: CompileEq<U, BecauseTypesMustBeNotEq>,
    BecauseTypesMustBeNotEq: CompileEqResult,
{
}

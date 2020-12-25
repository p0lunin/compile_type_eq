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
//! You can use a trait for checking of equality instead:
//! ```rust
//! use compile_type_eq::CompileEq;
//!
//! fn do_some_when_not_eq<T, U, BecauseTypesMustBeNotEq>(t: T, u: U) -> String
//! where
//!     T: CompileEq<U, BecauseTypesMustBeNotEq>
//! {
//!     "Some".to_string()
//! }
//!
//! do_some_when_not_eq(1, "a");
//! ```
//!
//! ```compile_fail
//! use compile_type_eq::CompileEq;
//!
//! fn do_some_when_not_eq<T, U, BecauseTypesMustBeNotEq>(t: T, u: U) -> String
//! where
//!     T: CompileEq<U, BecauseTypesMustBeNotEq>
//! {
//!     "Some".to_string()
//! }
//!
//! do_some_when_not_eq("a", "b");
//! ```

use crate::structs::{Eq, NotEq};

mod structs;

/// Trait for compile-time equality checking
pub trait CompileEq<U, Inf> {}

impl<T> CompileEq<T, Eq> for T {}
impl<T, U> CompileEq<U, NotEq> for T {}

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
    T: CompileEq<U, Eq>,
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
{
}

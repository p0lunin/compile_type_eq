//! Learn how comparison works.
//!
//! Core of library is following lines:
//! ```rust
//! struct NotEq;
//! struct Eq;
//!
//! trait CompileEq<T, Result> {}
//! impl<T> CompileEq<T, Eq> for T {}
//! impl<T, U> CompileEq<U, NotEq> for T {}
//! ```
//!
//! Than we creating a function for checking:
//! ```no_checking
//! fn assert_types_not_eq<T, U, Proof>()
//! where
//!     T: CompileEq<U, Proof>,
//! { }
//! ```
//! Why it works? Suppose we are passing to a function two different types `T` and `U`. Let's go find
//! implementations of `CompileEq` for `T`, as if we were a compiler. We can find only one
//! implementation: `impl<T, U> CompileEq<U, NotEq> for T {}`. Because there are no other
//! candidats, compiler substite this implementation.
//!
//! Than suppose that we have passed the same type `T` to a function twice. Let's go find
//! implementations of `CompileEq` for `T`, as if we were a compiler. We can find *two*
//! implementations: `impl<T> CompileEq<T, Eq> for T {}` and
//! `impl<T, U> CompileEq<U, NotEq> for T {}`. Because there are two different impls, the compiler
//! generates a type inference error.

/// Use this struct when you want to check that two types are equal.
///
/// Examples:
/// ```rust
/// use compile_type_eq::*;
///
/// fn foo<T, U>() where T: CompileEq<U, TypesEq> {}
/// ```
pub struct TypesEq;
pub struct TypesNotEq;

pub struct Sealed;

/// Trait is used to be affirmative that there are no external implementations for `CompileEq`.
pub trait CompileEqResult {
    fn assert_no_external_implements(_: Sealed);
}

impl CompileEqResult for TypesEq {
    fn assert_no_external_implements(_: Sealed) {}
}
impl CompileEqResult for TypesNotEq {
    fn assert_no_external_implements(_: Sealed) {}
}

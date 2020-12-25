use compile_type_eq::{CompileEq, CompileEqResult};

struct Barbarians;
struct Greeks;
struct Romans;

trait War<Other, BecauseWarriorsMustBeFromDifferentCountries> {
    fn battle(self, other: Other);
}
impl<T, U, Proof> War<U, Proof> for T
where
    T: CompileEq<U, Proof>,
    Proof: CompileEqResult,
{
    fn battle(self, _: U) {}
}

fn main() {
    Barbarians.battle(Greeks); // ok
    Barbarians.battle(Romans); // ok
    Romans.battle(Greeks); // ok
    Greeks.battle(Greeks); // error
}

use compile_type_eq::{CompileEq, CompileEqResult};

struct Barbarians;
struct Greeks;
struct Romans;

fn battle<Attackers, Defenders, BecauseWarriorsMustBeFromDifferentCountries>(
    _attackers: Attackers,
    _defenders: Defenders,
) where
    Attackers: CompileEq<Defenders, BecauseWarriorsMustBeFromDifferentCountries>,
    BecauseWarriorsMustBeFromDifferentCountries: CompileEqResult,
{
}

fn main() {
    battle(Barbarians, Greeks); // ok
    battle(Barbarians, Romans); // ok
    battle(Romans, Greeks); // ok
    battle(Greeks, Greeks); // error
}

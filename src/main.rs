mod wrappers;
mod interpreter;
mod stdfuncs;

use interpreter::*;

fn main() {
    let mut interpreter = TinInterpreter::new();

    //let program = interpreter.parse("|→n(.nι``.n%∀1.n>)∀←n|→|ℙ| (10ι{ℙ}) $");
    let program = interpreter.parse("|!1<?⟨⊲!⊲∇↶∇+⟩|→|F| (25ι{F})$");
    let mut stack = vec!();

    interpreter.execute(&program, Option::None, &mut stack);
}

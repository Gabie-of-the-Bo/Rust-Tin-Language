mod wrappers;
mod interpreter;
mod stdfuncs;

use interpreter::*;

fn main() {
    let mut interpreter = TinInterpreter::new();

    let program = interpreter.parse("997 →n(.nι``.n%∀1.n>)∀←n $");
    //let program = interpreter.parse("|◊⟨!!⊲∇·→n⟩:⟨1→n⟩.n←n|→|F|");

    let mut stack = vec!();

    interpreter.execute(&program, &mut stack);
}

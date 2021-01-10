mod wrappers;
mod interpreter;
mod stdfuncs;
mod tests;

use interpreter::*;

fn main() {
    let mut interpreter = TinInterpreter::new();
    
    let program = interpreter.parse("25 !!→n1<?⟨2ι→r ⊲ι{(.r1↓ .r∑)→r}.r1↓→n⟩.n←n $");
    let mut stack = vec!();

    interpreter.execute(&program, Option::None, &mut stack);
}

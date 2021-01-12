use tin::interpreter::*;

#[cfg(not(tarpaulin_include))]
fn main(){
    let mut intrp = TinInterpreter::new();

    let program_it = intrp.parse("!!→n1<?⟨2ι→r ⊲ι{(.r1↓ .r∑)→.r}.r1↓→.n⟩.n←r←n");
    let mut stack = vec!(TinValue::INT(0));

    intrp.execute(&program_it, Option::None, &mut stack);

    println!("{:?}", intrp.variables);
}
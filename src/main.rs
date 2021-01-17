use tin::interpreter::*;

#[cfg(not(tarpaulin_include))]
fn main(){
    let mut intrp = TinInterpreter::new();

    let program_it = intrp.parse("→.n .n2↶%◊⟨3.n·⊳→.n⟩:⟨2.n/→.n⟩ .n$ .n1<?⟨.n∇⟩ ←n");
    let mut stack = vec!(TinValue::INT(35));

    intrp.execute(&program_it, Option::None, &mut stack);

    //println!("{:?}", intrp.variables);
}
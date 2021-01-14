use tin::interpreter::*;

#[cfg(not(tarpaulin_include))]
fn main(){
    let mut intrp = TinInterpreter::new();

    let program_it = intrp.parse("(1 1 1 1) 𝔹!∑↶⍴-¬ $");
    let mut stack = vec!();

    intrp.execute(&program_it, Option::None, &mut stack);

    println!("{:?}", intrp.variables);
}
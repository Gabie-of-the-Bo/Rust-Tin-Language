use tin::interpreter::*;

#[cfg(not(tarpaulin_include))]
fn main(){
    let mut intrp = TinInterpreter::new();

    let program_it = intrp.parse("|ι⊳∏|→|F| (ι⊳{F1.0/}2)∑ $");
    let mut stack = vec!(TinValue::INT(10));

    intrp.execute(&program_it, Option::None, &mut stack);

    println!("{:?}", intrp.variables);
}
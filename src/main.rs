use std::time::Instant;

use tin::interpreter::*;

#[cfg(not(tarpaulin_include))]
fn main(){
    let mut intrp = TinInterpreter::new();
    intrp.parallel = true;

    let program_it = intrp.parse("1000ι{1000000ι∑} $");
    let mut stack = vec!();

    let now = Instant::now();

    intrp.execute(&program_it, Option::None, &mut stack);

    println!("{}", now.elapsed().as_secs());

    //println!("{:?}", intrp.variables);
}
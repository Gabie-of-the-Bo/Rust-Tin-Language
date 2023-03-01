use std::time::Instant;

use tin::interpreter::*;
use tin::parallelism;

#[cfg(not(tarpaulin_include))]
fn main(){
    let mut intrp = TinInterpreter::new();

    let program_it = intrp.parse("|⦑!1<?⟨⊲!⊲∇↶∇+⟩⦒|→|F|30ι[F]$").unwrap();

    let now = Instant::now();

    println!("{}", parallelism::get_parallelization());

    for _ in 0..1{
        let mut stack = vec!();
        intrp.execute(&program_it, Option::None, &mut stack).unwrap();
        println!("Stack: {:?}", stack)
    }

    println!("Elapsed time: {:?}", now.elapsed());
}
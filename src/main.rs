use std::time::Instant;
use rand::Rng;

use tin::interpreter::*;
use tin::parallelism;

#[cfg(not(tarpaulin_include))]
fn main(){
    let mut rng = rand::thread_rng();

    let mut intrp = TinInterpreter::new();
    parallelism::set_parallelization(true);

    let program_it = intrp.parse("→n(.n{.n↶#})!⌈º0↓.n↶↓←n");

    let now = Instant::now();

    for _ in 0..10{
        let mut stack = vec!(TinValue::VECTOR((0..20000).map(|_| TinValue::INT(rng.gen_range(0..100))).collect()));
        intrp.execute(&program_it, Option::None, &mut stack);
    }

    println!("{}", now.elapsed().as_millis() as f64 / 10.0);

    //println!("{:?}", intrp.variables);
}
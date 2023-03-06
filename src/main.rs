use std::time::Instant;

use tin::interpreter::*;
use tin::parallelism;

#[cfg(not(tarpaulin_include))]
fn main(){
    let mut intrp = TinInterpreter::new();

    let program_it = intrp.parse("
    |
        !!⨯[!⋮=◊¡:∑]    ∴ Sum of distinct pairs
        !⊂⨝             ∴ Zip with # of appearances
        [⋮2=:¡]         ∴ Keep the ones with 2 appearances
        ![2↷↶#]         ∴ Get appearances in the previous its.
        ⨝[⋮?¡]          ∴ Keep the new ones
        ⌊,              ∴ Add the smallest to the list
    |→|Ut|

    |(1 2)↶ι{¡Ut}|→|U|  ∴ Generator function 

    100U$
    ").unwrap();

    let now = Instant::now();

    println!("{}", parallelism::get_parallelization());

    for _ in 0..1{
        let mut stack = vec!();
        intrp.execute(&program_it, Option::None, &mut stack).unwrap();
        println!("Stack: {:?}", stack)
    }

    println!("Elapsed time: {:?}", now.elapsed());
}
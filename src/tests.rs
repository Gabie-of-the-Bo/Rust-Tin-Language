#[cfg(test)]
mod tests{
    use crate::interpreter::*;

    #[test]
    fn naive_primality_test(){
        fn result(n: i64) -> i64{
            for i in 2..n{
                if n % i == 0 {
                    return 0;
                }
            }
            
            return (n > 1) as i64;
        }

        let mut intrp = TinInterpreter::new();

        let code = "→n(.nι``.n%∀1.n>)∀←n";
        let program = intrp.parse(code);

        for i in 0..1000{
            let mut stack = vec!(TinValue::INT(i));
            intrp.execute(&program, Option::None, &mut stack);
            let correct_res = TinValue::INT(result(i));

            if *stack.last().unwrap() != correct_res {
                panic!(format!("Invalid output for input {}: {} != {}", i, stack.last().unwrap().to_string(), correct_res.to_string()));
            }
        }
    }

    #[test]
    fn recursive_fibonacci(){
        fn result(n: i64) -> i64{
            let mut its = [0, 1];

            if n < 2 {
                return n;
            
            } else {
                for _ in 1..n {
                    its = [its[1], its[0] + its[1]];
                }

                return its[1];
            }
        }

        let mut intrp = TinInterpreter::new();

        let code = "!1<?⟨⊲!⊲∇↶∇+⟩";
        let program = intrp.parse(code);

        for i in 0..20{
            let mut stack = vec!(TinValue::INT(i));
            intrp.execute(&program, Option::None, &mut stack);
            let correct_res = TinValue::INT(result(i));

            if *stack.last().unwrap() != correct_res {
                panic!(format!("Invalid output for input {}: {} != {}", i, stack.last().unwrap().to_string(), correct_res.to_string()));
            }
        }
    }

    #[test]
    fn iterative_fibonacci(){
        fn result(n: i64) -> i64{
            let mut its = [0, 1];

            if n < 2 {
                return n;
            
            } else {
                for _ in 1..n {
                    its = [its[1], its[0] + its[1]];
                }

                return its[1];
            }
        }

        let mut intrp = TinInterpreter::new();

        let code = "!!→n1<?⟨2ι→r ⊲ι{(.r1↓ .r∑)→.r}.r1↓→.n⟩.n←r←n";
        let program = intrp.parse(code);

        for i in 0..45{
            let mut stack = vec!(TinValue::INT(i));
            intrp.execute(&program, Option::None, &mut stack);
            let correct_res = TinValue::INT(result(i));

            if *stack.last().unwrap() != correct_res {
                panic!(format!("Invalid output for input {}: {} != {}", i, stack.last().unwrap().to_string(), correct_res.to_string()));
            }
        }
    }
}
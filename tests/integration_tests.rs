#[cfg(test)]
mod full_programs{
    use std::collections::HashMap;
    use rand::Rng;
    
    use tin::interpreter::*;
    use tin::wrappers;
    
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

        let code = "→n.nι``.n%∀1.n>∧←n";
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
    fn divisor_count(){
        fn result(n: i64) -> i64{
            let mut res = 0;

            for i in 0..n{
                res += (n % (i + 1) == 0) as i64;
            }
            
            return res;
        }

        let mut intrp = TinInterpreter::new();

        let code = "!ι⊳↶%𝔹¬∑";
        let program = intrp.parse(code);

        for i in 0..100{
            let mut stack = vec!(TinValue::INT(i));
            intrp.execute(&program, Option::None, &mut stack);
            let correct_res = TinValue::INT(result(i));

            if *stack.last().unwrap() != correct_res {
                panic!(format!("Invalid output for input {}: {} != {}", i, stack.last().unwrap().to_string(), correct_res.to_string()));
            }
        }
    }

    #[test]
    fn iterative_factorial(){
        fn result(n: i64) -> i64{
            return match n {
                0 => 1,
                n => n * result(n - 1)
            };
        }

        let mut intrp = TinInterpreter::new();

        let code = "ι⊳∏";
        let program = intrp.parse(code);

        for i in 0..20{
            let mut stack = vec!(TinValue::INT(i));
            intrp.execute(&program, Option::None, &mut stack);
            let correct_res = TinValue::INT(result(i));

            assert_eq!(*stack.last().unwrap(), correct_res, 
                        "Invalid output for input {}: {} != {}", i, stack.last().unwrap().to_string(), correct_res.to_string());
        }
    }

    #[test]
    fn e_approximation(){
        fn result(m: i64) -> f64{
            fn fact(n: i64) -> i64{
                return match n {
                    0 => 1,
                    n => n * fact(n - 1)
                };
            }

            return (0..m + 1).map(|i| 1 as f64 / fact(i) as f64).sum();
        }

        let mut intrp = TinInterpreter::new();

        let code = "|ι⊳∏|→|F| (ι⊳{F1.0/}2)∑";
        let program = intrp.parse(code);

        for i in 2..20{
            let mut stack = vec!(TinValue::INT(i));
            intrp.execute(&program, Option::None, &mut stack);
            let correct_res = TinValue::FLOAT(result(i));

            match (stack.last().unwrap(), &correct_res){
                (TinValue::FLOAT(a), TinValue::FLOAT(b)) => {
                    assert!((a - b).abs() < 0.001, "Invalid output for input {}: {} != {}", i, stack.last().unwrap().to_string(), correct_res.to_string());
                },

                _ => unreachable!()
            }
        }
    }

    #[test]
    fn recursive_factorial(){
        fn result(n: i64) -> i64{
            return match n {
                0 => 1,
                n => n * result(n - 1)
            };
        }

        let mut intrp = TinInterpreter::new();

        let code = "◊⟨!!⊲∇·→n⟩:⟨1→n⟩.n←n";
        let program = intrp.parse(code);

        for i in 0..20{
            let mut stack = vec!(TinValue::INT(i));
            intrp.execute(&program, Option::None, &mut stack);
            let correct_res = TinValue::INT(result(i));

            assert_eq!(*stack.last().unwrap(), correct_res, 
                        "Invalid output for input {}: {} != {}", i, stack.last().unwrap().to_string(), correct_res.to_string());
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

            assert_eq!(*stack.last().unwrap(), correct_res, 
                        "Invalid output for input {}: {} != {}", i, stack.last().unwrap().to_string(), correct_res.to_string());
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

            assert_eq!(*stack.last().unwrap(), correct_res, 
                        "Invalid output for input {}: {} != {}", i, stack.last().unwrap().to_string(), correct_res.to_string());
        }
    }

    #[test]
    fn binet_fibonacci(){
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

        let code = "2 5√⊳/^5√↶/.5+⌋";
        let program = intrp.parse(code);

        for i in 0..45{
            let mut stack = vec!(TinValue::INT(i));
            intrp.execute(&program, Option::None, &mut stack);
            let correct_res = TinValue::INT(result(i));

            assert_eq!(*stack.last().unwrap(), correct_res, 
                        "Invalid output for input {}: {} != {}", i, stack.last().unwrap().to_string(), correct_res.to_string());
        }
    }

    #[test]
    fn mean(){
        fn result(v: Vec<TinValue>) -> TinValue{
            let mut res = TinValue::INT(0);
            let count = TinValue::INT(v.len() as i64);

            for i in v{
                res = wrappers::sum(&res, &i);
            }

            return wrappers::div(&res, &count);
        }

        let mut rng = rand::thread_rng();

        let mut intrp = TinInterpreter::new();

        let code = "!⍴↶∑/";
        let program = intrp.parse(code);

        for _ in 0..100{
            let mut v = vec!();

            for _ in 0..100{
                v.push(TinValue::INT(rng.gen_range(0..10)));
            }

            let mut stack = vec!(TinValue::VECTOR(v.clone()));
            intrp.execute(&program, Option::None, &mut stack);
            let correct_res = result(v.clone());

            assert_eq!(*stack.last().unwrap(), correct_res, 
                        "Invalid output for input {}: {} != {}", TinValue::VECTOR(v).to_string(), stack.last().unwrap().to_string(), correct_res.to_string());
        }
    }

    #[test]
    fn variance(){
        fn result(v: Vec<TinValue>) -> TinValue{
            let mut res = TinValue::INT(0);
            let count = TinValue::INT(v.len() as i64);

            for i in &v{
                res = wrappers::sum(&res, &i);
            }

            let mean = wrappers::div(&res, &count);
            res = TinValue::INT(0);

            for i in &v{
                let mut factor = wrappers::sub(&i, &mean);
                factor = wrappers::mul(&factor, &factor);
                res = wrappers::sum(&res, &factor);
            }

            return wrappers::div(&res, &count);
        }

        let mut rng = rand::thread_rng();

        let mut intrp = TinInterpreter::new();

        let code = "!!!⍴↶∑/-2↶^∑↶⍴↶/";
        let program = intrp.parse(code);

        for _ in 0..100{
            let mut v = vec!();

            for _ in 0..100{
                v.push(TinValue::INT(rng.gen_range(0..10)));
            }

            let mut stack = vec!(TinValue::VECTOR(v.clone()));
            intrp.execute(&program, Option::None, &mut stack);
            let correct_res = result(v.clone());

            assert_eq!(*stack.last().unwrap(), correct_res, 
                        "Invalid output for input {}: {} != {}", TinValue::VECTOR(v).to_string(), stack.last().unwrap().to_string(), correct_res.to_string());
        }
    }

    #[test]
    fn mode(){
        fn result(v: Vec<TinValue>) -> TinValue{
            let mut counts = HashMap::new();

            for i in &v {
                if let TinValue::INT(j) = i {
                    counts.entry(j).or_insert(0);
                    *counts.get_mut(&j).unwrap() += 1;
                }
            }

            let max_count = *counts.iter().max_by_key(|t| t.1).unwrap().1;

            for i in &v {
                if let TinValue::INT(j) = i{
                    if *counts.get(&j).unwrap() == max_count {
                        return i.clone();
                    }
                }
            }

            return TinValue::INT(0);
        }

        let mut rng = rand::thread_rng();

        let mut intrp = TinInterpreter::new();

        let code = "→n(.n{.n↶#})!⌈º0↓.n↶↓←n";
        let program = intrp.parse(code);

        for _ in 0..100{
            let mut v = vec!();

            for _ in 0..100{
                v.push(TinValue::INT(rng.gen_range(0..10)));
            }

            let mut stack = vec!(TinValue::VECTOR(v.clone()));
            intrp.execute(&program, Option::None, &mut stack);
            let correct_res = result(v.clone());

            assert_eq!(*stack.last().unwrap(), correct_res, 
                        "Invalid output for input {}: {} != {}", TinValue::VECTOR(v).to_string(), stack.last().unwrap().to_string(), correct_res.to_string());
        }
    }

    #[test]
    fn zero_vector_generation(){
        fn result(n: i64) -> TinValue{
            return TinValue::VECTOR(vec!(TinValue::INT(0); n as usize));
        }

        let mut intrp = TinInterpreter::new();

        let code = "(⊳ι{¡0})";
        let program = intrp.parse(code);

        for i in 1..20{
            let mut stack = vec!(TinValue::INT(i));
            intrp.execute(&program, Option::None, &mut stack);
            let correct_res = result(i);

            assert_eq!(*stack.last().unwrap(), correct_res, 
                        "Invalid output for input {}: {} != {}", i, stack.last().unwrap().to_string(), correct_res.to_string());
        }
    }

    #[test]
    fn min_in_matrix(){
        fn result(v: Vec<TinValue>) -> TinValue{
            let mut res = Option::None;

            for i in v{
                if let TinValue::VECTOR(vv) = i {
                    for j in vv{
                        if res.is_none() {
                            res = Some(j);
                        
                        } else if let TinValue::INT(1) = wrappers::lt(&j, res.as_ref().unwrap()){
                            res = Some(j);
                        }
                    }
                }
            }

            return res.unwrap();
        }

        let mut rng = rand::thread_rng();

        let mut intrp = TinInterpreter::new();

        let code = "(!{{}})⌊↶¡";
        let program = intrp.parse(code);

        for i in 1..20{
            let two = TinValue::INT(2);
            let mut v = vec!();

            for _ in 0..i{
                let mut row = vec!();

                for _ in 0..i{
                    let elem = TinValue::INT(rng.gen_range(0..1000000));
                    row.push(wrappers::div(&elem, &two));
                }

                v.push(TinValue::VECTOR(row));
            }


            let mut stack = vec!(TinValue::VECTOR(v.clone()));
            println!("{}", stack.last().unwrap().to_string());
            intrp.execute(&program, Option::None, &mut stack);
            let correct_res = result(v);

            assert_eq!(*stack.last().unwrap(), correct_res, 
                        "Invalid output for input {}: {} != {}", i, stack.last().unwrap().to_string(), correct_res.to_string());
        }
    }

    #[test]
    fn identity_matrix(){
        fn result(n: i64) -> TinValue{
            let mut v = vec!();

            for i in 0..n {
                let mut row = vec!(TinValue::INT(0); n as usize);
                row[i as usize] = TinValue::INT(1);

                v.push(TinValue::VECTOR(row));
            }

            return TinValue::VECTOR(v);
        }

        let mut intrp = TinInterpreter::new();

        let code = "→n(.nι{.nι!-↶1↶↑})←n";
        let program = intrp.parse(code);

        for i in 1..20{
            let mut stack = vec!(TinValue::INT(i));
            intrp.execute(&program, Option::None, &mut stack);
            let correct_res = result(i);

            assert_eq!(*stack.last().unwrap(), correct_res, 
                        "Invalid output for input {}: {} != {}", i, stack.last().unwrap().to_string(), correct_res.to_string());
        }
    }
}

#[cfg(test)]
mod equivalences{
    use rand::Rng;
    
    use tin::interpreter::*;

    #[test]
    fn all(){
        let mut rng = rand::thread_rng();
        let mut intrp = TinInterpreter::new();

        let codes = vec!(
            "∀",
            "1↶𝔹{∧}",
            "𝔹!∑↶⍴-¬",
            "1→r 𝔹{.r∧→.r}.r ←r",
        ).iter().map(|i| intrp.parse(i)).collect::<Vec<_>>(); 

        let gen = |i, l, s| (0..i).map(move |_| TinValue::VECTOR((0..l).map(|_| TinValue::INT(rng.gen_range(0..s))).collect()));

        for test_data in gen(1000, 10, 10){
            let mut results = vec!();

            for code in &codes{
                let mut stack = vec!(test_data.clone());
                intrp.execute(&code, Option::None, &mut stack);
                results.push(stack.pop().unwrap());
            }

            assert!(results.windows(2).all(|w| w[0] == w[1]), "equality failed for input {}: {:?}", test_data.to_string(), results)
        }
    }

    #[test]
    fn some(){
        let mut rng = rand::thread_rng();
        let mut intrp = TinInterpreter::new();

        let codes = vec!(
            "∃",
            "𝔹∑0<",
            "0↶𝔹{∨}",
        ).iter().map(|i| intrp.parse(i)).collect::<Vec<_>>(); 

        let gen = |i, l, s| (0..i).map(move |_| TinValue::VECTOR((0..l).map(|_| TinValue::INT(rng.gen_range(0..s))).collect()));

        for test_data in gen(1000, 5, 2){
            let mut results = vec!();

            for code in &codes{
                let mut stack = vec!(test_data.clone());
                intrp.execute(&code, Option::None, &mut stack);
                results.push(stack.pop().unwrap());
            }

            assert!(results.windows(2).all(|w| w[0] == w[1]), "equality failed for input {}: {:?}", test_data.to_string(), results)
        }
    }

    #[test]
    fn none(){
        let mut rng = rand::thread_rng();
        let mut intrp = TinInterpreter::new();

        let codes = vec!(
            "∄",
            "∃¬",
            "𝔹∑¬",
            "0↶𝔹{∨}¬",
        ).iter().map(|i| intrp.parse(i)).collect::<Vec<_>>(); 

        let gen = |i, l, s| (0..i).map(move |_| TinValue::VECTOR((0..l).map(|_| TinValue::INT(rng.gen_range(0..s))).collect()));

        for test_data in gen(1000, 5, 2){
            let mut results = vec!();

            for code in &codes{
                let mut stack = vec!(test_data.clone());
                intrp.execute(&code, Option::None, &mut stack);
                results.push(stack.pop().unwrap());
            }

            assert!(results.windows(2).all(|w| w[0] == w[1]), "equality failed for input {}: {:?}", test_data.to_string(), results)
        }
    }

    #[test]
    fn from_index(){
        let mut rng = rand::thread_rng();
        let mut intrp = TinInterpreter::new();

        let codes = vec!(
            "@",
            "→i→v (.i{.v↶↓}) ←i←v"
        ).iter().map(|i| intrp.parse(i)).collect::<Vec<_>>(); 

        let gen = |rng: &mut rand::rngs::ThreadRng| (0..1000).map(|_| TinValue::VECTOR((0..10).map(|_| TinValue::INT(rng.gen_range(0..10))).collect())).collect::<Vec<_>>();

        for test_data in gen(&mut rng).iter().zip(gen(&mut rng)){
            let mut results = vec!();

            for code in &codes{
                let mut stack = vec!(test_data.0.clone(), test_data.1.clone());
                intrp.execute(&code, Option::None, &mut stack);
                results.push(stack.pop().unwrap());
            }

            assert!(results.windows(2).all(|w| w[0] == w[1]), "equality failed for inputs {} {}: {:?}", test_data.0.to_string(), test_data.1.to_string(), results)
        }
    }
}
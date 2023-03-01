#[cfg(test)]
mod system_checks{
    use tin::parallelism;

    #[test]
    fn parallelism_check(){
        let msg = format!("Parallelism: {} (system with {} physical cores)", parallelism::get_parallelization(), *parallelism::CORES);

        println!("\n+{}+", "-".repeat(msg.len() + 2));
        println!("| {} |", msg);
        println!("+{}+\n", "-".repeat(msg.len() + 2));
    }
}

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

        let code = "→n.n√⊳ι``.n%∀1.n>∧←n";
        let program = intrp.parse(code).unwrap();

        for i in (0..1000).chain(10000..10100){
            let mut stack = vec!(TinValue::Int(i));
            intrp.execute(&program, Option::None, &mut stack).unwrap();
            let correct_res = TinValue::Int(result(i));

            if *stack.last().unwrap() != correct_res {
                panic!("Invalid output for input {}: {} != {}", i, stack.last().unwrap().to_string(), correct_res.to_string());
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
        let program = intrp.parse(code).unwrap();

        for i in (0..1000).chain(10000..10100){
            let mut stack = vec!(TinValue::Int(i));
            intrp.execute(&program, Option::None, &mut stack).unwrap();
            let correct_res = TinValue::Int(result(i));

            if *stack.last().unwrap() != correct_res {
                panic!("Invalid output for input {}: {} != {}", i, stack.last().unwrap().to_string(), correct_res.to_string());
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
        let program = intrp.parse(code).unwrap();

        for i in 0..20{
            let mut stack = vec!(TinValue::Int(i));
            intrp.execute(&program, Option::None, &mut stack).unwrap();
            let correct_res = TinValue::Int(result(i));

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

            return (0..m).map(|i| 1 as f64 / fact(i) as f64).sum();
        }

        let mut intrp = TinInterpreter::new();

        let code = "|ι⊳∏|→|F| ι[F1.0/]∑";
        let program = intrp.parse(code).unwrap();

        for i in 2..20{
            let mut stack = vec!(TinValue::Int(i));
            intrp.execute(&program, Option::None, &mut stack).unwrap();
            let correct_res = TinValue::Float(result(i));

            match (stack.last().unwrap(), &correct_res){
                (TinValue::Float(a), TinValue::Float(b)) => {
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
        let program = intrp.parse(code).unwrap();

        for i in 0..20{
            let mut stack = vec!(TinValue::Int(i));
            intrp.execute(&program, Option::None, &mut stack).unwrap();
            let correct_res = TinValue::Int(result(i));

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
        let program = intrp.parse(code).unwrap();

        for i in 0..20{
            let mut stack = vec!(TinValue::Int(i));
            intrp.execute(&program, Option::None, &mut stack).unwrap();
            let correct_res = TinValue::Int(result(i));

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
        let program = intrp.parse(code).unwrap();

        for i in 0..45{
            let mut stack = vec!(TinValue::Int(i));
            intrp.execute(&program, Option::None, &mut stack).unwrap();
            let correct_res = TinValue::Int(result(i));

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
        let program = intrp.parse(code).unwrap();

        for i in 0..45{
            let mut stack = vec!(TinValue::Int(i));
            intrp.execute(&program, Option::None, &mut stack).unwrap();
            let correct_res = TinValue::Int(result(i));

            assert_eq!(*stack.last().unwrap(), correct_res, 
                        "Invalid output for input {}: {} != {}", i, stack.last().unwrap().to_string(), correct_res.to_string());
        }
    }

    #[test]
    fn mean(){
        fn result(v: Vec<TinValue>) -> TinValue{
            let mut res = TinValue::Int(0);
            let count = TinValue::Int(v.len() as i64);

            for i in v{
                res = wrappers::sum(&res, &i);
            }

            return wrappers::div(&res, &count);
        }

        let mut rng = rand::thread_rng();

        let mut intrp = TinInterpreter::new();

        let code = "!⍴↶∑/";
        let program = intrp.parse(code).unwrap();

        for _ in (0..100).chain(10000..10100){
            let mut v = vec!();

            for _ in 0..100{
                v.push(TinValue::Int(rng.gen_range(0..10)));
            }

            let mut stack = vec!(TinValue::Vector(v.clone()));
            intrp.execute(&program, Option::None, &mut stack).unwrap();
            let correct_res = result(v.clone());

            assert_eq!(*stack.last().unwrap(), correct_res, 
                        "Invalid output for input {}: {} != {}", TinValue::Vector(v).to_string(), stack.last().unwrap().to_string(), correct_res.to_string());
        }
    }

    #[test]
    fn variance(){
        fn result(v: Vec<TinValue>) -> TinValue{
            let mut res = TinValue::Int(0);
            let count = TinValue::Int(v.len() as i64);

            for i in &v{
                res = wrappers::sum(&res, &i);
            }

            let mean = wrappers::div(&res, &count);
            res = TinValue::Int(0);

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
        let program = intrp.parse(code).unwrap();

        for _ in 0..100{
            let mut v = vec!();

            for _ in (0..100).chain(10000..10100){
                v.push(TinValue::Int(rng.gen_range(0..10)));
            }

            let mut stack = vec!(TinValue::Vector(v.clone()));
            intrp.execute(&program, Option::None, &mut stack).unwrap();
            let correct_res = result(v.clone());

            assert_eq!(*stack.last().unwrap(), correct_res, 
                        "Invalid output for input {}: {} != {}", TinValue::Vector(v).to_string(), stack.last().unwrap().to_string(), correct_res.to_string());
        }
    }

    #[test]
    fn mode(){
        fn result(v: Vec<TinValue>) -> TinValue{
            let mut counts = HashMap::new();

            for i in &v {
                if let TinValue::Int(j) = i {
                    counts.entry(j).or_insert(0);
                    *counts.get_mut(&j).unwrap() += 1;
                }
            }

            let max_count = *counts.iter().max_by_key(|t| t.1).unwrap().1;

            for i in &v {
                if let TinValue::Int(j) = i{
                    if *counts.get(&j).unwrap() == max_count {
                        return i.clone();
                    }
                }
            }

            return TinValue::Int(0);
        }

        let mut rng = rand::thread_rng();

        let mut intrp = TinInterpreter::new();

        let code = "→n(.n{.n↶#})!⌈º0↓.n↶↓←n";
        let program = intrp.parse(code).unwrap();

        for _ in 0..100{
            let mut v = vec!();

            for _ in (0..100).chain(10000..10010){
                v.push(TinValue::Int(rng.gen_range(0..10)));
            }

            let mut stack = vec!(TinValue::Vector(v.clone()));
            intrp.execute(&program, Option::None, &mut stack).unwrap();
            let correct_res = result(v.clone());

            assert_eq!(*stack.last().unwrap(), correct_res, 
                        "Invalid output for input {}: {} != {}", TinValue::Vector(v).to_string(), stack.last().unwrap().to_string(), correct_res.to_string());
        }
    }

    #[test]
    fn zero_vector_generation(){
        fn result(n: i64) -> TinValue{
            return TinValue::Vector(vec!(TinValue::Int(0); n as usize));
        }

        let mut intrp = TinInterpreter::new();

        let code = "(⊳ι{¡0})";
        let program = intrp.parse(code).unwrap();

        for i in 1..20{
            let mut stack = vec!(TinValue::Int(i));
            intrp.execute(&program, Option::None, &mut stack).unwrap();
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
                if let TinValue::Vector(vv) = i {
                    for j in vv{
                        if res.is_none() {
                            res = Some(j);
                        
                        } else if let TinValue::Int(1) = wrappers::lt(&j, res.as_ref().unwrap()){
                            res = Some(j);
                        }
                    }
                }
            }

            return res.unwrap();
        }

        let mut rng = rand::thread_rng();

        let mut intrp = TinInterpreter::new();

        let code = "[{}]⌊";
        let program = intrp.parse(code).unwrap();

        for i in 1..110{
            let two = TinValue::Int(2);
            let mut v = vec!();

            for _ in 0..i{
                let mut row = vec!();

                for _ in 0..i{
                    let elem = TinValue::Int(rng.gen_range(0..1000000));
                    row.push(wrappers::div(&elem, &two));
                }

                v.push(TinValue::Vector(row));
            }


            let mut stack = vec!(TinValue::Vector(v.clone()));
            println!("{}", stack.last().unwrap().to_string());
            intrp.execute(&program, Option::None, &mut stack).unwrap();
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
                let mut row = vec!(TinValue::Int(0); n as usize);
                row[i as usize] = TinValue::Int(1);

                v.push(TinValue::Vector(row));
            }

            return TinValue::Vector(v);
        }

        let mut intrp = TinInterpreter::new();

        let code = "→n(.nι{.nι!-↶1↶↑})←n";
        let program = intrp.parse(code).unwrap();

        for i in 1..20{
            let mut stack = vec!(TinValue::Int(i));
            intrp.execute(&program, Option::None, &mut stack).unwrap();
            let correct_res = result(i);

            assert_eq!(*stack.last().unwrap(), correct_res, 
                        "Invalid output for input {}: {} != {}", i, stack.last().unwrap().to_string(), correct_res.to_string());
        }
    }

    #[test]
    fn collatz_conjecture(){
        let mut intrp = TinInterpreter::new();

        let code = "→.n .n2↶%◊⟨3.n·⊳→.n⟩:⟨2.n/→.n⟩ .n1<?⟨.n∇⟩ ←n";
        let program = intrp.parse(code).unwrap();

        for i in 1..100{
            let mut stack = vec!(TinValue::Int(i));
            intrp.execute(&program, Option::None, &mut stack).unwrap(); // This only has to execute in order to be correct
        }
    }
}

#[cfg(test)]
mod equivalences{
    use rand::Rng;
    
    use tin::interpreter::*;

    fn generate_vector(length: i64, max_size: i64) -> TinValue{
        let mut rng = rand::thread_rng();
        return TinValue::Vector((0..length).map(|_| TinValue::Int(rng.gen_range(0..max_size))).collect())
    }

    #[test]
    fn all(){
        let mut intrp = TinInterpreter::new();

        let codes = vec!(
            "∀",
            "1↶𝔹{∧}",
            "𝔹!∑↶⍴-¬",
            "1→r 𝔹{.r∧→.r}.r ←r",
        ).iter().map(|i| intrp.parse(i)).collect::<Vec<_>>(); 

        let gen = |i, l, s| (0..i).map(move |_| generate_vector(l, s));

        for test_data in gen(1000, 10, 10).chain(gen(25, 10100, 10000)){
            let mut results = vec!();

            for code in &codes{
                let mut stack = vec!(test_data.clone());
                intrp.execute(&code.as_ref().unwrap(), Option::None, &mut stack).unwrap();
                results.push(stack.pop().unwrap());
            }

            assert!(results.windows(2).all(|w| w[0] == w[1]), "equality failed for input {}: {:?}", test_data.to_string(), results)
        }
    }

    #[test]
    fn some(){
        let mut intrp = TinInterpreter::new();

        let codes = vec!(
            "∃",
            "𝔹∑0<",
            "0↶𝔹{∨}",
        ).iter().map(|i| intrp.parse(i)).collect::<Vec<_>>(); 

        let gen = |i, l, s| (0..i).map(move |_| generate_vector(l, s));

        for test_data in gen(1000, 5, 2).chain(gen(25, 10100, 2)){
            let mut results = vec!();

            for code in &codes{
                let mut stack = vec!(test_data.clone());
                intrp.execute(&code.as_ref().unwrap(), Option::None, &mut stack).unwrap();
                results.push(stack.pop().unwrap());
            }

            assert!(results.windows(2).all(|w| w[0] == w[1]), "equality failed for input {}: {:?}", test_data.to_string(), results)
        }
    }

    #[test]
    fn none(){
        let mut intrp = TinInterpreter::new();

        let codes = vec!(
            "∄",
            "∃¬",
            "𝔹∑¬",
            "0↶𝔹{∨}¬",
        ).iter().map(|i| intrp.parse(i)).collect::<Vec<_>>(); 

        let gen = |i, l, s| (0..i).map(move |_| generate_vector(l, s));

        for test_data in gen(1000, 5, 2).chain(gen(25, 10100, 2)){
            let mut results = vec!();

            for code in &codes{
                let mut stack = vec!(test_data.clone());
                intrp.execute(&code.as_ref().unwrap(), Option::None, &mut stack).unwrap();
                results.push(stack.pop().unwrap());
            }

            assert!(results.windows(2).all(|w| w[0] == w[1]), "equality failed for input {}: {:?}", test_data.to_string(), results)
        }
    }

    #[test]
    fn from_index(){
        let mut intrp = TinInterpreter::new();

        let codes = vec!(
            "@",
            "[*↓]↶¡",
            "→i→v (.i{.v↶↓}) ←i←v"
        ).iter().map(|i| intrp.parse(i)).collect::<Vec<_>>(); 

        let gen = || (0..1000).map(|_| generate_vector(10, 10));

        for test_data in gen().zip(gen()){
            let mut results = vec!();

            for code in &codes{
                let mut stack = vec!(test_data.0.clone(), test_data.1.clone());
                intrp.execute(&code.as_ref().unwrap(), Option::None, &mut stack).unwrap();
                results.push(stack.pop().unwrap());
            }

            assert!(results.windows(2).all(|w| w[0] == w[1]), "equality failed for inputs {} {}: {:?}", test_data.0.to_string(), test_data.1.to_string(), results)
        }
    }

    #[test]
    fn sort_asc(){
        let mut intrp = TinInterpreter::new();

        let codes = vec!(
            "⇑",
            "!.⇑@"
        ).iter().map(|i| intrp.parse(i)).collect::<Vec<_>>(); 

        let gen = |i, l, s| (0..i).map(move |_| generate_vector(l, s));

        for test_data in gen(1000, 100, 1000).chain(gen(25, 10100, 10000)){
            let mut results = vec!();

            for code in &codes{
                let mut stack = vec!(test_data.clone());
                intrp.execute(&code.as_ref().unwrap(), Option::None, &mut stack).unwrap();
                results.push(stack.pop().unwrap());
            }

            assert!(results.windows(2).all(|w| w[0] == w[1]), "equality failed for input {}: {:?}", test_data.to_string(), results)
        }
    }

    #[test]
    fn sort_desc(){
        let mut intrp = TinInterpreter::new();

        let codes = vec!(
            "⇓",
            "!.⇓@"
        ).iter().map(|i| intrp.parse(i)).collect::<Vec<_>>(); 

        let gen = |i, l, s| (0..i).map(move |_| generate_vector(l, s));

        for test_data in gen(1000, 100, 1000).chain(gen(25, 10100, 10000)){
            let mut results = vec!();

            for code in &codes{
                let mut stack = vec!(test_data.clone());
                intrp.execute(&code.as_ref().unwrap(), Option::None, &mut stack).unwrap();
                results.push(stack.pop().unwrap());
            }

            assert!(results.windows(2).all(|w| w[0] == w[1]), "equality failed for input {}: {:?}", test_data.to_string(), results)
        }
    }

    #[test]
    fn counts(){
        let mut intrp = TinInterpreter::new();

        let codes = vec!(
            "⊂",
            "![*#]↶¡",
            "!→n[.n↶#]←n",
            "→n(.n{.n↶#})←n"
        ).iter().map(|i| intrp.parse(i)).collect::<Vec<_>>(); 

        let gen = |i, l, s| (0..i).map(move |_| generate_vector(l, s));

        for test_data in gen(1000, 100, 10){
            let mut results = vec!();

            for code in &codes{
                let mut stack = vec!(test_data.clone());
                intrp.execute(&code.as_ref().unwrap(), Option::None, &mut stack).unwrap();
                results.push(stack.pop().unwrap());
            }

            assert!(results.windows(2).all(|w| w[0] == w[1]), "equality failed for input {}: {:?}", test_data.to_string(), results)
        }
    }

    #[test]
    fn merge(){
        let mut intrp = TinInterpreter::new();

        let codes = vec!(
            "_",
            "{,}",
        ).iter().map(|i| intrp.parse(i)).collect::<Vec<_>>(); 

        let gen = |i, l, s| (0..i).map(move |_| generate_vector(l, s));

        for test_data in gen(1000, 100, 1000).zip(gen(1000, 100, 1000)){
            let mut results = vec!();

            for code in &codes{
                let mut stack = vec!(test_data.0.clone(), test_data.1.clone());
                intrp.execute(&code.as_ref().unwrap(), Option::None, &mut stack).unwrap();
                results.push(stack.pop().unwrap());
            }

            assert!(results.windows(2).all(|w| w[0] == w[1]), "equality failed for inputs {} {}: {:?}", test_data.0.to_string(), test_data.1.to_string(), results)
        }
    }
}
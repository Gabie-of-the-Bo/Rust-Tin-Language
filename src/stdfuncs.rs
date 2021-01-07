use crate::wrappers;
use crate::interpreter::{*};

use regex::Regex;

fn tin_dup(_tok: String, _intrp: &mut TinInterpreter, _prog: &Vec<TinToken>, _prog_parent: Option<&Vec<TinToken>>, _ip: &mut usize, stack: &mut Vec<TinValue>) -> TinValue{
    stack.push(stack.last().cloned().unwrap());

    return TinValue::NONE;
}

fn tin_swap(_tok: String, _intrp: &mut TinInterpreter, _prog: &Vec<TinToken>, _prog_parent: Option<&Vec<TinToken>>, _ip: &mut usize, stack: &mut Vec<TinValue>) -> TinValue{
    let last_index = stack.len() - 1;
    stack.swap(last_index, last_index - 1);
    
    return TinValue::NONE;
}

fn tin_copy(_tok: String, _intrp: &mut TinInterpreter, _prog: &Vec<TinToken>, _prog_parent: Option<&Vec<TinToken>>, _ip: &mut usize, stack: &mut Vec<TinValue>) -> TinValue{
    if let TinValue::INT(n) = stack.pop().unwrap() {
        let item = stack.get(stack.len() - 1 - n as usize).cloned().unwrap();
        stack.push(item);
    
    } else {
        panic!();
    }
    
    return TinValue::NONE;
}

fn tin_define_var(tok: String, intrp: &mut TinInterpreter, _prog: &Vec<TinToken>, _prog_parent: Option<&Vec<TinToken>>, _ip: &mut usize, stack: &mut Vec<TinValue>) -> TinValue{
    let ctx = intrp.variables.entry(tok).or_insert(vec!());
    ctx.push(stack.pop().unwrap());

    return TinValue::NONE;
}

fn tin_delete_var(tok: String, intrp: &mut TinInterpreter, _prog: &Vec<TinToken>, _prog_parent: Option<&Vec<TinToken>>, _ip: &mut usize, _stack: &mut Vec<TinValue>) -> TinValue{
    let ctx = intrp.variables.get_mut(&tok).unwrap();
    ctx.pop();

    if ctx.len() == 0 {
        intrp.variables.remove(&tok);
    }

    return TinValue::NONE;
}

fn tin_get_var(tok: String, intrp: &mut TinInterpreter, _prog: &Vec<TinToken>, _prog_parent: Option<&Vec<TinToken>>, _ip: &mut usize, _stack: &mut Vec<TinValue>) -> TinValue{
    let ctx = &intrp.variables[&tok];

    return ctx.last().cloned().unwrap();
}

fn tin_skip(_tok: String, _intrp: &mut TinInterpreter, _prog: &Vec<TinToken>, _prog_parent: Option<&Vec<TinToken>>, ip: &mut usize, stack: &mut Vec<TinValue>) -> TinValue{
    let last_t = stack.pop().unwrap().truthy();

    if !last_t{
        *ip += 1;
    }

    return TinValue::NONE;
}

fn tin_skip_dup(_tok: String, _intrp: &mut TinInterpreter, _prog: &Vec<TinToken>, _prog_parent: Option<&Vec<TinToken>>, ip: &mut usize, stack: &mut Vec<TinValue>) -> TinValue{
    let last_t = stack.last().as_ref().unwrap().truthy();

    if !last_t{
        *ip += 1;
    }

    return TinValue::NONE;
}

fn tin_skip_inv(_tok: String, _intrp: &mut TinInterpreter, _prog: &Vec<TinToken>, _prog_parent: Option<&Vec<TinToken>>, ip: &mut usize, stack: &mut Vec<TinValue>) -> TinValue{
    let last_t = stack.pop().unwrap().truthy();

    if last_t{
        *ip += 1;
    }

    return TinValue::NONE;
}

fn tin_foreach_init(_tok: String, intrp: &mut TinInterpreter, _prog: &Vec<TinToken>, _prog_parent: Option<&Vec<TinToken>>, ip: &mut usize, stack: &mut Vec<TinValue>) -> TinValue{
    if !intrp.loop_stack.is_empty() && intrp.loop_stack.last().unwrap().0 == *ip{
        intrp.loop_stack.last_mut().unwrap().2 += 1;

    } else{
        match stack.pop().unwrap(){
            TinValue::VECTOR(v) => intrp.loop_stack.push((*ip, v, 0)),

            _ => unreachable!()
        }
    }

    stack.push(intrp.loop_stack.last().unwrap().1[intrp.loop_stack.last().unwrap().2].clone());

    return TinValue::NONE;
}

fn tin_foreach_end(_tok: String, intrp: &mut TinInterpreter, _prog: &Vec<TinToken>, _prog_parent: Option<&Vec<TinToken>>, ip: &mut usize, _stack: &mut Vec<TinValue>) -> TinValue{
    let (pos, arr, idx) = intrp.loop_stack.last().unwrap();

    if *idx < arr.len() - 1 {
        *ip = pos - 1;

    } else{
        intrp.loop_stack.pop();
    }

    return TinValue::NONE;
}

fn tin_storer_init(_tok: String, intrp: &mut TinInterpreter, _prog: &Vec<TinToken>, _prog_parent: Option<&Vec<TinToken>>, _ip: &mut usize, stack: &mut Vec<TinValue>) -> TinValue{
    intrp.storer_stack.push(stack.len());
    return TinValue::NONE;
}

fn tin_storer_end(_tok: String, intrp: &mut TinInterpreter, _prog: &Vec<TinToken>, _prog_parent: Option<&Vec<TinToken>>, _ip: &mut usize, stack: &mut Vec<TinValue>) -> TinValue{
    let idx = intrp.storer_stack.pop().unwrap();
    
    let arr = TinValue::VECTOR(stack.drain(idx..).collect::<Vec<_>>());
    stack.push(arr);

    return TinValue::NONE;
}

fn nabla(_tok: String, intrp: &mut TinInterpreter, prog: &Vec<TinToken>, prog_parent: Option<&Vec<TinToken>>, _ip: &mut usize, stack: &mut Vec<TinValue>) -> TinValue{
    if prog_parent.is_some(){
        intrp.execute(prog_parent.unwrap(), Option::None, stack);

    } else{
        intrp.execute(prog, prog_parent, stack);
    }

    return TinValue::NONE;
}

fn block(tok: String, intrp: &mut TinInterpreter, prog: &Vec<TinToken>, _prog_parent: Option<&Vec<TinToken>>, _ip: &mut usize, stack: &mut Vec<TinValue>) -> TinValue{
    let new_tok = &tok[3..tok.len() - 3];
    let program = intrp.parse(new_tok);
    intrp.execute(&program, Option::Some(prog), stack);

    return TinValue::NONE;
}

fn tin_lt(_tok: String, _intrp: &mut TinInterpreter, _prog: &Vec<TinToken>, _prog_parent: Option<&Vec<TinToken>>, _ip: &mut usize, stack: &mut Vec<TinValue>) -> TinValue{
    let a = stack.pop().unwrap();
    let b = stack.pop().unwrap();

    return wrappers::lt(&a, &b);
}

fn tin_gt(_tok: String, _intrp: &mut TinInterpreter, _prog: &Vec<TinToken>, _prog_parent: Option<&Vec<TinToken>>, _ip: &mut usize, stack: &mut Vec<TinValue>) -> TinValue{
    let a = stack.pop().unwrap();
    let b = stack.pop().unwrap();

    return wrappers::gt(&a, &b);
}

fn tin_sum(_tok: String, _intrp: &mut TinInterpreter, _prog: &Vec<TinToken>, _prog_parent: Option<&Vec<TinToken>>, _ip: &mut usize, stack: &mut Vec<TinValue>) -> TinValue{
    let a = stack.pop().unwrap();
    let b = stack.pop().unwrap();

    return wrappers::sum(&a, &b);
}

fn tin_sub(_tok: String, _intrp: &mut TinInterpreter, _prog: &Vec<TinToken>, _prog_parent: Option<&Vec<TinToken>>, _ip: &mut usize, stack: &mut Vec<TinValue>) -> TinValue{
    let a = stack.pop().unwrap();
    let b = stack.pop().unwrap();

    return wrappers::sub(&a, &b);
}

fn tin_mul(_tok: String, _intrp: &mut TinInterpreter, _prog: &Vec<TinToken>, _prog_parent: Option<&Vec<TinToken>>, _ip: &mut usize, stack: &mut Vec<TinValue>) -> TinValue{
    let a = stack.pop().unwrap();
    let b = stack.pop().unwrap();

    return wrappers::mul(&a, &b);
}

fn tin_div(_tok: String, _intrp: &mut TinInterpreter, _prog: &Vec<TinToken>, _prog_parent: Option<&Vec<TinToken>>, _ip: &mut usize, stack: &mut Vec<TinValue>) -> TinValue{
    let a = stack.pop().unwrap();
    let b = stack.pop().unwrap();

    return wrappers::div(&a, &b);
}

fn tin_mod(_tok: String, _intrp: &mut TinInterpreter, _prog: &Vec<TinToken>, _prog_parent: Option<&Vec<TinToken>>, _ip: &mut usize, stack: &mut Vec<TinValue>) -> TinValue{
    let a = stack.pop().unwrap();
    let b = stack.pop().unwrap();

    return wrappers::modl(&a, &b);
}

fn tin_inc(_tok: String, _intrp: &mut TinInterpreter, _prog: &Vec<TinToken>, _prog_parent: Option<&Vec<TinToken>>, _ip: &mut usize, stack: &mut Vec<TinValue>) -> TinValue{
    let a = stack.pop().unwrap();
    let b = TinValue::INT(1);

    return wrappers::sum(&a, &b);
}

fn tin_dec(_tok: String, _intrp: &mut TinInterpreter, _prog: &Vec<TinToken>, _prog_parent: Option<&Vec<TinToken>>, _ip: &mut usize, stack: &mut Vec<TinValue>) -> TinValue{
    let a = stack.pop().unwrap();
    let b = TinValue::INT(1);

    return wrappers::sub(&a, &b);
}

fn tin_floor(_tok: String, _intrp: &mut TinInterpreter, _prog: &Vec<TinToken>, _prog_parent: Option<&Vec<TinToken>>, _ip: &mut usize, stack: &mut Vec<TinValue>) -> TinValue{
    let v = stack.pop().unwrap();

    return wrappers::floor(&v);
}

fn tin_ceil(_tok: String, _intrp: &mut TinInterpreter, _prog: &Vec<TinToken>, _prog_parent: Option<&Vec<TinToken>>, _ip: &mut usize, stack: &mut Vec<TinValue>) -> TinValue{
    let v = stack.pop().unwrap();

    return wrappers::ceil(&v);
}

fn tin_truthy(_tok: String, _intrp: &mut TinInterpreter, _prog: &Vec<TinToken>, _prog_parent: Option<&Vec<TinToken>>, _ip: &mut usize, stack: &mut Vec<TinValue>) -> TinValue{
    let v = stack.pop().unwrap();

    return wrappers::truthy(&v);
}

fn tin_any(_tok: String, _intrp: &mut TinInterpreter, _prog: &Vec<TinToken>, _prog_parent: Option<&Vec<TinToken>>, _ip: &mut usize, stack: &mut Vec<TinValue>) -> TinValue{
    return match stack.pop().unwrap(){
        TinValue::VECTOR(v) => TinValue::INT(v.iter().any(TinValue::truthy) as i64),

        _ => unreachable!()
    };
}

fn tin_none(_tok: String, _intrp: &mut TinInterpreter, _prog: &Vec<TinToken>, _prog_parent: Option<&Vec<TinToken>>, _ip: &mut usize, stack: &mut Vec<TinValue>) -> TinValue{
    return match stack.pop().unwrap(){
        TinValue::VECTOR(v) => TinValue::INT(!v.iter().any(TinValue::truthy) as i64),

        _ => unreachable!()
    };
}

fn tin_all(_tok: String, _intrp: &mut TinInterpreter, _prog: &Vec<TinToken>, _prog_parent: Option<&Vec<TinToken>>, _ip: &mut usize, stack: &mut Vec<TinValue>) -> TinValue{
    return match stack.pop().unwrap(){
        TinValue::VECTOR(v) => TinValue::INT(v.iter().all(TinValue::truthy) as i64),

        _ => unreachable!()
    };
}

fn iota(_tok: String, _intrp: &mut TinInterpreter, _prog: &Vec<TinToken>, _prog_parent: Option<&Vec<TinToken>>, _ip: &mut usize, stack: &mut Vec<TinValue>) -> TinValue{
    return match stack.pop().unwrap() {
        TinValue::INT(a) => TinValue::VECTOR((0..a).map(TinValue::INT).collect::<Vec<_>>()),
        TinValue::FLOAT(a) => TinValue::VECTOR((0..a as i64).map(TinValue::INT).collect::<Vec<_>>()),

        _ => unreachable!()
    };
}

fn boxed(_tok: String, _intrp: &mut TinInterpreter, _prog: &Vec<TinToken>, _prog_parent: Option<&Vec<TinToken>>, _ip: &mut usize, stack: &mut Vec<TinValue>) -> TinValue{
    return TinValue::VECTOR(vec!(stack.pop().unwrap()));
}

fn set(_tok: String, _intrp: &mut TinInterpreter, _prog: &Vec<TinToken>, _prog_parent: Option<&Vec<TinToken>>, _ip: &mut usize, stack: &mut Vec<TinValue>) -> TinValue{
    let idx = stack.pop().unwrap();
    let elem = stack.pop().unwrap();
    let v = stack.last_mut().unwrap();

    match (idx, v) {
        (TinValue::INT(a), TinValue::VECTOR(v)) => v[a as usize] = elem,

        _ => unreachable!()
    };

    return TinValue::NONE;
}

fn get(_tok: String, _intrp: &mut TinInterpreter, _prog: &Vec<TinToken>, _prog_parent: Option<&Vec<TinToken>>, _ip: &mut usize, stack: &mut Vec<TinValue>) -> TinValue{
    let idx = stack.pop().unwrap();
    let v = stack.pop().unwrap();

    return match (idx, v) {
        (TinValue::INT(a), TinValue::VECTOR(v)) => v[a as usize].clone(),

        _ => unreachable!()
    };
}

fn tin_sum_all(_tok: String, _intrp: &mut TinInterpreter, _prog: &Vec<TinToken>, _prog_parent: Option<&Vec<TinToken>>, _ip: &mut usize, stack: &mut Vec<TinValue>) -> TinValue{
    match stack.pop().unwrap(){
        TinValue::VECTOR(v) => {
            let mut res = TinValue::INT(0);

            for i in v{
                res = wrappers::sum(&res, &i);
            }

            return res;
        },

        _ => unreachable!()
    };
}

fn tin_mul_all(_tok: String, _intrp: &mut TinInterpreter, _prog: &Vec<TinToken>, _prog_parent: Option<&Vec<TinToken>>, _ip: &mut usize, stack: &mut Vec<TinValue>) -> TinValue{
    match stack.pop().unwrap(){
        TinValue::VECTOR(v) => {
            let mut res = TinValue::INT(1);

            for i in v{
                res = wrappers::mul(&res, &i);
            }

            return res;
        },

        _ => unreachable!()
    };
}

fn tin_len(_tok: String, _intrp: &mut TinInterpreter, _prog: &Vec<TinToken>, _prog_parent: Option<&Vec<TinToken>>, _ip: &mut usize, stack: &mut Vec<TinValue>) -> TinValue{
    return match stack.pop().unwrap() {
        TinValue::VECTOR(v) => TinValue::INT(v.len() as i64),

        _ => unreachable!()
    };
}

fn tin_max(_tok: String, _intrp: &mut TinInterpreter, _prog: &Vec<TinToken>, _prog_parent: Option<&Vec<TinToken>>, _ip: &mut usize, stack: &mut Vec<TinValue>) -> TinValue{
    match stack.pop().unwrap() {
        TinValue::VECTOR(v) => {
            let mut v_it = v.iter();
            let mut res = v_it.next().unwrap();

            for i in v_it{
                if let TinValue::INT(1) = wrappers::gt(&i, &res){
                    res = i;
                }
            }

            stack.push(res.clone());
        }

        _ => unreachable!()
    };

    return TinValue::NONE;
}

fn tin_min(_tok: String, _intrp: &mut TinInterpreter, _prog: &Vec<TinToken>, _prog_parent: Option<&Vec<TinToken>>, _ip: &mut usize, stack: &mut Vec<TinValue>) -> TinValue{
    match stack.pop().unwrap() {
        TinValue::VECTOR(v) => {
            let mut v_it = v.iter();
            let mut res = v_it.next().unwrap();

            for i in v_it{
                if let TinValue::INT(1) = wrappers::lt(&i, &res){
                    res = i;
                }
            }

            stack.push(res.clone());
        }

        _ => unreachable!()
    };

    return TinValue::NONE;
}

fn tin_count(_tok: String, _intrp: &mut TinInterpreter, _prog: &Vec<TinToken>, _prog_parent: Option<&Vec<TinToken>>, _ip: &mut usize, stack: &mut Vec<TinValue>) -> TinValue{
    let elem = stack.pop().unwrap();
    
    match stack.pop().unwrap() {
        TinValue::VECTOR(v) => {
            let mut res = 0;

            for i in v{
                if i == elem {
                    res += 1;
                }
            }

            stack.push(TinValue::INT(res));
        }

        _ => unreachable!()
    };

    return TinValue::NONE;
}

fn tin_index(_tok: String, _intrp: &mut TinInterpreter, _prog: &Vec<TinToken>, _prog_parent: Option<&Vec<TinToken>>, _ip: &mut usize, stack: &mut Vec<TinValue>) -> TinValue{
    let elem = stack.pop().unwrap();
    
    match stack.pop().unwrap() {
        TinValue::VECTOR(v) => {
            let mut res = vec!();

            for (idx, i) in v.iter().enumerate(){
                if *i == elem {
                    res.push(TinValue::INT(idx as i64));
                }
            }

            stack.push(TinValue::VECTOR(res));
        }

        _ => unreachable!()
    };

    return TinValue::NONE;
}

fn drop_first(_tok: String, _intrp: &mut TinInterpreter, _prog: &Vec<TinToken>, _prog_parent: Option<&Vec<TinToken>>, _ip: &mut usize, stack: &mut Vec<TinValue>) -> TinValue{
    match stack.last_mut().unwrap() {
        TinValue::VECTOR(v) => {
            if v.len() > 0 {
                v.remove(0);
            }
        },

        _ => unreachable!()
    };

    return TinValue::NONE;
}

fn drop_last(_tok: String, _intrp: &mut TinInterpreter, _prog: &Vec<TinToken>, _prog_parent: Option<&Vec<TinToken>>, _ip: &mut usize, stack: &mut Vec<TinValue>) -> TinValue{
    match stack.last_mut().unwrap() {
        TinValue::VECTOR(v) => {
            if v.len() > 0 {
                v.pop();
            }
        },

        _ => unreachable!()
    };

    return TinValue::NONE;
}

fn tin_print(_tok: String, _intrp: &mut TinInterpreter, _prog: &Vec<TinToken>, _prog_parent: Option<&Vec<TinToken>>, _ip: &mut usize, stack: &mut Vec<TinValue>) -> TinValue{
    println!("{}", stack.pop().unwrap().to_string());

    return TinValue::NONE
}

pub fn std_tin_functions() -> Vec<(Regex, fn(&str) -> TinToken)>{
    let res_str: Vec<(&str, fn(&str) -> TinToken)> = vec!(

        // Literals
        (r"\d+", |s| TinToken::INT(s.parse::<i64>().unwrap())),
        (r"\d*\.\d+", |s| TinToken::FLOAT(s.parse::<f64>().unwrap())),

        // Meta
        (r"!", |s| TinToken::FN(s.to_string(), tin_dup)),
        (r"↶", |s| TinToken::FN(s.to_string(), tin_swap)),
        (r"↷", |s| TinToken::FN(s.to_string(), tin_copy)),

        (r"→[a-z_]+", |s| TinToken::FN(s[3..].to_string(), tin_define_var)),
        (r"←[a-z_]+", |s| TinToken::FN(s[3..].to_string(), tin_delete_var)),
        (r"\.[a-z_]+", |s| TinToken::FN(s[1..].to_string(), tin_get_var)),

        (r"\|[^|]+\|→\|[^|]+\|", |s| TinToken::DEF(s.to_string())),
        (r"⟨[^⟨⟩]+⟩", |s| TinToken::FN(s.to_string(), block)),

        (r"\?", |s| TinToken::FN(s.to_string(), tin_skip)),
        (r"◊", |s| TinToken::FN(s.to_string(), tin_skip_dup)),
        (r":", |s| TinToken::FN(s.to_string(), tin_skip_inv)),
        (r"\{", |s| TinToken::FN(s.to_string(), tin_foreach_init)),
        (r"\}", |s| TinToken::FN(s.to_string(), tin_foreach_end)),
        (r"\(", |s| TinToken::FN(s.to_string(), tin_storer_init)),
        (r"\)", |s| TinToken::FN(s.to_string(), tin_storer_end)),

        (r"∇", |s| TinToken::FN(s.to_string(), nabla)),

        // Basic arithmetic
        (r"\+", |s| TinToken::FN(s.to_string(), tin_sum)),
        (r"\-", |s| TinToken::FN(s.to_string(), tin_sub)),
        (r"·", |s| TinToken::FN(s.to_string(), tin_mul)),
        (r"/", |s| TinToken::FN(s.to_string(), tin_div)),
        (r"%", |s| TinToken::FN(s.to_string(), tin_mod)),

        (r"⊳", |s| TinToken::FN(s.to_string(), tin_inc)),
        (r"⊲", |s| TinToken::FN(s.to_string(), tin_dec)),

        (r"⌉", |s| TinToken::FN(s.to_string(), tin_floor)),
        (r"⌋", |s| TinToken::FN(s.to_string(), tin_ceil)),

        (r"𝔹", |s| TinToken::FN(s.to_string(), tin_truthy)),

        // Logic
        (r"<", |s| TinToken::FN(s.to_string(), tin_lt)),
        (r">", |s| TinToken::FN(s.to_string(), tin_gt)),

        (r"∃", |s| TinToken::FN(s.to_string(), tin_any)),
        (r"∄", |s| TinToken::FN(s.to_string(), tin_none)),
        (r"∀", |s| TinToken::FN(s.to_string(), tin_all)),

        // Array operations
        (r"ι", |s| TinToken::FN(s.to_string(), iota)),
        (r"□", |s| TinToken::FN(s.to_string(), boxed)),
        (r"↓", |s| TinToken::FN(s.to_string(), get)),
        (r"↑", |s| TinToken::FN(s.to_string(), set)),

        (r"∑", |s| TinToken::FN(s.to_string(), tin_sum_all)),
        (r"∏", |s| TinToken::FN(s.to_string(), tin_mul_all)),

        (r"⍴", |s| TinToken::FN(s.to_string(), tin_len)),

        (r"⌈", |s| TinToken::FN(s.to_string(), tin_max)),
        (r"⌊", |s| TinToken::FN(s.to_string(), tin_min)),

        (r"#", |s| TinToken::FN(s.to_string(), tin_count)),
        (r"º", |s| TinToken::FN(s.to_string(), tin_index)),

        // Functional array manipulation
        (r"`", |s| TinToken::FN(s.to_string(), drop_first)),
        (r"´", |s| TinToken::FN(s.to_string(), drop_last)),

        // IO
        (r"\$", |s| TinToken::FN(s.to_string(), tin_print))
    );

    return res_str.iter().map(|t| (Regex::new(t.0).unwrap(), t.1)).collect::<Vec<_>>();
}
use crate::wrappers;
use crate::interpreter::{*};

use regex::Regex;

fn tin_dup(_tok: String, _intrp: &mut TinInterpreter, _prog: &Vec<TinToken>, _ip: &mut usize, stack: &mut Vec<TinValue>) -> TinValue{
    stack.push(stack.last().cloned().unwrap());

    return TinValue::NONE;
}

fn tin_swap(_tok: String, _intrp: &mut TinInterpreter, _prog: &Vec<TinToken>, _ip: &mut usize, stack: &mut Vec<TinValue>) -> TinValue{
    let last_index = stack.len() - 1;
    stack.swap(last_index, last_index - 1);
    
    return TinValue::NONE;
}

fn tin_copy(_tok: String, _intrp: &mut TinInterpreter, _prog: &Vec<TinToken>, _ip: &mut usize, stack: &mut Vec<TinValue>) -> TinValue{
    if let TinValue::INT(n) = stack.pop().unwrap() {
        let item = stack.get(stack.len() - 1 - n as usize).cloned().unwrap();
        stack.push(item);
    
    } else {
        panic!();
    }
    
    return TinValue::NONE;
}

fn tin_define_var(tok: String, intrp: &mut TinInterpreter, _prog: &Vec<TinToken>, _ip: &mut usize, stack: &mut Vec<TinValue>) -> TinValue{
    let ctx = intrp.variables.entry(tok).or_insert(vec!());
    ctx.push(stack.pop().unwrap());

    return TinValue::NONE;
}

fn tin_delete_var(tok: String, intrp: &mut TinInterpreter, _prog: &Vec<TinToken>, _ip: &mut usize, _stack: &mut Vec<TinValue>) -> TinValue{
    let ctx = intrp.variables.get_mut(&tok).unwrap();
    ctx.pop();

    if ctx.len() == 0 {
        intrp.variables.remove(&tok);
    }

    return TinValue::NONE;
}

fn tin_get_var(tok: String, intrp: &mut TinInterpreter, _prog: &Vec<TinToken>, _ip: &mut usize, _stack: &mut Vec<TinValue>) -> TinValue{
    let ctx = &intrp.variables[&tok];

    return ctx.last().cloned().unwrap();
}

fn tin_foreach_init(_tok: String, intrp: &mut TinInterpreter, _prog: &Vec<TinToken>, ip: &mut usize, stack: &mut Vec<TinValue>) -> TinValue{
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

fn tin_foreach_end(_tok: String, intrp: &mut TinInterpreter, _prog: &Vec<TinToken>, ip: &mut usize, _stack: &mut Vec<TinValue>) -> TinValue{
    let (pos, arr, idx) = intrp.loop_stack.last().unwrap();

    if *idx < arr.len() - 1 {
        *ip = pos - 1;

    } else{
        intrp.loop_stack.pop();
    }

    return TinValue::NONE;
}

fn tin_storer_init(_tok: String, intrp: &mut TinInterpreter, _prog: &Vec<TinToken>, _ip: &mut usize, stack: &mut Vec<TinValue>) -> TinValue{
    intrp.storer_stack.push(stack.len());
    return TinValue::NONE;
}

fn tin_storer_end(_tok: String, intrp: &mut TinInterpreter, _prog: &Vec<TinToken>, _ip: &mut usize, stack: &mut Vec<TinValue>) -> TinValue{
    let idx = intrp.storer_stack.pop().unwrap();

    let arr = TinValue::VECTOR(stack.drain(idx..).collect::<Vec<_>>());
    stack.push(arr);

    return TinValue::NONE;
}

fn nabla(_tok: String, intrp: &mut TinInterpreter, prog: &Vec<TinToken>, _ip: &mut usize, stack: &mut Vec<TinValue>) -> TinValue{
    intrp.execute(prog, stack);

    return TinValue::NONE;
}

fn tin_gt(_tok: String, _intrp: &mut TinInterpreter, _prog: &Vec<TinToken>, _ip: &mut usize, stack: &mut Vec<TinValue>) -> TinValue{
    let a = stack.pop().unwrap();
    let b = stack.pop().unwrap();

    return wrappers::gt(&a, &b);
}

fn tin_sum(_tok: String, _intrp: &mut TinInterpreter, _prog: &Vec<TinToken>, _ip: &mut usize, stack: &mut Vec<TinValue>) -> TinValue{
    let a = stack.pop().unwrap();
    let b = stack.pop().unwrap();

    return wrappers::sum(&a, &b);
}

fn tin_sub(_tok: String, _intrp: &mut TinInterpreter, _prog: &Vec<TinToken>, _ip: &mut usize, stack: &mut Vec<TinValue>) -> TinValue{
    let a = stack.pop().unwrap();
    let b = stack.pop().unwrap();

    return wrappers::sub(&a, &b);
}

fn tin_mul(_tok: String, _intrp: &mut TinInterpreter, _prog: &Vec<TinToken>, _ip: &mut usize, stack: &mut Vec<TinValue>) -> TinValue{
    let a = stack.pop().unwrap();
    let b = stack.pop().unwrap();

    return wrappers::mul(&a, &b);
}

fn tin_div(_tok: String, _intrp: &mut TinInterpreter, _prog: &Vec<TinToken>, _ip: &mut usize, stack: &mut Vec<TinValue>) -> TinValue{
    let a = stack.pop().unwrap();
    let b = stack.pop().unwrap();

    return wrappers::div(&a, &b);
}

fn tin_mod(_tok: String, _intrp: &mut TinInterpreter, _prog: &Vec<TinToken>, _ip: &mut usize, stack: &mut Vec<TinValue>) -> TinValue{
    let a = stack.pop().unwrap();
    let b = stack.pop().unwrap();

    return wrappers::modl(&a, &b);
}

fn tin_all(_tok: String, _intrp: &mut TinInterpreter, _prog: &Vec<TinToken>, _ip: &mut usize, stack: &mut Vec<TinValue>) -> TinValue{
    return match stack.pop().unwrap(){
        TinValue::VECTOR(v) => TinValue::INT(v.iter().all(TinValue::truthy) as i64),

        _ => unreachable!()
    };
}

fn iota(_tok: String, _intrp: &mut TinInterpreter, _prog: &Vec<TinToken>, _ip: &mut usize, stack: &mut Vec<TinValue>) -> TinValue{
    return match stack.pop().unwrap() {
        TinValue::INT(a) => TinValue::VECTOR((0..a).map(TinValue::INT).collect::<Vec<_>>()),
        TinValue::FLOAT(a) => TinValue::VECTOR((0..a as i64).map(TinValue::INT).collect::<Vec<_>>()),

        _ => unreachable!()
    };
}

fn drop_first(_tok: String, _intrp: &mut TinInterpreter, _prog: &Vec<TinToken>, _ip: &mut usize, stack: &mut Vec<TinValue>) -> TinValue{
    return match stack.pop().unwrap() {
        TinValue::VECTOR(v) => TinValue::VECTOR(v[std::cmp::min(v.len(), 1)..].to_vec()),

        _ => unreachable!()
    };
}

fn tin_print(_tok: String, _intrp: &mut TinInterpreter, _prog: &Vec<TinToken>, _ip: &mut usize, stack: &mut Vec<TinValue>) -> TinValue{
    println!("{:?}", stack.pop().unwrap());

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

        (r"\{", |s| TinToken::FN(s.to_string(), tin_foreach_init)),
        (r"\}", |s| TinToken::FN(s.to_string(), tin_foreach_end)),
        (r"\(", |s| TinToken::FN(s.to_string(), tin_storer_init)),
        (r"\)", |s| TinToken::FN(s.to_string(), tin_storer_end)),

        (r"∇", |s| TinToken::FN(s.to_string(), nabla)),

        // Basic arithmetic
        (r"\+", |s| TinToken::FN(s.to_string(), tin_sum)),
        (r"\-", |s| TinToken::FN(s.to_string(), tin_sub)),
        (r"\*", |s| TinToken::FN(s.to_string(), tin_mul)),
        (r"/", |s| TinToken::FN(s.to_string(), tin_div)),
        (r"%", |s| TinToken::FN(s.to_string(), tin_mod)),

        // Logic
        (r">", |s| TinToken::FN(s.to_string(), tin_gt)),

        (r"∀", |s| TinToken::FN(s.to_string(), tin_all)),

        // Array operations
        (r"ι", |s| TinToken::FN(s.to_string(), iota)),

        // Functional array manipulation
        (r"`", |s| TinToken::FN(s.to_string(), drop_first)),
        //(r"´", |s| TinToken::FN(s.to_string(), drop_last)),

        // IO
        (r"\$", |s| TinToken::FN(s.to_string(), tin_print))
    );

    return res_str.iter().map(|t| (Regex::new(t.0).unwrap(), t.1)).collect::<Vec<_>>();
}